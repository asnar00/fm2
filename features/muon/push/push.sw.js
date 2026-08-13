// web push: show the notification even when the app is closed; tapping it
// opens (or focuses) the app
self.addEventListener('push', (e) => {
  let d = {};
  try { d = e.data ? e.data.json() : {}; } catch (err) {}
  e.waitUntil(self.registration.showNotification(d.title || 'muon', {
    body: d.body || '', icon: 'icon-192.png', badge: 'icon-192.png' }));
});
self.addEventListener('notificationclick', (e) => {
  e.notification.close();
  e.waitUntil(clients.matchAll({ type: 'window' }).then((list) =>
    list.length ? list[0].focus() : clients.openWindow('/')));
});
