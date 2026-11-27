use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_letters: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_matrix: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_numbers: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_value: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
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

fn __sh_case(s: &str, mode: &str) -> String {
    let mut it = s.chars();
    match mode {
        "^^" => s.to_uppercase(),
        ",," => s.to_lowercase(),
        "^" => it.next().map(|c| c.to_uppercase().collect::<String>() + it.as_str()).unwrap_or_default(),
        "," => it.next().map(|c| c.to_lowercase().collect::<String>() + it.as_str()).unwrap_or_default(),
        _ => s.to_string(),
    }
}

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){pat}") } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

fn __sh_div(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a / b }
}

fn __sh_mod(a: i64, b: i64) -> i64 {
    if b == 0 { __SH_ARITH_ERR.store(true, Ordering::Relaxed); 0 } else { a % b }
}

fn __sh_dirname_args(argv: &[String]) -> Option<String> {
    if argv.is_empty() {
        eprintln!("dirname: missing operand\nTry 'dirname --help' for more information.");
        return None;
    }
    let mut out: Vec<String> = Vec::new();
    for p in argv {
        if p.is_empty() {
            out.push(".".to_string());
            continue;
        }
        let t = p.trim_end_matches('/');
        if t.is_empty() {
            out.push("/".to_string());
            continue;
        }
        out.push(match t.rfind('/') {
            None => ".".to_string(),
            Some(i) => {
                let d = t[..i].trim_end_matches('/');
                if d.is_empty() { "/".to_string() } else { d.to_string() }
            }
        });
    }
    Some(out.join("\n"))
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
    let mut a = 0i64;
    let mut b = 0i64;
    let mut c = 0i64;
    let file_info: String;
    let result: String;
    print!("=== Advanced Bash Idioms Examples ===\n");
    println!();
    print!("1. Nested loops with conditional logic and array manipulation:\n");
    __SHV_numbers.with(|v| *v.borrow_mut() = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()]);
    __SHV_letters.with(|v| *v.borrow_mut() = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]);
    __SH_RC.store(0, Ordering::Relaxed);
    for _num in ["1", "2", "3", "4", "5"] {
        for _letter in ["a", "b", "c", "d", "e"] {
            if (_num.to_string().trim().parse::<i64>().unwrap_or(0) > 3) && (_letter.to_string() != "c".to_string()) {
                println!("  Number {} with letter {} (filtered)", _num.to_string(), _letter.to_string());
            }
        }
    }
    println!();
    print!("2. Function with nested case statements and parameter expansion:\n");
    __SH_RC.store(0, Ordering::Relaxed);
    process_data("string".to_string(), "Hello".to_string());
    process_data("string".to_string(), "Bye".to_string());
    process_data("number".to_string(), "42".to_string());
    process_data("number".to_string(), "17".to_string());
    println!();
    print!("3. Complex conditional with command substitution and arithmetic:\n");
    __SH_RC.store(0, Ordering::Relaxed);
    let file_count = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'find' '.' '-maxdepth' '1' '-type' 'f'", "'wc' '-l'"), Some(&__sh_pbuf2)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    let dir_count = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf3: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", "'find' '.' '-maxdepth' '1' '-type' 'd'", "'wc' '-l'"), Some(&__sh_pbuf3)), Ordering::Relaxed);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    if (file_count.trim().parse::<i64>().unwrap_or(0) > 0) && (dir_count.trim().parse::<i64>().unwrap_or(0) > 1) {
        if file_count.trim().parse::<i64>().unwrap_or(0) > dir_count.trim().parse::<i64>().unwrap_or(0) {
            println!("  More files ({}) than directories ({})", file_count.clone(), dir_count.clone());
        } else {
            if file_count.trim().parse::<i64>().unwrap_or(0) == dir_count.trim().parse::<i64>().unwrap_or(0) {
                println!("  Equal count: {} files and {} directories", file_count.clone(), dir_count.clone());
            } else {
                println!("  More directories ({}) than files ({})", dir_count.clone(), file_count.clone());
            }
        }
    } else {
        print!("  Insufficient items for comparison\n");
    }
    println!();
    print!("4. Nested here-documents with parameter expansion:\n");
    let _ = "admin".to_string();
    let _ = "localhost".to_string();
    let _ = 22;
    print!("    SSH Configuration:\n    $(cat <<'INNER'\n        User: $user\n        Host: $host\n        Port: $port\n        Status: $(ping -c 1 $host >/dev/null 2>&1 && echo \"Online\" || echo \"Offline\")\nINNER\n    )\n");
    println!();
    print!("5. Array processing with nested loops and conditional logic:\n");
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,0".to_string(), "1".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,1".to_string(), "2".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,2".to_string(), "3".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,0".to_string(), "4".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,1".to_string(), "5".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,2".to_string(), "6".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,0".to_string(), "7".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,1".to_string(), "8".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,2".to_string(), "9".to_string()); });
    __SH_RC.store(0, Ordering::Relaxed);
    for __sh_w4 in [0, 1, 2] {
        __SHV_i.with(|v| v.set(__sh_w4));
        for __sh_w5 in [0, 1, 2] {
            __SHV_j.with(|v| v.set(__sh_w5));
            __SHV_value.with(|v| *v.borrow_mut() = __SHV_matrix.with(|v| v.borrow().get(&format!("{},{}", __SHV_i.with(|v| v.get()).to_string(), __SHV_j.with(|v| v.get()).to_string())).cloned().unwrap_or_default()));
            __SH_RC.store(0, Ordering::Relaxed);
            if __SHV_value.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) > 5 {
                print!("  [{}] ", __SHV_value.with(|v| v.borrow().clone()));
                __SH_RC.store(0, Ordering::Relaxed);
            } else {
                print!("  {} ", __SHV_value.with(|v| v.borrow().clone()));
                __SH_RC.store(0, Ordering::Relaxed);
            }
        }
        println!();
        __SH_RC.store(0, Ordering::Relaxed);
    }
    println!();
    print!("6. Process substitution with nested commands and error handling:\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  First word: {}", { let __sh_str: String = std::env::var("test_string").unwrap_or_default().to_string(); let __sh_delim: &str = " "; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  Last word: {}", { let __sh_str: String = std::env::var("test_string").unwrap_or_default().to_string(); let __sh_delim: &str = " "; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  Middle: {}", { let __sh_str: String = std::env::var("test_string").unwrap_or_default().to_string(); let __sh_delim: &str = " "; match __sh_str.find(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  Middle: {}", { let __sh_str: String = std::env::var("test_string").unwrap_or_default().to_string(); let __sh_delim: &str = " "; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  Uppercase: {}", std::env::var("test_string").unwrap_or_default().to_uppercase());
    println!("  Lowercase: {}", std::env::var("test_string").unwrap_or_default().to_lowercase());
    println!("  Capitalize: {}", __sh_case(&std::env::var("test_string").unwrap_or_default(), "^"));
    println!();
    print!("11. Complex arithmetic with nested expressions:\n");
    a = 10;
    b = 5;
    c = 3;
    __SH_RC.store(0, Ordering::Relaxed);
    result = { let __sh_value = ((((a + b) * c) - __sh_div(__sh_mod(a,b),c))).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
    print!("  Expression: (a + b) * c - (a % b) / c\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  Values: a={}, b={}, c={}", a, b, c);
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  Result: {}", result.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    if ((a > b) && (b < c)) || ((a % 2) == 0) {
        print!("  Complex condition met: a > b AND (b < c OR a is even)\n");
    }
    println!();
    print!("12. Nested command substitution with error handling:\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("  Current directory: {}", {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
});
    println!("  Parent directory: {}", match __sh_dirname_args(&vec![{
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}]) { Some(__sh_ch) => { __SH_RC.store(0, Ordering::Relaxed); __sh_ch } None => { __SH_RC.store(1, Ordering::Relaxed); String::new() } });
    println!("  Home directory: {}", match __sh_dirname_args(&vec![match __sh_dirname_args(&vec![{
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    __sh_print_words(&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]], true, false);
    __SH_RC.store(0, Ordering::Relaxed);
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
}]) { Some(__sh_ch) => { __SH_RC.store(0, Ordering::Relaxed); __sh_ch } None => { __SH_RC.store(1, Ordering::Relaxed); String::new() } }]) { Some(__sh_ch) => { __SH_RC.store(0, Ordering::Relaxed); __sh_ch } None => { __SH_RC.store(1, Ordering::Relaxed); String::new() } });
    __SH_RC.store(0, Ordering::Relaxed);
    file_info = {
    let __cap_oldout = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __sh_savedout = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap_oldin = __SH_STDIN.lock().unwrap().take();
    { let __sh_buf = {
__SH_RC.store(__sh_spawn(&format!("{} 2> {}", "'stat' '-c' '%s %y' 'nonexistent_file'", __sh_q("/dev/null")), None), Ordering::Relaxed);
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        __sh_puts("File not found\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    *__SH_STDIN.lock().unwrap() = __cap_oldin;
    let __sh_captured = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __sh_savedout;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap_oldout);
    let mut __sh_str = String::from_utf8_lossy(&__sh_captured).to_string();
    while __sh_str.ends_with('\n') { __sh_str.pop(); }
    __sh_str
};
    println!("  File info: {}", file_info.clone());
    println!();
    print!("=== Advanced Bash Idioms Examples Complete ===\n");
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn process_data(p0: String, p1: String) {
    let __sh_loc_value: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    let data_type = p0.clone();
    __SHV_value.with(|v| *v.borrow_mut() = p1.clone());
    __SH_RC.store(0, Ordering::Relaxed);
    if data_type.clone() == "string".to_string() {
        let __sh2_casesubj_0_ = format!("{}", __SHV_value.with(|v| v.borrow().clone()).to_lowercase());
        __SH_RC.store(0, Ordering::Relaxed);
        if (__sh2_casesubj_0_.clone() == "hello".to_string()) || (__sh2_casesubj_0_.clone() == "hi".to_string()) {
            __sh_print_words(&[vec![format!("  Greeting detected: {}", __SHV_value.with(|v| v.borrow().clone()))]], true, false);
            __SH_RC.store(0, Ordering::Relaxed);
        } else if (__sh2_casesubj_0_.clone() == "bye".to_string()) || (__sh2_casesubj_0_.clone() == "goodbye".to_string()) {
            __sh_print_words(&[vec![format!("  Farewell detected: {}", __SHV_value.with(|v| v.borrow().clone()))]], true, false);
            __SH_RC.store(0, Ordering::Relaxed);
        } else {
            __sh_print_words(&[vec![format!("  Unknown string: {}", __SHV_value.with(|v| v.borrow().clone()))]], true, false);
            __SH_RC.store(0, Ordering::Relaxed);
        }
    } else if data_type.clone() == "number".to_string() {
        if { let __sh_mapped = __sh_regex(&__SHV_value.with(|v| v.borrow().clone()).to_string(), &"^[0-9]+$".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } } {
            if (__SHV_value.with(|v| v.borrow().trim().parse::<i64>().unwrap_or(0)) % 2i64) == 0 {
                __sh_print_words(&[vec![format!("  Even number: {}", __SHV_value.with(|v| v.borrow().clone()))]], true, false);
                __SH_RC.store(0, Ordering::Relaxed);
            } else {
                __sh_print_words(&[vec![format!("  Odd number: {}", __SHV_value.with(|v| v.borrow().clone()))]], true, false);
                __SH_RC.store(0, Ordering::Relaxed);
            }
        } else {
            __sh_print_words(&[vec![format!("  Invalid number: {}", __SHV_value.with(|v| v.borrow().clone()))]], true, false);
            __SH_RC.store(0, Ordering::Relaxed);
        }
    } else {
        __sh_print_words(&[vec![format!("  Unknown data type: {}", data_type.clone())]], true, false);
        __SH_RC.store(0, Ordering::Relaxed);
    }
    match __sh_loc_value.take() { Some(__sh_loc) => { __SHV_value.with(|v| *v.borrow_mut() = __sh_loc); } _ => {} };
}