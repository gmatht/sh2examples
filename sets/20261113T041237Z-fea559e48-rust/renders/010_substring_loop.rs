use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_ARITH_ERR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_ch: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

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

#[allow(non_upper_case_globals)]
fn main() {
    let s = "HELLOWORLD";
    let mut i = "0".to_string();
    let len = 10;
    while i.clone().trim().parse::<i64>().unwrap_or(0) < len {
        __SHV_ch.with(|v| *v.borrow_mut() = __sh_substr(&s, i.clone().trim().parse::<i64>().unwrap_or(0), 1));
        print!("{}", format!("{}", __SHV_ch.with(|v| v.borrow().clone())));
        i = { let __v = ((i.clone().trim().parse::<i64>().unwrap_or(0) + 1i64)).to_string(); let __e = __SH_ARITH_ERR.swap(false, Ordering::SeqCst); __SH_RC.store(if __e { 1 } else { 0 }, Ordering::SeqCst); if __e { String::new() } else { __v } };
    }
    print!("\n");
}