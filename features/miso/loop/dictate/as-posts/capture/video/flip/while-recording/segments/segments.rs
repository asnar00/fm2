struct feature_Segments;
impl feature_Segments {
    // ---- the marks reach the parts directory before the join --------------
    // /streams joins on the RecShared that announces a recording, and it joins
    // BEFORE the rest of the chain runs — which is where /mirror writes the
    // index. So the marks cannot be read off the index at that moment: they
    // are not there yet. This link is outermost, so it writes them beside the
    // pieces first and the join finds them however it is reached — by the
    // announcement, or by a piece arriving late afterwards.
    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "RecShared" {
            return existing.handle_msg(msg);
        }
        let id = m["data"]["id"].as_str().unwrap_or("").to_string();
        let segs = m["data"]["segs"].clone();
        if streams_id_ok(&id) && segs.is_array() {
            let from = m["_from"].as_str().unwrap_or("").to_string();
            let world = if from.is_empty() { "_local".to_string() } else { from };
            let dir = streams_parts_dir(world, id);
            let _ = std::fs::create_dir_all(dir.clone());
            let _ = std::fs::write(format!("{}/segs.json", dir), segs.to_string());
        }
        existing.handle_msg(msg)
    }

    // ---- the join ----------------------------------------------------------
    // one segment is one container cut at byte boundaries, and /streams' own
    // concatenation is exactly right for it. Two or more are two or more
    // CONTAINERS, and gluing those end to end gives a file whose first header
    // describes only the first take — a player reads that and stops. So the
    // pieces of each segment are run together into a file of its own and
    // ffmpeg is asked to concatenate those.
    //
    // `-c copy`, because every segment came off the same phone through the
    // same MediaRecorder: iOS hands back `video/mp4; codecs=avc1.42000a,
    // mp4a.40.2` for all of them, so the streams match and there is nothing to
    // re-encode. A mismatch would need a transcode; it is named in the spec and
    // not built, because a phone that changed codec mid-take is not a thing
    // that happens and building for it blind would be worse than saying so.
    fn streams_join(world: String, id: String, want: u64) -> bool {
        let dir = streams_parts_dir(world.clone(), id.clone());
        let marks = segments_marks(dir.clone(), want);
        if marks.len() < 2 {
            return existing.streams_join(world, id, want);
        }
        let file = format!("{}/{}/{}", streams_root(), world, id);
        if std::path::Path::new(&file).exists() {
            let _ = std::fs::remove_dir_all(dir);
            return false;                 // already whole; nothing new happened
        }
        // every piece must be here before anything is written: a segment file
        // built from half its pieces is a file that looks finished and is not
        let mut n: u64 = 0;
        while n < want {
            if !std::path::Path::new(&format!("{}/{}", dir, n)).exists() {
                return false;             // a piece is still on its way
            }
            n = n + 1;
        }
        let ff = segments_ffmpeg();
        if ff.is_empty() {
            println!("segments: no ffmpeg on this machine — {} left in pieces", id);
            return false;
        }
        // through MPEG-TS, and this is the whole reason the join is not one
        // command. The concat DEMUXER offsets each input by what the one before
        // it says its duration is — and a MediaRecorder mp4 is written
        // incrementally, so what its header says is not always what it holds.
        // Joined that way, three takes came out right and the fourth claimed
        // 991 seconds for ten seconds of video: 434 frames with packet stamps
        // running to 947 s, the audio beside it a sane 10.1 (rig-found,
        // 2026-09-04). TS carries no global duration to be wrong about, so
        // each segment is copied into it and the pieces are concatenated as
        // byte streams. Still no re-encode: two copy passes and a bitstream
        // filter each way, which is the standard road for h264+aac.
        let mut chain = String::new();
        let mut k = 0usize;
        while k < marks.len() {
            let from = marks[k];
            let to = if k + 1 < marks.len() { marks[k + 1] } else { want };
            let seg = format!("{}/seg-{}.mp4", dir, k);
            let ts = format!("{}/seg-{}.ts", dir, k);
            if !segments_write_one(dir.clone(), from, to, seg.clone()) {
                return false;
            }
            if !segments_ff(ff.clone(), vec![
                "-i".to_string(), seg.clone(),
                "-c".to_string(), "copy".to_string(),
                "-bsf:v".to_string(), "h264_mp4toannexb".to_string(),
                "-f".to_string(), "mpegts".to_string(),
                ts.clone()], id.clone()) {
                return false;
            }
            if !chain.is_empty() {
                chain.push('|');
            }
            chain.push_str(&ts);
            k = k + 1;
        }
        let _ = std::fs::create_dir_all(format!("{}/{}", streams_root(), world));
        let out = format!("{}/joined.mp4", dir);
        if !segments_ff(ff, vec![
            "-i".to_string(), format!("concat:{}", chain),
            "-c".to_string(), "copy".to_string(),
            "-bsf:a".to_string(), "aac_adtstoasc".to_string(),
            "-movflags".to_string(), "+faststart".to_string(),
            out.clone()], id.clone()) {
            return false;
        }
        if std::fs::rename(&out, &file).is_err() {
            // a rename across devices can refuse; the copy is the fallback
            match std::fs::read(&out) {
                Ok(bytes) => {
                    if std::fs::write(&file, &bytes).is_err() {
                        println!("segments: could not write the joined {}", id);
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }
        let _ = std::fs::remove_dir_all(dir);
        println!("segments: {} joined from {} segments over {} pieces",
                 id, marks.len(), want);
        true
    }

    // one ffmpeg run, with `-y` and quiet logging in front of whatever it is
    // asked to do. A refusal is said in the log and answered `false`, so the
    // clip stays in pieces rather than being written half-joined.
    fn segments_ff(ff: String, args: Vec<String>, id: String) -> bool {
        let mut cmd = std::process::Command::new(ff);
        cmd.arg("-y").arg("-hide_banner").arg("-loglevel").arg("error");
        for a in args.iter() {
            cmd.arg(a);
        }
        match cmd.output() {
            Ok(o) => {
                if !o.status.success() {
                    println!("segments: ffmpeg refused {} — {}", id,
                             String::from_utf8_lossy(&o.stderr).replace('\n', " "));
                }
                o.status.success()
            }
            Err(e) => {
                println!("segments: could not run ffmpeg for {} — {}", id, e);
                false
            }
        }
    }

    // the pieces of one segment, run together as they were cut. Within a
    // segment this is /streams' own concatenation and is right for the same
    // reason: MediaRecorder's timeslices are one container cut at byte
    // boundaries.
    fn segments_write_one(dir: String, from: u64, to: u64, out: String) -> bool {
        let mut all: Vec<u8> = Vec::new();
        let mut n = from;
        while n < to {
            match std::fs::read(format!("{}/{}", dir, n)) {
                Ok(mut bytes) => all.append(&mut bytes),
                Err(_) => return false,
            }
            n = n + 1;
        }
        if all.is_empty() {
            return false;
        }
        std::fs::write(&out, &all).is_ok()
    }

    // the marks the phone sent, read back off the sidecar: the part index each
    // container starts at. Anything that is not a run of ascending indices
    // inside the piece count is not trusted and the clip is joined the old way
    // — a bad mark would cut a segment in the wrong place, and a file that
    // plays its first take is a better failure than one that plays nothing.
    fn segments_marks(dir: String, want: u64) -> Vec<u64> {
        let raw = std::fs::read_to_string(format!("{}/segs.json", dir))
            .unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        let arr = v.as_array().unwrap_or(&empty);
        let mut out: Vec<u64> = Vec::new();
        let mut last: i64 = -1;
        for m in arr.iter() {
            let n = match m.as_u64() {
                Some(n) => n,
                None => return Vec::new(),
            };
            if n >= want || (n as i64) <= last {
                return Vec::new();
            }
            last = n as i64;
            out.push(n);
        }
        if out.is_empty() || out[0] != 0 {
            return Vec::new();
        }
        out
    }

    // where ffmpeg is. The server is started by launchd with a PATH that has
    // no brew in it (deploy.md), so the usual places are tried by name before
    // the PATH is trusted at all.
    fn segments_ffmpeg() -> String {
        let mut tries: Vec<String> = Vec::new();
        if let Ok(set) = std::env::var("MISO_FFMPEG") {
            tries.push(set);
        }
        tries.push("/opt/homebrew/bin/ffmpeg".to_string());
        tries.push("/usr/local/bin/ffmpeg".to_string());
        tries.push("/usr/bin/ffmpeg".to_string());
        for t in tries.iter() {
            if std::path::Path::new(t).exists() {
                return t.clone();
            }
        }
        match std::process::Command::new("which").arg("ffmpeg").output() {
            Ok(o) => {
                let p = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if !p.is_empty() && std::path::Path::new(&p).exists() {
                    return p;
                }
                String::new()
            }
            Err(_) => String::new(),
        }
    }
}
