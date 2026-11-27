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

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){pat}") } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    {
        let __oldin = __SH_STDIN.lock().unwrap().take();
        let __oldinp = __SH_STDIN_PATH.lock().unwrap().take();
        *__SH_STDIN.lock().unwrap() = Some(Box::new(std::fs::File::open("/etc/passwd").unwrap_or_else(|_| { let f = std::fs::File::open("/dev/null").unwrap(); f })));
        *__SH_STDIN_PATH.lock().unwrap() = Some(std::path::PathBuf::from("/etc/passwd"));
        let mut __sh_while_ran0 = false;
        let mut __sh_while_last1 = 0;
        while {
let (__sh_line, __sh_gotline) = { let mut __sh_buf: Vec<u8> = Vec::new(); let mut __sh_one = [0u8; 1]; let mut __sh_any = false; let mut __sh_src: Box<dyn std::io::Read + Send> = match __SH_STDIN.lock().unwrap().take() { Some(__sh_r) => __sh_r, None => Box::new(std::io::stdin()) }; loop { match std::io::Read::read(&mut __sh_src, &mut __sh_one) { Ok(0) => break, Ok(_) => { __sh_any = true; if __sh_one[0] == b'\n' { break; } __sh_buf.push(__sh_one[0]); }, Err(_) => break } } *__SH_STDIN.lock().unwrap() = Some(__sh_src); (String::from_utf8_lossy(&__sh_buf).to_string(), __sh_any) };
let __sh_field: Vec<String> = { let __sh_ifs = ":"; let __sh_n = 7usize; let __sh_l = &__sh_line; if __sh_n <= 1 || __sh_ifs.is_empty() { vec![__sh_l.to_string()] } else { let __sh_ws = __sh_ifs.chars().all(|c| c == ' ' || c == '\t' || c == '\n'); let __sh_ch: Vec<char> = __sh_l.chars().collect(); let mut __sh_f: Vec<String> = Vec::new(); let mut __sh_i = 0usize; while __sh_i < __sh_ch.len() && __sh_ifs.contains(__sh_ch[__sh_i]) { __sh_i += 1; } while __sh_f.len() < __sh_n - 1 && __sh_i < __sh_ch.len() { let __sh_s = __sh_i; while __sh_i < __sh_ch.len() && !__sh_ifs.contains(__sh_ch[__sh_i]) { __sh_i += 1; } __sh_f.push(__sh_ch[__sh_s..__sh_i].iter().collect()); if __sh_i >= __sh_ch.len() { break; } __sh_i += 1; if __sh_ws { while __sh_i < __sh_ch.len() && __sh_ifs.contains(__sh_ch[__sh_i]) { __sh_i += 1; } } } __sh_f.push(__sh_ch[__sh_i.min(__sh_ch.len())..].iter().collect()); __sh_f } };
__SHV_user.with(|v| *v.borrow_mut() = __sh_field.get(0).cloned().unwrap_or_default());
__SHV_pass.with(|v| *v.borrow_mut() = __sh_field.get(1).cloned().unwrap_or_default());
__SHV_uid.with(|v| *v.borrow_mut() = __sh_field.get(2).cloned().unwrap_or_default());
__SHV_gid.with(|v| *v.borrow_mut() = __sh_field.get(3).cloned().unwrap_or_default());
__SHV_info.with(|v| *v.borrow_mut() = __sh_field.get(4).cloned().unwrap_or_default());
__SHV_home.with(|v| *v.borrow_mut() = __sh_field.get(5).cloned().unwrap_or_default());
__SHV_shell.with(|v| *v.borrow_mut() = __sh_field.get(6).cloned().unwrap_or_default());
__SH_RC.store(if __sh_gotline { 0 } else { 1 }, Ordering::Relaxed);
__sh_gotline
} {
            __sh_while_ran0 = true;
            if (__SHV_uid.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) > 1000) && (__SHV_shell.with(|v| v.borrow().clone()) != "/bin/false".to_string()) {
                if { let __sh_mapped = __sh_regex(&__SHV_home.with(|v| v.borrow().clone()).to_string(), &"^/home/".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } {
                    println!("User: {} (UID: {}) - {}", __SHV_user.with(|v| v.borrow().clone()), __SHV_uid.with(|v| v.borrow().clone()), __SHV_home.with(|v| v.borrow().clone()));
                    __SH_RC.store(0, Ordering::Relaxed);
                } else {
                    __SH_RC.store(0, Ordering::Relaxed);
                }
            } else {
                __SH_RC.store(0, Ordering::Relaxed);
            }
            __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
        }
        if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
        *__SH_STDIN.lock().unwrap() = __oldin;
        *__SH_STDIN_PATH.lock().unwrap() = __oldinp;
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}