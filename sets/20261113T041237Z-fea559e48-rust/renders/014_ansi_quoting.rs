fn main() {
    print!("== ANSI-C quoting ==\n");
    print!("line1\nline2\tTabbed\n");
    print!("== Escape sequences ==\n");
    print!("bell\x07\n");
    print!("backspace\x08\n");
    print!("formfeed\x0c\n");
    print!("newline\n\n");
    print!("carriage\rreturn\n");
    print!("tab\tseparated\n");
    print!("vertical\x0btab\n");
    print!("== Unicode and hex ==\n");
    print!("Hello\n");
    print!("Hello\n");
    print!("== Practical examples ==\n");
    print!("{}", format!("{:<10} {:<10} {}\n", "Name".to_string(), "Age".to_string(), "City".to_string()));
    print!("{}", format!("{:<10} {:<10} {}\n", "John".to_string(), "25".to_string(), "NYC".to_string()));
    print!("{}", format!("{:<10} {:<10} {}\n", "Jane".to_string(), "30".to_string(), "LA".to_string()));
}