fn main() {
    let name = "John Doe";
    println!("{}", format!("Hello {}", "John_Doe".to_string()));
    println!("{}", format!("Length: {}", "8".to_string()));
    println!("{}", format!("First: {}", "John".to_string()));
    println!("{}", format!("Last: {}", name.chars().skip(name.chars().count().saturating_sub(3)).collect::<String>()));
}