// the scanned road, in login.html's own script scope — so `$`, `post` and
// /enrol are the page's own, and a claimed code finishes down exactly the road
// a finished PIN entry does. With no `?t=` this fragment does nothing at all
// and the login page is untouched.
{
  const fm_insT = new URLSearchParams(location.search).get('t') || '';
  if (fm_insT) {
    const step = $("phoneStep");
    if (step) step.classList.add("hide");
    $("hint").textContent = "signing you in…";
    (async () => {
      const r = await post("/users/invite/instant/claim", { t: fm_insT });
      if (!r.ok) {
        $("hint").textContent = "";
        $("err").textContent = r.error || "this link isn't valid";
        return;
      }
      const who = await fetch("/auth/whoami", { cache: "no-store" })
        .then((x) => x.json()).catch(() => null);
      if (who && who.authed) {
        // this account has no number, so this device's credentials are the only
        // way back into it — enrolment matters more here, not less
        $("hint").textContent = "hello " + (r.name || "");
        if (typeof feature_Enrol !== "undefined") await feature_Enrol.run();
        location.replace("/?in=" + Date.now());
        return;
      }
      $("hint").textContent = "";
      $("err").textContent = "signed in, but this browser did not keep the cookie";
    })();
  }
}
