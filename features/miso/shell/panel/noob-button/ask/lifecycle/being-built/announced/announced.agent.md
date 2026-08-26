# announced — a build asked for in conversation is announced to everyone

An ask that arrives through the app has a record in the asker's world and is
stamped `building` and `shipped` with `tools/stamp_ask.py --text`. An ask that
arrives in conversation with the builder has no record — so before building
it, announce it, and at ship, close it:

    MISO_HOST=microserver@185.96.221.52 python3 tools/stamp_ask.py --announce "<the ask, in the asker's words>" --status building
    MISO_HOST=microserver@185.96.221.52 python3 tools/stamp_ask.py --announce "<the same words>" --status shipped --build <N>

The words are the key: the shipping call matches the building one by text.
Every user's sheet shows the building entries under "building", so the
whole team sees what is under way (#p150a).
