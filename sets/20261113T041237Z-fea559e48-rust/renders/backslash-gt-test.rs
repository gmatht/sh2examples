fn main() {
    let a = 5;
    let b = 10;
    if a.to_string() > b.to_string() {
        print!("a > b\n");
    }
    print!("{}", format!("{}=[{}]\n", "a".to_string(), "5".to_string()));
    print!("{}", format!("{}=[{}]\n", "b".to_string(), "10".to_string()));
}