// Stream utilities for extension (mirrors apps/injector/src/shared/stream-utils.ts)

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

export interface MuxedPair {
  label: string;
  videoUrl: string;
  audioUrl: string;
  videoFormatId?: string;
  audioFormatId?: string;
  videoCodec?: string;
  audioCodec?: string;
}

/** Filter out WebM video-only streams not supported by fMP4 muxer */
export function filterDownloadableFormats(formats: StreamFormat[]): StreamFormat[] {
  return formats.filter((f) => {
    if (f.is_audio_only) return true;
    if (!f.has_audio && f.ext === 'webm') return false;
    return true;
  });
}

/** Pair each video-only stream with best audio stream */
export function buildMuxedPairs(formats: StreamFormat[]): MuxedPair[] {
  const videoStreams = formats.filter((f) => !f.is_audio_only && !f.has_audio);
  const audioStreams = formats.filter((f) => f.is_audio_only);
  const bestAudio = [...audioStreams].sort((a, b) => {
    const aRank = a.ext === 'm4a' || a.ext === 'mp4' ? 0 : 1;
    const bRank = b.ext === 'm4a' || b.ext === 'mp4' ? 0 : 1;
    if (aRank !== bRank) return aRank - bRank;
    return (b.bitrate ?? 0) - (a.bitrate ?? 0);
  })[0];
  if (!bestAudio) return [];

  return videoStreams.map((v) => ({
    label: v.quality,
    videoUrl: v.url,
    audioUrl: bestAudio.url,
    videoFormatId: v.format_id,
    audioFormatId: bestAudio.format_id,
    videoCodec: v.codec_label?.toLowerCase().includes('264') ? 'h264'
              : v.codec_label?.toLowerCase().includes('265') ? 'h265'
              : undefined,
    audioCodec: bestAudio.codec_label?.toLowerCase().includes('aac') ? 'aac'
              : bestAudio.codec_label?.toLowerCase().includes('opus') ? 'opus'
              : undefined,
  }));
}

/** Build app-domain launcher URL for mux-job flow. */
export function buildMuxJobLaunchUrl(
  apiBase: string,
  pair: MuxedPair,
  title: string,
  sourceUrl?: string,
): string {
  const params = new URLSearchParams({ video_url: pair.videoUrl, audio_url: pair.audioUrl, title });
  if (sourceUrl) params.set('source_url', sourceUrl);
  if (pair.videoFormatId) params.set('video_format_id', pair.videoFormatId);
  if (pair.audioFormatId) params.set('audio_format_id', pair.audioFormatId);
  return `${apiBase}/download/mux-job?${params}`;
}
