fn main() -> std::process::ExitCode {
    println!("all={}", "abc,def,ghi");
    println!("front={}", "XYZ:def:ghi");
    println!("back={}", "abc:def:XYZ");
    std::process::ExitCode::SUCCESS
}