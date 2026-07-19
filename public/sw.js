// Ultralearn service worker - offline installability.
// Strategy: cache-first for the app shell (hashed assets), network-first for
// navigation so the SPA always boots, falling back to the cached shell.
// Guarded so that if this file is ever injected as a normal page script
// (e.g. by a bundler), it is a harmless no-op.
if (typeof ServiceWorkerGlobalScope !== "undefined" && self instanceof ServiceWorkerGlobalScope) {
  const CACHE = "ultralearn-v1";
  const SHELL = ["/", "/index.html", "/favicon.svg", "/manifest.webmanifest"];

  self.addEventListener("install", (event) => {
    event.waitUntil(
      caches.open(CACHE).then((cache) => cache.addAll(SHELL)).then(() => self.skipWaiting())
    );
  });

  self.addEventListener("activate", (event) => {
    event.waitUntil(
      caches.keys().then((keys) =>
        Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k)))
      ).then(() => self.clients.claim())
    );
  });

  self.addEventListener("fetch", (event) => {
    const req = event.request;
    if (req.method !== "GET") return;

    if (req.mode === "navigate") {
      event.respondWith(
        fetch(req).catch(() => caches.match("/index.html").then((r) => r || caches.match("/")))
      );
      return;
    }

    event.respondWith(
      caches.match(req).then((cached) => {
        if (cached) return cached;
        return fetch(req).then((res) => {
          if (res && res.status === 200 && res.type === "basic") {
            const copy = res.clone();
            caches.open(CACHE).then((cache) => cache.put(req, copy));
          }
          return res;
        });
      })
    );
  });
}
