fn main() {
    let x = "hello world";
    println!("{}", x.chars().skip(0).take(x.chars().count().saturating_sub(2)).collect::<String>());
    println!("{}", x.chars().skip(x.chars().count().saturating_sub(3)).collect::<String>());
}