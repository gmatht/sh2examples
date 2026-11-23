use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_casesubj_0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_args: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_flags: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

thread_local! { #[allow(non_upper_case_globals)] static __SHV_options: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }

fn __sh_ret_rc(v: &str) -> i32 {
    match v.trim().parse::<i64>() { Ok(n) => (n & 0xff) as i32, Err(_) => 255 }
}

fn main() -> std::process::ExitCode {
    println!("{}", { let __sh_operand: Vec<String> = vec!["--flag1".to_string(), "--option1=value1".to_string(), "-abc".to_string()]; complex_function(&__sh_operand) });
    std::process::ExitCode::SUCCESS
}

#[allow(dead_code)]
fn complex_function(__argv: &[String]) -> String {
    let mut key = String::new();
    let mut value = String::new();
    __SHV_args.with(|v| *v.borrow_mut() = __argv.to_vec());
    __SH_RC.store(0, Ordering::Relaxed);
    { let __words = Vec::<String>::new(); __SHV_options.with(|v| { let mut __sh_mapped = v.borrow_mut(); __sh_mapped.clear(); for __sh_word in __words { if let Some(rest) = __sh_word.strip_prefix('[') { if let Some(eq) = rest.find(']') { let __sh_key = rest[..eq].to_string(); let __sh_value = rest[eq + 2..].to_string(); __sh_mapped.insert(__sh_key, __sh_value); } } } }); }
    let mut i = "0".to_string();
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_while_ran10 = false;
    let mut __sh_while_last11 = 0;
    while i.trim().parse::<i64>().unwrap_or(0) < __SHV_args.with(|v| v.borrow().len() as i64) {
        __sh_while_ran10 = true;
        __SHV___sh2_casesubj_0_.with(|v| *v.borrow_mut() = format!("{}", { let __sh_idx = i.trim().parse::<i64>().unwrap_or(0); let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }));
        __SH_RC.store(0, Ordering::Relaxed);
        if __SHV___sh2_casesubj_0_.with(|v| v.borrow().clone()).starts_with("--") {
            let mut key = { let __sh_str: String = { let __sh_idx = i.trim().parse::<i64>().unwrap_or(0); let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }.to_string(); if let Some(__sh_res) = __sh_str.strip_prefix("--") { __sh_res.to_string() } else { __sh_str } };
            __SH_RC.store(0, Ordering::Relaxed);
            let mut value = { let __sh_idx = i.trim().parse::<i64>().unwrap_or(0) + 1; let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } };
            __SH_RC.store(0, Ordering::Relaxed);
            __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", key.clone()), value.clone()); });
            __SH_RC.store(0, Ordering::Relaxed);
            let __sh_arith_value12 = { i = (i.trim().parse::<i64>().unwrap_or(0) + 2).to_string(); i.trim().parse::<i64>().unwrap_or(0) };
            let __sh_num13 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value12 };
            __SH_RC.store(if __sh_num13 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        } else if __SHV___sh2_casesubj_0_.with(|v| v.borrow().clone()).starts_with("-") {
            __SHV_flags.with(|v| *v.borrow_mut() = { let __sh_str: String = { let __sh_idx = i.trim().parse::<i64>().unwrap_or(0); let __sh_num = __SHV_args.with(|v| v.borrow().len()) as i64; let __sh_pos = if __sh_idx < 0 { __sh_num + __sh_idx } else { __sh_idx }; if __sh_pos >= 0 && __sh_pos < __sh_num { __SHV_args.with(|v| v.borrow()[__sh_pos as usize].clone()) } else { String::new() } }.to_string(); if let Some(__sh_res) = __sh_str.strip_prefix("-") { __sh_res.to_string() } else { __sh_str } });
            let mut j = "0".to_string();
            __SH_RC.store(0, Ordering::Relaxed);
            let mut __sh_while_ran14 = false;
            let mut __sh_while_last15 = 0;
            while j.trim().parse::<i64>().unwrap_or(0) < __SHV_flags.with(|v| v.borrow().clone()).chars().count() as i64 {
                __sh_while_ran14 = true;
                __SHV_options.with(|v| { v.borrow_mut().insert(format!("{}", { let __sh_s = &__SHV_flags.with(|v| v.borrow().clone()); let __sh_o = j.trim().parse::<i64>().unwrap_or(0); let __sh_l = 1; let __sh_ch: Vec<char> = __sh_s.chars().collect(); let __sh_n = __sh_ch.len() as i64; let mut __sh_st = if __sh_o < 0 { __sh_n + __sh_o } else { __sh_o }; if __sh_st < 0 { __sh_st = 0; } if __sh_st > __sh_n { String::new() } else { let mut __sh_cn = if __sh_l == i64::MIN { __sh_n - __sh_st } else if __sh_l < 0 { __sh_n + __sh_l - __sh_st } else { __sh_l }; if __sh_cn < 0 { __sh_cn = 0; } if __sh_st + __sh_cn > __sh_n { __sh_cn = __sh_n - __sh_st; } __sh_ch[__sh_st as usize..(__sh_st + __sh_cn) as usize].iter().collect::<String>() } }), "true".to_string()); });
                __SH_RC.store(0, Ordering::Relaxed);
                let __sh_arith_value16 = { let __sh_out = j.trim().parse::<i64>().unwrap_or(0); j = (j.trim().parse::<i64>().unwrap_or(0) +1).to_string(); __sh_out };
                let __sh_num17 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value16 };
                __SH_RC.store(if __sh_num17 != 0 { 0 } else { 1 }, Ordering::Relaxed);
                __sh_while_last15 = __SH_RC.load(Ordering::Relaxed);
            }
            if !__sh_while_ran14 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last15, Ordering::Relaxed); }
            let __sh_arith_value18 = { let __sh_out = i.trim().parse::<i64>().unwrap_or(0); i = (i.trim().parse::<i64>().unwrap_or(0) +1).to_string(); __sh_out };
            let __sh_num19 = if __SH_ARITH_ERR.swap(false, Ordering::Relaxed) { 0 } else { __sh_arith_value18 };
            __SH_RC.store(if __sh_num19 != 0 { 0 } else { 1 }, Ordering::Relaxed);
        } else {
            __SH_RC.store(0, Ordering::Relaxed);
            __sh_while_last11 = __SH_RC.load(Ordering::Relaxed);
            break;
        }
        __sh_while_last11 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran10 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last11, Ordering::Relaxed); }
    let __sh_value = format!("Processed {} options", __SHV_options.with(|v| v.borrow().len() as i64).to_string()); *__SH_RET.lock().unwrap() = __sh_value.clone(); __SH_RC.store(__sh_ret_rc(&__sh_value), Ordering::Relaxed); return __sh_value;
}