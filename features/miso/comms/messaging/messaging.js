const feature_Messaging = {
  key: 'misoOutbox',
  queue: [],
  lastV: 0,
  flushing: false,
  replaying() {
    return typeof feature_Replay !== 'undefined' && feature_Replay.active;
  },
  load() {
    try { this.queue = JSON.parse(localStorage[this.key]) || []; } catch (e) {}
  },
  save() {
    try { localStorage[this.key] = JSON.stringify(this.queue); } catch (e) {}
  },
  // pull _send out of the loop's state into the persistent outbox
  drain() {
    try {
      const s = JSON.parse(feature_Loop.state || '{}');
      if (Array.isArray(s._send) && s._send.length) {
        for (const m of s._send) this.queue.push(m);
        delete s._send;
        feature_Loop.state = JSON.stringify(s);
        this.save();
        this.flush();
      }
    } catch (e) {}
  },
  // the seam for what a refused POST means for the message at the head of the
  // queue. The base answers false — keep it and wait, because the server may
  // be up or the cookie may be back next time, and order is the outbox's whole
  // promise. A later node may answer true for an answer that cannot succeed on
  // a retry, so the queue behind it is not held hostage.
  refused(status, msg) {
    return false;
  },
  // deliver FIFO; replies with a type become events. offline = stop and wait.
  async flush() {
    if (this.flushing || this.replaying()) return;
    this.flushing = true;
    while (this.queue.length) {
      try {
        const r = await fetch('/msg', { method: 'POST',
          body: JSON.stringify(this.queue[0]) });
        if (!r.ok) {
          if (!this.refused(r.status, this.queue[0])) break;
          this.queue.shift();
          this.save();
          continue;
        }
        const reply = await r.json();
        this.queue.shift();
        this.save();
        if (reply && reply.type) feature_Loop.send(reply);
      } catch (e) { break; }
    }
    this.flushing = false;
  },
  // the perpetual long-poll: broadcasts arrive as events
  async wait() {
    for (;;) {
      if (this.replaying()) {
        await new Promise((res) => setTimeout(res, 3000));
        continue;
      }
      try {
        const r = await fetch('/msg/wait', { method: 'POST',
          body: JSON.stringify({ since: this.lastV }) });
        if (!r.ok) throw new Error('wait ' + r.status);
        const b = await r.json();
        if (b.v) this.lastV = b.v;
        for (const m of (b.msgs || [])) {
          if (m && m.type) feature_Loop.send(m);
        }
      } catch (e) {
        await new Promise((res) => setTimeout(res, 3000));
      }
    }
  },
  init() {
    this.load();
    const orig = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      orig.call(this, p);
      self.drain();
    };
    setInterval(() => {
      if (document.visibilityState === 'visible') this.flush();
    }, 5000);
    window.addEventListener('online', () => this.flush());
    this.flush();
    this.wait();
  },
};
// comms linearises before loop, so install the wrap once feature_Loop exists
const fm_msgInit = setInterval(() => {
  if (typeof feature_Loop !== 'undefined' && feature_Loop.apply) {
    clearInterval(fm_msgInit);
    feature_Messaging.init();
  }
}, 50);
