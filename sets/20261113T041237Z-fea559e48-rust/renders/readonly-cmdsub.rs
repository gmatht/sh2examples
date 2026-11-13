use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stderr: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stdout: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

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

fn main() {
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 1: Basic readonly variable".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["01-readonly-var".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly MY_VAR=\"hello_world\"\n        echo \"MY_VAR=${MY_VAR}\"\n        # Attempt to change it (will fail, but we capture)\n        MY_VAR=\"changed\" 2>/dev/null || echo \"Assignment denied: $?\"\n        echo \"MY_VAR after attempted change=${MY_VAR}\"\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 2: readonly -p  (print readonly variables)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["02-readonly-p".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly FOO=alpha\n        readonly BAR=beta\n        readonly -p | head -5\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 3: readonly -f  (readonly functions)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["03-readonly-f".to_string(), "bash".to_string(), "-c".to_string(), "\n        myfunc() { echo \"Hello from myfunc\"; }\n        readonly -f myfunc\n        myfunc\n        # Attempt to unset the function (will fail)\n        unset -f myfunc 2>/dev/null || echo \"unset -f denied\"\n        # Verify function still exists\n        myfunc\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 4: readonly -a  (indexed array)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["04-readonly-a".to_string(), "bash".to_string(), "-c".to_string(), "\n        declare -a MY_ARR=(zero one two)\n        readonly -a MY_ARR\n        echo \"MY_ARR[0]=${MY_ARR[0]}\"\n        echo \"MY_ARR[1]=${MY_ARR[1]}\"\n        # Attempt to change an element (will fail)\n        MY_ARR[0]=ZERO 2>/dev/null || echo \"Array assignment denied\"\n        echo \"MY_ARR[0] after attempt=${MY_ARR[0]}\"\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 5: readonly -A  (associative array)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["05-readonly-A".to_string(), "bash".to_string(), "-c".to_string(), "\n        declare -A MY_ASSOC=([key1]=val1 [key2]=val2)\n        readonly -A MY_ASSOC\n        echo \"MY_ASSOC[key1]=${MY_ASSOC[key1]}\"\n        echo \"MY_ASSOC[key2]=${MY_ASSOC[key2]}\"\n        # Attempt to change (will fail)\n        MY_ASSOC[key1]=CHANGED 2>/dev/null || echo \"Assoc assignment denied\"\n        echo \"MY_ASSOC[key1] after attempt=${MY_ASSOC[key1]}\"\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 6: readonly --  (no more options)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["06a-readonly-dash-ok".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly -- MYVAR=hello\n        echo \"MYVAR=${MYVAR}\"\n        # Confirm it is indeed readonly\n        MYVAR=world 2>/dev/null || echo \"Assignment denied (expected)\"\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    { let __a: Vec<String> = vec!["06b-readonly-dash-invalid".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly -- -notvalid 2>&1 || true\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 7: Invalid variable name (error case)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["07-invalid-name".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly 123invalid 2>&1 || true\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "============================================================".to_string());
    println!("{}", " SECTION 8: readonly -f -p  (list readonly functions)".to_string());
    println!("{}", "============================================================".to_string());
    { let __a: Vec<String> = vec!["08-readonly-f-p".to_string(), "bash".to_string(), "-c".to_string(), "\n        f1() { :; }\n        f2() { :; }\n        readonly -f f1 f2\n        readonly -f -p | head -5\n    ".to_string()]; __sh_fn_capture(__a.get(0).cloned().unwrap_or_default(), &__a[(1).min(__a.len())..]); }
    println!("{}", "".to_string());
    println!("{}", "All readonly demo sections completed.".to_string());
    std::process::exit(0);
}

fn __sh_fn_capture(p0: String, __rest: &[String]) {
    let label = p0.clone();
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_tmp_stdout.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("/tmp/readonly_demo_stdout_XXXXXX").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    __SHV_tmp_stderr.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("mktemp").arg("/tmp/readonly_demo_stderr_XXXXXX").stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    __SH_RC.store(__sh_spawn(&format!("{} 2> {}", format!("{} > {}", { let mut __o = String::new(); for (__i, __w) in {&[__rest.to_vec()].concat()}.iter().enumerate() { if __i > 0 { __o.push(' '); } __o.push_str(&{ let __t: &str = {__w}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }); } __o }, { let __t: &str = {&__SHV_tmp_stdout.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }), { let __t: &str = {&__SHV_tmp_stderr.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }), None), Ordering::SeqCst);
    let ec = { let __si = __SH_RC.load(Ordering::SeqCst).to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" ");
    __SH_RC.store(0, Ordering::SeqCst);
    let so = { let (__c, __r) = { let __sc = std::process::Command::new("cat").arg(__SHV_tmp_stdout.with(|v| v.borrow().clone())).stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    __SH_RC.store(0, Ordering::SeqCst);
    let se = { let (__c, __r) = { let __sc = std::process::Command::new("cat").arg(__SHV_tmp_stderr.with(|v| v.borrow().clone())).stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c };
    __SH_RC.store(__sh_spawn(&format!("{}{}", "'rm' '-f' ", vec![{ let __t: &str = {&__SHV_tmp_stdout.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }, { let __t: &str = {&__SHV_tmp_stderr.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o }].join(" ")), None), Ordering::SeqCst);
    println!("{}", format!("--- [{}] ---", label.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("  cmd     : {}", __rest.to_vec().join(" ")));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("  exitcode: {}", ec.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    if so.clone() != "".to_string() {
        println!("{}", format!("  stdout  : {}", so.clone()));
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if se.clone() != "".to_string() {
        println!("{}", format!("  stderr  : {}", se.clone()));
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    print!("{}", "\n");
    __SH_RC.store(0, Ordering::SeqCst);
    __SH_RC.store((ec.clone().trim().parse::<i64>().unwrap_or(0)) as i32, Ordering::SeqCst); return;
}