# vonage
*real SMS delivery via the Vonage account*

> (transcripts/2026-08-13-fm-spec.md#p40)
> ../ftr does user login using my vonage account - take a peek in there

## spec

Extends `send_sms` with real delivery through Vonage (`rest.nexmo.com/sms/json`), using the credentials already configured on the mini in `~/.agent-config.json` (`{ "vonage": { "api_key", "api_secret", "from_number" } }`) — the same account ftr uses. The HTTP call shells out to `curl` (stdlib Rust has no TLS). Missing credentials fall back to the previous chain link — console delivery — so dev machines work without setup.

## user

Nothing to do if `~/.agent-config.json` has vonage credentials; codes arrive by text. Without credentials, codes print to the server log (console fallback).

## glossary

(no new terms)

## code description

`vonage.rs` is a single `send_sms` /extension/. It reads the credentials from `~/.agent-config.json`, and falls back to the previous chain link — console delivery — via `existing.send_sms(to, text)` when they're absent.

With credentials, it POSTs the form-encoded fields using `curl --data-urlencode` and checks `messages[0].status == "0"` in the JSON reply, returning `""` on success or an error string.
