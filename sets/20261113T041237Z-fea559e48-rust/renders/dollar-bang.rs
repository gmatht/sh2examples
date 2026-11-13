use std::sync::atomic::Ordering;

static __SH_BGPID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    println!("{}", format!("background: {}", {{ let __p = __SH_BGPID.load(Ordering::SeqCst); if __p == 0 {{ String::new() }} else {{ __p.to_string() }} }}));
}