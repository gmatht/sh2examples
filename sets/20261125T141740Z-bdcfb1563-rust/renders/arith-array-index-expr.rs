fn main() -> std::process::ExitCode {
    let arr = [10, 20, 30];
    let i = 1;
    println!("{}", ({ let k = i; let n = 3; let i = if k < 0 { n + k } else { k }; if i < 0 || i >= n { std::process::exit(1); } arr[i as usize] } * 2));
    println!("{}", (arr[0] + arr[2]));
    println!("{}", ((arr[1] + arr[2]) & 50i64));
    std::process::ExitCode::SUCCESS
}