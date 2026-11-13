use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_tc0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = { let (__c, __r) = { let __sc = std::process::Command::new("readlink").arg(std::env::var("FILE").unwrap_or_default()).stdin(std::process::Stdio::inherit()).output(); match __sc { Ok(__sc) => { let mut __ss = String::from_utf8_lossy(&__sc.stdout).to_string(); while __ss.ends_with('\n') { __ss.pop(); } (__ss, __sc.status.code().unwrap_or(1)) }, Err(e) => (String::new(), if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 }) } }; __SH_RC.store(__r, Ordering::SeqCst); __c });
    let _ = { let __b = ((!std::fs::symlink_metadata(&std::env::var("FILE").unwrap_or_default()).map(|m| m.file_type().is_symlink()).unwrap_or(false)) && std::path::Path::new(&std::env::var("FILE").unwrap_or_default()).is_dir()) || (std::fs::symlink_metadata(&std::env::var("FILE").unwrap_or_default()).map(|m| m.file_type().is_symlink()).unwrap_or(false) && (__SHV___sh2_tc0_.with(|v| v.borrow().clone()) == "target".to_string())); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b };
    print!("{}", "parsed OK\n".to_string());
}