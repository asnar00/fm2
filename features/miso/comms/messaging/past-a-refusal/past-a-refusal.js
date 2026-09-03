// a refusal the same bytes will always earn again: set that message aside and
// let the queue behind it move. 401/403 (the cookie arrives), 408/429 (the
// moment passes) and everything outside 4xx keep /messaging's wait-and-retry.
if (typeof feature_Messaging !== 'undefined') {
  // the bounded local record of what was dropped: last 10, oldest out. It
  // exists so the evidence survives a report that could not be posted; it
  // holds reports, never messages, so what it evicts is never data.
  feature_Messaging.DROPPED_KEY = 'misoDropped';
  feature_Messaging.DROPPED_KEEP = 10;
  feature_Messaging.dropped = function () {
    try {
      const v = JSON.parse(localStorage[this.DROPPED_KEY] || '[]');
      return Array.isArray(v) ? v : [];
    } catch (e) { return []; }
  };
  feature_Messaging.noteDropped = function (row) {
    try {
      const log = this.dropped();
      log.push(row);
      while (log.length > this.DROPPED_KEEP) log.shift();
      localStorage[this.DROPPED_KEY] = JSON.stringify(log);
    } catch (e) {}
  };
  feature_Messaging.refused = function (status, msg) {
    if (status < 400 || status > 499) return false;
    if (status === 401 || status === 403 || status === 408 || status === 429) {
      return false;
    }
    let size = 0;
    let type = '?';
    try {
      size = JSON.stringify(msg).length;
      type = (msg && msg.type) || '?';
    } catch (e) {}
    const row = { t: new Date().toISOString(), dropped: type, bytes: size,
                  status, behind: Math.max(0, (this.queue || []).length - 1) };
    this.noteDropped(row);
    if (typeof feature_Diag !== 'undefined' && feature_Diag.report) {
      feature_Diag.report(row);
    }
    return true;
  };
}
