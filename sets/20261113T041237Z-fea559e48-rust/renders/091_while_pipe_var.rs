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

fn __sh_readline() -> (String, bool) {
    let mut buf: Vec<u8> = Vec::new();
    let mut one = [0u8; 1];
    let mut any = false;
    let src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() {
        Some(r) => r,
        None => Box::new(std::io::stdin()),
    };
    let mut r = src;
    loop {
        match r.read(&mut one) {
            Ok(0) => break,
            Ok(_) => {
                if one[0] == b'\n' { any = true; break; }
                buf.push(one[0]);
                any = true;
            }
            Err(_) => break,
        }
    }
    *__SH_STDIN.lock().unwrap() = Some(r);
    (String::from_utf8_lossy(&buf).to_string(), any)
}

fn __sh_read_fields(line: &str, ifs: &str, n: usize) -> Vec<String> {
    if n <= 1 || ifs.is_empty() { return vec![line.to_string()]; }
    let ws = ifs.chars().all(|c| c == ' ' || c == '\t' || c == '\n');
    let ch: Vec<char> = line.chars().collect();
    let is_sep = |c: char| ifs.contains(c);
    let mut fields: Vec<String> = Vec::new();
    let mut i = 0;
    while i < ch.len() && is_sep(ch[i]) { i += 1; }
    while fields.len() < n - 1 && i < ch.len() {
        let start = i;
        while i < ch.len() && !is_sep(ch[i]) { i += 1; }
        fields.push(ch[start..i].iter().collect());
        if i >= ch.len() { break; }
        i += 1;
        if ws { while i < ch.len() && is_sep(ch[i]) { i += 1; } }
    }
    let rest: String = ch[i.min(ch.len())..].iter().collect();
    fields.push(rest);
    fields
}

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_count.with(|v| v.set(0));
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
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
let (__ln, __any) = __sh_readline();
let __f = __sh_read_fields(&__ln, " \t\n", 1);
__SHV_line.with(|v| *v.borrow_mut() = __f.get(0).cloned().unwrap_or_default());
__SH_RC.store(if __any { 0 } else { 1 }, Ordering::SeqCst);
__any
} {
    __sh_while_ran2 = true;
    __SHV_count.with(|v| v.set(__SHV_count.with(|v| v.get()) + 1));
    __SH_RC.store(0, Ordering::SeqCst);
    { let __w: &[Vec<String>] = {&[vec![format!("{}: {}", __SHV_count.with(|v| v.get()), __SHV_line.with(|v| v.borrow().clone()))]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_while_last3 = __SH_RC.load(Ordering::SeqCst);
}
if !__sh_while_ran2 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last3, Ordering::SeqCst); }
};
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("head").arg("-3").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf0); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}