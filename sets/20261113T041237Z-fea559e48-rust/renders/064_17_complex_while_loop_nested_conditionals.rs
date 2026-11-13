use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_gid: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_home: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_info: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_pass: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_shell: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_uid: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_user: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

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

fn __sh_regex(text: &str, pat: &str, flags: &str) -> bool {
    let cmd = if flags.contains('i') {
        "shopt -s nocasematch; [[ $1 =~ $2 ]]"
    } else {
        "[[ $1 =~ $2 ]]"
    };
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd).arg("sh").arg(text).arg(pat);
    c.status().map(|s| s.success()).unwrap_or(false)
}

#[allow(non_upper_case_globals)]
fn main() {
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/etc/passwd").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/etc/passwd"));
        let mut __sh_while_ran0 = false;
        let mut __sh_while_last1 = 0;
        while {
let (__ln, __any) = __sh_readline();
let __f = __sh_read_fields(&__ln, ":", 7);
__SHV_user.with(|v| *v.borrow_mut() = __f.get(0).cloned().unwrap_or_default());
__SHV_pass.with(|v| *v.borrow_mut() = __f.get(1).cloned().unwrap_or_default());
__SHV_uid.with(|v| *v.borrow_mut() = __f.get(2).cloned().unwrap_or_default());
__SHV_gid.with(|v| *v.borrow_mut() = __f.get(3).cloned().unwrap_or_default());
__SHV_info.with(|v| *v.borrow_mut() = __f.get(4).cloned().unwrap_or_default());
__SHV_home.with(|v| *v.borrow_mut() = __f.get(5).cloned().unwrap_or_default());
__SHV_shell.with(|v| *v.borrow_mut() = __f.get(6).cloned().unwrap_or_default());
__SH_RC.store(if __any { 0 } else { 1 }, Ordering::SeqCst);
__any
} {
            __sh_while_ran0 = true;
            if (__SHV_uid.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) > 1000) && (__SHV_shell.with(|v| v.borrow().clone()) != "/bin/false".to_string()) {
                if __sh_regex(&__SHV_home.with(|v| v.borrow().clone()), "^/home/", "") {
                    println!("{}", format!("User: {} (UID: {}) - {}", __SHV_user.with(|v| v.borrow().clone()), __SHV_uid.with(|v| v.borrow().clone()), __SHV_home.with(|v| v.borrow().clone())));
                    __SH_RC.store(0, Ordering::SeqCst);
                } else {
                    __SH_RC.store(0, Ordering::SeqCst);
                }
            } else {
                __SH_RC.store(0, Ordering::SeqCst);
            }
            __sh_while_last1 = __SH_RC.load(Ordering::SeqCst);
        }
        if !__sh_while_ran0 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last1, Ordering::SeqCst); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}