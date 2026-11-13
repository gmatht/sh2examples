fn main() {
    if std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file() && std::fs::File::open(&std::env::var("file").unwrap_or_default()).is_ok() {
        print!("readable\n");
    } else {
    }
}