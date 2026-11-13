fn main() {
    if !{ let __v = std::env::var("MYVAR").unwrap_or_default(); if __v.is_empty() { String::new() } else { "set".to_string() } }.is_empty() {
        print!("MYVAR is set\n");
    } else {
    }
}