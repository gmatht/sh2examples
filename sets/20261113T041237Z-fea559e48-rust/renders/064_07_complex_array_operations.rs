use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_IFS: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_config: std::cell::RefCell<std::collections::BTreeMap<String, String>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_sorted: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_config.with(|v| { v.borrow_mut().insert("user".to_string(), "admin".to_string()); });
    __SHV_config.with(|v| { v.borrow_mut().insert("host".to_string(), "localhost".to_string()); });
    __SHV_config.with(|v| { v.borrow_mut().insert("port".to_string(), "8080".to_string()); });
    __SHV_IFS.with(|v| *v.borrow_mut() = "\n".to_string());
    __SHV_sorted.with(|v| *v.borrow_mut() = [{ let __si = {
    let (__c, __r) = { let __sc = std::process::Command::new("sort").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __sc { Ok(mut __sc) => { if let Some(mut __ci) = __sc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, format!("{}\n", __SHV_config.with(|v| { let mut __vs = v.borrow().values().cloned().collect::<Vec<String>>(); __vs.sort(); __vs.join(" ") })).as_bytes()); } let __so = __sc.wait_with_output(); match __so { Ok(__so) => { let mut __ss = String::from_utf8_lossy(&__so.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __so.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } };
    __SH_RC.store(__r, Ordering::SeqCst);
    __c
}; let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }].concat());
    println!("{}", format!("Config: {}", __SHV_sorted.with(|v| v.borrow().clone()).join(" ")));
}