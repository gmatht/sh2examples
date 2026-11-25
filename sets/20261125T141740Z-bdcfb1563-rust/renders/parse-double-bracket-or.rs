fn main() -> std::process::ExitCode {
    if (std::path::Path::new("/path1").is_dir() || std::path::Path::new("/path2").is_dir()) || std::path::Path::new("/path3").is_dir() {
        print!("found\n");
    } else {
    }
    println!("status: {}", "ok");
    std::process::ExitCode::SUCCESS
}