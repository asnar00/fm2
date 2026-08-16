const feature_MirroredWords = {
  // the announcer: transcripts not yet shared ride the persistent outbox.
  // misoWordsShared maps id -> highest grade announced from this device.
  key: 'misoWordsShared',
  shared: {},
  load() { try { this.shared = JSON.parse(localStorage[this.key]) || {}; } catch (e) {} },
  save() { try { localStorage[this.key] = JSON.stringify(this.shared); } catch (e) {} },

  scan() {
    if (feature_Messaging.replaying()) return;   // re-enactment sends no events
    let files = [];
    try { files = JSON.parse(feature_Loop.state || '{}').dict_files || []; } catch (e) { return; }
    let queued = false;
    for (const f of files) {
      if (!f || !f.id || !f.transcript || !(f.t_grade > 0)) continue;
      if ((this.shared[f.id] || 0) >= f.t_grade) continue;
      this.shared[f.id] = f.t_grade;
      feature_Messaging.queue.push({ type: 'TranscriptShared',
        data: { id: f.id, text: f.transcript, rung: f.t_rung, grade: f.t_grade } });
      queued = true;
    }
    if (queued) {
      this.save();
      feature_Messaging.save();
      feature_Messaging.flush();
    }
  },

  init() {
    this.load();
    const fm_wordsApply = feature_Loop.apply;
    const self = this;
    feature_Loop.apply = function (p) {
      fm_wordsApply.call(this, p);
      try { self.scan(); } catch (e) {}
    };
    window.addEventListener('online', () => self.scan());
    this.scan();
  },
};
const fm_wordsInit = setInterval(() => {
  if (typeof feature_Messaging !== 'undefined' && feature_Messaging.queue &&
      typeof feature_Loop !== 'undefined' && feature_Loop.apply) {
    clearInterval(fm_wordsInit);
    feature_MirroredWords.init();
  }
}, 100);
