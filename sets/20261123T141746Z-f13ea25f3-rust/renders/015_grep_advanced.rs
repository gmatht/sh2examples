use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn __sh_echo_esc(s: &str) -> (String, bool) {
    let mut o = String::new();
    let ch: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < ch.len() {
        if ch[i] == '\\' && i + 1 < ch.len() {
            i += 1;
            match ch[i] {
                'n' => o.push('\n'), 't' => o.push('\t'), 'r' => o.push('\r'),
                'a' => o.push('\x07'), 'b' => o.push('\x08'), 'f' => o.push('\x0c'),
                'v' => o.push('\x0b'), 'e' => o.push('\x1b'), '\\' => o.push('\\'),
                'c' => return (o, true),
                '0'..='7' => {
                    let mut n = 0; let mut k = 0;
                    while k < 3 && i + k < ch.len() && ch[i + k].is_ascii_digit() && ch[i + k] <= '7' {
                        n = n * 8 + ch[i + k].to_digit(8).unwrap(); k += 1;
                    }
                    o.push(char::from_u32(n & 0xff).unwrap_or('\0'));
                    i += k - 1;
                }
                'x' => {
                    let mut n = 0; let mut k = 0;
                    while k < 2 && i + 1 + k < ch.len() && ch[i + 1 + k].is_ascii_hexdigit() {
                        n = n * 16 + ch[i + 1 + k].to_digit(16).unwrap(); k += 1;
                    }
                    if k > 0 { o.push(char::from_u32(n).unwrap_or('\0')); i += k; }
                    else { o.push('x'); }
                }
                c => { o.push('\\'); o.push(c); }
            }
            i += 1;
        } else { o.push(ch[i]); i += 1; }
    }
    (o, false)
}

fn __sh_print_words(ws: &[Vec<String>], nl: bool, esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    let mut cut = false;
    if esc { let (t, c) = __sh_echo_esc(&s); s = t; cut = c; }
    if nl && !cut { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_puts(s: &str) {
    __sh_sink(s.as_bytes());
}

fn __sh_spawn(cmd: &str, input: Option<&[u8]>) -> i32 {
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
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if input.is_none() {
        if let Some(p) = &stdin_path {
            if let Ok(f) = std::fs::File::open(p) {
                c.stdin(std::process::Stdio::from(f));
            }
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

fn __sh_run(cmd: &str) -> i32 {
    __sh_spawn(cmd, None)
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout1 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("match1\nmatch2\nmatch3\nmatch4\n");
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout1);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'grep' '-m' '2' 'match'", Some(&__sh_pbuf0)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout3 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("text with pattern in it\n");
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout3);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'grep' '-b' 'pattern'", Some(&__sh_pbuf2)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    let _ = std::fs::write("temp_file.txt", "content\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SH_RC.store(__sh_run("'grep' '-h' 'content' 'temp_file.txt'"), Ordering::Relaxed);
    __SH_RC.store(__sh_run("'grep' '-H' 'content' 'temp_file.txt'"), Ordering::Relaxed);
    let mut __sh_pbuf4: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'grep' '-Z' '-l' 'pattern' 'temp_file.txt'", "'tr' '\\0' '\\n'"), Some(&__sh_pbuf4)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = {
let mut __sh_pbuf5: Vec<u8> = Vec::new();
__SH_PIPESTATUS.lock().unwrap().clear();
{
    let __sh_oldout6 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("text with pattern in it\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf5 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout6);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
__SH_RC.store(__sh_spawn("'grep' '--color=always' 'pattern'", Some(&__sh_pbuf5)), Ordering::Relaxed);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("Color not supported\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_buf = { let __sh_res = __sh_run("'grep' '-q' 'pattern' 'temp_file.txt'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_res == 0 } && {
print!("found\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("not found\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec!["temp_file.txt".to_string()] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            if __sh_mapped.is_dir() {
                eprintln!("rm: cannot remove '{}': Is a directory", __sh_path);
                __sh_status = 1;
                continue;
            }
            let __sh_res = std::fs::remove_file(__sh_path);
                    if let Err(__sh_err) = __sh_res {
                        eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                        __sh_status = 1;
                    }
                }
                Err(__sh_err) => {
                        eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                        __sh_status = 1;
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    let matched = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __SH_RC.store(__sh_spawn("'grep' '-c' '.*'", Some(format!("{}\n", "test").as_bytes())), Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("  grep_exit: {}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("  match_count: {}", matched.clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}