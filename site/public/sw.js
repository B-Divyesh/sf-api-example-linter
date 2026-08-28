const CACHE = 'api-example-linter-v2';
const SHELL = ['/', '/index.html', '/demo/', '/privacy/', '/terms/', '/404.html', '/assets/contract-loom.webp', '/assets/contract-loom-social.png', '/assets/inter-latin.woff2', '/assets/jetbrains-mono-latin.woff2', '/favicon.svg', '/apple-touch-icon.png', /* BUILD_ASSETS */];
self.addEventListener('install', event => event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(SHELL))));
self.addEventListener('activate', event => event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key !== CACHE).map(key => caches.delete(key))))));
self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET' || new URL(event.request.url).origin !== location.origin) return;
  event.respondWith(caches.match(event.request).then(cached => cached || fetch(event.request).then(response => {
    if (response.ok) {
      const copy = response.clone(); caches.open(CACHE).then(cache => cache.put(event.request, copy));
    }
    return response;
  }).catch(() => event.request.mode === 'navigate' ? caches.match('/index.html') : undefined)));
});
