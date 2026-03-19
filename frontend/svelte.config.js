import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter(),
		alias: {
			$components: './src/components',
			$stores: './src/stores'
		},
		prerender: {
			// Landing pages linked from homepage may not exist yet during parallel phase builds.
			// adapter-node serves them dynamically at runtime — 404s here are safe to ignore.
			handleHttpError: 'warn'
		}
	}
};

export default config;
