use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() -> std::process::ExitCode {
    let mut ch = String::new();
    let s = "HELLOWORLD";
    let mut i = "0".to_string();
    let len = s.len() as i64;
    let mut __sh_while_ran0 = false;
    let mut __sh_while_last1 = 0;
    while i.trim().parse::<i64>().unwrap_or(0) < len.trim().parse::<i64>().unwrap_or(0) {
        __sh_while_ran0 = true;
        ch = { let __sh_s = &s; let __sh_o = i.trim().parse::<i64>().unwrap_or(0); let __sh_l = 1; let __sh_ch: Vec<char> = __sh_s.chars().collect(); let __sh_n = __sh_ch.len() as i64; let mut __sh_st = if __sh_o < 0 { __sh_n + __sh_o } else { __sh_o }; if __sh_st < 0 { __sh_st = 0; } if __sh_st > __sh_n { String::new() } else { let mut __sh_cn = if __sh_l == i64::MIN { __sh_n - __sh_st } else if __sh_l < 0 { __sh_n + __sh_l - __sh_st } else { __sh_l }; if __sh_cn < 0 { __sh_cn = 0; } if __sh_st + __sh_cn > __sh_n { __sh_cn = __sh_n - __sh_st; } __sh_ch[__sh_st as usize..(__sh_st + __sh_cn) as usize].iter().collect::<String>() } };
        print!("{}", ch.clone());
        i = { let __sh_value = ((i.trim().parse::<i64>().unwrap_or(0) + 1i64)).to_string(); let __sh_failed = __SH_ARITH_ERR.swap(false, Ordering::Relaxed); __SH_RC.store(if __sh_failed { 1 } else { 0 }, Ordering::Relaxed); if __sh_failed { String::new() } else { __sh_value } };
        __SH_RC.store(0, Ordering::Relaxed);
        __sh_while_last1 = __SH_RC.load(Ordering::Relaxed);
    }
    if !__sh_while_ran0 { __SH_RC.store(0, Ordering::Relaxed); } else { __SH_RC.store(__sh_while_last1, Ordering::Relaxed); }
    println!();
    std::process::ExitCode::SUCCESS
}