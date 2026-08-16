# comms
*how miso places talk: notifications and messages*

> (transcripts/2026-08-13-fm-spec.md#p115)
> aren't "diag" and "events"/"blackbox" all part of the same feature group? … loop is good - go for it

## user

Browse the children: notifications (`/push`) and the message pipe (`/messaging`).

## spec

Grouping node, born of the tree restructure that also united the observability family under `/diag` and renamed the app core to `/loop`: everything about places talking to each other lives here. `/push` carries messages to devices via the platform's notification relay; `/messaging` carries them between miso places over miso's own pipe. Contributes no code.

## glossary

(no new terms)

## code description

No implementation files — a grouping node; `order.md` orders the children.
