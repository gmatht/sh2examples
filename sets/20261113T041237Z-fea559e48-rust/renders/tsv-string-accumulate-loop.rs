fn main() {
    let mut tsv = "".to_string();
    for i in [1, 2, 3, 4, 5] {
        tsv = format!("{}{},", tsv.clone(), i);
    }
    println!("{}", tsv);
}