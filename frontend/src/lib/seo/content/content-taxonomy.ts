/**
 * Display metadata for content categories and hub pages.
 * Icons use Material Symbols identifiers (same convention as existing components).
 */

import type { ContentCategory, ContentHubConfig, ContentPageType } from './content-types';

export const CATEGORY_CONFIG: Record<
	ContentCategory,
	{ label: string; icon: string; description: string }
> = {
	'how-to': {
		label: 'How-To Guides',
		icon: 'book_open',
		description: 'Step-by-step tutorials'
	},
	troubleshooting: {
		label: 'Troubleshooting',
		icon: 'build',
		description: 'Fix common issues'
	},
	'format-quality': {
		label: 'Formats & Quality',
		icon: 'high_quality',
		description: 'Understanding video formats'
	},
	workflow: {
		label: 'Workflows',
		icon: 'route',
		description: 'Download workflows'
	},
	device: {
		label: 'Device Guides',
		icon: 'devices',
		description: 'Platform-specific guides'
	},
	comparison: {
		label: 'Comparisons',
		icon: 'compare',
		description: 'Tool comparisons'
	},
	'best-for': {
		label: 'Best For',
		icon: 'star',
		description: 'Best tools for specific needs'
	},
	education: {
		label: 'Learn',
		icon: 'school',
		description: 'Technical concepts explained'
	}
};

export const HUB_CONFIGS: Record<ContentPageType, ContentHubConfig> = {
	guide: {
		pageType: 'guide',
		title: 'YouTube Download Guides | Snapvie',
		metaDescription:
			'Step-by-step guides for downloading YouTube videos. Troubleshooting, format guides, device-specific tutorials.',
		h1: 'YouTube Download Guides',
		subtitle: 'Everything you need to know about downloading YouTube videos'
	},
	compare: {
		pageType: 'compare',
		title: 'YouTube Downloader Comparisons | Snapvie',
		metaDescription:
			'Honest comparisons of YouTube downloaders. Feature matrices, speed tests, quality analysis.',
		h1: 'YouTube Downloader Comparisons',
		subtitle: 'Find the best YouTube downloader for your needs'
	}
};
