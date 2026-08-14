// the Join is queued by init() through the state outbox; this nudge covers
// the boot race where the payload was applied before messaging wrapped
// apply — once boot state exists, drain the outbox so the Join ships now
// rather than on the next event.
const fm_joinInit = setInterval(() => {
  if (typeof feature_Messaging === 'undefined' ||
      typeof feature_Loop === 'undefined' || !feature_Loop.state) return;
  clearInterval(fm_joinInit);
  feature_Messaging.drain();
}, 100);
