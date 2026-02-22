//! Frame queue with backpressure for VRAM management
//!
//! This module provides a bounded channel for frames to prevent
//! VRAM exhaustion when the encoder is slower than the decoder.

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use bytes::Bytes;
use tracing::{debug, trace, warn};

/// Default capacity for the frame queue.
/// Set to 8 frames to balance throughput vs VRAM usage.
/// At 1080p, each frame uses ~4MB VRAM (NV12 format).
pub const DEFAULT_FRAME_QUEUE_CAPACITY: usize = 8;

/// Maximum concurrent GPU jobs to prevent NVENC session exhaustion.
/// RTX 3090 supports up to 8 concurrent NVENC sessions.
/// We use 6 to leave headroom for other processes.
pub const DEFAULT_MAX_CONCURRENT_JOBS: usize = 6;

/// A decoded video frame in the queue.
#[derive(Debug, Clone)]
pub struct QueuedFrame {
    /// Frame data (NV12 format in CPU RAM, not VRAM)
    pub data: Bytes,
    /// Presentation timestamp
    pub pts: i64,
    /// Frame width
    pub width: i32,
    /// Frame height
    pub height: i32,
    /// Pixel format (AVPixelFormat value)
    pub format: i32,
}

/// Bounded frame queue with backpressure.
///
/// Uses a tokio mpsc channel with limited capacity. When the queue
/// is full, the decoder will block, preventing VRAM exhaustion.
pub struct FrameQueue {
    sender: mpsc::Sender<QueuedFrame>,
    receiver: mpsc::Receiver<QueuedFrame>,
    capacity: usize,
}

impl FrameQueue {
    /// Create a new frame queue with the specified capacity.
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        Self {
            sender,
            receiver,
            capacity,
        }
    }

    /// Get the sender handle for pushing frames.
    pub fn sender(&self) -> mpsc::Sender<QueuedFrame> {
        self.sender.clone()
    }

    /// Get the receiver handle for popping frames.
    pub fn receiver(&mut self) -> &mut mpsc::Receiver<QueuedFrame> {
        &mut self.receiver
    }

    /// Get the queue capacity.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the number of frames currently in the queue.
    pub fn len(&self) -> usize {
        self.sender.max_capacity() - self.sender.capacity()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if the queue is full.
    pub fn is_full(&self) -> bool {
        self.sender.capacity() == 0
    }
}

impl Default for FrameQueue {
    fn default() -> Self {
        Self::new(DEFAULT_FRAME_QUEUE_CAPACITY)
    }
}

/// GPU job semaphore for limiting concurrent transcoding operations.
///
/// Prevents NVENC session exhaustion by limiting the number of
/// concurrent transcoding jobs per GPU.
pub struct GpuSemaphore {
    semaphore: Arc<Semaphore>,
    max_permits: usize,
}

impl GpuSemaphore {
    /// Create a new GPU semaphore with the specified number of permits.
    pub fn new(max_permits: usize) -> Self {
        debug!("Creating GPU semaphore with {} permits", max_permits);
        Self {
            semaphore: Arc::new(Semaphore::new(max_permits)),
            max_permits,
        }
    }

    /// Acquire a permit from the semaphore.
    ///
    /// Returns None if no permits are available (GPU at capacity).
    pub fn try_acquire(&self) -> Option<tokio::sync::SemaphorePermit> {
        match self.semaphore.try_acquire() {
            Ok(permit) => {
                trace!("Acquired GPU permit, remaining: {}", self.semaphore.available_permits());
                Some(permit)
            }
            Err(_) => {
                warn!("GPU at capacity, no permits available");
                None
            }
        }
    }

    /// Acquire a permit, waiting if necessary.
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit {
        let permit = self.semaphore.acquire().await.expect("Semaphore closed");
        trace!("Acquired GPU permit (after wait), remaining: {}", self.semaphore.available_permits());
        permit
    }

    /// Get the number of available permits.
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Get the maximum number of permits.
    pub fn max_permits(&self) -> usize {
        self.max_permits
    }

    /// Check if the GPU has available capacity.
    pub fn has_capacity(&self) -> bool {
        self.semaphore.available_permits() > 0
    }
}

impl Default for GpuSemaphore {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_CONCURRENT_JOBS)
    }
}

impl Clone for GpuSemaphore {
    fn clone(&self) -> Self {
        Self {
            semaphore: Arc::clone(&self.semaphore),
            max_permits: self.max_permits,
        }
    }
}

/// VRAM usage tracker for monitoring GPU memory.
pub struct VramTracker {
    max_vram_bytes: u64,
    current_vram_bytes: std::sync::atomic::AtomicU64,
}

impl VramTracker {
    /// Create a new VRAM tracker with the specified limit.
    pub fn new(max_vram_bytes: u64) -> Self {
        Self {
            max_vram_bytes,
            current_vram_bytes: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Allocate VRAM and return true if successful.
    pub fn allocate(&self, bytes: u64) -> bool {
        let current = self.current_vram_bytes.load(std::sync::atomic::Ordering::Relaxed);
        let new = current + bytes;
        if new > self.max_vram_bytes {
            return false;
        }
        self.current_vram_bytes
            .fetch_add(bytes, std::sync::atomic::Ordering::Relaxed);
        true
    }

    /// Free allocated VRAM.
    pub fn free(&self, bytes: u64) {
        self.current_vram_bytes
            .fetch_sub(bytes, std::sync::atomic::Ordering::Relaxed);
    }

    /// Get current VRAM usage in bytes.
    pub fn current_usage(&self) -> u64 {
        self.current_vram_bytes.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Get maximum allowed VRAM in bytes.
    pub fn max_vram(&self) -> u64 {
        self.max_vram_bytes
    }

    /// Check if VRAM is available for allocation.
    pub fn can_allocate(&self, bytes: u64) -> bool {
        let current = self.current_vram_bytes.load(std::sync::atomic::Ordering::Relaxed);
        current + bytes <= self.max_vram_bytes
    }
}

impl Default for VramTracker {
    fn default() -> Self {
        // Default to 4GB max VRAM per job
        Self::new(4 * 1024 * 1024 * 1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_queue_capacity() {
        let queue = FrameQueue::new(4);
        assert_eq!(queue.capacity(), 4);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_gpu_semaphore() {
        let sem = GpuSemaphore::new(2);
        assert_eq!(sem.max_permits(), 2);
        assert!(sem.has_capacity());

        let permit1 = sem.try_acquire();
        assert!(permit1.is_some());
        assert_eq!(sem.available_permits(), 1);

        let permit2 = sem.try_acquire();
        assert!(permit2.is_some());
        assert_eq!(sem.available_permits(), 0);
        assert!(!sem.has_capacity());

        // Third acquire should fail
        let permit3 = sem.try_acquire();
        assert!(permit3.is_none());
    }

    #[test]
    fn test_vram_tracker() {
        let tracker = VramTracker::new(1000);
        assert_eq!(tracker.max_vram(), 1000);
        assert!(tracker.can_allocate(500));
        assert!(tracker.allocate(500));
        assert_eq!(tracker.current_usage(), 500);

        assert!(tracker.can_allocate(400));
        assert!(!tracker.can_allocate(600));

        tracker.free(200);
        assert_eq!(tracker.current_usage(), 300);
    }

    #[tokio::test]
    async fn test_frame_queue_send_recv() {
        let mut queue = FrameQueue::new(2);
        let sender = queue.sender();

        let frame = QueuedFrame {
            data: Bytes::from(vec![1, 2, 3]),
            pts: 0,
            width: 1920,
            height: 1080,
            format: 0,
        };

        sender.send(frame.clone()).await.unwrap();
        assert_eq!(queue.len(), 1);

        let received = queue.receiver().recv().await.unwrap();
        assert_eq!(received.pts, 0);
        assert_eq!(received.data, vec![1, 2, 3]);
    }
}
