// the tour never starts for a greeted person: the Rust marks tour_seen on
// done, but that write reaches the state one frame late (/payload's older
// link), and the tour would take that frame. Told here, on the tap itself.
const feature_LastWord = { active: true };
document.addEventListener('pointerdown', (e) => {
  const go = e.target && e.target.closest ? e.target.closest('#greetSheet .greet-go') : null;
  if (!go || typeof feature_Tour === 'undefined') return;
  feature_Tour.at = -2;
  try { localStorage.misoTourSeen = '1'; } catch (err) {}
}, true);
