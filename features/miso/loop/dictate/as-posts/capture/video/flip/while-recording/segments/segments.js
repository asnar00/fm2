// a flip mid-take closes the clip and opens another on the other camera, and
// the mini joins the pieces into one file afterwards.
//
// /while-recording drew the camera onto a canvas and recorded THAT, so the
// recorder never saw a track change. It works, and it costs a copy of every
// frame for the whole take and resamples the camera onto a second clock — when
// the phone is busy the animation tick slips and the same frame is captured
// twice, which reads as a stutter. Ash asked for the other road (#p139): record
// the camera natively, cut a new recording at the flip, join afterwards.
//
// So this node stands the canvas down through the seam its parent opened, and
// takes the flip itself.
const feature_Segments = {
  // a container shorter than this is not worth making: MediaRecorder will
  // write one, but a fraction of a second of mp4 is a header and little else,
  // and ffmpeg's concat demuxer is entitled to dislike it. Under it, the
  // segment is dropped and the take starts clean on the new camera.
  MIN_MS: 300,

  marks: [],        // the part index at which each container starts: [0, 7, …]
  segStart: 0,      // when the current segment's recorder was started
  swapping: false,

  // ---- the camera, natively ------------------------------------------------
  // the recorder is handed /video's own stream, which is what it recorded
  // before /while-recording existed. Its canvas, its draw loop and its source
  // element are never made, and `drawing` stays false so its own flip watcher
  // does nothing — this node's takes its place.
  begin() {
    this.marks = [0];
    this.segStart = Date.now();
    // a new take starts clean whatever the last one left behind. A cut that
    // never finished — the camera prompt was up and `getUserMedia` waited
    // behind it while the take was stopped — would otherwise leave this set
    // and every flip of every later take would be ignored. Rig-found,
    // 2026-09-04.
    this.swapping = false;
    // and the camera this take opened on, so the first paint is not read as a
    // flip: the watcher compares against it.
    feature_WhileRecording.facing = feature_WhileRecording.wanted();
    return feature_Video.media;
  },

  // ---- the flip ------------------------------------------------------------
  // noticed at the paint, exactly as /while-recording noticed it: the answer
  // the whole chain gives for the camera, whoever composed it.
  onApply() {
    if (this.swapping) return;
    if (typeof feature_Video === 'undefined' || !feature_Video.recorder) return;
    if (feature_Video.recorder.state !== 'recording') return;
    const want = feature_WhileRecording.wanted();
    if (!want || want === feature_WhileRecording.facing) return;
    this.cut(want);
  },

  // wait for a recorder to have handed over its last piece. `onstop` fires
  // after the final `dataavailable`, so this resolves with every byte of the
  // segment already in `chunks` and already posted by /streams.
  settled(rec) {
    return new Promise((done) => {
      let over = false;
      const end = () => { if (!over) { over = true; done(); } };
      rec.onstop = end;
      rec.onerror = end;
      setTimeout(end, 4000);          // a recorder that never stops is not a reason to hang
      try { rec.stop(); } catch (e) { end(); }
    });
  },

  async cut(facing) {
    const v = feature_Video;
    this.swapping = true;
    let next = null;
    try {
      // the new camera FIRST: if it refuses, the take carries on untouched and
      // nothing has been closed.
      next = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: facing }, audio: false });
    } catch (e) {
      feature_WhileRecording.facing = feature_WhileRecording.wanted();
      this.swapping = false;
      return;
    }
    // the camera can take a while to answer — a permission sheet stands in
    // front of it the first time — and the take may be over by the time it
    // does. Nothing is closed in that case and the camera just opened is let
    // go again.
    if (!v.recorder || v.recorder.state !== 'recording') {
      next.getTracks().forEach((t) => t.stop());
      this.swapping = false;
      return;
    }
    const short = Date.now() - this.segStart < this.MIN_MS;
    const rec = v.recorder;
    await this.settled(rec);

    // a segment too short to be a container is thrown away and the take starts
    // clean on the new camera. Nothing has been posted yet — the timeslice is
    // two seconds and this is under a third of one — so there is nothing on
    // the exchange to contradict.
    if (short) {
      v.chunks = [];
      this.marks = [0];
      if (typeof feature_Streams !== 'undefined') { feature_Streams.count = 0; }
    } else {
      this.marks.push(v.chunks.length);
    }

    // the microphone never moves: it is the same microphone whichever way the
    // camera points, and a second one would be a second voice in the room.
    const audio = v.media ? v.media.getAudioTracks() : [];
    const tracks = [next.getVideoTracks()[0]];
    if (audio.length) tracks.push(audio[0]);
    const stream = new MediaStream(tracks);

    // release the camera we came off — /video's own stream keeps its
    // microphone, which the new recorder is still holding.
    const old = feature_WhileRecording.cam;
    if (old === v.media) {
      old.getVideoTracks().forEach((t) => t.stop());
    } else if (old) {
      old.getTracks().forEach((t) => t.stop());
    }
    feature_WhileRecording.cam = next;
    feature_WhileRecording.facing = facing;
    feature_WhileRecording.extra.push(next);   // its teardown releases them

    try {
      v.recorder = new MediaRecorder(stream, v.opts());
      // /video's own handler, rebuilt: the part number keeps counting across
      // segments, so /streams' `parts` and the server's join loop see one
      // unbroken run of pieces whatever happened in the middle.
      v.recorder.ondataavailable = (e) => {
        if (!e.data.size) return;
        v.chunks.push(e.data);
        v.onChunk(e.data, v.chunks.length - 1);
      };
      v.recorder.onstop = () => v.save();
      v.recorder.start(v.timeslice());
      this.segStart = Date.now();
    } catch (e) {
      // no recorder: the take is over rather than silently dead
      feature_Loop.send({ type: 'click', ev: 'vid_stop' });
    }

    // the viewfinder follows at once — a flip you cannot see is a flip that
    // lies about which way the phone is pointing
    if (v.view) {
      v.view.srcObject = stream;
      const p = v.view.play(); if (p && p.catch) p.catch(() => {});
    }
    this.swapping = false;
  },

  // ---- what the mini is told -----------------------------------------------
  // the marks ride on the clip's own metadata, beside /streams' `parts`, so
  // they reach the exchange on the RecShared that announces the recording and
  // land on its index with everything else. One segment is no marks at all:
  // the server then joins the pieces by concatenation, exactly as today.
  metaFor(meta) {
    if (this.marks.length < 2) return meta;
    return Object.assign({}, meta, { segs: this.marks.slice() });
  },

  // ---- what this phone plays -----------------------------------------------
  // the local copy is every piece run together, which is several containers in
  // one file: a player reads the first and stops. That is the first segment,
  // and it is honest — but it is not the take. Once the mini has joined the
  // pieces, `blob/<id>` is the whole thing, so the next play of a multi-segment
  // clip asks for it and keeps it. If the exchange has not joined it yet (or
  // is not reachable) the first segment is what plays, and the next play tries
  // again. Nothing is ever deleted: a take always has something to show.
  installFetch() {
    if (typeof feature_Dictate === 'undefined' || feature_Dictate.fm_segments) return false;
    feature_Dictate.fm_segments = true;
    const orig = feature_Dictate.getBlob.bind(feature_Dictate);
    feature_Dictate.getBlob = async (id) => {
      const local = await orig(id);
      if (!local) return local;
      let meta = null;
      try { meta = await feature_Dictate.getBlob('meta:' + id); } catch (e) {}
      if (!meta || !meta.segs || meta.segs.length < 2 || meta.joined) return local;
      try {
        const r = await fetch('blob/' + id);
        if (!r.ok) return local;
        const whole = await r.blob();
        // any file at all under this id is the joined one: the exchange only
        // writes `blob/<id>` once ffmpeg has succeeded, and 404s until then.
        // Its SIZE is not the test — a re-mux has one header where the pieces
        // had four, so the joined file is often the smaller of the two.
        if (!whole || !whole.size) return local;
        await feature_Dictate.put(id, whole);
        await feature_Dictate.put('meta:' + id, Object.assign({}, meta, { joined: true }));
        return whole;
      } catch (e) { return local; }
    };
    return true;
  },

  install() {
    if (typeof feature_WhileRecording === 'undefined' || typeof feature_Video === 'undefined') {
      return false;
    }
    // /streams REPLACES metaFor rather than wrapping it, and installs off an
    // interval of its own — so this one waits for it before wrapping, or there
    // is a race in which the marks are quietly dropped. With /streams not
    // composed there is nothing to wait for.
    if (typeof feature_Streams !== 'undefined' && !feature_Video.fm_streams) {
      return false;
    }
    if (feature_WhileRecording.fm_segments) return true;
    feature_WhileRecording.fm_segments = true;
    const self = this;
    // the canvas road stands down: the parent's own seam, answered with the
    // camera's stream and nothing built.
    feature_WhileRecording.begin = function () { return self.begin(); };
    feature_WhileRecording.onApply = function () { self.onApply(); };
    const fm_segMeta = feature_Video.metaFor.bind(feature_Video);
    feature_Video.metaFor = function (meta) { return self.metaFor(fm_segMeta(meta)); };
    self.installFetch();
    return true;
  },
};

{
  let fm_segTries = 0;
  const fm_segInit = setInterval(() => {
    fm_segTries = fm_segTries + 1;
    if (feature_Segments.install() || fm_segTries > 100) clearInterval(fm_segInit);
  }, 100);
}
