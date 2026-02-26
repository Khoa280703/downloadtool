/**
 * Shared TypeScript types for video extractors
 */

/**
 * Video stream representation
 */
export interface Stream {
  /** Direct stream URL */
  url: string;
  /** Quality label (e.g., "1080p", "720p") */
  quality: string;
  /** Format identifier (e.g., "mp4", "webm") */
  format: string;
  /** MIME type */
  mime: string;
  /** Bitrate in bits per second */
  bitrate?: number;
  /** Video codec string (e.g., "avc1.64001F, mp4a.40.2") */
  codec?: string;
  /** Human-readable codec label (e.g., "H.264", "VP9", "AV1", "AAC") */
  codecLabel?: string;
  /** True if stream contains an audio track */
  hasAudio: boolean;
  /** True if stream is audio-only (no video) */
  isAudioOnly: boolean;
  /** Width in pixels */
  width?: number;
  /** Height in pixels */
  height?: number;
  /** File size in bytes if known */
  filesize?: number;
}

/**
 * Result of a successful extraction
 */
export interface ExtractionResult {
  /** Available video streams */
  streams: Stream[];
  /** Video title */
  title: string;
  /** Channel/author name */
  channel?: string;
  /** View count */
  viewCount?: number;
  /** Platform identifier */
  platform: string;
  /** Thumbnail URL */
  thumbnail?: string;
  /** Duration in seconds */
  duration?: number;
  /** Video description */
  description?: string;
}

/**
 * Extractor function type
 */
export type ExtractFn = (url: string, cookies?: string) => Promise<ExtractionResult>;

/**
 * Platform identifier
 */
export type Platform = "youtube" | "unknown";

/**
 * Error thrown during extraction
 */
export class ExtractionError extends Error {
  constructor(
    message: string,
    public readonly platform: Platform,
    public readonly cause?: unknown
  ) {
    super(message);
    this.name = "ExtractionError";
  }
}
