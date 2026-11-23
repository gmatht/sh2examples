fn main() -> std::process::ExitCode {
    let mut total = 0;
    for _i in [1, 2, 3, 4, 5] {
        total = total + _i;
    }
    println!("total={}", total);
    std::process::ExitCode::SUCCESS
}