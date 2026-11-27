fn main() -> std::process::ExitCode {
    let a = [10, 20, 30, 40];
    if 1 < (a[1] + a[2]) {
        print!("arith-test-ok\n");
    } else {
        print!("arith-test-bad\n");
    }
    print!("done\n");
    std::process::ExitCode::SUCCESS
}