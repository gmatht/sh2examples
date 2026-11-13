use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o140000) == 0o140000).unwrap_or(false) } {
        print!("is a socket\n");
    }
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o140000) == 0o140000).unwrap_or(false) } {
        print!("is a socket\n");
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if !({ let __b = { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o140000) == 0o140000).unwrap_or(false) }; __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } && {
print!("is_socket: yes\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("is_socket: no\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", format!("done: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}