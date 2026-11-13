fn main() {
    print!("== Practical examples ==\n");
    print!("{}", format!("{:<10} {:<10} {}\n", "Name".to_string(), "Age".to_string(), "City".to_string()));
    print!("{}", format!("{:<10} {:<10} {}\n", "John".to_string(), "25".to_string(), "NYC".to_string()));
    print!("{}", format!("{:<10} {:<10} {}\n", "Jane".to_string(), "30".to_string(), "LA".to_string()));
}