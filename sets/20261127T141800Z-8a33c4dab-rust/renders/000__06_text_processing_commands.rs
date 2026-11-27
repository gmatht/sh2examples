use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

fn __sh_puts(s: &str) {
    __sh_sink(s.as_bytes());
}

fn __sh_capture_rc_argv(argv: &[&str]) -> (String, i32) {
    if argv.is_empty() { return (String::new(), 0); }
    let mut c = std::process::Command::new(argv[0]);
    c.args(&argv[1..]);
    c.stdout(std::process::Stdio::piped());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let s = String::from_utf8_lossy(&out).trim_end_matches('\n').to_string();
    (s, rc)
}

fn __sh_spawn(cmd: &str, input: Option<&[u8]>) -> i32 {
    use std::process::Stdio;
    let mut input = input.map(|d| d.to_vec());
    let stdin_path = __SH_STDIN_PATH.lock().unwrap().clone();
    if input.is_none() && stdin_path.is_none() {
        if let Some(mut r) = __SH_STDIN.lock().unwrap().take() {
            let mut d = Vec::new();
            let _ = std::io::Read::read_to_end(&mut r, &mut d);
            input = Some(d);
        }
    }
    let want_out = __SH_OUT.lock().unwrap().is_some() || __SH_OUTFILE_TL.with(|v| v.borrow().is_some());
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    if !want_out {
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    c.stdout(if want_out { Stdio::piped() } else { Stdio::inherit() });
    if let (None, Some(p)) = (&input, &stdin_path) {
        if let Ok(f) = std::fs::File::open(p) {
            c.stdin(Stdio::from(f));
        }
    }
    if input.is_some() { c.stdin(Stdio::piped()); }
    let Ok(mut ch) = c.spawn() else { return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() { __sh_sink(&out); }
    rc
}

fn __sh_spawn_argv(argv: &[&str], input: Option<&[u8]>) -> i32 {
    if argv.is_empty() { return 0; }
    let mut input = input.map(|d| d.to_vec());
    let stdin_path = __SH_STDIN_PATH.lock().unwrap().clone();
    if input.is_none() && stdin_path.is_none() {
        if let Some(mut r) = __SH_STDIN.lock().unwrap().take() {
            let mut d = Vec::new();
            let _ = std::io::Read::read_to_end(&mut r, &mut d);
            input = Some(d);
        }
    }
    let want_out = __SH_OUT.lock().unwrap().is_some() || __SH_OUTFILE_TL.with(|v| v.borrow().is_some());
    let mut c = std::process::Command::new(argv[0]);
    c.args(&argv[1..]);
    if !want_out {
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if let (None, Some(p)) = (&input, &stdin_path) {
        if let Ok(f) = std::fs::File::open(p) {
            c.stdin(std::process::Stdio::from(f));
        }
    }
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let Ok(mut ch) = c.spawn() else { return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() { __sh_sink(&out); }
    rc
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    let cut_result: String;
    let head_result: String;
    let line_count: String;
    let tail_result: String;
    let tr_result: String;
    let word_count: String;
    print!("=== Text Processing Commands ===\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let file_content = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'cat' 'src/main.rs'", "'head' '-5'"), Some(&__sh_pbuf0)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    print!("First 5 lines of main.rs:\n");
    println!("{}", file_content.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let grep_result = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["grep", "-n", "fn", "src/main.rs"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    print!("Lines containing 'fn':\n");
    println!("{}", grep_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let sed_result = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf1: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout2 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("Hello World\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf1 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout2);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["sed", "s/World/Universe/"], Some(&__sh_pbuf1)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("Sed result: {}", sed_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let awk_result = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf3: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout4 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("1 2 3 4 5\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf3 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout4);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'awk' '{print $1 + $2}'", Some(&__sh_pbuf3)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("Awk sum result: {}", awk_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let sort_result = { let __sh_text: String = "zebra\napple\nbanana".to_string(); let mut __sh_lines: Vec<String> = if __sh_text.is_empty() { Vec::new() } else { let mut __sh_collected: Vec<String> = __sh_text.split('\n').map(|__sh_line| __sh_line.to_string()).collect(); if __sh_text.ends_with('\n') { __sh_collected.pop(); } __sh_collected }; let __sort_key = |__sh_line: &String| -> String { __sh_line.clone() }; __sh_lines.sort_by(|__sh_left, __sh_right| __sort_key(__sh_left).cmp(&__sort_key(__sh_right)));  __sh_lines.join("\n") };
    print!("Sorted words:\n");
    println!("{}", sort_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let uniq_result = { let __sh_text: String = "apple\napple\nbanana\nbanana\ncherry".to_string(); let mut __sh_lines: Vec<String> = if __sh_text.is_empty() { Vec::new() } else { let mut __sh_collected: Vec<String> = __sh_text.split('\n').map(|__sh_line| __sh_line.to_string()).collect(); if __sh_text.ends_with('\n') { __sh_collected.pop(); } __sh_collected }; let __sh_key = |__sh_line: &String| -> String { __sh_line.chars().skip(0).collect::<String>() }; let mut __sh_output: Vec<String> = Vec::new(); let mut __sh_begin = 0; while __sh_begin < __sh_lines.len() { let mut __sh_end = __sh_begin + 1; while __sh_end < __sh_lines.len() && __sh_key(&__sh_lines[__sh_end]) == __sh_key(&__sh_lines[__sh_begin]) { __sh_end += 1; } __sh_output.push(__sh_lines[__sh_begin].clone()); __sh_begin = __sh_end; } __sh_output.join("\n") };
    print!("Unique words:\n");
    println!("{}", uniq_result.clone());
    word_count = ("Hello World".to_string().split_whitespace().count() as i64).to_string();
    line_count = ("line1\nline2\nline3\n".to_string().matches('\n').count() as i64).to_string();
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("Word count: {}", word_count.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Line count: {}", line_count.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    head_result = { let __sh_text: String = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10".to_string(); let __sh_num: i64 = 3; let __sh_lines: Vec<&str> = __sh_text.split('\n').collect(); let __sh_count = __sh_lines.len(); if __sh_num <= 0 { String::new() } else { let __sh_stop = (__sh_num as usize).min(__sh_count); __sh_lines[..__sh_stop].join("\n") } };
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("First 3 numbers: {}", head_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    tail_result = { let __sh_text: String = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10".to_string(); let __sh_num: i64 = 3; let __sh_lines: Vec<&str> = __sh_text.split('\n').collect(); let __sh_count = __sh_lines.len(); if __sh_num <= 0 { String::new() } else { let __sh_start = __sh_count.saturating_sub(__sh_num as usize); __sh_lines[__sh_start..].join("\n") } };
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("Last 3 numbers: {}", tail_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    cut_result = { let __sh_s: String = "apple:banana:cherry".to_string(); let __sh_d: &str = ":"; if !__sh_s.contains(__sh_d) { if false { String::new() } else { __sh_s } } else { let __sh_f: Vec<&str> = __sh_s.split(__sh_d).collect(); vec![1usize].iter().copied() .filter(|&__sh_i| __sh_i < __sh_f.len()) .map(|__sh_i| __sh_f[__sh_i]).collect::<Vec<&str>>().join(":") } };
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("Second field: {}", cut_result.clone());
    let _ = std::fs::write("temp1.txt", "1\n2\n3\n");
    let _ = std::fs::write("temp2.txt", "a\nb\nc\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let paste_result = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf5: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'paste' 'temp1.txt' 'temp2.txt'", "'sed' 's/\\t/ /g'"), Some(&__sh_pbuf5)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    print!("Pasted columns:\n");
    println!("{}", paste_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{__e}") } };
        for __sh_path in &vec!["temp1.txt".to_string(), "temp2.txt".to_string()] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            if __sh_mapped.is_dir() {
                eprintln!("rm: cannot remove '{}': Is a directory", __sh_path);
                __sh_status = 1;
                continue;
            }
            let __sh_res = std::fs::remove_file(__sh_path);
                    if let Err(__sh_err) = __sh_res {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                    }
                }
                Err(__sh_err) => {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    let _ = std::fs::write("file1.txt", "apple\nbanana\ncherry\n");
    let _ = std::fs::write("file2.txt", "banana\ncherry\ndate\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let comm_result = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["comm", "-12", "file1.txt", "file2.txt"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    print!("Common lines:\n");
    println!("{}", comm_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let diff_result = { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["diff", "file1.txt", "file2.txt"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch };
    print!("File differences:\n");
    println!("{}", diff_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    tr_result = { let __sh_from_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); let __sh_to_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); "HELLO WORLD".to_string().chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() };
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("Lowercase: {}", tr_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let xargs_result = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf6: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout7 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("1 2 3\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf6 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout7);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'xargs' '-n' '1' 'echo' 'Number:'", Some(&__sh_pbuf6)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    print!("Xargs result:\n");
    println!("{}", xargs_result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{__e}") } };
        for __sh_path in &vec!["file1.txt".to_string(), "file2.txt".to_string()] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            if __sh_mapped.is_dir() {
                eprintln!("rm: cannot remove '{}': Is a directory", __sh_path);
                __sh_status = 1;
                continue;
            }
            let __sh_res = std::fs::remove_file(__sh_path);
                    if let Err(__sh_err) = __sh_res {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                    }
                }
                Err(__sh_err) => {
                        if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                            eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                            __sh_status = 1;
                        }
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}