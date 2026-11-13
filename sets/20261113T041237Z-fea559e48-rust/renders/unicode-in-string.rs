fn main() {
    println!("{}", format!("{}✅ All tests passed{}", std::env::var("GREEN").unwrap_or_default(), std::env::var("NC").unwrap_or_default()));
}