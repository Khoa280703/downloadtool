import { defineConfig } from 'vite';
import monkey from 'vite-plugin-monkey';

// Builds src/userscript.ts → dist/youtube-downloader.user.js with proper ==UserScript== header
// Uses vite-plugin-monkey (NOT vite-plugin-banner — ESBuild strips comments)
export default defineConfig({
  plugins: [
    monkey({
      entry: 'src/userscript.ts',
      userscript: {
        name: 'YouTube Downloader',
        namespace: 'https://yourdomain.com',
        version: '1.0.0',
        description: 'Download YouTube videos in the best quality',
        match: ['https://www.youtube.com/watch*', 'https://youtube.com/watch*'],
        grant: ['GM_xmlhttpRequest'],
        connect: ['yourdomain.com'],
        'run-at': 'document-idle',
        updateURL: 'https://yourdomain.com/userscript',
        downloadURL: 'https://yourdomain.com/userscript',
      },
      build: {
        fileName: 'youtube-downloader.user.js',
      },
    }),
  ],
  build: {
    outDir: 'dist',
    emptyOutDir: false, // Don't wipe bm.js from previous build step
  },
});
