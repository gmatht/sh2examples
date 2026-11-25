use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_count: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_line: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
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

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_count.with(|v| v.set(0));
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout1 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/etc/passwd").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/etc/passwd"));
        let _ = {
let mut __sh_while_ran2 = false;
let mut __sh_while_last3 = 0;
while {
let (__sh_line, __sh_gotline) = { let mut __sh_buf: Vec<u8> = Vec::new(); let mut __sh_one = [0u8; 1]; let mut __sh_any = false; let mut __sh_src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() { Some(__sh_r) => __sh_r, None => Box::new(std::io::stdin()) }; loop { match std::io::Read::read(&mut __sh_src, &mut __sh_one) { Ok(0) => break, Ok(_) => { __sh_any = true; if __sh_one[0] == b'\n' { break; } __sh_buf.push(__sh_one[0]); }, Err(_) => break } } *__SH_STDIN.lock().unwrap() = Some(__sh_src); (String::from_utf8_lossy(&__sh_buf).to_string(), __sh_any) };
let __sh_field: Vec<String> = vec![__sh_line.clone()];
__SHV_line.with(|v| *v.borrow_mut() = __sh_field.get(0).cloned().unwrap_or_default());
__SH_RC.store(if __sh_gotline { 0 } else { 1 }, Ordering::Relaxed);
__sh_gotline
} {
    __sh_while_ran2 = true;
    __SHV_count.with(|v| v.set(__SHV_count.with(|v| v.get()) + 1));
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("{}: {}", __SHV_count.with(|v| v.get()), __SHV_line.with(|v| v.borrow().clone()))]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_while_last3 = __SH_RC.load(Ordering::Relaxed);
}
if !__sh_while_ran2 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last3, Ordering::Relaxed); }
};
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout1);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["head", "-3"], Some(&__sh_pbuf0)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}