fn main() {
    let mut total = 0;
    for i in [1, 2, 3, 4, 5] {
        total = total + i;
    }
    println!("{}", format!("total={}", total));
}