use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_param1: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let Ok(mut ch) = c.spawn() else { return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
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

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
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
    print!("Testing ambiguous operators...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_result.with(|v| *v.borrow_mut() = { let __sh_value = (512).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } });
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("2**3**2 = {}", __SHV_result.with(|v| v.borrow().clone()))]], true, false);
    print!("Testing complex parameter expansions...\n");
    let complex_var = "hello world".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![{ let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } }]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![{ let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } }]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![{ let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } }]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![{ let __sh_str: String = complex_var.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } }]], true, false);
    print!("Testing complex here-documents...\n");
    print!("This is a test line\nThis is not a test line\nThis is another test line\n");
    print!("Testing nested arithmetic...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_result.with(|v| *v.borrow_mut() = { let __sh_value = (40).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } });
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Complex arithmetic: {}", __SHV_result.with(|v| v.borrow().clone()))]], true, false);
    print!("Testing nested command substitution...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Current dir: {}", { let __sh_value = std::env::var("PWD").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", __sh_capture_rc(&"pwd").0) } else { __sh_value.to_string() } });
    println!("User: {}", { let __sh_value = std::env::var("USER").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", __sh_capture_rc(&"whoami").0) } else { __sh_value.to_string() } });
    print!("Testing process substitution...\n");
    print!("Testing complex brace expansion...\n");
    print!("a1x a1y a1z a2x a2y a2z a3x a3y a3z b1x b1y b1z b2x b2y b2z b3x b3y b3z c1x c1y c1z c2x c2y c2z c3x c3y c3z\n");
    print!("Testing simple case patterns...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    if __sh_arg(0) == "test".to_string() {
        print!("Matched test\n");
    } else {
        print!("Default case\n");
    }
    print!("Testing simple pipeline...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{} | {}", "'ls' '-l' '-a'".to_string(), "'grep' '^d'".to_string()), "'head' '-5'"), Some(&__sh_pbuf2)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    print!("Testing mixed arithmetic...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_result.with(|v| *v.borrow_mut() = { let __sh_value = (776).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } });
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Mixed base result: {}", __SHV_result.with(|v| v.borrow().clone()))]], true, false);
    print!("Testing complex string interpolation...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let message = format!("Hello, {}! Your home is {}", { let __sh_value = std::env::var("USER").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", __sh_capture_rc(&"whoami").0) } else { __sh_value.to_string() } }, { let __sh_value = std::env::var("HOME").unwrap_or_default(); if __sh_value.is_empty() { format!("{}", __sh_capture_rc(&"echo ~").0) } else { __sh_value.to_string() } });
    __sh_print_words(&[vec![message.clone()]], true, false);
    print!("Testing simple test expressions...\n");
    __SH_RC.store(0, Ordering::Relaxed);
    if std::path::Path::new("file.txt").is_file() {
        print!("File exists\n");
    } else {
        print!("File does not exist\n");
    }
    print!("Testing complex array operations...\n");
    __SHV_array.with(|v| *v.borrow_mut() = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()]);
    __SHV_array.with(|v| { let mut b = v.borrow_mut(); for __sh_word in vec!["item4".to_string()] { b.push(__sh_word); } });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("Array: {}", __SHV_array.with(|v| v.borrow().clone()).join(" "));
    println!("Length: {}", __SHV_array.with(|v| v.borrow().len() as i64).to_string());
    println!("First item: {}", { let __sh_idx = 0; let __sh_num = __SHV_array.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_array.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    complex_function("test\"quote".to_string(), "second_param".to_string(), "third\"param".to_string());
    test_locals("hello".to_string(), "world".to_string());
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn complex_function(p0: String, p1: String, p2: String) {
    __SHV_param1.with(|v| *v.borrow_mut() = p0.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    let param2 = { let __sh_value = p1.clone(); if __sh_value.is_empty() { "default".to_string() } else { __sh_value.to_string() } };
    let param3 = p2.clone().replace("\"", "\\\"");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Param1: {}", __SHV_param1.with(|v| v.borrow().clone()))]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Param2: {}", param2.clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Param3: {}", param3.clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_result.with(|v| *v.borrow_mut() = { let __sh_split_s: String = {
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
    __sh_print_words(&[vec![__SHV_param1.with(|v| v.borrow().clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf3 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout4);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn("'sed' 's/old/new/g'", Some(&__sh_pbuf3)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }.join(" "));
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Result: {}", __SHV_result.with(|v| v.borrow().clone()))]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
}

#[allow(dead_code)]
fn test_locals(p0: String, p1: String) {
    let var1 = p0.clone();
    __SH_RC.store(0, Ordering::Relaxed);
    let var2 = { let __sh_value = p1.clone(); if __sh_value.is_empty() { "default_value".to_string() } else { __sh_value.to_string() } };
    __SH_RC.store(0, Ordering::Relaxed);
    let var3 = { let __sh_from_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); let __sh_to_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); var1.clone().chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() };
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Var1: {}", var1.clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Var2: {}", var2.clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_print_words(&[vec![format!("Var3: {}", var3.clone())]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
}