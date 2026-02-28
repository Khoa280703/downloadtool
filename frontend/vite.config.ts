import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 5168,
		host: true,
		proxy: {
			'/api': 'http://127.0.0.1:3068'
		}
	}
});
