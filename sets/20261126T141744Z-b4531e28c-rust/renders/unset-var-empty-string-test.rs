fn main() -> std::process::ExitCode {
    let never_set2 = String::new();
    if std::env::var("never_set").unwrap_or_default() == "".to_string() {
        print!("unset-is-empty\n");
    } else {
        print!("unset-not-empty\n");
    }
    if never_set2.clone() == "".to_string() {
        print!("zero-len\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}