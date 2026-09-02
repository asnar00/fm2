// the live pin once a second: the phone reports its position every second
// while the app is in front, and a map that is up asks every second. The
// two constants are /live's own; this node sets them at load (a property
// replacement, the idiom every child here uses). The server's sixty-second
// life for an entry is untouched — a phone that goes quiet still leaves
// within the minute. Cost: one small POST a second per phone in front, one
// GET a second per open map; the store is memory and answers in
// microseconds. Battery: getCurrentPosition with maximumAge lets the phone
// answer from its last fix when nothing moved.
if (typeof feature_Live !== 'undefined') {
  feature_Live.BEAT_MS = 1000;
  feature_Live.POLL_MS = 1000;
}
