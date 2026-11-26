use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_L: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

fn __sh_cap_bytes(cmd: &str, input: Option<&[u8]>) -> Vec<u8> {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let Ok(mut ch) = c.spawn() else { return Vec::new() };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let _ = ch.wait();
    out
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __sh_pbuf0 = __sh_cap_bytes(&format!("{} | {}", "'yes' 'Line:LINE'", "'head' '-n' '100'"), Some(&__sh_pbuf0));
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    {
    let __oldin = __SH_STDIN.lock().unwrap().take();
    *__SH_STDIN.lock().unwrap() = Some(Box::new(std::io::Cursor::new(__sh_pbuf0.clone())));
    let mut __sh_while_ran1 = false;
    let mut __sh_while_last2 = 0;
    while {
let (__sh_line, __sh_gotline) = { let mut __sh_buf: Vec<u8> = Vec::new(); let mut __sh_one = [0u8; 1]; let mut __sh_any = false; let mut __sh_src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() { Some(__sh_r) => __sh_r, None => Box::new(std::io::stdin()) }; loop { match std::io::Read::read(&mut __sh_src, &mut __sh_one) { Ok(0) => break, Ok(_) => { __sh_any = true; if __sh_one[0] == b'\n' { break; } __sh_buf.push(__sh_one[0]); }, Err(_) => break } } *__SH_STDIN.lock().unwrap() = Some(__sh_src); (String::from_utf8_lossy(&__sh_buf).to_string(), __sh_any) };
let __sh_field: Vec<String> = vec![__sh_line.clone()];
__SHV_L.with(|v| *v.borrow_mut() = __sh_field.get(0).cloned().unwrap_or_default());
__SH_RC.store(if __sh_gotline { 0 } else { 1 }, Ordering::Relaxed);
__sh_gotline
} {
        __sh_while_ran1 = true;
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
        __SH_RC.store(0, Ordering::Relaxed);
        println!("Line:{}", __SHV_i.with(|v| v.get()));
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last2 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran1 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last2, Ordering::Relaxed); }
    *__SH_STDIN.lock().unwrap() = __oldin;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}