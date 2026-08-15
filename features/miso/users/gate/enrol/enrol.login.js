const feature_Enrol = {
  // first-login device setup: Face ID + notifications, automatically — both
  // ride the login tap's user activation; failures degrade into the app with
  // reasons logged, and the system panel buttons remain as retry.
  async run() {
    try {
      if (window.PublicKeyCredential && !localStorage.misoPasskey) {
        $("hint").textContent = "setting up Face ID…";
        const o = await post("/auth/passkey/register-challenge", {});
        if (o.ok) {
          const cred = await navigator.credentials.create({ publicKey: {
            challenge: fm_b64uToBuf(o.challenge),
            rp: { id: o.rp_id, name: "miso" },
            user: { id: fm_b64uToBuf(o.user_id), name: o.user_name,
                    displayName: o.user_name },
            pubKeyCredParams: [{ type: "public-key", alg: -7 }],
            authenticatorSelection: { authenticatorAttachment: "platform",
              residentKey: "required", userVerification: "required" },
            attestation: "none" } });
          const rr = await post("/auth/passkey/register", {
            id: cred.id,
            attestation: fm_bufToB64u(cred.response.attestationObject),
            client_data: fm_bufToB64u(cred.response.clientDataJSON) });
          if (rr.ok) { localStorage.misoPasskey = "1"; this.log("passkey enrolled"); }
          else this.log("passkey register failed: " + (rr.error || "?"));
        }
      }
    } catch (e) { this.log("passkey skipped: " + (e && e.name ? e.name : String(e))); }
    try {
      const standalone = matchMedia("(display-mode: standalone)").matches
        || navigator.standalone === true;
      if ("PushManager" in window && standalone && !localStorage.misoPush) {
        $("hint").textContent = "enabling notifications…";
        const reg = await Promise.race([
          navigator.serviceWorker.ready,
          new Promise((resolve, reject) =>
            setTimeout(() => reject(new Error("no sw")), 3000))]);
        const key = await fetch("/push/vapid-key").then((x) => x.text());
        const sub = await reg.pushManager.subscribe({
          userVisibleOnly: true, applicationServerKey: fm_b64uToBuf(key.trim()) });
        const rr = await post("/push/subscribe", {
          endpoint: sub.endpoint,
          p256dh: fm_bufToB64u(sub.getKey("p256dh")),
          auth: fm_bufToB64u(sub.getKey("auth")) });
        if (rr.ok) { localStorage.misoPush = "1"; this.log("push enrolled"); }
        else this.log("push subscribe failed: " + (rr.error || "?"));
      }
    } catch (e) { this.log("push skipped: " + (e && e.name ? e.name : String(e))); }
  },
  log: (m) => fetch("/diag/report", { method: "POST",
    body: JSON.stringify({ t: new Date().toISOString(), enrol: m }) }).catch(() => {}),
};
