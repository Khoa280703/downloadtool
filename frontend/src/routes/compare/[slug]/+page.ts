import type { PageLoad } from './$types';
import { getContentBySlug } from '$lib/seo/content/content-registry';
import { error } from '@sveltejs/kit';

// Disable prerendering until compare entries are added to content-registry.ts
export const prerender = false;

export const load: PageLoad = ({ params }) => {
	const entry = getContentBySlug(params.slug);
	if (!entry || entry.pageType !== 'compare') throw error(404, 'Comparison not found');
	return { entry };
};
