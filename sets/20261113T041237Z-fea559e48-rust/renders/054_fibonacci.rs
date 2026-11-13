thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_temp: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    print!("=== Fibonacci Sequence (first 20 numbers) ===\n");
    let mut a = 0;
    let mut b = 1;
    print!("Fibonacci numbers:\n");
    print!("{}", format!("{} {} ", a, b));
    for __sh_w0 in [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20] {
        __SHV_i.with(|v| v.set(__sh_w0));
        __SHV_temp.with(|v| v.set(a + b));
        print!("{}", format!("{} ", __SHV_temp.with(|v| v.get())));
        a = b;
        b = __SHV_temp.with(|v| v.get());
    }
    print!("\n");
    print!("Done!\n");
}