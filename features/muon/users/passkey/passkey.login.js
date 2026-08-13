const feature_Passkey = {
  async signIn() {
    const o = await post("/auth/passkey/challenge", {});
    if (!o.ok) throw new Error(o.error || "no challenge");
    const cred = await navigator.credentials.get({ publicKey: {
      challenge: fm_b64uToBuf(o.challenge), rpId: o.rp_id,
      userVerification: "required" } });
    const r = await post("/auth/passkey/login", {
      id: cred.id,
      auth_data: fm_bufToB64u(cred.response.authenticatorData),
      client_data: fm_bufToB64u(cred.response.clientDataJSON),
      signature: fm_bufToB64u(cred.response.signature) });
    if (!r.ok) throw new Error(r.error || "Face ID sign-in failed");
  },
};
const fm_faceidBtn = $("faceid");
if (fm_faceidBtn && window.PublicKeyCredential) fm_faceidBtn.style.display = "";
if (fm_faceidBtn) fm_faceidBtn.onclick = async () => {
  $("err").textContent = "";
  try {
    await feature_Passkey.signIn();
    localStorage.muonPasskey = "1";   // just proved one exists
    if (typeof feature_Enrol !== "undefined") await feature_Enrol.run();
    location.replace("/?in=" + Date.now());
  } catch (e) {
    $("err").textContent = (e && e.message) ? e.message : "Face ID cancelled or unavailable";
  }
};
