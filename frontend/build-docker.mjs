/**
 * Docker build script — bypasses vite.config.ts loading.
 *
 * vite uses esbuild to bundle vite.config.ts before importing it, which can
 * fail silently in Alpine Docker (esbuild binary issues). This script imports
 * vite and @sveltejs/kit directly from node_modules — no temp file, no esbuild
 * bundling of the config. Equivalent to running `vite build` with the same plugins.
 */
import { build } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

await build({
	configFile: false, // do NOT try to load vite.config.ts
	plugins: [sveltekit()],
	build: {
		outDir: 'build',
	},
});
