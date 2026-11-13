fn main() {
    if !{ let __v = std::env::var("ZSH_VERSION").unwrap_or_default(); if __v.is_empty() { String::new() } else { __v.to_string() } }.is_empty() {
        print!("zsh\n");
    } else if !{ let __v = "5.1.16".to_string(); if __v.is_empty() { String::new() } else { __v.to_string() } }.is_empty() {
        print!("bash\n");
    } else {
    }
}