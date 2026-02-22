import adapter from '@sveltejs/adapter-cloudflare';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		prerender: {
			entries: ['/'],
			handleMissingId: 'ignore'
		},
		alias: {
			$components: './src/components',
			$stores: './src/stores'
		}
	}
};

export default config;
