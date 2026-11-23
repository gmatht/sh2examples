thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    let mut temp = 0i64;
    print!("=== Fibonacci Sequence (first 20 numbers) ===\n");
    let mut a = 0;
    let mut b = 1;
    print!("Fibonacci numbers:\n");
    print!("{} {} ", a, b);
    for __sh_w0 in [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20] {
        __SHV_i.with(|v| v.set(__sh_w0));
        temp = a + b;
        print!("{} ", temp);
        a = b;
        b = temp;
    }
    println!();
    print!("Done!\n");
    std::process::ExitCode::SUCCESS
}