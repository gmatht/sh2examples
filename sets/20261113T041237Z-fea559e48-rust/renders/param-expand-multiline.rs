fn main() {
    let x = { let __v = std::env::var("foo").unwrap_or_default(); if __v.is_empty() { "bar\nbaz".to_string() } else { __v.to_string() } };
    println!("{}", x);
}