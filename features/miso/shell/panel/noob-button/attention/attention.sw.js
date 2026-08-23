// the foreground/background fork (#p18), made where the push actually lands.
// A notification is for someone who is not looking; if a window of this app is
// visible, the push is handed to that page — which flashes the lozenge, or has
// already updated in place — and nothing rings. With no visible window it
// rings exactly as it always did.
//
// The seam is `showNotification` rather than the push listener, because two
// push listeners would both fire: the wrap suppresses at the point of display
// without /push's own file knowing, and unticking this node restores it.
{
  const fm_attnShow = self.registration.showNotification.bind(self.registration);
  self.registration.showNotification = function (title, opts) {
    return clients.matchAll({ type: 'window', includeUncontrolled: true })
      .then((list) => {
        const seen = list.filter((c) => c.visibilityState === 'visible');
        if (!seen.length) return fm_attnShow(title, opts);
        for (const c of seen) {
          c.postMessage({ fm: 'attention', title: title,
                          body: (opts && opts.body) || '' });
        }
        return undefined;
      })
      // if the fork cannot be made, ring: a notification too many is a smaller
      // failure than an alert that never arrives
      .catch(() => fm_attnShow(title, opts));
  };
}
