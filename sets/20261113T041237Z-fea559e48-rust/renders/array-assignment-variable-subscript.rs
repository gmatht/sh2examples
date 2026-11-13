fn main() {
    let FilesystemOptions = ["a".to_string(), "b".to_string(), "c".to_string()];
    let i = "1";
    println!("{}", { let k: i64 = ((2 * i.trim().parse::<i64>().unwrap_or(0)) - 1); let n: i64 = 3; let i = if k < 0 { n + k } else { k }; if i < 0 || i >= n { std::process::exit(1); } FilesystemOptions[i as usize].clone() });
}