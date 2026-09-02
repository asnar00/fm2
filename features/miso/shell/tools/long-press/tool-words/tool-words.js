// the words under a long press are the tool's current words, kept here in
// one place: the card otherwise shows the registering node's own user
// paragraph, which describes the increment that node made, not the tool as
// it stands today (👤 said "a profile page is coming" a fortnight after the
// page came). A tool's line says what it is FOR; what each button inside it
// does is that button's own line (ash, 2026-09-02). A tool or button not
// listed keeps its node's words.
const feature_ToolWords = {
  // tools: one line, the purpose
  TOOLS: {
    account: { name: 'people', intro: 'Your page, and everyone you hold.' },
    invite: { name: 'invite', intro: 'Bring someone in.' },
    posts: { name: 'posts', intro: 'What people have seen and said, newest first.' },
    projects: { name: 'projects', intro: 'What you are trying to get done, together.' },
    reports: { name: 'reports', intro: 'Questions asked of the whole picture, and their answers.' },
    taps: { name: 'taps', intro: 'A shared counter, the simplest proof two phones agree.' },
    dictate: { name: 'dictate', intro: 'Say it; it becomes a post.' },
    cards: { name: 'cards', intro: 'Everything you hold, in one place.' },
  },
  // the buttons inside tools and the view picker, by their event
  BUTTONS: {
    browse_grid: { name: 'grid', intro: 'Tiles: picture and title.' },
    browse_list: { name: 'list', intro: 'One line each, with the words.' },
    browse_map: { name: 'map', intro: 'Everything with a place, on the ground. People with the app open stand where their phone is.' },
    tools_home: { name: 'back', intro: 'Back to the toolbar.' },
    ctx_undo: { name: 'undo', intro: 'Take back the last thing you did here.' },
    posts_new: { name: 'new post', intro: 'Write one, take a photo, or record a video, from where you stand.' },
    posts_delete: { name: 'delete', intro: 'Remove this post. Undo brings it back.' },
    posts_promote: { name: 'promote', intro: 'Lift this post to the top.' },
    capture_photo: { name: 'photo', intro: 'A picture from the camera or your library.' },
    vid_rec: { name: 'record', intro: 'Start a video.' },
    vid_stop: { name: 'stop', intro: 'End the video; it becomes the post.' },
    vid_flip: { name: 'flip', intro: 'Front camera, back camera.' },
    dict_rec: { name: 'record', intro: 'Start talking.' },
    dict_stop: { name: 'stop', intro: 'Done; what you said becomes a post.' },
    oneadd_pick: { name: 'add', intro: 'Choose what to add to this post.' },
    tap_reset: { name: 'reset', intro: 'Back to zero, for everyone.' },
    tap_dec: { name: '−1', intro: 'One down.' },
    tap_double: { name: '×2', intro: 'Double it.' },
    tap_square: { name: 'square', intro: 'Multiply it by itself.' },
    proj_select: { name: 'select', intro: 'Make this the project you are working in.' },
    proj_open: { name: 'open', intro: 'Read the project.' },
    projects_delete: { name: 'delete', intro: 'Remove this project. Undo brings it back.' },
  },
  words(ev) {
    if (!ev) return null;
    if (ev.startsWith('tool_')) return this.TOOLS[ev.slice(5)] || null;
    const base = ev.split(':')[0];
    return this.BUTTONS[ev] || this.BUTTONS[base] || null;
  },
};
{
  if (typeof feature_LongPress !== 'undefined') {
    const fm_twContent = feature_LongPress.contentFor.bind(feature_LongPress);
    feature_LongPress.contentFor = async function (btn) {
      const w = feature_ToolWords.words(btn.getAttribute('data-ev') || '');
      if (w) return { name: w.name, intro: w.intro };
      return fm_twContent(btn);
    };
    // the view picker's buttons (grid, list, map) are not tool buttons, so
    // /sub-tool-cards does not arm them; arm them the same way
    document.addEventListener('pointerdown', (e) => {
      const btn = e.target && e.target.closest ? e.target.closest('.browse-view[data-ev]') : null;
      if (!btn) return;
      feature_LongPress.disarm();
      feature_LongPress.fired = false;
      feature_LongPress.armed = btn;
      feature_LongPress.x = e.clientX;
      feature_LongPress.y = e.clientY;
      feature_LongPress.timer = setTimeout(() => feature_LongPress.show(btn), 500);
    });
    document.addEventListener('click', (e) => {
      if (feature_LongPress.fired && e.target && e.target.closest && e.target.closest('.browse-view[data-ev]')) {
        e.stopPropagation();
        e.preventDefault();
        feature_LongPress.fired = false;
      }
    }, true);
  }
}
