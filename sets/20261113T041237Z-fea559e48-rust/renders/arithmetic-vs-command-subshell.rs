fn main() {
    let result = String::new();
    print!("{}", format!("result={}\n", { let __v = result.clone(); if __v.is_empty() { "empty".to_string() } else { __v.to_string() } }));
}