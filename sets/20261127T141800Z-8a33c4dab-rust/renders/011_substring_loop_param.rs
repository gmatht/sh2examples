use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() -> std::process::ExitCode {
    print_chars("HELLOWORLD".to_string(), "10".to_string());
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}

#[allow(dead_code)]
fn print_chars(p0: String, p1: String) {
    let mut ch = String::new();
    let s = p0.clone();
    let mut i = "0".to_string();
    let len = p1.clone();
    __SH_RC.store(0, Ordering::Relaxed);
    let mut __sh_while_ran2 = false;
    let mut __sh_while_last3 = 0;
    while i.trim().parse::<i64>().unwrap_or(0) < len.trim().parse::<i64>().unwrap_or(0) {
        __sh_while_ran2 = true;
        ch = { let __sh_s = &s.clone(); let __sh_o = i.trim().parse::<i64>().unwrap_or(0); let __sh_l = 1; let __sh_n = __sh_s.chars().count() as i64; let mut __sh_st = if __sh_o < 0 { __sh_n + __sh_o } else { __sh_o }; if __sh_st < 0 { __sh_st = 0; } if __sh_st > __sh_n { String::new() } else { let mut __sh_cn = if __sh_l == i64::MIN { __sh_n - __sh_st } else if __sh_l < 0 { __sh_n + __sh_l - __sh_st } else { __sh_l }; if __sh_cn < 0 { __sh_cn = 0; } if __sh_st + __sh_cn > __sh_n { __sh_cn = __sh_n - __sh_st; } __sh_s.chars().skip(__sh_st as usize).take(__sh_cn as usize).collect::<String>() } };
        print!("{}", ch.clone());
        __SH_RC.store(0, Ordering::Relaxed);
        i = { let __sh_value = ((i.trim().parse::<i64>().unwrap_or(0) + 1i64)).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last3 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran2 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last3, Ordering::Relaxed); }
    println!();
    __SH_RC.store(0, Ordering::Relaxed);
}