fn main() {
    print!("== Indexed arrays ==\n");
    let arr = ["one".to_string(), "two".to_string(), "three".to_string()];
    println!("{}", arr[1].clone());
    println!("{}", 3i64);
    for x in arr.iter() {
        print!("{}", format!("{} ", x.to_string()));
    }
    print!("\n");
}