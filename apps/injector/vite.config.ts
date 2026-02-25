import { defineConfig } from 'vite';
import { resolve } from 'path';

// Default config (unused directly — see vite.bookmarklet.config.ts and vite.userscript.config.ts)
// Each IIFE entry must be built separately (Rollup limitation: IIFE + multiple inputs = error)
export default defineConfig({
  build: {
    outDir: 'dist',
    lib: {
      entry: resolve(__dirname, 'src/bookmarklet.ts'),
      name: 'DownloadTool',
      formats: ['iife'],
      fileName: () => 'bm.js',
    },
    minify: true,
    emptyOutDir: true,
  },
});
