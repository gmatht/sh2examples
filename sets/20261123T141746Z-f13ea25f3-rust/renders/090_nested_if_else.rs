fn main() -> std::process::ExitCode {
    if "hello" == "hello" {
        print!("greeting\n");
    } else if "hello" == "bye" {
        print!("farewell\n");
    } else {
        print!("unknown\n");
    }
    std::process::ExitCode::SUCCESS
}