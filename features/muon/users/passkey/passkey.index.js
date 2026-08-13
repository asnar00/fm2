const feature_Passkey = {
  // panel-side: offer enrolment when the platform supports passkeys and this
  // device hasn't got one (retry path for first-login setup)
  offerEnrol() {
    if (!$('passkeyRow')) return;
    $('passkeyRow').style.display =
      (window.PublicKeyCredential && !localStorage.muonPasskey) ? '' : 'none';
  },
  async enrol() {
    const o = await fetch('auth/passkey/register-challenge', { method: 'POST' })
      .then((r) => r.json());
    if (!o.ok) throw new Error(o.error || 'no challenge');
    const cred = await navigator.credentials.create({ publicKey: {
      challenge: fm_b64uToBuf(o.challenge),
      rp: { id: o.rp_id, name: 'muon' },
      user: { id: fm_b64uToBuf(o.user_id), name: o.user_name,
              displayName: o.user_name },
      pubKeyCredParams: [{ type: 'public-key', alg: -7 }],
      authenticatorSelection: { authenticatorAttachment: 'platform',
        residentKey: 'required', userVerification: 'required' },
      attestation: 'none' } });
    const r = await fetch('auth/passkey/register', { method: 'POST',
      body: JSON.stringify({
        id: cred.id,
        attestation: fm_bufToB64u(cred.response.attestationObject),
        client_data: fm_bufToB64u(cred.response.clientDataJSON) }) })
      .then((x) => x.json());
    if (!r.ok) throw new Error(r.error || 'register failed');
    localStorage.muonPasskey = '1';
  },
};
const fm_passkeyBtn = $('passkeyBtn');
if (fm_passkeyBtn) fm_passkeyBtn.onclick = async () => {
  try {
    await feature_Passkey.enrol();
    $('passkeyBtn').textContent = 'Face ID enabled ✓';
    setTimeout(() => { $('passkeyRow').style.display = 'none'; }, 1500);
  } catch (e) {
    if (typeof feature_Diag !== 'undefined')
      feature_Diag.report({ error: 'passkey enrol: ' + (e && e.message ? e.message : String(e)) });
    $('passkeyBtn').textContent = 'Face ID setup failed — try again';
  }
};
