#!/bin/bash
# The mini's wifi watchdog (ash, 2026-09-03, after the evening of 2026-09-02:
# the mini's own wifi link dropped four times, once for 67 minutes of failed
# re-associations, while the app was needed). Every 30 s (launchd
# com.noob.wifiwatchdog) it pings the default gateway, then 1.1.1.1; after
# WIFI_FAILS consecutive misses (2 = about a minute) it cycles the wifi radio
# once, and not again within WIFI_COOLDOWN seconds — a router that is
# genuinely off gets one cycle every five minutes, not a thrash. Every
# transition is a line in $WIFI_LOG (DOWN, CYCLE, UP), plus one OK line an
# hour so the log shows the watchdog itself is alive. WIFI_DRY=1 logs the
# cycle without touching the radio (the test).
IF=${WIFI_IF:-en1}
LOG=${WIFI_LOG:-$HOME/wifi-watchdog.log}
STATE=${WIFI_STATE:-/tmp/wifi-watchdog.state}
FAILS_NEEDED=${WIFI_FAILS:-2}
COOLDOWN=${WIFI_COOLDOWN:-300}
NET=${WIFI_NET:-1.1.1.1}
GW=${WIFI_GW:-$(route -n get default 2>/dev/null | awk '/gateway/{print $2}')}
now=$(date '+%Y-%m-%d %H:%M:%S'); epoch=$(date +%s)

ok=0
if [ -n "$GW" ] && ping -c 2 -t 3 "$GW" >/dev/null 2>&1; then ok=1
elif ping -c 2 -t 3 "$NET" >/dev/null 2>&1; then ok=1; fi

fails=0; lastcycle=0; last=up; beat=0
[ -f "$STATE" ] && read -r fails lastcycle last beat < "$STATE"

if [ "$ok" = 1 ]; then
  if [ "$last" != up ]; then echo "$now UP gw=${GW:-none} after $fails failed checks" >> "$LOG"; fi
  if [ $((epoch - beat)) -ge 3600 ]; then
    echo "$now OK gw=${GW:-none} $(networksetup -getairportpower "$IF" 2>/dev/null)" >> "$LOG"; beat=$epoch
  fi
  echo "0 $lastcycle up $beat" > "$STATE"; exit 0
fi

fails=$((fails + 1))
if [ "$last" = up ]; then echo "$now DOWN gw=${GW:-none} (check 1)" >> "$LOG"; fi
if [ "$fails" -ge "$FAILS_NEEDED" ] && [ $((epoch - lastcycle)) -ge "$COOLDOWN" ]; then
  echo "$now CYCLE $IF after $fails failed checks ($(networksetup -getairportpower "$IF" 2>/dev/null))${WIFI_DRY:+ [dry]}" >> "$LOG"
  if [ -z "$WIFI_DRY" ]; then
    networksetup -setairportpower "$IF" off; sleep 4; networksetup -setairportpower "$IF" on
  fi
  lastcycle=$epoch
fi
echo "$fails $lastcycle down $beat" > "$STATE"
