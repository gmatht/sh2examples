fn main() -> std::process::ExitCode {
    let mut tsv = "".to_string();
    for _i in [1, 2, 3, 4, 5] {
        tsv = format!("{}{},", tsv.clone(), _i);
    }
    println!("{}", tsv.clone());
    std::process::ExitCode::SUCCESS
}