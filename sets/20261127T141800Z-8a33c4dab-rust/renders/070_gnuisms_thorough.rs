use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_aa: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_arr: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_k: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_pow(a: i64, b: i64) -> i64 {
    if b < 0 { eprintln!("sh2: {} ** {}: exponent less than 0", a, b); std::process::exit(1); }
    a.wrapping_pow(b.clamp(0, u32::MAX as i64) as u32)
}

fn __sh_atoi(s: &str) -> i64 {
    let t = s.trim();
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        return i64::from_str_radix(h, 16).unwrap_or(0);
    }
    t.parse::<i64>().unwrap_or(0)
}

fn __sh_print_words(ws: &[Vec<String>], nl: bool, _esc: bool) {
    if __SH_ARITH_WORD_FAIL.swap(false, Ordering::Relaxed) { return; }
    let mut s = ws.iter().flatten().map(String::as_str).collect::<Vec<_>>().join(" ");
    if nl { s.push('\n'); }
    __sh_sink(s.as_bytes());
}

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

fn __sh_run_argv(argv: &[&str]) -> i32 {
    __sh_spawn_argv(argv, None)
}

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){pat}") } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

fn __sh_cat_stmt(argv: &[String]) -> i32 {
    let mut rc = 0;
    for p in argv {
        match std::fs::read(p) {
            Ok(d) => __sh_sink(&d),
            Err(e) => {
                let m = match e.kind() {
                    std::io::ErrorKind::NotFound => "No such file or directory".into(),
                    std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
                    std::io::ErrorKind::IsADirectory => "Is a directory".into(),
                    _ => format!("{e}"),
                };
                eprintln!("cat: {}: {}", p, m);
                rc = 1;
            }
        }
    }
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
    let mut ly = 0i64;
    let pv: String;
    { let __sh_dir = "/tmp".to_string(); let __sh_res = std::env::set_current_dir(&__sh_dir); if __sh_res.is_ok() { std::env::set_var("PWD", std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()); } __SH_RC.store(if __sh_res.is_ok() { 0 } else { 1 }, Ordering::Relaxed); };
    { let __sh_buf = 5 > 3; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("arith:  {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = "abc" == "abc"; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("str:    {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = std::path::Path::new("/etc/passwd").is_file(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("file:   {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = { let __sh_mapped = __sh_regex(&"hello world".to_string().to_string(), &"^hello".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("regex:  {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = "".to_string().is_empty(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("empty:  {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = !"x".to_string().is_empty(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("nonempty: {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = "a".to_string() < "b".to_string(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("lt:     {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = "a" != "b"; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("ne:    {}", __SH_RC.load(Ordering::Relaxed));
    { let __sh_buf = !std::path::Path::new("/no/such/file").exists(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); __sh_buf };
    println!("notfile: {}", __SH_RC.load(Ordering::Relaxed));
    let x = "HeLLo WoRLD".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    println!("up={}", x.clone().to_uppercase());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("down={}", x.clone().to_lowercase());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("firstup={}", "HeLLo WoRLD");
    let y = "one-two-one".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    println!("gsub={}", "1-two-1");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("sub={}", "1-two-one");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("prefix={}", "two-one");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("suffix={}", "one-two");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("shortest={}", { let __sh_str: String = y.clone().to_string(); let __sh_delim: &str = "o"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("greedy={}", { let __sh_str: String = y.clone().to_string(); let __sh_delim: &str = "-"; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    let z = "0123456789".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    println!("mid={}", z.clone().chars().skip((2) as usize).take((3) as usize).collect::<String>());
    println!("tail={}", z.clone().chars().skip(5).collect::<String>());
    println!("neg={}", { let __sh_s = &z.clone(); let __sh_o = -3; let __sh_l = i64::MIN; let __sh_n = __sh_s.chars().count() as i64; let mut __sh_st = if __sh_o < 0 { __sh_n + __sh_o } else { __sh_o }; if __sh_st < 0 { __sh_st = 0; } if __sh_st > __sh_n { String::new() } else { let mut __sh_cn = if __sh_l == i64::MIN { __sh_n - __sh_st } else if __sh_l < 0 { __sh_n + __sh_l - __sh_st } else { __sh_l }; if __sh_cn < 0 { __sh_cn = 0; } if __sh_st + __sh_cn > __sh_n { __sh_cn = __sh_n - __sh_st; } __sh_s.chars().skip(__sh_st as usize).take(__sh_cn as usize).collect::<String>() } });
    println!("len={}", (z.clone().len() as i64));
    __SHV_arr.with(|v| *v.borrow_mut() = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]);
    __SH_RC.store(0, Ordering::Relaxed);
    println!("idx={}", { let __sh_idx = 1; let __sh_num = __SHV_arr.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_arr.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    println!("count={}", __SHV_arr.with(|v| v.borrow().len() as i64).to_string());
    println!("all={}", __SHV_arr.with(|v| v.borrow().clone()).join(" "));
    for _el in ["alpha", "beta", "gamma"] {
        println!("el={}", _el.to_string());
    }
    __SHV_aa.with(|v| { v.borrow_mut().insert("one".to_string(), "1".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_aa.with(|v| { v.borrow_mut().insert("two".to_string(), "2".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_aa.with(|v| { v.borrow_mut().insert("three".to_string(), "3".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("aa-one={}", __SHV_aa.with(|v| v.borrow().get("one").cloned().unwrap_or_default()));
    let __sh_items3 = { let __sh_split_s: String = {
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
    __sh_print_words(&[__SHV_aa.with(|v| v.borrow().keys().cloned().collect::<Vec<String>>())], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf1 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout2);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'tr' ' ' '\\n'", "'sort'"), Some(&__sh_pbuf1)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}; __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() };
    let mut __sh_index4: usize = 0;
    loop {
    if !(__sh_index4 < __sh_items3.len()) { break; }
        __SHV_k.with(|v| *v.borrow_mut() = __sh_items3[__sh_index4].clone());
        println!("aa[{}]={}", __SHV_k.with(|v| v.borrow().clone()), __SHV_aa.with(|v| v.borrow().get(&__SHV_k.with(|v| v.borrow().clone())).cloned().unwrap_or_default()));
    __sh_index4 += 1;
    }
    { let __sh_c = format!("{}\n", "hello herestring"); __sh_sink(__sh_c.as_bytes()); __SH_RC.store(0, Ordering::Relaxed); }
    {
        print!("both\n");
    }
    __SH_RC.store(__sh_cat_stmt(&vec!["/tmp/gnu_both.txt".to_string()]), Ordering::Relaxed);
    {
        let __oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = Some(std::fs::File::create("/dev/null").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap())));
        __sh_print_words(&[vec!["err-to-stderr".to_string()]], true, false);
        __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __oldout);
    }
    let mut ax = 8;
    __SH_RC.store(0, Ordering::Relaxed);
    println!("arith={}", ax);
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_arith_value5 = { let __sh_value = ax * 2; ly = __sh_value; __sh_value };
    let __sh_num6 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value5 };
    __SH_RC.store(if __sh_num6 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    println!("let={}", ly);
    __SH_RC.store(0, Ordering::Relaxed);
    let __sh_arith_value7 = { let __sh_out = ax; ax = ax +1; __sh_out };
    let __sh_num8 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value7 };
    __SH_RC.store(if __sh_num8 != 0 { 0 } else { 1 }, Ordering::Relaxed);
    println!("postfix={}", ax);
    __SH_RC.store(0, Ordering::Relaxed);
    println!("pow={}", __sh_pow(2,10));
    let mut __sh_pbuf9: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout10 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __SH_RC.store(1, Ordering::Relaxed);
    __sh_pbuf9 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout10);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    {
    let __oldin = __SH_STDIN.lock().unwrap().take();
    *__SH_STDIN.lock().unwrap() = Some(Box::new(std::io::Cursor::new(__sh_pbuf9.clone())));
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __oldin;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    println!("ps={},{}", { let __sh_pipestatus = __SH_PIPESTATUS.lock().unwrap(); __sh_pipestatus.get(0 as usize).cloned().unwrap_or(0).to_string() }, { let __sh_pipestatus = __SH_PIPESTATUS.lock().unwrap(); __sh_pipestatus.get(1 as usize).cloned().unwrap_or(0).to_string() });
    print!("no-nl");
    print!(".\n");
    print!("tab\there\n");
    __SH_RC.store(0, Ordering::Relaxed);
    pv = format!("value-{}", __sh_atoi("42"));
    println!("{}", pv.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("seq: {}", {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf11: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'seq' '1' '3'", "'tr' '\\n' ' '"), Some(&__sh_pbuf11)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
});
    __SH_RC.store(__sh_run_argv(&["ln", "-s", "-f", "/etc/passwd", "/tmp/gnu_link"]), Ordering::Relaxed);
    println!("readlink-f: {}", { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["readlink", "-f", "/tmp/gnu_link"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    println!("readlink-e: {}", { let (__sh_ch, __sh_res) = __sh_capture_rc_argv(&["readlink", "-e", "/tmp/gnu_link"]); __SH_RC.store(__sh_res, Ordering::Relaxed); __sh_ch });
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'readlink' '-m' '/no/such/path'", __sh_q("/dev/null")), None), Ordering::Relaxed);
    println!("readlink-m rc={}", __SH_RC.load(Ordering::Relaxed));
    let _ = std::fs::write("/tmp/gnu_a.txt", "abc\n");
    let _ = std::fs::write("/tmp/gnu_b.txt", "abd\n");
    __SH_RC.store(__sh_run_argv(&["cmp", "-b", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt"]), Ordering::Relaxed);
    println!("cmp-b rc={}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(__sh_run_argv(&["cmp", "-n", "2", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt"]), Ordering::Relaxed);
    println!("cmp-n rc={}", __SH_RC.load(Ordering::Relaxed));
    __SH_RC.store(__sh_run_argv(&["cmp", "-i", "2", "/tmp/gnu_a.txt", "/tmp/gnu_b.txt"]), Ordering::Relaxed);
    println!("cmp-i rc={}", __SH_RC.load(Ordering::Relaxed));
    let mut __sh_pbuf12: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout13 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec!["foo\nbar123\n".to_string()]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf12 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout13);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-E' '[0-9]+'", __sh_q("/dev/null")), Some(&__sh_pbuf12)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!("grep-P rc={}", __SH_RC.load(Ordering::Relaxed));
    let mut __sh_pbuf14: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout15 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_puts("a1b2c3\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf14 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout15);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["sed", "-r", "s/[0-9]+/N/g"], Some(&__sh_pbuf14)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!("{}", { let __sh_from_set: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect(); let __sh_to_set: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(); "AbCdEf".to_string().chars().map(|__sh_ch| match __sh_from_set.iter().position(|&__sh_from_ch| __sh_from_ch == __sh_ch) { Some(__sh_begin) => __sh_to_set[__sh_begin.min(__sh_to_set.len() - 1)], None => __sh_ch }).collect::<String>() });
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    println!("{}", { let __sh_mapped: String = "  spaced  ".to_string(); let __sh_squeeze: Vec<char> = " ".chars().collect(); let mut __sh_out = String::new(); for __sh_ch in __sh_mapped.chars() { if __sh_out.chars().last() == Some(__sh_ch) && __sh_squeeze.contains(&__sh_ch) { continue; } __sh_out.push(__sh_ch); } __sh_out });
    __SH_RC.store((0) as i32, Ordering::Relaxed);
    let mut __sh_pbuf16: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout17 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec!["10K\n2M\n500\n".to_string()]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf16 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout17);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["sort", "-h"], Some(&__sh_pbuf16)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    let mut __sh_pbuf18: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout19 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec!["short\nvery-long-line-here\n".to_string()]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf18 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout19);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["wc", "-L"], Some(&__sh_pbuf18)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    let mut __sh_pbuf20: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout21 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec!["abcdef".to_string()]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf20 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout21);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["head", "-c", "3"], Some(&__sh_pbuf20)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!();
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_pbuf22: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout23 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    __sh_print_words(&[vec!["abcdef".to_string()]], false, false);
    __SH_RC.store(0, Ordering::Relaxed);
    __sh_pbuf22 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout23);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
}
    __SH_RC.store(__sh_spawn_argv(&["tail", "-c", "2"], Some(&__sh_pbuf22)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    println!();
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = "ABC".to_string().to_lowercase() == "abc".to_string().to_lowercase(); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        print!("nocase-match\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = "ABC" == "abc" && {
print!("case-matters\n");
__SH_RC.store(0, Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("case-sensitive\n");
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
            _ => format!("{__e}") } };
        for __sh_path in &vec!["/tmp/gnu_both.txt".to_string(), "/tmp/gnu_link".to_string(), "/tmp/gnu_a.txt".to_string(), "/tmp/gnu_b.txt".to_string()] {
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