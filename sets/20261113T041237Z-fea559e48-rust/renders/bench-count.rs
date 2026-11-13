thread_local! { #[allow(non_upper_case_globals)] static __SHV___n: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let mut i = 1;
    __SHV___n.with(|v| v.set(0));
    while __SHV___n.with(|v| v.get()) < 1000 {
        i = i + 1;
        __SHV___n.with(|v| v.set(__SHV___n.with(|v| v.get()) + 1));
    }
    println!("{}", format!("count={}", i));
}