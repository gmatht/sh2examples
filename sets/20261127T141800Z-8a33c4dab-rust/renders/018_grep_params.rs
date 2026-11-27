use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_longline: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

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

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
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

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("== Basic grep parameters ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = {
let mut __sh_pbuf0: Vec<u8> = Vec::new();
__SH_PIPESTATUS.lock().unwrap().clear();
{
    let __sh_oldout1 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("text with pattern\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout1);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
__SH_RC.store(__sh_spawn_argv(&["grep", "-i", "PATTERN"], Some(&__sh_pbuf0)), Ordering::Relaxed);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
__SH_RC.load(Ordering::Relaxed) == 0
} && {
print!("  -i match: OK\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("  -i match: FAIL\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    let count = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout3 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("line1\nline2\nline3\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout3);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'grep' '-v' 'line2'", "'wc' '-l'"), Some(&__sh_pbuf2)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("  -v count: {} (expected 2)", count.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let mut matched = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf4: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout5 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("match\nno match\nmatch again\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf4 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout5);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["grep", "-c", "match"], Some(&__sh_pbuf4)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("  -c count: {} (expected 2)", matched.clone());
    print!("== Context parameters ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_pbuf6: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout7 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("line1\nline2\nTARGET\nline4\nline5\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf6 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout7);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-A' '2' 'TARGET'", __sh_q("/tmp/grep_out.txt")), Some(&__sh_pbuf6)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!("  -A 2 lines: {} (expected 3)", { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'", __sh_q("/tmp/grep_out.txt"))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    let mut __sh_pbuf8: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout9 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("line1\nline2\nTARGET\nline4\nline5\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf8 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout9);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-B' '2' 'TARGET'", __sh_q("/tmp/grep_out.txt")), Some(&__sh_pbuf8)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!("  -B 2 lines: {} (expected 3)", { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'", __sh_q("/tmp/grep_out.txt"))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    let mut __sh_pbuf10: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout11 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("line1\nline2\nTARGET\nline4\nline5\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf10 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout11);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-C' '1' 'TARGET'", __sh_q("/tmp/grep_out.txt")), Some(&__sh_pbuf10)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!("  -C 1 lines: {} (expected 3)", { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'", __sh_q("/tmp/grep_out.txt"))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    print!("== File handling parameters ==\n");
    let _ = std::fs::write("/tmp/grep_file.txt", "content\n");
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = { let __sh_res = __sh_run("'grep' '-c' 'content' '/tmp/grep_file.txt'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_res == 0 } && {
print!("  -c file: OK\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("  -c file: FAIL\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_buf = { let __sh_res = __sh_run("'grep' '-l' 'content' '/tmp/grep_file.txt'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_res == 0 } && {
print!("  -l: found\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("  -l: not found\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    { let __sh_buf = { let __sh_res = __sh_run("'grep' '-L' 'nonexistent' '/tmp/grep_file.txt'"); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_res == 0 } && {
print!("  -L: not found (correct)\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("  -L: found (wrong)\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    print!("== Output formatting parameters ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    matched = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let (__sh_mapped, __sh_res) = { let __sh_text = "text with pattern in it"; let __sh_num = __sh_text.matches("pattern").count(); (if __sh_num == 0 { String::new() } else { vec!["pattern"; __sh_num].join("\n") }, if __sh_num == 0 { 1 } else { 0 }) };
    if !__sh_mapped.is_empty() { __sh_print_words(&[vec![__sh_mapped]], true, false); }
    __SH_RC.store(__sh_res, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("  -o match: '{}' (expected 'pattern')", matched.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let lineno = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf12: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout13 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("text with pattern in it\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf12 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout13);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'grep' '-n' 'pattern'", "'cut' '-d' ':' '-f' '1'"), Some(&__sh_pbuf12)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("  -n line: {} (expected 1)", lineno.clone());
    print!("== Recursive parameters ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = {
{
    let mut __sh_status: i32 = 0;
    for __sh_path in &vec!["/tmp/grep_sub".to_string()] {
        let __sh_res = if true { std::fs::create_dir_all(__sh_path) } else { std::fs::create_dir(__sh_path) };
        match __sh_res {
            Ok(_) => {
                if false { println!("mkdir: created directory '{}'", __sh_path); }
            }
            Err(__sh_err) => {
                // MEASURED shapes, which are NOT one format: this host's
                // mkdir names the path on `File exists` and omits it on
                // `Permission denied` / `No such file or directory`.
                match __sh_err.kind() {
                    std::io::ErrorKind::AlreadyExists => eprintln!("mkdir: {}: File exists", __sh_path),
                    std::io::ErrorKind::PermissionDenied => eprintln!("mkdir: Permission denied"),
                    std::io::ErrorKind::NotFound => eprintln!("mkdir: No such file or directory"),
                    _ => eprintln!("mkdir: {}: {}", __sh_path, __sh_err),
                }
                __sh_status = 1;
            }
        }
    }
    __SH_RC.store(__sh_status, Ordering::Relaxed);
}
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        {
            let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create("/tmp/grep_sub/file.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
            __sh_puts("subfile content\n");
            __SH_RC.store(0, Ordering::Relaxed);
            __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
        }
    }
    }
    let found = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf14: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{} 2> {}", "'grep' '-r' 'subfile' '/tmp/grep_sub'", __sh_q("/dev/null")), "'wc' '-l'"), Some(&__sh_pbuf14)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("  -r recursive: {} files matched (expected 1)", found.clone());
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
        for __sh_path in &vec!["/tmp/grep_sub".to_string()] {
            match std::fs::symlink_metadata(__sh_path) {
                Ok(__sh_mapped) => {
            let __sh_res = if __sh_mapped.is_dir() {
                std::fs::remove_dir_all(__sh_path)
            } else {
                std::fs::remove_file(__sh_path)
            };
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
    print!("== Line length parameters ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_longline.with(|v| *v.borrow_mut() = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
});
    { let __sh_buf = {
let mut __sh_pbuf15: Vec<u8> = Vec::new();
__SH_PIPESTATUS.lock().unwrap().clear();
{
    let __sh_oldout16 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec![__SHV_longline.with(|v| v.borrow().clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf15 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout16);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
__SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-m' '1' 'a'", __sh_q("/dev/null")), Some(&__sh_pbuf15)), Ordering::Relaxed);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
__SH_RC.load(Ordering::Relaxed) == 0
} && {
print!("  -m 1 (long line): OK\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("  -m 1 (long line): FAIL\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    print!("== Word-regexp and line-regexp parameters ==\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_pbuf17: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout18 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("foo\nfoobar\nbar\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf17 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout18);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-w' 'foo'", __sh_q("/tmp/grep_out.txt")), Some(&__sh_pbuf17)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!("  -w word match lines: {} (expected 1)", { let (__sh_ch, __sh_res) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'", __sh_q("/tmp/grep_out.txt"))); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{__e}") } };
        for __sh_path in &vec!["/tmp/grep_file.txt".to_string(), "/tmp/grep_out.txt".to_string()] {
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