use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_file: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

fn main() {
    get_file_size("test_simple_function.sh".to_string().to_string());
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}

fn get_file_size(p0: String) {
    __SHV_file.with(|v| *v.borrow_mut() = format!("{}", p0.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
    let size = [{ let __si = { let (__c, __r) = __sh_capture_rc(&format!("{} < {}", "'wc' '-c'".to_string(), { let __t: &str = {&__SHV_file.with(|v| v.borrow().clone())}; let mut __o = String::with_capacity(__t.len() + 8); __o.push('\''); for __c in __t.chars() { if __c == '\'' { __o.push_str("'\\''"); } else { __o.push(__c); } } __o.push('\''); __o })); __SH_RC.store(__r, Ordering::SeqCst); __c }; let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat().join(" ");
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("File {} has {} bytes", __SHV_file.with(|v| v.borrow().clone()), size.clone()));
    __SH_RC.store(0, Ordering::SeqCst);
}