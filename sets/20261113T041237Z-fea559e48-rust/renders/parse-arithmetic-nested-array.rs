fn main() {
    let mut j = "0".to_string();
    let arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    j = (j.clone().trim().parse::<i64>().unwrap_or(0) + (100 / 3i64)).to_string();
    println!("{}", j);
    print!("{}", format!("{}=[{}]\n", "arr".to_string(), { let __v = arr[0].clone(); if __v.is_empty() { "".to_string() } else { __v.to_string() } }));
}