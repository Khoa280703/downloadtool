import type { PageLoad } from './$types';
import { getContentByType } from '$lib/seo/content/content-registry';
import { HUB_CONFIGS } from '$lib/seo/content/content-taxonomy';

export const prerender = true;

export const load: PageLoad = () => {
	const entries = getContentByType('compare');
	const config = HUB_CONFIGS.compare;
	return { entries, config };
};
