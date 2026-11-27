fn main() -> std::process::ExitCode {
    let __sh2_tc0_: String;
    __sh2_tc0_ = "foo".to_string();
    if __sh2_tc0_.trim().parse::<i64>().unwrap_or(0) != 0 {
        print!("bar\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}