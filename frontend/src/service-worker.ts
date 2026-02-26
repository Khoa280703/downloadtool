/// <reference types="@sveltejs/kit" />
/// <reference lib="webworker" />
// Service worker for PWA: offline cache + Background Fetch API

import { build, files, version } from '$service-worker';

declare const self: ServiceWorkerGlobalScope;

const CACHE_NAME = `videodl-${version}`;
const OFFLINE_URL = '/offline.html';

// Assets to precache on install (dedupe to avoid addAll duplicate requests)
const PRECACHE_ASSETS = Array.from(new Set([...build, ...files, OFFLINE_URL]));

// Install: precache app shell + offline page
self.addEventListener('install', (event) => {
  event.waitUntil(
    caches
      .open(CACHE_NAME)
      .then((cache) => cache.addAll(PRECACHE_ASSETS))
      .then(() => self.skipWaiting()),
  );
});

// Activate: delete old caches
self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(keys.filter((k) => k !== CACHE_NAME).map((k) => caches.delete(k))),
      )
      .then(() => self.clients.claim()),
  );
});

// Fetch: network-first; fall back to offline.html for navigation failures
self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET') return;

  // Skip API requests and stream URLs — always network
  const url = new URL(event.request.url);
  if (url.pathname.startsWith('/api/') || url.pathname.startsWith('/bm.js')) return;

  event.respondWith(
    fetch(event.request)
      .then((res) => {
        // Cache fresh navigation responses
        if (res.ok && event.request.mode === 'navigate') {
          const clone = res.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(event.request, clone));
        }
        return res;
      })
      .catch(async () => {
        // Navigation failure → serve offline page
        if (event.request.mode === 'navigate') {
          const cache = await caches.open(CACHE_NAME);
          return cache.match(OFFLINE_URL) ?? Response.error();
        }
        // Other failures → try cache
        return caches.match(event.request) ?? Response.error();
      }),
  );
});

// Background Fetch: listen for completion to notify user
self.addEventListener('backgroundfetchsuccess', (event: Event) => {
  const bgFetch = (event as BackgroundFetchUpdateUIEvent).registration;

  event.waitUntil(
    (async () => {
      const records = await bgFetch.matchAll();
      const firstRecord = records[0];
      if (!firstRecord) return;

      const response = await firstRecord.responseReady;
      const blob = await response.blob();
      const url = URL.createObjectURL(blob);

      // Notify all open clients to trigger download
      const clients = await self.clients.matchAll({ type: 'window' });
      for (const client of clients) {
        client.postMessage({ type: 'bg-fetch-complete', url, title: bgFetch.id });
      }

      await bgFetch.updateUI({ title: `Download complete: ${bgFetch.id}` });
    })(),
  );
});

self.addEventListener('backgroundfetchfail', (event: Event) => {
  const bgFetch = (event as BackgroundFetchUpdateUIEvent).registration;
  bgFetch.updateUI({ title: `Download failed: ${bgFetch.id}` });
});
