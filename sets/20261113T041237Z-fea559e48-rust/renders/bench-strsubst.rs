fn main() {
    println!("{}", format!("all={}", "abc,def,ghi".to_string()));
    println!("{}", format!("front={}", "XYZ:def:ghi".to_string()));
    println!("{}", format!("back={}", "abc:def:XYZ".to_string()));
}