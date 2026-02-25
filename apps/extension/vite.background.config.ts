import { defineConfig } from 'vite';
import { resolve } from 'path';

// Builds background.ts → dist/unpacked/background.js (IIFE)
export default defineConfig({
  define: {
    __API_BASE__: JSON.stringify(process.env.API_BASE ?? 'https://your-domain.com'),
  },
  build: {
    outDir: 'dist/unpacked',
    emptyOutDir: false, // Don't wipe content-script.js
    lib: {
      entry: resolve(__dirname, 'src/background.ts'),
      name: 'DownloadToolBackground',
      formats: ['iife'],
      fileName: () => 'background.js',
    },
    minify: true,
  },
});
