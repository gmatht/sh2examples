use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_a: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_b: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_c: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_file_info: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_letters: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_matrix: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_numbers: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_result: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_value: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_is_composite_id(s: &str) -> bool {
    // tolerate the trailing newline a `$( )` capture (or the value
    // function's own trailing echo) leaves on the id
    let s = s.strip_suffix('\n').unwrap_or(s);
    if s.len() < 6 { return false; }
    if !(s.starts_with("list#") || s.starts_with("obj#")) { return false; }
    s[5..].parse::<usize>().is_ok()
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return (String::new(), 1) };
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
    // a shell-out inside a native stage inherits the stage's stdin
    // (`echo x | grep -f <(…)` — the grep reads the echo's buffer)
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
        // the child inherits fd 1 raw — flush Rust's buffered stdout
        // first so prints BEFORE the child hit the pipe first
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if input.is_none() {
        if let Some(p) = &stdin_path {
            // the child must see the REAL device fd (`tty < /dev/pts/5`
            // needs isatty, which a byte pipe lacks)
            if let Ok(f) = std::fs::File::open(p) {
                c.stdin(std::process::Stdio::from(f));
            }
        }
    }
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() {
        let mut used = false;
        __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &out); used = true; } });
        if !used {
            if let Some(b) = __SH_OUT.lock().unwrap().as_mut() {
                b.extend_from_slice(&out);
            } else {
                let _ = std::io::Write::write_all(&mut std::io::stdout(), &out);
            }
        }
    }
    rc
}

fn __sh_split_ifs(s: &str, ifs: &str) -> Vec<String> {
    // A composite store id (`list#3` / `obj#7`) is an OPAQUE token, not
    // text: word-splitting it would shred the id into "list#" and "3" and
    // lose the reference (t108 `mktoks2() ([]tok, error)`, whose return
    // value rides a capture that word-splits). Such a value is already a
    // single field, so the split is the identity on it.
    if __sh_is_composite_id(s) { return vec![s.to_string()]; }
    if ifs.is_empty() { return vec![s.to_string()]; }
    let ws = ifs.chars().all(|c| c == ' ' || c == '\t' || c == '\n');
    let ch: Vec<char> = s.chars().collect();
    let is_sep = |c: char| ifs.contains(c);
    let mut out = Vec::new();
    let mut i = 0;
    while i < ch.len() && is_sep(ch[i]) { i += 1; }
    while i < ch.len() {
        let start = i;
        while i < ch.len() && !is_sep(ch[i]) { i += 1; }
        out.push(ch[start..i].iter().collect());
        if i >= ch.len() { break; }
        i += 1;
        if ws { while i < ch.len() && is_sep(ch[i]) { i += 1; } }
    }
    out
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

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "=== Advanced Bash Idioms Examples ===".to_string());
    println!("{}", "".to_string());
    println!("{}", "1. Nested loops with conditional logic and array manipulation:".to_string());
    __SHV_numbers.with(|v| *v.borrow_mut() = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()]);
    __SHV_letters.with(|v| *v.borrow_mut() = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]);
    __SH_RC.store(0, Ordering::SeqCst);
    for num in ["1", "2", "3", "4", "5"] {
        for letter in ["a", "b", "c", "d", "e"] {
            if (num.trim().parse::<i64>().unwrap_or(0) > 3) && (letter != "c".to_string()) {
                println!("{}", format!("  Number {} with letter {} (filtered)", num, letter));
            }
        }
    }
    println!("{}", "".to_string());
    println!("{}", "2. Function with nested case statements and parameter expansion:".to_string());
    println!("{}", process_data("string".to_string().to_string(), "Hello".to_string().to_string()));
    println!("{}", process_data("string".to_string().to_string(), "Bye".to_string().to_string()));
    println!("{}", process_data("number".to_string().to_string(), "42".to_string().to_string()));
    println!("{}", process_data("number".to_string().to_string(), "17".to_string().to_string()));
    println!("{}", "".to_string());
    println!("{}", "3. Complex conditional with command substitution and arithmetic:".to_string());
    let file_count = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("find").arg(".").arg("-maxdepth").arg("1").arg("-type").arg("f").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("wc").arg("-l").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    let dir_count = {
    let __cap1_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap1_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap1_in = __SH_STDIN.lock().unwrap().take();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("find").arg(".").arg("-maxdepth").arg("1").arg("-type").arg("d").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("wc").arg("-l").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap1_in;
    let __cap1_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap1_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap1_outf);
    let mut __s = String::from_utf8_lossy(&__cap1_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    if (file_count.clone().trim().parse::<i64>().unwrap_or(0) > 0) && (dir_count.clone().trim().parse::<i64>().unwrap_or(0) > 1) {
        if file_count.clone().trim().parse::<i64>().unwrap_or(0) > dir_count.clone().trim().parse::<i64>().unwrap_or(0) {
            println!("{}", format!("  More files ({}) than directories ({})", file_count.clone(), dir_count.clone()));
        } else {
            if file_count.clone().trim().parse::<i64>().unwrap_or(0) == dir_count.clone().trim().parse::<i64>().unwrap_or(0) {
                println!("{}", format!("  Equal count: {} files and {} directories", file_count.clone(), dir_count.clone()));
            } else {
                println!("{}", format!("  More directories ({}) than files ({})", dir_count.clone(), file_count.clone()));
            }
        }
    } else {
        println!("{}", "  Insufficient items for comparison".to_string());
    }
    println!("{}", "".to_string());
    println!("{}", "4. Nested here-documents with parameter expansion:".to_string());
    let user = "admin".to_string();
    let host = "localhost".to_string();
    let port = 22;
    print!("{}", "    SSH Configuration:\n    $(cat <<'INNER'\n        User: $user\n        Host: $host\n        Port: $port\n        Status: $(ping -c 1 $host >/dev/null 2>&1 && echo \"Online\" || echo \"Offline\")\nINNER\n    )\n".to_string());
    println!("{}", "".to_string());
    println!("{}", "5. Array processing with nested loops and conditional logic:".to_string());
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,0".to_string(), "1".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,1".to_string(), "2".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("0,2".to_string(), "3".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,0".to_string(), "4".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,1".to_string(), "5".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("1,2".to_string(), "6".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,0".to_string(), "7".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,1".to_string(), "8".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_matrix.with(|v| { v.borrow_mut().insert("2,2".to_string(), "9".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    for __sh_w6 in ["0", "1", "2"] {
        __SHV_i.with(|v| *v.borrow_mut() = __sh_w6.to_string());
        for __sh_w7 in ["0", "1", "2"] {
            __SHV_j.with(|v| *v.borrow_mut() = __sh_w7.to_string());
            __SHV_value.with(|v| *v.borrow_mut() = __SHV_matrix.with(|v| v.borrow().get(&(format!("{},{}", __SHV_i.with(|v| v.borrow().clone()), __SHV_j.with(|v| v.borrow().clone())))).cloned().unwrap_or_default()));
            if __SHV_value.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) > 5 {
                print!("{}", format!("  [{}] ", __SHV_value.with(|v| v.borrow().clone())));
            } else {
                print!("{}", format!("  {} ", __SHV_value.with(|v| v.borrow().clone())));
            }
        }
        println!("{}", "".to_string());
    }
    println!("{}", "".to_string());
    println!("{}", "6. Process substitution with nested commands and error handling:".to_string());
    println!("{}", format!("  First word: {}", { let __v = {&std::env::var("test_string").unwrap_or_default()}.to_string(); match __v.find(" ") { Some(__i) => __v[..__i].to_string(), None => __v } }));
    println!("{}", format!("  Last word: {}", { let __v = {&std::env::var("test_string").unwrap_or_default()}.to_string(); match __v.rfind(" ") { Some(__i) => __v[__i + " ".len()..].to_string(), None => __v } }));
    println!("{}", format!("  Middle: {}", { let __v = {&std::env::var("test_string").unwrap_or_default()}.to_string(); match __v.find(" ") { Some(__i) => __v[__i + " ".len()..].to_string(), None => __v } }));
    println!("{}", format!("  Middle: {}", { let __v = {&std::env::var("test_string").unwrap_or_default()}.to_string(); match __v.rfind(" ") { Some(__i) => __v[..__i].to_string(), None => __v } }));
    println!("{}", format!("  Uppercase: {}", std::env::var("test_string").unwrap_or_default().to_uppercase()));
    println!("{}", format!("  Lowercase: {}", std::env::var("test_string").unwrap_or_default().to_lowercase()));
    println!("{}", format!("  Capitalize: {}", { let __t = std::env::var("test_string").unwrap_or_default(); let mut __c = __t.chars(); match __c.next() { Some(__f) => __f.to_uppercase().collect::<String>() + __c.as_str(), None => String::new() } }));
    println!("{}", "".to_string());
    println!("{}", "11. Complex arithmetic with nested expressions:".to_string());
    __SHV_a.with(|v| v.set(10));
    __SHV_b.with(|v| v.set(5));
    __SHV_c.with(|v| v.set(3));
    __SHV_result.with(|v| *v.borrow_mut() = { let __v = ((((__SHV_a.with(|v| v.get()) + __SHV_b.with(|v| v.get())) * __SHV_c.with(|v| v.get())) - { let __a = { let __a = __SHV_a.with(|v| v.get()); let __b = __SHV_b.with(|v| v.get()); if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a % __b } }; let __b = __SHV_c.with(|v| v.get()); if __b == 0 { __SH_ARITH_ERR.store(true, Ordering::SeqCst); 0 } else { __a / __b } })).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } });
    println!("{}", "  Expression: (a + b) * c - (a % b) / c".to_string());
    println!("{}", format!("  Values: a={}, b={}, c={}", __SHV_a.with(|v| v.get()), __SHV_b.with(|v| v.get()), __SHV_c.with(|v| v.get())));
    println!("{}", format!("  Result: {}", __SHV_result.with(|v| v.borrow().clone())));
    if ((__SHV_a.with(|v| v.get()) > __SHV_b.with(|v| v.get())) && (__SHV_b.with(|v| v.get()) < __SHV_c.with(|v| v.get()))) || ((__SHV_a.with(|v| v.get()) % 2) == 0) {
        println!("{}", "  Complex condition met: a > b AND (b < c OR a is even)".to_string());
    }
    println!("{}", "".to_string());
    println!("{}", "12. Nested command substitution with error handling:".to_string());
    println!("{}", format!("  Current directory: {}", {
    let __cap2_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap2_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap2_in = __SH_STDIN.lock().unwrap().take();
    { let __w: &[Vec<String>] = {&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap2_in;
    let __cap2_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap2_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap2_outf);
    let mut __s = String::from_utf8_lossy(&__cap2_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
}));
    println!("{}", format!("  Parent directory: {}", { let (__c, __r) = __sh_capture_rc(&{ let mut __o = String::new(); for (__i, __w) in {&[vec!["dirname".to_string()], { let __si = {
    let __cap3_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap3_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap3_in = __SH_STDIN.lock().unwrap().take();
    { let __w: &[Vec<String>] = {&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap3_in;
    let __cap3_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap3_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap3_outf);
    let mut __s = String::from_utf8_lossy(&__cap3_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
}; let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }); __SH_RC.store(__r, Ordering::SeqCst); __c }));
    println!("{}", format!("  Home directory: {}", { let (__c, __r) = __sh_capture_rc(&{ let mut __o = String::new(); for (__i, __w) in {&[vec!["dirname".to_string()], { let __si = { let (__c, __r) = __sh_capture_rc(&{ let mut __o = String::new(); for (__i, __w) in {&[vec!["dirname".to_string()], __sh_split_ifs(&{
    let __cap4_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap4_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap4_in = __SH_STDIN.lock().unwrap().take();
    { let __w: &[Vec<String>] = {&[vec![std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default()]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap4_in;
    let __cap4_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap4_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap4_outf);
    let mut __s = String::from_utf8_lossy(&__cap4_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
}, " \t\n")].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }); __SH_RC.store(__r, Ordering::SeqCst); __c }; let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }); __SH_RC.store(__r, Ordering::SeqCst); __c }));
    __SHV_file_info.with(|v| *v.borrow_mut() = {
    let __cap5_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap5_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap5_in = __SH_STDIN.lock().unwrap().take();
    if !{
__SH_RC.store(__sh_spawn(&format!("{} 2> {}", "'stat' '-c' '%s %y' 'nonexistent_file'".to_string(), "'/dev/null'".to_string()), None), Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
} {
        { let __t: &str = {"File not found\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
        __SH_RC.store(0, Ordering::SeqCst);
    }
    *__SH_STDIN.lock().unwrap() = __cap5_in;
    let __cap5_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap5_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap5_outf);
    let mut __s = String::from_utf8_lossy(&__cap5_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    println!("{}", format!("  File info: {}", __SHV_file_info.with(|v| v.borrow().clone())));
    println!("{}", "".to_string());
    println!("{}", "=== Advanced Bash Idioms Examples Complete ===".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
}

fn process_data(p0: String, p1: String) -> String {
    let data_type = p0.clone();
    __SHV_value.with(|v| *v.borrow_mut() = p1.clone());
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_case8 = data_type.clone();
    if (__sh_case8 == "string") {
        let __sh_case9 = __SHV_value.with(|v| v.borrow().clone()).to_lowercase();
        if (__sh_case9 == "hello") || (__sh_case9 == "hi") {
            let __v = format!("  Greeting detected: {}", __SHV_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
        } else if ((__sh_case9 == "bye") || (__sh_case9 == "goodbye")) {
            let __v = format!("  Farewell detected: {}", __SHV_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
        } else if true {
            let __v = format!("  Unknown string: {}", __SHV_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
        } else {
            __SH_RC.store(0, Ordering::SeqCst);
        }
    } else if ((__sh_case8 == "number")) {
        if __sh_regex(&__SHV_value.with(|v| v.borrow().clone()), "^[0-9]+$", "") {
            if (__SHV_value.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) % 2) == 0 {
                let __v = format!("  Even number: {}", __SHV_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
            } else {
                let __v = format!("  Odd number: {}", __SHV_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
            }
        } else {
            let __v = format!("  Invalid number: {}", __SHV_value.with(|v| v.borrow().clone())); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
        }
    } else if true {
        let __v = format!("  Unknown data type: {}", data_type.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    __SH_RET.lock().unwrap().clone()
}