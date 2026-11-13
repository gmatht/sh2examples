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
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return Vec::new() };
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
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __sh_pbuf0 = __sh_cap_bytes(&format!("{} | {}", "'yes' 'Line:LINE'".to_string(), "'head' '-n' '100'".to_string()), Some(&__sh_pbuf0));
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    {
    let __oldin = __SH_STDIN.lock().unwrap().take();
    *__SH_STDIN.lock().unwrap() = Some(Box::new(std::io::Cursor::new(__sh_pbuf0.clone())));
    let mut __sh_while_ran1 = false;
    let mut __sh_while_last2 = 0;
    while {
let (__ln, __any) = __sh_readline();
let __f = __sh_read_fields(&__ln, " \t\n", 1);
__SHV_L.with(|v| *v.borrow_mut() = __f.get(0).cloned().unwrap_or_default());
__SH_RC.store(if __any { 0 } else { 1 }, Ordering::SeqCst);
__any
} {
        __sh_while_ran1 = true;
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
        __SH_RC.store(0, Ordering::SeqCst);
        println!("{}", format!("Line:{}", __SHV_i.with(|v| v.get())));
        __SH_RC.store(0, Ordering::SeqCst);
        __sh_while_last2 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran1 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last2, Ordering::SeqCst); }
    *__SH_STDIN.lock().unwrap() = __oldin;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}