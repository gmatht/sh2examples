thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    let mut s = 0;
    __SHV_i.with(|v| v.set(0));
    while __SHV_i.with(|v| v.get()) < 100 {
        s = s + __SHV_i.with(|v| v.get());
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
    }
    println!("{}", format!("sum={}", s));
}