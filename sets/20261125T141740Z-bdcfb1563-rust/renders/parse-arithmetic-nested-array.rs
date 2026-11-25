fn main() -> std::process::ExitCode {
    let mut j = "0".to_string();
    let arr = ["a".to_string(), "b".to_string(), "c".to_string()];
    j = (j.trim().parse::<i64>().unwrap_or(0) + (100 / 3i64)).to_string();
    println!("{}", j.clone());
    print!("arr=[{}]\n", { let __sh_value = arr[0].clone(); if __sh_value.is_empty() { "".to_string() } else { __sh_value.to_string() } });
    std::process::ExitCode::SUCCESS
}