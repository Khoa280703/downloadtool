/** Source of truth for all public indexable pages — used by sitemap and robots. */

export interface PublicPage {
	path: string;
	priority: number;
	changefreq: 'daily' | 'weekly' | 'monthly';
	lastmod?: string; // ISO date
}

export const SITE_URL = 'https://snapvie.com';

export const PUBLIC_PAGES: PublicPage[] = [
	{ path: '/', priority: 1.0, changefreq: 'weekly' },
	{ path: '/download-youtube-8k-hdr', priority: 0.8, changefreq: 'monthly' },
	{ path: '/download-youtube-playlist', priority: 0.8, changefreq: 'monthly' },
	{ path: '/download-youtube-shorts', priority: 0.8, changefreq: 'monthly' },
	{ path: '/download-youtube-4k', priority: 0.8, changefreq: 'monthly' },
	{ path: '/download-youtube-mp3', priority: 0.8, changefreq: 'monthly' },
	{ path: '/about', priority: 0.4, changefreq: 'monthly' },
	{ path: '/contact', priority: 0.4, changefreq: 'monthly' },
	{ path: '/terms', priority: 0.3, changefreq: 'monthly' },
	{ path: '/privacy', priority: 0.3, changefreq: 'monthly' },
	{ path: '/dmca', priority: 0.2, changefreq: 'monthly' },
	// Supporting content cluster — informational / pain-point pages
	{ path: '/why-youtube-downloads-show-360p-only', priority: 0.6, changefreq: 'monthly' },
	{ path: '/how-to-download-youtube-playlists', priority: 0.6, changefreq: 'monthly' },
	{ path: '/download-youtube-shorts-with-audio', priority: 0.6, changefreq: 'monthly' },
	{ path: '/best-format-for-youtube-downloads-mp4-vs-webm', priority: 0.6, changefreq: 'monthly' },
	{ path: '/why-youtube-downloads-need-muxing', priority: 0.6, changefreq: 'monthly' },
	{ path: '/how-to-use-snapvie', priority: 0.6, changefreq: 'monthly' }
];
