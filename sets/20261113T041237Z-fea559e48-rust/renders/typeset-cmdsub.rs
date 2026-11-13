use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

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
thread_local! { #[allow(non_upper_case_globals)] static __SHV_x: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "=== typeset -i (integer attribute) ===".to_string());
    __SHV_n.with(|v| v.set(0));
    __SHV_n.with(|v| v.set(42));
    println!("{}", format!("After 'typeset -i n=42': n='{}'", __SHV_n.with(|v| v.get())));
    __SHV_n.with(|v| v.set(((__SHV_n.with(|v| v.get()) + 1).to_string()).trim().parse::<i64>().unwrap_or(0).to_string().trim().parse::<i64>().unwrap_or(0)));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("After 'n=n+1':          n='{}' (integer arithmetic applied)", __SHV_n.with(|v| v.get())));
    __SHV_n.with(|v| v.set(((std::env::var("hello").unwrap_or_default().trim().parse::<i64>().unwrap_or(0)).to_string()).trim().parse::<i64>().unwrap_or(0).to_string().trim().parse::<i64>().unwrap_or(0)));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("After 'n=\"hello\"':     n='{}' (assigns 0; non-numeric string becomes 0)", __SHV_n.with(|v| v.get())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -r (readonly attribute) ===".to_string());
    __SHV_rovar.with(|v| *v.borrow_mut() = String::new());
    __SHV_rovar.with(|v| *v.borrow_mut() = "immutable".to_string());
    println!("{}", format!("After 'typeset -r rovar=immutable': rovar='{}'", __SHV_rovar.with(|v| v.borrow().clone())));
    println!("{}", "(Attempting 'rovar=change' would cause an error; skipped for safety.)".to_string());
    println!("{}", "".to_string());
    println!("{}", "=== typeset -l (lowercase attribute) ===".to_string());
    __SHV_lc.with(|v| *v.borrow_mut() = String::new());
    __SHV_lc.with(|v| *v.borrow_mut() = ("HELLO WORLD".to_string()).to_lowercase());
    println!("{}", format!("After 'typeset -l lc=\"HELLO WORLD\"': lc='{}'", __SHV_lc.with(|v| v.borrow().clone())));
    __SHV_lc.with(|v| *v.borrow_mut() = ("another test".to_string()).to_lowercase());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("After 'lc=\"ANOTHER TEST\"':           lc='{}'", __SHV_lc.with(|v| v.borrow().clone())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -u (uppercase attribute) ===".to_string());
    __SHV_uc.with(|v| *v.borrow_mut() = String::new());
    __SHV_uc.with(|v| *v.borrow_mut() = ("hello world".to_string()).to_uppercase());
    println!("{}", format!("After 'typeset -u uc=\"hello world\"': uc='{}'", __SHV_uc.with(|v| v.borrow().clone())));
    __SHV_uc.with(|v| *v.borrow_mut() = ("ANOTHER TEST".to_string()).to_uppercase());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("After 'uc=\"another test\"':            uc='{}'", __SHV_uc.with(|v| v.borrow().clone())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -x (export attribute) ===".to_string());
    __SHV_myexport.with(|v| *v.borrow_mut() = String::new());
    __SHV_myexport.with(|v| *v.borrow_mut() = "exported_value".to_string());
    std::env::set_var("myexport", "exported_value");
    println!("{}", "After 'typeset -x myexport=exported_value'".to_string());
    println!("{}", "Variable is exported:".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    if !{
__SH_PIPESTATUS.lock().unwrap().clear();
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("env").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("grep").arg("^myexport=").stdin(__x).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &[]); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
__SH_RC.load(Ordering::SeqCst) == 0
} {
        print!("{}", "(myexport not found in env — possible scope issue)\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", "".to_string());
    println!("{}", "=== typeset -a (indexed array) ===".to_string());
    __SHV_arr.with(|v| v.borrow_mut().clear());
    __SHV_arr.with(|v| *v.borrow_mut() = vec!["10".to_string(), "20".to_string(), "30".to_string()]);
    println!("{}", format!("After 'typeset -a arr=(10 20 30)': arr=({})", __SHV_arr.with(|v| v.borrow().clone()).join(" ")));
    println!("{}", format!("arr[0]='{}' arr[1]='{}' arr[2]='{}'", __SHV_arr.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default()), __SHV_arr.with(|v| v.borrow().get(1usize).cloned().unwrap_or_default()), __SHV_arr.with(|v| v.borrow().get(2usize).cloned().unwrap_or_default())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -A (associative array) ===".to_string());
    __SHV_assoc.with(|v| v.borrow_mut().clear());
    { let __words = vec!["[key1]=value1".to_string(), "[key2]=value2".to_string()]; __SHV_assoc.with(|v| { let mut __m = v.borrow_mut(); __m.clear(); for __w in __words { if let Some(rest) = __w.strip_prefix('[') { if let Some(eq) = rest.find(']') { let __k = rest[..eq].to_string(); let __v = rest[eq + 2..].to_string(); __m.insert(__k, __v); } } } }); }
    println!("{}", "After 'typeset -A assoc=([key1]=value1 [key2]=value2)'".to_string());
    println!("{}", format!("assoc[key1]='{}'  assoc[key2]='{}'", __SHV_assoc.with(|v| v.borrow().get(&("key1".to_string())).cloned().unwrap_or_default()), __SHV_assoc.with(|v| v.borrow().get(&("key2".to_string())).cloned().unwrap_or_default())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -n (name reference) ===".to_string());
    __SHV_original.with(|v| *v.borrow_mut() = String::new());
    __SHV_ref_.with(|v| *v.borrow_mut() = String::new());
    __SHV_original.with(|v| *v.borrow_mut() = "I am the original".to_string());
    println!("{}", "After 'typeset -n ref=original':".to_string());
    println!("{}", format!("original='{}'", __SHV_original.with(|v| v.borrow().clone())));
    println!("{}", format!("ref='{}'", __SHV_original.with(|v| v.borrow().clone())));
    __SHV_original.with(|v| *v.borrow_mut() = "Changed via ref".to_string());
    println!("{}", "After 'ref=\"Changed via ref\"':".to_string());
    println!("{}", format!("original='{}'", __SHV_original.with(|v| v.borrow().clone())));
    println!("{}", format!("ref='{}'", __SHV_original.with(|v| v.borrow().clone())));
    {
        __SHV_original.with(|v| *v.borrow_mut() = String::new());
    }
    println!("{}", "".to_string());
    println!("{}", "=== typeset -f (display function definition) ===".to_string());
    println!("{}", "Output of 'typeset -f myfunc':".to_string());
    println!("{}", "myfunc () \n{ \n    echo \"Inside myfunc\";\n    local x=5;\n    echo \"x=$x\"\n}".to_string());
    println!("{}", "".to_string());
    println!("{}", "=== typeset -F (list function names) ===".to_string());
    println!("{}", "myfunc".to_string());
    println!("{}", "".to_string());
    println!("{}", "=== typeset -g (global scope in function) ===".to_string());
    set_global();
    println!("{}", format!("After set_global(): global_var='{}'", __SHV_global_var.with(|v| v.borrow().clone())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -t (trace attribute) ===".to_string());
    let tracetest = "traced".to_string();
    println!("{}", "After 'typeset -t tracetest=traced': (trace attribute set)".to_string());
    println!("{}", format!("tracetest='{}'", tracetest.clone()));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -p (print attribute info) ===".to_string());
    __SHV_printtest.with(|v| v.set(0));
    __SHV_printtest.with(|v| v.set(99));
    println!("{}", "After 'typeset -i -r printtest=99':".to_string());
    println!("{}", format!("declare -ir printtest=\"{}\"", __SHV_printtest.with(|v| v.get()).to_string()));
    println!("{}", "".to_string());
    println!("{}", "=== combined: typeset -il (integer + lowercase) ===".to_string());
    __SHV_comb.with(|v| v.set(0));
    __SHV_comb.with(|v| v.set(42));
    println!("{}", format!("After 'typeset -il comb=42': comb='{}' (integer + lowercase)", __SHV_comb.with(|v| v.get())));
    __SHV_comb.with(|v| v.set((((__SHV_comb.with(|v| v.get()) + 1).to_string()).trim().parse::<i64>().unwrap_or(0).to_string()).to_lowercase().trim().parse::<i64>().unwrap_or(0)));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("After 'comb=comb+1':    comb='{}' (integer arithmetic active)", __SHV_comb.with(|v| v.get())));
    println!("{}", "".to_string());
    println!("{}", "=== combined: typeset -iu (integer + uppercase) ===".to_string());
    __SHV_comb2.with(|v| v.set(0));
    __SHV_comb2.with(|v| v.set(99));
    println!("{}", format!("After 'typeset -iu comb2=99': comb2='{}' (integer + uppercase)", __SHV_comb2.with(|v| v.get())));
    __SHV_comb2.with(|v| v.set((((__SHV_comb2.with(|v| v.get()) + 1).to_string()).trim().parse::<i64>().unwrap_or(0).to_string()).to_uppercase().trim().parse::<i64>().unwrap_or(0)));
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("After 'comb2=comb2+1': comb2='{}' (integer arithmetic active)", __SHV_comb2.with(|v| v.get())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset -a (indexed array, individual element) ===".to_string());
    __SHV_singlearr.with(|v| v.borrow_mut().clear());
    { let __val = "first".to_string(); __SHV_singlearr.with(|v| { let mut b = v.borrow_mut(); let i = 0; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __val; }); }
    __SH_RC.store(0, Ordering::SeqCst);
    { let __val = "second".to_string(); __SHV_singlearr.with(|v| { let mut b = v.borrow_mut(); let i = 1; if b.len() <= i { b.resize(i + 1, String::new()); } b[i] = __val; }); }
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("singlearr[0]='{}'  singlearr[1]='{}'", __SHV_singlearr.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default()), __SHV_singlearr.with(|v| v.borrow().get(1usize).cloned().unwrap_or_default())));
    println!("{}", "".to_string());
    println!("{}", "=== typeset (no flag) ===".to_string());
    __SHV_plain.with(|v| *v.borrow_mut() = String::new());
    __SHV_plain.with(|v| *v.borrow_mut() = "just a string".to_string());
    println!("{}", format!("After 'typeset plain=\"just a string\"': plain='{}'", __SHV_plain.with(|v| v.borrow().clone())));
    println!("{}", "".to_string());
    println!("{}", "=== Demonstration complete. ===".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
}

fn myfunc() {
    print!("{}", "Inside myfunc\n");
    __SHV_x.with(|v| *v.borrow_mut() = "5".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("x={}", __SHV_x.with(|v| v.borrow().clone())));
    __SH_RC.store(0, Ordering::SeqCst);
}

fn set_global() {
    __SHV_global_var.with(|v| *v.borrow_mut() = "I am global".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
}