use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    if !({ let __b = std::env::var("HOME").unwrap_or_default() == std::env::var("HOME").unwrap_or_default(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } && {
print!("1\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("-\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if !({ let __b = format!("{}{}", std::env::var("HOME").unwrap_or_default(), "/Documents") == std::env::var("HOME").unwrap_or_default(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } && {
print!("2\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("-\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if !({ let __b = format!("{}{}", std::env::var("HOME").unwrap_or_default(), "/Documents") == format!("{}/Documents", std::env::var("HOME").unwrap_or_default()); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } && {
print!("3\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("-\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}