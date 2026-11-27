use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_arr: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_assoc: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_comb: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_comb2: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_global_var: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_lc: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_myexport: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_n: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_original: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_plain: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_printtest: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_ref_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_rovar: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_singlearr: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_uc: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

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

fn __sh_sink(b: &[u8]) {
    let mut used = false;
    __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, b); used = true; } });
    if used { return; }
    if let Some(o) = __SH_OUT.lock().unwrap().as_mut() { o.extend_from_slice(b); return; }
    let _ = std::io::Write::write_all(&mut std::io::stdout(), b);
}

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("=== typeset -i (integer attribute) ===\n");
    __SHV_n.with(|v| v.set(0));
    __SHV_n.with(|v| v.set(42));
    println!("After 'typeset -i n=42': n='{}'", __SHV_n.with(|v| v.get()));
    __SHV_n.with(|v| v.set(__SHV_n.with(|v| v.get()) + 1));
    __SH_RC.store(0, Ordering::Relaxed);
    println!("After 'n=n+1':          n='{}' (integer arithmetic applied)", __SHV_n.with(|v| v.get()));
    __SHV_n.with(|v| v.set(std::env::var("hello").unwrap_or_default().trim().parse::<i64>().unwrap_or(0)));
    __SH_RC.store(0, Ordering::Relaxed);
    println!("After 'n=\"hello\"':     n='{}' (assigns 0; non-numeric string becomes 0)", __SHV_n.with(|v| v.get()));
    println!();
    print!("=== typeset -r (readonly attribute) ===\n");
    __SHV_rovar.with(|v| *v.borrow_mut() = String::new());
    __SHV_rovar.with(|v| *v.borrow_mut() = "immutable".to_string());
    println!("After 'typeset -r rovar=immutable': rovar='{}'", __SHV_rovar.with(|v| v.borrow().clone()));
    print!("(Attempting 'rovar=change' would cause an error; skipped for safety.)\n");
    println!();
    print!("=== typeset -l (lowercase attribute) ===\n");
    __SHV_lc.with(|v| *v.borrow_mut() = String::new());
    __SHV_lc.with(|v| *v.borrow_mut() = ("HELLO WORLD".to_string()).to_lowercase());
    println!("After 'typeset -l lc=\"HELLO WORLD\"': lc='{}'", __SHV_lc.with(|v| v.borrow().clone()));
    __SHV_lc.with(|v| *v.borrow_mut() = ("another test".to_string()).to_lowercase());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("After 'lc=\"ANOTHER TEST\"':           lc='{}'", __SHV_lc.with(|v| v.borrow().clone()));
    println!();
    print!("=== typeset -u (uppercase attribute) ===\n");
    __SHV_uc.with(|v| *v.borrow_mut() = String::new());
    __SHV_uc.with(|v| *v.borrow_mut() = ("hello world".to_string()).to_uppercase());
    println!("After 'typeset -u uc=\"hello world\"': uc='{}'", __SHV_uc.with(|v| v.borrow().clone()));
    __SHV_uc.with(|v| *v.borrow_mut() = ("ANOTHER TEST".to_string()).to_uppercase());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("After 'uc=\"another test\"':            uc='{}'", __SHV_uc.with(|v| v.borrow().clone()));
    println!();
    print!("=== typeset -x (export attribute) ===\n");
    __SHV_myexport.with(|v| *v.borrow_mut() = String::new());
    __SHV_myexport.with(|v| *v.borrow_mut() = "exported_value".to_string());
    std::env::set_var("myexport", "exported_value");
    print!("After 'typeset -x myexport=exported_value'\n");
    print!("Variable is exported:\n");
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_buf = {
let mut __sh_pbuf0: Vec<u8> = Vec::new();
__SH_PIPESTATUS.lock().unwrap().clear();
__SH_RC.store(__sh_spawn(&format!("{} | {}", "'env'", "'grep' '^myexport='"), Some(&__sh_pbuf0)), Ordering::Relaxed);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::Relaxed));
__SH_RC.load(Ordering::Relaxed) == 0
}; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if !__sh_buf {
        print!("(myexport not found in env — possible scope issue)\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    println!();
    print!("=== typeset -a (indexed array) ===\n");
    __SHV_arr.with(|v| v.borrow_mut().clear());
    __SHV_arr.with(|v| *v.borrow_mut() = vec!["10".to_string(), "20".to_string(), "30".to_string()]);
    println!("After 'typeset -a arr=(10 20 30)': arr=({})", __SHV_arr.with(|v| v.borrow().clone()).join(" "));
    println!("arr[0]='{}' arr[1]='{}' arr[2]='{}'", { let __sh_idx = 0; let __sh_num = __SHV_arr.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_arr.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }, { let __sh_idx = 1; let __sh_num = __SHV_arr.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_arr.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }, { let __sh_idx = 2; let __sh_num = __SHV_arr.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_arr.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    println!();
    print!("=== typeset -A (associative array) ===\n");
    __SHV_assoc.with(|v| v.borrow_mut().clear());
    { let __words = vec!["[key1]=value1".to_string(), "[key2]=value2".to_string()]; __SHV_assoc.with(|v| { let mut __sh_mapped = v.borrow_mut(); __sh_mapped.clear(); for __sh_word in __words { if let Some(rest) = __sh_word.strip_prefix('[') { if let Some(eq) = rest.find(']') { let __sh_key = rest[..eq].to_string(); let __sh_value = rest[eq + 2..].to_string(); __sh_mapped.insert(__sh_key, __sh_value); } } } }); }
    print!("After 'typeset -A assoc=([key1]=value1 [key2]=value2)'\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("assoc[key1]='{}'  assoc[key2]='{}'", __SHV_assoc.with(|v| v.borrow().get("key1").cloned().unwrap_or_default()), __SHV_assoc.with(|v| v.borrow().get("key2").cloned().unwrap_or_default()));
    println!();
    print!("=== typeset -n (name reference) ===\n");
    __SHV_original.with(|v| *v.borrow_mut() = String::new());
    __SHV_ref_.with(|v| *v.borrow_mut() = String::new());
    __SHV_original.with(|v| *v.borrow_mut() = "I am the original".to_string());
    print!("After 'typeset -n ref=original':\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("original='{}'", __SHV_original.with(|v| v.borrow().clone()));
    __SH_RC.store(0, Ordering::Relaxed);
    println!("ref='{}'", __SHV_original.with(|v| v.borrow().clone()));
    __SHV_original.with(|v| *v.borrow_mut() = "Changed via ref".to_string());
    print!("After 'ref=\"Changed via ref\"':\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("original='{}'", __SHV_original.with(|v| v.borrow().clone()));
    __SH_RC.store(0, Ordering::Relaxed);
    println!("ref='{}'", __SHV_original.with(|v| v.borrow().clone()));
    __SH_RC.store(0, Ordering::Relaxed);
    {
        __SHV_original.with(|v| *v.borrow_mut() = String::new());
    }
    println!();
    print!("=== typeset -f (display function definition) ===\n");
    print!("Output of 'typeset -f myfunc':\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("{}", "myfunc () \n{ \n    echo \"Inside myfunc\";\n    local x=5;\n    echo \"x=$x\"\n}");
    println!();
    print!("=== typeset -F (list function names) ===\n");
    println!("{}", "myfunc");
    println!();
    print!("=== typeset -g (global scope in function) ===\n");
    __SH_RC.store(0, Ordering::Relaxed);
    set_global();
    println!("After set_global(): global_var='{}'", __SHV_global_var.with(|v| v.borrow().clone()));
    println!();
    print!("=== typeset -t (trace attribute) ===\n");
    let tracetest = "traced".to_string();
    print!("After 'typeset -t tracetest=traced': (trace attribute set)\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("tracetest='{}'", tracetest.clone());
    println!();
    print!("=== typeset -p (print attribute info) ===\n");
    __SHV_printtest.with(|v| v.set(0));
    __SHV_printtest.with(|v| v.set(99));
    print!("After 'typeset -i -r printtest=99':\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("declare -ir printtest=\"{}\"", __SHV_printtest.with(|v| v.get()).to_string());
    println!();
    print!("=== combined: typeset -il (integer + lowercase) ===\n");
    __SHV_comb.with(|v| v.set(0));
    __SHV_comb.with(|v| v.set(42));
    println!("After 'typeset -il comb=42': comb='{}' (integer + lowercase)", __SHV_comb.with(|v| v.get()));
    __SHV_comb.with(|v| v.set(__SHV_comb.with(|v| v.get()) + 1));
    __SH_RC.store(0, Ordering::Relaxed);
    println!("After 'comb=comb+1':    comb='{}' (integer arithmetic active)", __SHV_comb.with(|v| v.get()));
    println!();
    print!("=== combined: typeset -iu (integer + uppercase) ===\n");
    __SHV_comb2.with(|v| v.set(0));
    __SHV_comb2.with(|v| v.set(99));
    println!("After 'typeset -iu comb2=99': comb2='{}' (integer + uppercase)", __SHV_comb2.with(|v| v.get()));
    __SHV_comb2.with(|v| v.set(__SHV_comb2.with(|v| v.get()) + 1));
    __SH_RC.store(0, Ordering::Relaxed);
    println!("After 'comb2=comb2+1': comb2='{}' (integer arithmetic active)", __SHV_comb2.with(|v| v.get()));
    println!();
    print!("=== typeset -a (indexed array, individual element) ===\n");
    __SHV_singlearr.with(|v| v.borrow_mut().clear());
    { let __sh_piece = "first".to_string(); __SHV_singlearr.with(|v| { let mut b = v.borrow_mut(); let i = 0 as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __sh_piece; }); }
    __SH_RC.store(0, Ordering::Relaxed);
    { let __sh_piece = "second".to_string(); __SHV_singlearr.with(|v| { let mut b = v.borrow_mut(); let i = 1 as usize; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __sh_piece; }); }
    __SH_RC.store(0, Ordering::Relaxed);
    println!("singlearr[0]='{}'  singlearr[1]='{}'", { let __sh_idx = 0; let __sh_num = __SHV_singlearr.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_singlearr.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }, { let __sh_idx = 1; let __sh_num = __SHV_singlearr.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_singlearr.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } });
    println!();
    print!("=== typeset (no flag) ===\n");
    __SHV_plain.with(|v| *v.borrow_mut() = String::new());
    __SHV_plain.with(|v| *v.borrow_mut() = "just a string".to_string());
    __SH_RC.store(0, Ordering::Relaxed);
    println!("After 'typeset plain=\"just a string\"': plain='{}'", __SHV_plain.with(|v| v.borrow().clone()));
    println!();
    print!("=== Demonstration complete. ===\n");
    __SH_RC.store(0, Ordering::Relaxed);
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn myfunc() {
    println!("{}", "Inside myfunc");
    let x = "5".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    println!("x={}", x.clone());
    __SH_RC.store(0, Ordering::Relaxed);
}

#[allow(dead_code)]
fn set_global() {
    __SHV_global_var.with(|v| *v.borrow_mut() = "I am global".to_string());
    __SH_RC.store(0, Ordering::Relaxed);
}