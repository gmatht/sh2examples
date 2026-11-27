use std::sync::atomic::Ordering;

static __SH_BGPID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    println!("background: {}", { let __sh_bgpid = __SH_BGPID.load(Ordering::Relaxed); if __sh_bgpid == 0 { String::new() } else { __sh_bgpid.to_string() } });
    std::process::ExitCode::SUCCESS
}