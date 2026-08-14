const feature_Resume = {
  // by resume time messaging is long initialised, so pushing its queue
  // directly is race-free; the persistent outbox dedups and delivers.
  join() {
    if (typeof feature_Messaging === 'undefined' || !feature_Messaging.queue) return;
    if (feature_Messaging.queue.some((m) => m && m.type === 'Join')) return;
    feature_Messaging.queue.push({ type: 'Join' });
    feature_Messaging.save();
    feature_Messaging.flush();
  },
};
document.addEventListener('visibilitychange', () => {
  if (document.visibilityState === 'visible') feature_Resume.join();
});
window.addEventListener('online', () => feature_Resume.join());
