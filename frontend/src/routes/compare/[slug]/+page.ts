import type { PageLoad } from './$types';
import { getContentBySlug } from '$lib/seo/content/content-registry';
import { error } from '@sveltejs/kit';

export const prerender = true;

export const load: PageLoad = ({ params }) => {
	const entry = getContentBySlug(params.slug);
	if (!entry || entry.pageType !== 'compare') throw error(404, 'Comparison not found');
	return { entry };
};
