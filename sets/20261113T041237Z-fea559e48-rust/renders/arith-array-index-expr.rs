fn main() {
    let arr = [10, 20, 30];
    let i = 1;
    println!("{}", ({ let k: i64 = i; let n: i64 = 3; let i = if k < 0 { n + k } else { k }; if i < 0 || i >= n { std::process::exit(1); } arr[i as usize].to_string() }.trim().parse::<i64>().unwrap_or(0) * 2));
    println!("{}", (arr[0].to_string().trim().parse::<i64>().unwrap_or(0) + arr[2].to_string().trim().parse::<i64>().unwrap_or(0)));
    println!("{}", ((arr[1].to_string().trim().parse::<i64>().unwrap_or(0) + arr[2].to_string().trim().parse::<i64>().unwrap_or(0)) & 50i64));
}