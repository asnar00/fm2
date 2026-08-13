const feature_Diag = {
  report: (data) => fetch('/diag/report', { method: 'POST',
    body: JSON.stringify({ t: new Date().toISOString(), ...data }) }).catch(() => {}),
};
window.onerror = (msg, src, line, col) =>
  feature_Diag.report({ error: String(msg), at: `${src}:${line}:${col}` });
