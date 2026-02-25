// Utilities for filtering and building stream URLs

/** A stream format returned by POST /api/extract */
export interface StreamFormat {
  format_id: string;
  quality: string;
  ext: string;
  url: string;
  has_audio: boolean;
  is_audio_only: boolean;
  codec_label?: string;
  bitrate?: number;
  filesize?: number;
}

/** A pairing of video + audio stream for muxed download */
export interface MuxedPair {
  label: string;
  videoUrl: string;
  audioUrl: string;
  videoCodec?: string;
  audioCodec?: string;
}

/**
 * Filter out WebM video-only streams (not supported by fMP4 muxer).
 * Keep: H.264/H.265 video-only, audio-only (any codec), and streams with audio.
 */
export function filterDownloadableFormats(formats: StreamFormat[]): StreamFormat[] {
  return formats.filter((f) => {
    // Always include audio-only streams
    if (f.is_audio_only) return true;
    // Exclude WebM video-only (VP9/AV1 without audio)
    if (!f.has_audio && f.ext === 'webm') return false;
    return true;
  });
}

/**
 * Build muxed video+audio pairs for the quality picker.
 * Pairs each video-only stream with the best available audio stream.
 */
export function buildMuxedPairs(formats: StreamFormat[]): MuxedPair[] {
  const videoStreams = formats.filter((f) => !f.is_audio_only && !f.has_audio);
  const audioStreams = formats.filter((f) => f.is_audio_only);

  // Pick best audio: highest bitrate AAC preferred, then any
  const bestAudio = audioStreams.sort((a, b) => (b.bitrate ?? 0) - (a.bitrate ?? 0))[0];
  if (!bestAudio) return [];

  return videoStreams.map((v) => ({
    label: v.quality,
    videoUrl: v.url,
    audioUrl: bestAudio.url,
    videoCodec: v.codec_label?.toLowerCase().includes('264') ? 'h264'
              : v.codec_label?.toLowerCase().includes('265') ? 'h265'
              : undefined,
    audioCodec: bestAudio.codec_label?.toLowerCase().includes('aac') ? 'aac'
              : bestAudio.codec_label?.toLowerCase().includes('opus') ? 'opus'
              : undefined,
  }));
}

/** Build the GET /api/stream/muxed URL for a given pair and title */
export function buildMuxedUrl(apiBase: string, pair: MuxedPair, title: string): string {
  const params = new URLSearchParams({
    video_url: pair.videoUrl,
    audio_url: pair.audioUrl,
    title,
  });
  if (pair.videoCodec) params.set('video_codec', pair.videoCodec);
  if (pair.audioCodec) params.set('audio_codec', pair.audioCodec);
  return `${apiBase}/api/stream/muxed?${params.toString()}`;
}
