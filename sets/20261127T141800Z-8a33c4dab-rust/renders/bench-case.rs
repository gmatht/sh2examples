fn main() -> std::process::ExitCode {
    if "hello".to_string().ends_with("llo") {
        print!("case=match\n");
    } else {
        print!("case=no\n");
    }
    if "abc".to_string().ends_with("llo") {
        print!("case2=match\n");
    } else {
        print!("case2=no\n");
    }
    std::process::ExitCode::SUCCESS
}