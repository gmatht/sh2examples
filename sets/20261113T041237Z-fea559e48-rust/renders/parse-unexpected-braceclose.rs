use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    println!("{}", "hello".to_string());
    // TODO(unsupported): stmt Warn { expr: Str("syntax error near unexpected token `}'", SingleQuoted), carp: false }
    std::process::exit(2);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}
// 1 construct(s) lowered to TODO markers