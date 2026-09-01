struct feature_Flip;
impl feature_Flip {
    // ---- the camera, held on the device ------------------------------------
    // /one-add's own declaration and its own reasoning: which camera you film
    // with is how you work, not what you own, so it stays on the device and
    // its write queues no op. Every read goes to the live context rather than
    // the bridged state, because /payload republishes part-way down the update
    // chain and a render after this node's own write would be one turn stale.

    fn flip_read() -> String {
        with_context(|c| c.flip_facing_get())
    }

    fn flip_write(facing: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/dictate/as-posts/capture/video/flip", "facing",
                              serde_json::json!(facing.clone()));
        });
    }

    fn flip_front() -> bool {
        flip_read() == "front"
    }

    // ---- the control -------------------------------------------------------
    // in /one-add's kind picker, beside the kinds, and only when video is the
    // kind add will make: it is a setting for one kind, and a control for a
    // camera you are not about to use is noise. Untinted, so the kinds stay
    // the lit set and this reads as the quiet setting it is (/taste 2, and
    // /one-add's own reasoning for its mode button).
    //
    // The viewfinder was the other candidate and is the wrong place: it only
    // exists while recording, and a flip there either lies (it would take
    // effect next time) or costs you the clip, because a MediaRecorder cannot
    // be handed a different camera mid-take. The ask said "a persistent mode,
    // again" — the same setting /one-add's mode is — and this is where that
    // setting lives.

    fn one_add_choices(mode: String, photo: String, vid: String, rec: String) -> String {
        let strip = existing.one_add_choices(mode.clone(), photo, vid.clone(), rec);
        if mode != "video" || vid.is_empty() {
            return strip;
        }
        format!("{}{}", strip, flip_button(flip_front()))
    }

    fn flip_button(front: bool) -> String {
        let title = if front { "front camera" } else { "back camera" };
        let glyph = if front { flip_face_svg() } else { video_camera_svg() };
        format!("<div class=\"tool-button ctrl vid-flip\" data-ev=\"vid_flip\" title=\"{}\">{}</div>",
                title, glyph)
    }

    // ---- the tap -----------------------------------------------------------
    // the flip writes the device var and holds the picker open: you are
    // choosing a camera, not leaving the strip, and /one-add closes the picker
    // on every tap that is not one of its own. This node's update is composed
    // outside it, so the close happens and is then undone by the one tap that
    // means "stay".

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click"
            || e["ev"].as_str().unwrap_or("") != "vid_flip" {
            return state;
        }
        flip_write(if flip_front() { "back".to_string() } else { "front".to_string() });
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["oneadd_picking"] = serde_json::json!(true);
        s.to_string()
    }

    // ---- the glyph ---------------------------------------------------------
    // the control shows the camera you will get rather than the act of
    // flipping, so it is readable without colour and without a second tap:
    // the back camera is /capture/video's own camera mark, and the front one
    // is the person it points at — head and shoulders, the shape 👤 stands
    // for everywhere else here. Drawn, in currentColor, per /glyphs.

    fn flip_face_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<circle cx=\"12\" cy=\"8.2\" r=\"3.6\" fill=\"none\" stroke=\"currentColor\" ",
            "stroke-width=\"2.1\"/>",
            "<path d=\"M4.8 20.2c0 -3.7 3.2 -5.8 7.2 -5.8s7.2 2.1 7.2 5.8\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.1\" stroke-linecap=\"round\"/>",
            "</svg>"))
    }
}
