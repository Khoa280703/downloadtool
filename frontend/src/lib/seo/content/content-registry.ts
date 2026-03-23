/**
 * Central registry of all content pages (guides + comparisons).
 * Adding a new page = adding a new entry here — no route boilerplate needed.
 * Existing 6 support pages are migrated here as the initial seed.
 */

import type { ContentEntry, ContentCategory, ContentPageType } from './content-types';

export const CONTENT_REGISTRY: ContentEntry[] = [
	// ─── HOW-TO GUIDES ───────────────────────────────────────────────────────

	{
		slug: 'how-to-use-snapvie',
		pageType: 'guide',
		category: 'how-to',
		intent: 'informational',
		locale: 'en',
		title: 'How to Use Snapvie — Complete Guide to Downloading YouTube Videos — Snapvie',
		metaDescription:
			'Step-by-step guide to using Snapvie: download YouTube videos, playlists, and Shorts at 1080p, 4K, or 8K HDR. No account needed. Free and ad-free.',
		h1: 'How to Use Snapvie — Complete Guide to Downloading YouTube Videos',
		subtitle: 'Download any YouTube video, playlist, or Short in seconds — free, no account needed',
		quickAnswer:
			'Copy a YouTube URL, paste it into snapvie.com, select your quality, and click Download. For videos above 480p, Snapvie processes a mux job server-side — then you save the finished file. No account, no software, no ads.',
		sections: [
			{
				heading: 'Step-by-step guide',
				type: 'steps',
				content:
					'<ol><li><strong>Find the YouTube video</strong> — navigate to the video, playlist, or Short you want to save and copy the URL.</li><li><strong>Copy the URL</strong> — click the address bar and copy the full URL (youtube.com/watch?v=..., /shorts/..., or /playlist?list=...).</li><li><strong>Paste into Snapvie</strong> — go to snapvie.com and paste the URL into the input field. Snapvie auto-detects the content type.</li><li><strong>Choose quality</strong> — select from 360p up to 4K/8K HDR. Higher qualities use server-side muxing and take slightly longer.</li><li><strong>Download the file</strong> — click Download. Direct downloads save immediately; mux jobs show a progress indicator before the save prompt.</li></ol>'
			},
			{
				heading: 'Understanding quality options',
				type: 'table',
				content:
					'<p>360p/480p = Direct (fast). 720p/1080p = Muxed (medium). 4K/8K HDR = Muxed (takes longer). "Muxed" means Snapvie fetches video and audio streams separately and merges them — required for anything above 480p.</p>'
			},
			{
				heading: 'Downloading playlists',
				type: 'text',
				content:
					'<p>Paste a playlist URL (contains <code>playlist?list=</code>) and Snapvie queues all videos automatically. Each is processed individually through the mux pipeline. You can save videos as they complete without waiting for the full playlist.</p>'
			},
			{
				heading: 'Audio-only downloads',
				type: 'text',
				content:
					'<p>Select the audio-only option in the quality picker. Snapvie extracts the native audio stream (typically 128–256kbps AAC) and saves it directly — no silent video file to deal with.</p>'
			}
		],
		faqItems: [
			{
				q: 'Do I need an account to use Snapvie?',
				a: 'No account required. Paste a URL, select quality, and download. No sign-up, no email, no login.'
			},
			{
				q: 'Is Snapvie free?',
				a: 'Yes. Snapvie is completely free to use with no ads on the download experience. There are no hidden fees or premium tiers for basic downloads.'
			},
			{
				q: 'What video qualities does Snapvie support?',
				a: 'Snapvie supports 360p, 480p, 720p, 1080p, 1440p, 4K (2160p), and 8K HDR — depending on what the source video offers. Audio-only (MP3) downloads are also available.'
			},
			{
				q: 'Why does my high-quality download take longer than a low-quality one?',
				a: "Qualities above 480p require muxing — Snapvie downloads the video and audio streams separately and merges them server-side. Low-quality downloads (360p, 480p) are direct and faster because they use YouTube's pre-combined stream."
			},
			{
				q: 'Can I download YouTube playlists with Snapvie?',
				a: "Yes. Paste a playlist URL (containing playlist?list=) and Snapvie queues all videos automatically. You can track each video's progress in real time."
			},
			{
				q: 'What formats does Snapvie output?',
				a: 'Video downloads are saved as MP4 for maximum compatibility. Audio-only downloads are saved as MP3 or AAC depending on the source stream.'
			}
		],
		relatedMoneyPage: 'download-youtube-4k',
		relatedSlugs: [
			'how-to-download-youtube-playlists',
			'download-youtube-shorts-with-audio',
			'why-youtube-downloads-need-muxing'
		],
		datePublished: '2026-03-23',
		dateModified: '2026-03-23'
	},

	{
		slug: 'how-to-download-youtube-playlists',
		pageType: 'guide',
		category: 'how-to',
		intent: 'informational',
		locale: 'en',
		title: 'How to Download YouTube Playlists (All Videos, Any Quality) — Snapvie',
		metaDescription:
			'Step-by-step guide to downloading entire YouTube playlists. Get all videos at 1080p, 4K, or 8K HDR using Snapvie — no account, no software, no ads.',
		h1: 'How to Download YouTube Playlists (All Videos, Any Quality)',
		subtitle: 'Queue an entire playlist and get every video at full quality',
		quickAnswer:
			'Paste the YouTube playlist URL into Snapvie, select your quality, and click download. Snapvie queues all videos, muxes each one server-side for full quality (up to 4K/8K HDR), and lets you save them one by one as they complete. No account, no software to install.',
		sections: [
			{
				heading: 'Step-by-step: downloading a full YouTube playlist',
				type: 'steps',
				content:
					'<ol><li><strong>Open the YouTube playlist</strong> — copy the URL from the address bar (it should contain "playlist?list=" followed by the playlist ID).</li><li><strong>Paste into Snapvie</strong> — go to snapvie.com and paste the playlist URL. Snapvie detects it automatically and fetches all video metadata.</li><li><strong>Select your quality</strong> — choose from the dropdown. Snapvie supports up to 4K and 8K HDR for each video in the playlist.</li><li><strong>Download all videos</strong> — click Download. Snapvie queues each video for processing. Monitor progress in real time and save files as they complete.</li></ol>'
			},
			{
				heading: 'Why most playlist downloaders deliver low quality',
				type: 'text',
				content:
					'<p>YouTube stores anything above 480p as a separate video-only stream with no audio. A downloader that does not handle muxing will fall back to 360p or 480p for every video. Snapvie\'s mux pipeline processes each video individually, combining video and audio streams into a single full-quality file server-side.</p>'
			},
			{
				heading: 'What to expect with large playlists',
				type: 'text',
				content:
					'<p>Playlists with dozens or hundreds of videos are processed in order. Each video goes through the mux pipeline separately, so overall time scales with the number of videos and their lengths. Videos become available to save as each one finishes — you do not have to wait for the entire playlist. Private or deleted videos are skipped automatically.</p>'
			}
		],
		faqItems: [
			{
				q: 'Can I download an entire YouTube playlist at once?',
				a: 'Yes. Paste the playlist URL into Snapvie and it will queue all videos for download automatically. You can monitor progress for each video in real time.'
			},
			{
				q: 'Is there a limit to how many videos I can download from a playlist?',
				a: "Snapvie handles playlists of any size. Very large playlists (hundreds of videos) are processed in batches. There's no artificial cap, though download time scales with the number of videos."
			},
			{
				q: 'What quality can I download playlist videos at?',
				a: 'You can select any quality Snapvie offers — up to 4K or 8K HDR for videos that were uploaded at those resolutions. Quality is per-video and depends on what the uploader provided.'
			},
			{
				q: 'Will Snapvie skip private or deleted videos in a playlist?',
				a: 'Yes. Private or deleted videos in a playlist are automatically skipped. Snapvie will process all publicly accessible videos and report any that could not be fetched.'
			},
			{
				q: 'Do I need an account to download playlists?',
				a: 'No account required. Paste the playlist URL and download. No sign-up, no login, no email.'
			}
		],
		relatedMoneyPage: 'download-youtube-playlist',
		relatedSlugs: [
			'how-to-use-snapvie',
			'why-youtube-downloads-show-360p-only',
			'why-youtube-downloads-need-muxing'
		],
		datePublished: '2026-03-23',
		dateModified: '2026-03-23'
	},

	{
		slug: 'download-youtube-shorts-with-audio',
		pageType: 'guide',
		category: 'how-to',
		intent: 'informational',
		locale: 'en',
		title: 'Download YouTube Shorts With Audio — Why Audio Goes Missing & How to Fix It — Snapvie',
		metaDescription:
			'Why does your downloaded YouTube Short have no sound? Learn why audio gets stripped and how Snapvie downloads Shorts with full audio intact.',
		h1: 'Download YouTube Shorts With Audio — Why Audio Goes Missing & How to Fix It',
		subtitle: 'Get Shorts with the original audio track — no silent files',
		quickAnswer:
			'Many YouTube Shorts store audio and video in separate streams, just like regular high-quality videos. Downloaders that only grab the video stream produce a silent file. Snapvie fetches both streams and merges them, so you always get a Short with its original audio.',
		sections: [
			{
				heading: 'Why downloaded Shorts often have no sound',
				type: 'text',
				content:
					'<p>YouTube Shorts use the same content delivery infrastructure as regular videos — including the split stream format where video and audio are stored separately at higher qualities. When a downloader fetches a Short, it usually requests the simplest available stream, which is often video-only. The audio is there; the downloader just did not fetch it.</p>'
			},
			{
				heading: 'How Snapvie keeps the audio',
				type: 'text',
				content:
					'<p>Snapvie explicitly downloads the audio stream alongside the video stream for every Short. The two are merged server-side using the same mux pipeline used for full-length videos. The process is automatic — paste the Short URL and Snapvie handles the rest.</p>'
			},
			{
				heading: 'Shorts vs. regular video downloads',
				type: 'text',
				content:
					'<p>From Snapvie\'s perspective, almost nothing differs. The URL format is different (<code>youtube.com/shorts/xxxx</code> vs. a standard watch URL), but Snapvie detects both automatically. Shorts are typically vertical (9:16) and capped at 1080p — you will get the full 1080p with audio.</p>'
			},
			{
				heading: 'What to do if your Short download is still silent',
				type: 'text',
				content:
					'<p>If you downloaded a Short using a different tool and got a silent file, re-download it with Snapvie. If you already have a silent MP4 and want to add audio back without re-downloading, you would need FFmpeg to mux in the audio stream manually. Snapvie avoids this entirely by doing the mux step during download.</p>'
			}
		],
		faqItems: [
			{
				q: 'Why does my downloaded YouTube Short have no audio?',
				a: 'Many YouTube Shorts use the same dual-stream format as regular videos — the audio is stored separately. Downloaders that only grab the video stream will produce a silent file. Snapvie downloads both streams and merges them so you always get audio.'
			},
			{
				q: 'Can I download YouTube Shorts at full quality?',
				a: 'Yes. Snapvie downloads Shorts at up to 1080p (the typical maximum for Shorts) with full audio. The same muxing pipeline used for regular videos handles Shorts automatically.'
			},
			{
				q: 'What format are downloaded Shorts saved in?',
				a: 'Snapvie saves Shorts as MP4 with H.264 video and AAC audio — the most compatible format for phones, computers, and video editors.'
			},
			{
				q: 'Do I need to do anything different to download a Short vs. a regular video?',
				a: 'Nothing different. Paste the Short URL (e.g. youtube.com/shorts/xxxx) into Snapvie the same way as any video URL. Snapvie detects it automatically.'
			}
		],
		relatedMoneyPage: 'download-youtube-shorts',
		relatedSlugs: [
			'why-youtube-downloads-need-muxing',
			'why-youtube-downloads-show-360p-only',
			'how-to-use-snapvie'
		],
		datePublished: '2026-03-23',
		dateModified: '2026-03-23'
	},

	// ─── TROUBLESHOOTING ─────────────────────────────────────────────────────

	{
		slug: 'why-youtube-downloads-show-360p-only',
		pageType: 'guide',
		category: 'troubleshooting',
		intent: 'informational',
		locale: 'en',
		title: 'Why YouTube Downloads Show 360p Only (And How to Get Full Quality) — Snapvie',
		metaDescription:
			'Why does your YouTube download default to 360p? Learn why quality is capped at 360p for most tools and how Snapvie delivers 1080p, 4K, and 8K HDR downloads.',
		h1: 'Why YouTube Downloads Show 360p Only (And How to Get Full Quality)',
		subtitle: 'The real reason most YouTube downloaders cap at 360p — and how to fix it',
		quickAnswer:
			'YouTube stores 1080p and higher as a video-only stream with no audio. Most downloaders can only grab the combined stream, which is capped at 360p or 480p. To get full quality you need a tool that merges the video and audio streams — called muxing. Snapvie does this automatically.',
		sections: [
			{
				heading: 'The real reason quality is capped at 360p',
				type: 'text',
				content:
					'<p>In 2015, YouTube changed how it delivers high-resolution video. Instead of encoding a single file with both video and audio, YouTube started storing them as two separate streams: a video-only stream (available at 1080p, 4K, 8K HDR) and an audio-only stream (128kbps or 256kbps). The old combined streams are still available, but only up to 480p — and for many videos just 360p. Most YouTube downloader tools grab the combined stream because it\'s simpler, ending up with a 360p file even when the video was uploaded in 4K.</p>'
			},
			{
				heading: 'What muxing is and why it matters',
				type: 'text',
				content:
					'<p>Muxing (multiplexing) is the process of combining a video stream and an audio stream into a single playable file. The process: (1) download the video-only stream at the quality you want, (2) download the audio-only stream separately, (3) merge them into a single MP4 file. This requires actual server-side processing — tools without this capability cannot offer anything above 480p.</p>'
			},
			{
				heading: 'Why Snapvie gets you the full quality',
				type: 'text',
				content:
					'<p>Snapvie was built specifically to handle muxing. When you paste a YouTube URL and select a quality above 480p, Snapvie fetches the high-resolution video stream and best available audio stream, runs both through a server-side Rust mux pipeline, and delivers a single clean file. No third-party software, no command line.</p>'
			},
			{
				heading: 'How to check if your download is really full quality',
				type: 'text',
				content:
					'<p>After downloading, right-click the file and open Properties (Windows) or Get Info (Mac) to see the video resolution. A true 1080p download will report 1920×1080. If you see 640×360, the tool gave you the low-quality combined stream. In VLC, press Ctrl+J (Windows) or Cmd+J (Mac) to see codec information including resolution and bitrate.</p>'
			}
		],
		faqItems: [
			{
				q: 'Why does my downloaded YouTube video only show 360p?',
				a: "YouTube stores high-quality video and audio in separate streams. Most simple downloaders only grab the combined stream, which is capped at 360p or 480p. To get 1080p or higher you need a tool that can merge the separate video and audio tracks — this process is called muxing."
			},
			{
				q: 'Why is 1080p not available in YouTube downloads?',
				a: "Since 2015, YouTube has served 1080p (and above) video as a video-only stream with no embedded audio. Simple downloaders that don't support muxing can't access these streams and fall back to 360p or 480p."
			},
			{
				q: 'How does Snapvie get the full quality?',
				a: 'Snapvie downloads the high-quality video stream and the separate audio stream, then merges them server-side using a muxing pipeline. The result is a single file with both video and audio at the quality you selected.'
			},
			{
				q: 'What is the highest quality I can download?',
				a: 'Snapvie supports up to 8K HDR (7680×4320) when the source video provides it. Most videos offer up to 1080p or 4K. Quality depends entirely on what the original uploader published.'
			}
		],
		relatedMoneyPage: 'download-youtube-4k',
		relatedSlugs: [
			'why-youtube-downloads-need-muxing',
			'best-format-for-youtube-downloads-mp4-vs-webm',
			'how-to-use-snapvie'
		],
		datePublished: '2026-03-23',
		dateModified: '2026-03-23'
	},

	// ─── EDUCATION ───────────────────────────────────────────────────────────

	{
		slug: 'why-youtube-downloads-need-muxing',
		pageType: 'guide',
		category: 'education',
		intent: 'informational',
		locale: 'en',
		title: 'Why YouTube Downloads Need Muxing — Video + Audio Streams Explained — Snapvie',
		metaDescription:
			'Why do YouTube downloads require muxing? Learn how YouTube stores video and audio separately, why 1080p+ needs merging, and how Snapvie handles it automatically.',
		h1: 'Why YouTube Downloads Need Muxing — Video + Audio Streams Explained',
		subtitle: 'The technical reason 1080p+ YouTube downloads require server-side stream merging',
		quickAnswer:
			'YouTube stores 1080p and higher video as a separate stream with no audio track. To get a playable file at full quality, a downloader must fetch the video stream and the audio stream separately, then merge (mux) them together. This is why high-quality downloads take a bit longer. Snapvie handles this automatically.',
		sections: [
			{
				heading: 'How YouTube delivers video — the DASH format',
				type: 'text',
				content:
					"<p>Since 2015, YouTube has used DASH (Dynamic Adaptive Streaming over HTTP) to deliver video. In DASH, video and audio are stored as independent streams at multiple quality levels. The legacy \"progressive\" streams (video + audio in one file) are still available, but only up to 480p for most videos — and 360p for many. Everything above that requires working with DASH streams.</p>"
			},
			{
				heading: 'What muxing actually does',
				type: 'steps',
				content:
					'<ol><li>Download the video-only DASH stream (e.g. 4K VP9)</li><li>Download the audio-only DASH stream (e.g. 256kbps Opus)</li><li>Mux both into a single MP4 container — video and audio are now synchronized and playable together</li></ol><p>When done losslessly, this process does not re-encode anything. The data from both streams is repackaged into the container without quality loss.</p>'
			},
			{
				heading: 'Why most downloaders skip muxing',
				type: 'text',
				content:
					"<p>Implementing muxing correctly requires server-side processing. A simple download tool can serve the user a direct URL to a file on YouTube's CDN — fast and cheap. Muxing requires actually downloading both streams, processing them, and serving the merged output. That needs real compute resources. This is why many free downloaders cap at 360p or 480p — they have no muxing infrastructure. Snapvie runs a Rust-based mux pipeline built specifically for this.</p>"
			},
			{
				heading: 'Does muxing affect quality?',
				type: 'text',
				content:
					'<p>No — when streams are compatible, muxing is lossless. The video frames and audio samples are repackaged without modification. Snapvie uses a lossless mux path for all supported quality tiers. In cases where the container requires remuxing (e.g. VP9 into MP4), only the container wrapper changes — not the codec data.</p>'
			}
		],
		faqItems: [
			{
				q: 'What is muxing in the context of YouTube downloads?',
				a: 'Muxing (multiplexing) is the process of combining separate video and audio streams into a single playable file. YouTube stores high-quality video (1080p and above) and audio as separate streams, so any tool that wants to give you the full quality needs to download and merge both.'
			},
			{
				q: 'Why does YouTube use separate streams?',
				a: "Separate streams are more efficient for adaptive streaming — YouTube's player can switch video quality independently from audio based on your connection speed. This DASH architecture has been standard on YouTube since 2015."
			},
			{
				q: "Why can't I just download the video without muxing?",
				a: 'You can download the video-only stream without muxing, but it will have no audio. You can also download a combined stream (which includes audio), but those are only available up to 480p. To get anything above 480p with audio, muxing is required.'
			},
			{
				q: 'Does muxing reduce quality?',
				a: 'When done correctly, muxing is lossless — it just repackages the streams into a new container without re-encoding. Snapvie uses a lossless mux path for compatible stream combinations, preserving the original quality exactly.'
			},
			{
				q: 'How long does muxing take?',
				a: "Mux time depends on video length and resolution. A 10-minute 1080p video typically muxes in under 30 seconds on Snapvie's pipeline. Longer or higher-resolution videos take more time. You can track progress in real time on the download page."
			}
		],
		relatedMoneyPage: 'download-youtube-4k',
		relatedSlugs: [
			'why-youtube-downloads-show-360p-only',
			'best-format-for-youtube-downloads-mp4-vs-webm',
			'how-to-use-snapvie'
		],
		datePublished: '2026-03-23',
		dateModified: '2026-03-23'
	},

	// ─── FORMAT & QUALITY ────────────────────────────────────────────────────

	{
		slug: 'best-format-for-youtube-downloads-mp4-vs-webm',
		pageType: 'guide',
		category: 'format-quality',
		intent: 'comparison',
		locale: 'en',
		title: 'Best Format for YouTube Downloads: MP4 vs WebM — Which Should You Use? — Snapvie',
		metaDescription:
			'MP4 vs WebM for YouTube downloads — which format is better? Compare compatibility, file size, quality, and editing support to pick the right format for your needs.',
		h1: 'Best Format for YouTube Downloads: MP4 vs WebM',
		subtitle: 'Choose the right container format for compatibility, file size, and editing',
		quickAnswer:
			'Use MP4 for maximum compatibility — it plays on every device, phone, TV, and editor without issues. Choose WebM if you want smaller file sizes and your playback device/software supports VP9 or AV1. For most people, MP4 is the right default.',
		sections: [
			{
				heading: 'What MP4 and WebM actually are',
				type: 'table',
				content:
					'<p>MP4 and WebM are container formats — the outer wrapper that holds the video and audio tracks. MP4 typically uses H.264 (AVC) or H.265 (HEVC) video with AAC audio. WebM typically uses VP9 or AV1 video with Opus or Vorbis audio. YouTube uses VP9 WebM internally for high-resolution streams (1080p and above).</p>'
			},
			{
				heading: 'Compatibility: MP4 wins clearly',
				type: 'text',
				content:
					'<p>MP4 with H.264 plays on every smartphone (iOS and Android), every smart TV and streaming device, all web browsers natively, and every video editing application. WebM has good support in Chrome and Firefox but can fail in Safari on older iOS/macOS versions, on many smart TVs, and in some editing software.</p>'
			},
			{
				heading: 'File size: WebM is more efficient',
				type: 'text',
				content:
					'<p>VP9 (used in WebM) is noticeably more efficient than H.264 at the same visual quality. At 4K and above, a VP9 WebM file may be 30–50% smaller than an equivalent H.264 MP4. AV1 is even more efficient than VP9, offering another 20–30% reduction. If storage is a concern and your playback setup supports WebM well, it is worth considering.</p>'
			},
			{
				heading: 'Video editing: stick with MP4',
				type: 'text',
				content:
					'<p>H.264 MP4 is supported natively by Adobe Premiere Pro, Final Cut Pro, DaVinci Resolve, iMovie, CapCut, and every consumer editor. VP9 WebM support in editing software is inconsistent — DaVinci Resolve requires a plugin for VP9 on Windows, Final Cut Pro does not support WebM at all natively.</p>'
			},
			{
				heading: 'Which format does Snapvie use?',
				type: 'text',
				content:
					"<p>Snapvie outputs MP4 by default. When it fetches YouTube's VP9 video stream and Opus audio stream, the mux pipeline re-wraps them into an MP4 container for broad compatibility. You get the quality of the original YouTube stream in a format that plays everywhere.</p>"
			}
		],
		faqItems: [
			{
				q: 'Should I download YouTube videos as MP4 or WebM?',
				a: 'MP4 is the safer default — it plays on everything and is accepted by all video editors. WebM offers slightly better compression at the same quality but has limited support on older devices and in some editing software. Use MP4 unless you have a specific reason to need WebM.'
			},
			{
				q: 'Does WebM have better quality than MP4?',
				a: 'Not exactly. WebM uses the VP9 or AV1 codec, which can achieve the same visual quality at a smaller file size compared to H.264 in MP4. For 4K and 8K content, the file size difference can be significant — WebM files are often 30-50% smaller. But visual quality at the same bitrate is comparable.'
			},
			{
				q: 'Which format does YouTube use internally?',
				a: "YouTube stores most high-resolution content (1080p and above) as VP9 WebM for video and Opus for audio. When you download using a tool that muxes streams, the output format depends on what the tool chooses as the container — Snapvie defaults to MP4 for maximum compatibility."
			},
			{
				q: 'Can I edit a WebM file in my video editor?',
				a: 'It depends on your editor. DaVinci Resolve, Adobe Premiere, and Final Cut Pro have varying levels of WebM/VP9 support. MP4 with H.264 is universally supported. If you plan to edit the downloaded file, MP4 is the safer choice.'
			},
			{
				q: 'What about AV1 — is it worth using?',
				a: "AV1 is newer than VP9 and offers even better compression. YouTube uses AV1 for some 4K+ content. However, hardware decoding support for AV1 is still limited on older devices, so unless you know your device supports it, stick with MP4 (H.264) or VP9 WebM."
			}
		],
		relatedMoneyPage: 'download-youtube-4k',
		relatedSlugs: [
			'why-youtube-downloads-need-muxing',
			'why-youtube-downloads-show-360p-only',
			'how-to-use-snapvie'
		],
		datePublished: '2026-03-23',
		dateModified: '2026-03-23'
	}
];

// ─── Query helpers ────────────────────────────────────────────────────────────

export function getContentBySlug(slug: string, locale = 'en'): ContentEntry | undefined {
	return CONTENT_REGISTRY.find((e) => e.slug === slug && e.locale === locale);
}

export function getContentByType(pageType: ContentPageType, locale = 'en'): ContentEntry[] {
	return CONTENT_REGISTRY.filter((e) => e.pageType === pageType && e.locale === locale);
}

export function getContentByCategory(category: ContentCategory, locale = 'en'): ContentEntry[] {
	return CONTENT_REGISTRY.filter((e) => e.category === category && e.locale === locale);
}
