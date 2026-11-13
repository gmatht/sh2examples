fn main() {
    let a = 9;
    let b = 4;
    let c = a + b;
    print!("{}", format!("a=[{}]\n", a.to_string()));
    print!("{}", format!("b=[{}]\n", b.to_string()));
    print!("{}", format!("c=[{}]\n", c.to_string()));
    println!("{}", ((a + b) + c));
}