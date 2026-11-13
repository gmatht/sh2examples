fn main() {
    let x = format!("{}extra", std::env::var("y").unwrap_or_default());
    println!("{}", x);
}