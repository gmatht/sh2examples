fn main() {
    let x = format!("{}/file", std::env::var("HOME").unwrap_or_default());
    let z = format!("{} with {} and {}ext", x.clone(), "literal string".to_string(), x.clone());
    println!("{}", z);
    print!("{}", format!("{}=[{}]\n", "y".to_string(), "literal string".to_string()));
    print!("{}", format!("{}=[{}]\n", "x".to_string(), x.clone()));
}