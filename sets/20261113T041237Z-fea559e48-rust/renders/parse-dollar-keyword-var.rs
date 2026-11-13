use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    let prog = "dkms";
    if !{ let __b = std::path::Path::new("/usr/sbin/dkms").is_file(); __SH_RC.store(if __b { 0 } else { 1 }, Ordering::SeqCst); __b } {
        std::process::exit(0);
    }
    print!("{}", format!("{}=[{}]\n", "prog".to_string(), prog));
}