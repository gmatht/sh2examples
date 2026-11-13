use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    let __sh_case0 = __SH_RC.load(Ordering::SeqCst).to_string();
    if (__sh_case0 == "0") {
        println!("{}", (1i64 + 2i64));
    } else if true {
        print!("other\n");
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
}