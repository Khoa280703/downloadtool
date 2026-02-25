import { defineConfig } from 'vite';
import { resolve } from 'path';

// Default config: builds content-script.ts → dist/unpacked/content-script.js (IIFE)
export default defineConfig({
  define: {
    __API_BASE__: JSON.stringify(process.env.API_BASE ?? 'https://your-domain.com'),
  },
  build: {
    outDir: 'dist/unpacked',
    emptyOutDir: true,
    lib: {
      entry: resolve(__dirname, 'src/content-script.ts'),
      name: 'DownloadToolContentScript',
      formats: ['iife'],
      fileName: () => 'content-script.js',
    },
    minify: true,
  },
});
