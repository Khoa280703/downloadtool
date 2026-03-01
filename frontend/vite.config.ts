import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 5168,
		host: true,
		proxy: {
			'/api/stream': 'http://127.0.0.1:3068',
			'/api/stream/muxed': 'http://127.0.0.1:3068',
			'/api/batch': 'http://127.0.0.1:3068',
			'/health': 'http://127.0.0.1:3068'
		},
		// Pre-transform critical modules on server start so first browser load is fast
		warmup: {
			clientFiles: [
				'./src/routes/+page.svelte',
				'./src/routes/+layout.svelte',
				'./src/components/*.svelte',
				'./src/lib/api.ts',
				'./src/lib/types.ts',
				'./src/stores/*.ts'
			]
		}
	}
});
