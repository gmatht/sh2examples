fn main() {
    if (std::path::Path::new("/path1").is_dir() || std::path::Path::new("/path2").is_dir()) || std::path::Path::new("/path3").is_dir() {
        print!("found\n");
    }
    println!("{}", format!("status: {}", "ok".to_string()));
}