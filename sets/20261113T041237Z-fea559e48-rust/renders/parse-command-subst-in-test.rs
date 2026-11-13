use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

thread_local! { #[allow(non_upper_case_globals)] static __SHV___sh2_tc0_: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    __SHV___sh2_tc0_.with(|v| *v.borrow_mut() = {
    __SH_RC.store(0, Ordering::SeqCst);
    "foo".to_string()
});
    if __SHV___sh2_tc0_.with(|v| v.borrow().clone()).trim().parse::<i64>().unwrap_or(0) != 0 {
        println!("{}", "bar".to_string());
        __SH_RC.store(0, Ordering::SeqCst);
    } else {
        __SH_RC.store(0, Ordering::SeqCst);
    }
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}