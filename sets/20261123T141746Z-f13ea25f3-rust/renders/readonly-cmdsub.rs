use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_TRAPS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stderr: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_tmp_stdout: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn __sh_wq(ws: &[String]) -> String {
    ws.iter().map(|w| __sh_q(w)).collect::<Vec<_>>().join(" ")
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

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn __sh_run_traps() {
    let hs = std::mem::take(&mut *__SH_TRAPS.lock().unwrap());
    for h in hs { let _ = __sh_spawn(&h, None); }
}

fn __sh_catf(argv: &[String]) -> Option<String> {
    let mut out = String::new();
    for p in argv {
        match std::fs::read(p) {
            Ok(d) => out.push_str(&String::from_utf8_lossy(&d)),
            Err(_) => return None,
        }
    }
    while out.ends_with('\n') { out.pop(); }
    Some(out)
}

fn __sh_mktemp(dir: bool, tmpl: &str) -> String {
    let t = if tmpl.is_empty() {
        let d = std::env::var("TMPDIR").ok().filter(|d| !d.is_empty()).unwrap_or_else(|| "/tmp".to_string());
        format!("{}/tmp.XXXXXXXXXX", d.trim_end_matches('/'))
    } else {
        tmpl.to_string()
    };
    let base = t.trim_end_matches('X').to_string();
    let nx = t.len() - base.len();
    let nx = if nx == 0 { 10 } else { nx };
    let cs: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    for n in 0..100000u64 {
        let seed = (std::process::id() as u64).wrapping_mul(1_000_003)
            ^ n.wrapping_mul(7_919)
            ^ std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos() as u64)
                .unwrap_or(0);
        let mut x = seed | 1;
        let mut s = base.clone();
        for _ in 0..nx { s.push(cs[(x % 62) as usize] as char); x /= 62; }
        let ok = if dir {
            std::fs::create_dir(&s).is_ok()
        } else {
            std::fs::OpenOptions::new().write(true).create_new(true).open(&s).is_ok()
        };
        if ok { return s; }
    }
    String::new()
}

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

fn main() -> std::process::ExitCode {
    print!("============================================================\n");
    print!(" SECTION 1: Basic readonly variable\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["01-readonly-var".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly MY_VAR=\"hello_world\"\n        echo \"MY_VAR=${MY_VAR}\"\n        # Attempt to change it (will fail, but we capture)\n        MY_VAR=\"changed\" 2>/dev/null || echo \"Assignment denied: $?\"\n        echo \"MY_VAR after attempted change=${MY_VAR}\"\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 2: readonly -p  (print readonly variables)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["02-readonly-p".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly FOO=alpha\n        readonly BAR=beta\n        readonly -p | head -5\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 3: readonly -f  (readonly functions)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["03-readonly-f".to_string(), "bash".to_string(), "-c".to_string(), "\n        myfunc() { echo \"Hello from myfunc\"; }\n        readonly -f myfunc\n        myfunc\n        # Attempt to unset the function (will fail)\n        unset -f myfunc 2>/dev/null || echo \"unset -f denied\"\n        # Verify function still exists\n        myfunc\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 4: readonly -a  (indexed array)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["04-readonly-a".to_string(), "bash".to_string(), "-c".to_string(), "\n        declare -a MY_ARR=(zero one two)\n        readonly -a MY_ARR\n        echo \"MY_ARR[0]=${MY_ARR[0]}\"\n        echo \"MY_ARR[1]=${MY_ARR[1]}\"\n        # Attempt to change an element (will fail)\n        MY_ARR[0]=ZERO 2>/dev/null || echo \"Array assignment denied\"\n        echo \"MY_ARR[0] after attempt=${MY_ARR[0]}\"\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 5: readonly -A  (associative array)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["05-readonly-A".to_string(), "bash".to_string(), "-c".to_string(), "\n        declare -A MY_ASSOC=([key1]=val1 [key2]=val2)\n        readonly -A MY_ASSOC\n        echo \"MY_ASSOC[key1]=${MY_ASSOC[key1]}\"\n        echo \"MY_ASSOC[key2]=${MY_ASSOC[key2]}\"\n        # Attempt to change (will fail)\n        MY_ASSOC[key1]=CHANGED 2>/dev/null || echo \"Assoc assignment denied\"\n        echo \"MY_ASSOC[key1] after attempt=${MY_ASSOC[key1]}\"\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 6: readonly --  (no more options)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["06a-readonly-dash-ok".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly -- MYVAR=hello\n        echo \"MYVAR=${MYVAR}\"\n        # Confirm it is indeed readonly\n        MYVAR=world 2>/dev/null || echo \"Assignment denied (expected)\"\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    { let __sh_operand: Vec<String> = vec!["06b-readonly-dash-invalid".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly -- -notvalid 2>&1 || true\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 7: Invalid variable name (error case)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["07-invalid-name".to_string(), "bash".to_string(), "-c".to_string(), "\n        readonly 123invalid 2>&1 || true\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    print!("============================================================\n");
    print!(" SECTION 8: readonly -f -p  (list readonly functions)\n");
    print!("============================================================\n");
    { let __sh_operand: Vec<String> = vec!["08-readonly-f-p".to_string(), "bash".to_string(), "-c".to_string(), "\n        f1() { :; }\n        f2() { :; }\n        readonly -f f1 f2\n        readonly -f -p | head -5\n    ".to_string()]; __sh_fn_capture(__sh_operand.get(0).cloned().unwrap_or_default(), &__sh_operand[(1).min(__sh_operand.len())..]); }
    println!();
    print!("All readonly demo sections completed.\n");
    __sh_run_traps();
    std::process::exit(0);
}

#[allow(dead_code)]
fn __sh_fn_capture(p0: String, __rest: &[String]) {
    let label = p0.clone();
    __SH_RC.store(0, Ordering::Relaxed);
    __SHV_tmp_stdout.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, "/tmp/readonly_demo_stdout_XXXXXX"); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    __SHV_tmp_stderr.with(|v| *v.borrow_mut() = { let __sh_ch = __sh_mktemp(false, "/tmp/readonly_demo_stderr_XXXXXX"); __SH_RC.store(if __sh_ch.is_empty() { 1 } else { 0 }, Ordering::Relaxed); __sh_ch });
    __SH_RC.store(__sh_spawn(&format!("{} 2> {}", format!("{} > {}", __sh_wq(&__rest.to_vec()), __sh_q(&__SHV_tmp_stdout.with(|v| v.borrow().clone()))), __sh_q(&__SHV_tmp_stderr.with(|v| v.borrow().clone()))), None), Ordering::Relaxed);
    let ec = ({ let __sh_split_s: String = __SH_RC.load(Ordering::Relaxed).to_string(); __sh_split_s.split(|__sh_ch: char| __sh_ch == ' ' || __sh_ch == '\t' || __sh_ch == '\n').filter(|__sh_w| !__sh_w.is_empty()).map(|__sh_w| __sh_w.to_string()).collect::<Vec<String>>() }).join(" ");
    let so = { let __sh_ch = __sh_catf(&vec![__SHV_tmp_stdout.with(|v| v.borrow().clone())]); __SH_RC.store(if __sh_ch.is_some() { 0 } else { 1 }, Ordering::Relaxed); __sh_ch.unwrap_or_default() };
    let se = { let __sh_ch = __sh_catf(&vec![__SHV_tmp_stderr.with(|v| v.borrow().clone())]); __SH_RC.store(if __sh_ch.is_some() { 0 } else { 1 }, Ordering::Relaxed); __sh_ch.unwrap_or_default() };
    {
        let mut __sh_status: i32 = 0;
        let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
            std::io::ErrorKind::NotFound => "No such file or directory".into(),
            std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
            std::io::ErrorKind::AlreadyExists => "File exists".into(),
            std::io::ErrorKind::IsADirectory => "Is a directory".into(),
            std::io::ErrorKind::NotADirectory => "Not a directory".into(),
            _ => format!("{}", __e) } };
        for __sh_path in &vec![__SHV_tmp_stdout.with(|v| v.borrow().clone()), __SHV_tmp_stderr.with(|v| v.borrow().clone())] {
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
    println!("--- [{}] ---", label.clone());
    println!("  cmd     : {}", __rest.to_vec().join(" "));
    println!("  exitcode: {}", ec.clone());
    if so.clone() != "".to_string() {
        println!("  stdout  : {}", so.clone());
    } else {
    }
    if se.clone() != "".to_string() {
        println!("  stderr  : {}", se.clone());
    } else {
    }
    println!();
    let __sh_value = ec.clone(); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed); return;
}