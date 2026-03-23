/**
 * llms.txt — AI Search readiness file for Snapvie.
 *
 * Follows the emerging llms.txt spec (https://llmstxt.org/) so AI assistants
 * (ChatGPT, Perplexity, Gemini, etc.) can understand the site's purpose,
 * capabilities, and key pages when crawling for context.
 */

import type { RequestHandler } from './$types';

const CONTENT = `# Snapvie
> Fast, free YouTube video downloader with 4K/8K HDR support

## About
Snapvie is a web-based tool for downloading YouTube videos in the highest available quality.
Supports 4K, 8K HDR, playlists, Shorts, and audio extraction.
No ads, no watermarks, no software installation required.

## Key Pages
- https://snapvie.com/ - Main downloader tool
- https://snapvie.com/download-youtube-4k - Download YouTube videos in 4K
- https://snapvie.com/download-youtube-8k-hdr - Download YouTube videos in 8K HDR
- https://snapvie.com/download-youtube-playlist - Download entire YouTube playlists
- https://snapvie.com/download-youtube-shorts - Download YouTube Shorts
- https://snapvie.com/download-youtube-mp3 - Extract audio from YouTube videos
- https://snapvie.com/guides - Download guides and tutorials
- https://snapvie.com/compare - YouTube downloader comparisons

## Capabilities
- Download YouTube videos up to 8K HDR quality
- Download entire playlists in batch
- Extract audio as MP3/M4A
- Download YouTube Shorts with audio
- Streaming mux technology (no disk usage, zero wait)
- 34 language support

## Contact
- Website: https://snapvie.com
- About: https://snapvie.com/about
- Contact: https://snapvie.com/contact
`;

export const GET: RequestHandler = () =>
	new Response(CONTENT, {
		headers: {
			'Content-Type': 'text/plain; charset=utf-8',
			'Cache-Control': 'public, max-age=86400'
		}
	});
