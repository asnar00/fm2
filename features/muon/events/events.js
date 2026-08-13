const feature_Events = {
  instance: null,
  state: null,
  // load the wasm, apply the boot payload, wire one delegated listener:
  // any element with data-ev sends its event through the Rust update chain
  async boot() {
    const res = await fetch('client.wasm');
    const { instance } = await WebAssembly.instantiate(await res.arrayBuffer(), {});
    this.instance = instance;
    this.apply(this.read(instance.exports.fm_entry()));
    document.addEventListener('click', (e) => {
      const el = e.target.closest('[data-ev]');
      if (el) this.send({ type: 'click', ev: el.getAttribute('data-ev') });
    });
  },
  send(event) {
    const input = JSON.stringify({ state: this.state, event });
    const bytes = new TextEncoder().encode(input);
    const ptr = this.instance.exports.fm_alloc(bytes.length);
    new Uint8Array(this.instance.exports.memory.buffer, ptr, bytes.length).set(bytes);
    this.apply(this.read(this.instance.exports.fm_event(ptr, bytes.length)));
  },
  read(packed) {
    const ptr = Number(packed >> 32n), len = Number(packed & 0xFFFFFFFFn);
    return new TextDecoder().decode(
      new Uint8Array(this.instance.exports.memory.buffer, ptr, len));
  },
  apply(payloadJson) {
    const p = JSON.parse(payloadJson);
    this.state = p.state;
    $('app').innerHTML = p.html;
  },
};
