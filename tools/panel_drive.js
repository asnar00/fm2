// Full-stack verification rig: drives the REAL app (dev server on 8095)
// over Chrome DevTools Protocol using Node's built-in WebSocket — no
// puppeteer. Logs in as _test via the actual PIN flow (reads the PIN from
// /tmp/fm2-devserver.log), taps the real nøøb button, asserts panel
// behaviour DOM-level: hidden at boot, rows mounted, list scrolls, order,
// in-place expansion, reader open/close. Born during the #p78-#p81 panel
// debugging (it caught the env()-in-calc height bug and the display:flex
// boot leak that headless-wasm tests cannot see).
//
// Usage: start the dev server (products/miso/build: ./server/target/release/
// miso_server > /tmp/fm2-devserver.log), start Chrome:
//   chrome --headless=new --remote-debugging-port=9222 --user-data-dir=/tmp/x about:blank
// then: node tools/panel_drive.js
const CHROME_HTTP = 'http://127.0.0.1:9222';
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
(async () => {
  const targets = await (await fetch(CHROME_HTTP + '/json')).json();
  const page = targets.find((t) => t.type === 'page');
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise((r) => { ws.onopen = r; });
  let id = 0; const pending = {};
  ws.onmessage = (m) => { const d = JSON.parse(m.data); if (d.id && pending[d.id]) pending[d.id](d); };
  const send = (method, params = {}) => new Promise((res) => {
    const i = ++id; pending[i] = res; ws.send(JSON.stringify({ id: i, method, params }));
  });
  const evl = async (expr) => {
    const r = await send('Runtime.evaluate', { expression: expr, returnByValue: true, awaitPromise: true });
    return r.result && r.result.result ? r.result.result.value : undefined;
  };
  await send('Page.enable');
  await send('Runtime.enable');
  await send('Page.navigate', { url: 'http://127.0.0.1:8095/' });
  await sleep(2500);
  if ((await evl('location.pathname')).includes('login')) {
    console.log('logging in as _test…');
    await evl(`fetch('auth/request', { method: 'POST', body: JSON.stringify({ phone: '+15550001111' }) }).then(r => r.json())`);
    await sleep(700);
    const fs = await import('fs');
    const log = fs.readFileSync('/tmp/fm2-devserver.log', 'utf8');
    const pins = [...log.matchAll(/test user _test pin (\d+)/g)];
    const pin = pins.length ? pins[pins.length - 1][1] : '';
    console.log('pin from log:', pin ? 'found' : 'MISSING');
    const v = await evl(`fetch('auth/verify', { method: 'POST', body: JSON.stringify({ phone: '+15550001111', pin: '${pin}' }) }).then(r => r.json()).then(j => JSON.stringify(j))`);
    console.log('verify:', v);
    await send('Page.navigate', { url: 'http://127.0.0.1:8095/' });
    await sleep(1500);
  }
  for (let i = 0; i < 300; i++) {
    if (await evl(`!!document.querySelector('.toolbar')`)) break;
    await sleep(100);
  }
  if (!await evl(`!!document.querySelector('.toolbar')`)) {
    console.log('BOOT-FAIL');
    console.log('href=', await evl('location.href'));
    console.log('ready=', await evl('document.readyState'));
    console.log('app=', await evl(`document.getElementById('app') ? document.getElementById('app').innerHTML.slice(0,120) : 'NO #app'`));
    console.log('scripts=', await evl('document.scripts.length'));
    process.exit(1);
  }
  await sleep(500);
  console.log('BOOT', await evl(`(() => {
    const p = document.getElementById('panel');
    return 'panelHiddenAtBoot=' + (p.offsetParent === null && getComputedStyle(p).display === 'none');
  })()`));
  await evl(`document.getElementById('build').click()`);
  await sleep(2000);
  console.log('PANEL', await evl(`(() => {
    const c = document.getElementById('changes');
    const p = document.getElementById('panel');
    const who = document.querySelector('#panel .who') || document.getElementById('who');
    const order = [...p.querySelectorAll('*')].filter(x => x.id === 'who' || x.id === 'changes' || x.id === 'policySeg' || x.id === 'logoutBtn').map(x => x.id).join('>');
    return 'rows=' + (c ? c.querySelectorAll('.crow').length : -1)
      + ' home=' + (c && c.classList.contains('chooser-home'))
      + ' teaser=' + (c ? c.querySelectorAll('.change').length : -1)
      + ' panelH=' + p.offsetHeight + '/' + innerHeight
      + ' panelOnScreen=' + (p.getBoundingClientRect().top >= 0)
      + ' listH=' + c.clientHeight + ' listScrolls=' + (c.scrollHeight > c.clientHeight)
      + ' order=' + order
      + ' buttons=[' + [...document.querySelectorAll('#panel button')].map(b => b.textContent.trim()).join(',') + ']';
  })()`));
  await evl(`document.querySelector('#changes .crow[data-path]').click()`);
  await sleep(400);
  console.log('TAP', await evl(`(() => {
    const c = document.getElementById('changes');
    const r = document.getElementById('chooserRead');
    return 'morebox=' + !!c.querySelector('.cmore[style*="block"]')
      + ' readerAfterRowTap=' + (r && r.style.display === 'flex');
  })()`));
  await evl(`document.querySelector('#changes .cintro').click()`);
  await sleep(800);
  console.log('READER', await evl(`(() => {
    const r = document.getElementById('chooserRead');
    const open1 = r.style.display === 'flex';
    document.getElementById('chooserDismiss').click();
    return 'opensOnIntro=' + open1;
  })()`));
  await sleep(300);
  console.log('DISMISS', await evl(`'closed=' + (document.getElementById('chooserRead').style.display !== 'flex')`));
  process.exit(0);
})().catch((e) => { console.log('CDP-ERR', String(e).slice(0, 200)); process.exit(1); });
