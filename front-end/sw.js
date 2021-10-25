let dataCacheName = 'rust-lang.cc-datacache-v0.1.0'
let cacheName = 'rust-lang.cc-cache-v0.1.0'
let filesToCache = [
  '/',
  '/index.html',
  '/books.html',
  '/who_and_how.html',
  '/tailwind_purged.css',
  '/cn/index.html',
  '/cn/favicon.svg',
  '/cn/favicon.png',
  '/cn/css/variables.css',
  '/cn/css/general.css',
  '/cn/css/chrome.css',
  '/cn/css/print.css',
  '/cn/FontAwesome/css/font-awesome.css',
  '/cn/fonts/fonts.css',
  '/cn/highlight.css',
  '/cn/tomorrow-night.css',
  '/cn/ayu-highlight.css',
  '/cn/elasticlunr.min.js',
  '/cn/mark.min.js',
  '/cn/searcher.js',
  '/cn/clipboard.min.js',
  '/cn/highlight.js',
  '/cn/book.js',
  '/en/index.html',
  'pwa-icons/icon_144x144.png',
  'pwa-icons/icon_152x152.png',
  'pwa-icons/icon_192x192.png',
  'pwa-icons/icon_512x512.png'
]

self.addEventListener('install', function (e) {
  console.log('SW Install')
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      console.log('SW precaching')
      return cache.addAll(filesToCache)
    })
  )
  self.skipWaiting()
})

self.addEventListener('activate', function (e) {
  console.log('SW Activate')
  e.waitUntil(
    caches.keys().then(function (keyList) {
      return Promise.all(keyList.map(function (key) {
        if (key !== cacheName && key !== dataCacheName) {
          console.log('SW Removing old cache', key)
          return caches.delete(key)
        }
      }))
    })
  )
  return self.clients.claim()
})

self.addEventListener('fetch', function (e) {
  console.log('SW Fetch', e.request.url)
  // 如果数据相关的请求，需要请求更新缓存
  let dataUrl = '/mockData/'
  if (e.request.url.indexOf(dataUrl) > -1) {
    e.respondWith(
      caches.open(dataCacheName).then(function (cache) {
        return fetch(e.request).then(function (response){
          cache.put(e.request.url, response.clone())
          return response
        }).catch(function () {
          return caches.match(e.request)
        })
      })
    )
  } else {
    e.respondWith(
      caches.match(e.request).then(function (response) {
        return response || fetch(e.request)
      })
    )
  }
})