// the words under a long press are the tool's current words, kept here in
// one place: the card otherwise shows the registering node's own user
// paragraph, which describes the increment that node made, not the tool as
// it stands today (👤 said "a profile page is coming" a fortnight after the
// page came). A tool not listed here keeps its node's words.
const feature_ToolWords = {
  WORDS: {
    account: {
      name: 'people',
      intro: 'Your own page — name, picture, what you are here to do — and everyone whose card you hold, as a grid, a list, or a map. On the map, a person who has the app open right now stands where their phone is. The plus invites people: by name and number, or with a QR code for the room.',
    },
    invite: {
      name: 'invite',
      intro: 'Two ways in: type a name and number and they get a code by text, or show a QR code for the room and each person types their own. Under it, the people you have invited.',
    },
    posts: {
      name: 'posts',
      intro: 'The posts you hold — yours and the ones people you invited wrote — newest first, as pictures or a list. Tap + to write one, take a photo, or record a video from where you stand.',
    },
    projects: {
      name: 'projects',
      intro: 'The projects you are in — your own and the ones you have a part in. new makes one: a title, and what you are trying to get done.',
    },
    reports: {
      name: 'reports',
      intro: 'For support and above: your reports, newest first, with what each one asked and when it last answered.',
    },
    taps: {
      name: 'taps',
      intro: 'A shared counter everyone can tap. The first thing miso ever did, kept as the simplest way to see the app working across two phones.',
    },
    dictate: {
      name: 'dictate',
      intro: 'Tap the record button and talk; tap stop. What you said becomes a post, words and all.',
    },
  },
};
{
  if (typeof feature_LongPress !== 'undefined') {
    const fm_twContent = feature_LongPress.contentFor.bind(feature_LongPress);
    feature_LongPress.contentFor = async function (btn) {
      const got = await fm_twContent(btn);
      const id = (btn.getAttribute('data-ev') || '').replace(/^tool_/, '');
      const w = feature_ToolWords.WORDS[id];
      return w ? { name: w.name, intro: w.intro } : got;
    };
  }
}
