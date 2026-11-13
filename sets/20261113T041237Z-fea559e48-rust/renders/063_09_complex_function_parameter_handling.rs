use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_args: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_flags: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_j: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_key: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_options: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_value: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_substr(s: &str, off: i64, len: i64) -> String {
    let ch: Vec<char> = s.chars().collect();
    let n = ch.len() as i64;
    let mut o = if off < 0 { n + off } else { off };
    if o < 0 { o = 0; }
    if o > n { return String::new(); }
    let mut l = if len == i64::MIN { n - o } else if len < 0 { n + len - o } else { len };
    if l < 0 { l = 0; }
    if o + l > n { l = n - o; }
    ch[o as usize..(o + l) as usize].iter().collect()
}

fn main() {
    println!("{}", { let __a: Vec<String> = vec!["--flag1".to_string(), "--option1=value1".to_string(), "-abc".to_string()]; complex_function(&__a) });
}

fn complex_function(__argv: &[String]) -> String {
    __SHV_args.with(|v| *v.borrow_mut() = [__argv.to_vec()].concat());
    __SH_RC.store(0, Ordering::SeqCst);
    { let __words = Vec::<String>::new(); __SHV_options.with(|v| { let mut __m = v.borrow_mut(); __m.clear(); for __w in __words { if let Some(rest) = __w.strip_prefix('[') { if let Some(eq) = rest.find(']') { let __k = rest[..eq].to_string(); let __v = rest[eq + 2..].to_string(); __m.insert(__k, __v); } } } }); }
    __SHV_i.with(|v| *v.borrow_mut() = "0".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    let mut __sh_while_ran11 = false;
    let mut __sh_while_last12 = 0;
    while __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < __SHV_args.with(|v| v.borrow().len() as i64) {
        __sh_while_ran11 = true;
        let __sh_case13 = { let __k: i64 = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) };
        if __sh_case13.starts_with("--") {
            __SHV_key.with(|v| *v.borrow_mut() = { let __sp = { let __k: i64 = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) }; __sp.strip_prefix("--").unwrap_or(&__sp).to_string() });
            __SH_RC.store(0, Ordering::SeqCst);
            __SHV_value.with(|v| *v.borrow_mut() = { let __k: i64 = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + 1); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) });
            __SH_RC.store(0, Ordering::SeqCst);
            __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", __SHV_key.with(|v| v.borrow().clone())), __SHV_value.with(|v| v.borrow().clone())); });
            __SH_RC.store(0, Ordering::SeqCst);
            let __sh_arith_v14 = { __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) + 2).to_string()); __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) };
            let __n15 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v14 };
            __SH_RC.store(if __n15 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        } else if (__sh_case13.starts_with("-")) {
            __SHV_flags.with(|v| *v.borrow_mut() = { let __sp = { let __k: i64 = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); let __n = __SHV_args.with(|v| v.borrow().len()) as i64; let __idx = if __k < 0 { __n + __k } else { __k }; __SHV_args.with(|v| v.borrow().get(__idx as usize).cloned().unwrap_or_default()) }; __sp.strip_prefix("-").unwrap_or(&__sp).to_string() });
            __SHV_j.with(|v| *v.borrow_mut() = "0".to_string());
            __SH_RC.store(0, Ordering::SeqCst);
            let mut __sh_while_ran16 = false;
            let mut __sh_while_last17 = 0;
            while __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) < __SHV_flags.with(|v| v.borrow().clone()).chars().count() as i64 {
                __sh_while_ran16 = true;
                __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", __sh_substr(&__SHV_flags.with(|v| v.borrow().clone()), __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0), 1)), "true".to_string()); });
                __SH_RC.store(0, Ordering::SeqCst);
                let __sh_arith_v18 = { let __o = __SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); __SHV_j.with(|v| *v.borrow_mut() = (__SHV_j.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) +1).to_string()); __o };
                let __n19 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v18 };
                __SH_RC.store(if __n19 != 0 { 0 } else { 1 }, Ordering::SeqCst);
                __sh_while_last17 = __SH_RC.load(Ordering::SeqCst);
            }
            if !__sh_while_ran16 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last17, Ordering::SeqCst); }
            let __sh_arith_v20 = { let __o = __SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0); __SHV_i.with(|v| *v.borrow_mut() = (__SHV_i.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) +1).to_string()); __o };
            let __n21 = if __SH_ARITH_ERR.swap(false, Ordering::SeqCst) { 0 } else { __sh_arith_v20 };
            __SH_RC.store(if __n21 != 0 { 0 } else { 1 }, Ordering::SeqCst);
        } else if true {
            __SH_RC.store(0, Ordering::SeqCst);
            __sh_while_last12 = __SH_RC.load(Ordering::SeqCst);
            break;
        } else {
            __SH_RC.store(0, Ordering::SeqCst);
        }
        __sh_while_last12 = __SH_RC.load(Ordering::SeqCst);
    }
    if !__sh_while_ran11 { __SH_RC.store(0, Ordering::SeqCst); } else { __SH_RC.store(__sh_while_last12, Ordering::SeqCst); }
    let __v = format!("Processed {} options", __SHV_options.with(|v| v.borrow().len()).to_string()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}