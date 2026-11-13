use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_arr: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_k: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_map: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "== Indexed arrays ==".to_string());
    __SHV_arr.with(|v| *v.borrow_mut() = vec!["one".to_string(), "two".to_string(), "three".to_string()]);
    println!("{}", __SHV_arr.with(|v| v.borrow().get(1usize).cloned().unwrap_or_default()));
    println!("{}", __SHV_arr.with(|v| v.borrow().len()).to_string());
    for x in ["one", "two", "three"] {
        print!("{}", format!("{} ", x.to_string()));
    }
    println!("{}", "".to_string());
    println!("{}", "== Associative arrays ==".to_string());
    __SHV_map.with(|v| { v.borrow_mut().insert("foo".to_string(), "bar".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_map.with(|v| { v.borrow_mut().insert("answer".to_string(), "42".to_string()); });
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_map.with(|v| { v.borrow_mut().insert("two".to_string(), "1 + 1".to_string()); });
    println!("{}", __SHV_map.with(|v| v.borrow().get(&("foo".to_string())).cloned().unwrap_or_default()));
    println!("{}", __SHV_map.with(|v| v.borrow().get(&("answer".to_string())).cloned().unwrap_or_default()));
    let mut __sh_pbuf1: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __sh_items3 = [__SHV_map.with(|v| v.borrow().keys().cloned().collect::<Vec<String>>())].concat();
    let mut __sh_i4: usize = 0;
    loop {
    if !(__sh_i4 < __sh_items3.len()) { break; }
        __SHV_k.with(|v| *v.borrow_mut() = __sh_items3[__sh_i4].clone());
        { let __w: &[Vec<String>] = {&[vec![format!("{} => {}", __SHV_k.with(|v| v.borrow().clone()), __SHV_map.with(|v| v.borrow().get(&(__SHV_k.with(|v| v.borrow().clone()))).cloned().unwrap_or_default()))]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
        __SH_RC.store(0, Ordering::SeqCst);
    __sh_i4 += 1;
    }
    __sh_pbuf1 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("sort").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf1); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}