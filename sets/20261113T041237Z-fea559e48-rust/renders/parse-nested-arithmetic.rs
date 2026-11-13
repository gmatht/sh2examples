fn main() {
    let x = (std::env::var("n").unwrap_or_default().trim().parse::<i64>().unwrap_or(0) + 1) % 3;
    print!("{}", format!("{}=[{}]\n", "x".to_string(), x.to_string()));
}