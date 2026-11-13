fn main() {
    #[allow(non_snake_case)] let MYVAR = { let __v = std::env::var("UMASK").unwrap_or_default(); if __v.is_empty() { "0640".to_string() } else { __v.to_string() } };
    println!("{}", MYVAR);
}