# off-argv
*Vonage credentials off the process list*

> (transcripts/2026-08-23-fm-spec.md#p3)
> yeah, fix all the weaknesses

## user

No visible change — texts still send the same way. The Vonage api key and secret (and the login code itself) no longer show up in the machine's process list while a text is going out.

## spec

The base `/vonage` passed the api key, the api secret, and the message text — which carries the login PIN — as `curl --data-urlencode` arguments, so any local `ps` during the ~1s send read them straight off the argv. This node hands curl the whole request as a `-K -` config file on stdin instead: only `curl -s -K -` is on the argv, and nothing sensitive touches disk. Sending behaviour, response parsing, and the no-credentials console fallback are unchanged.

## code description

`off-argv.rs` redefines `/vonage`'s `send_sms`: it reads the same `~/.agent-config.json`, and when credentials are present builds a curl config (`url`, five `data-urlencode` lines) with `curl_escape` quoting each value, spawns `curl -s -K -`, writes the config to its stdin, and parses the same Vonage JSON reply. No credentials still falls through to `existing.send_sms` (the console printer).
