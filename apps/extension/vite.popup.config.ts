import { defineConfig } from 'vite';
import { resolve } from 'path';

// Builds popup.ts → dist/unpacked/popup/popup.js (IIFE)
// popup.html and popup.css are copied by build-extension.sh
export default defineConfig({
  define: {
    __API_BASE__: JSON.stringify(process.env.API_BASE ?? 'https://your-domain.com'),
  },
  build: {
    outDir: 'dist/unpacked/popup',
    emptyOutDir: false,
    lib: {
      entry: resolve(__dirname, 'src/popup/popup.ts'),
      name: 'DownloadToolPopup',
      formats: ['iife'],
      fileName: () => 'popup.js',
    },
    minify: true,
  },
});
