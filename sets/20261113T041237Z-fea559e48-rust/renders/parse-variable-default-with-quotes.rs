fn main() {
    println!("{}", { let __v = std::env::var("MYVAR").unwrap_or_default(); if __v.is_empty() { "default value".to_string() } else { __v.to_string() } });
}