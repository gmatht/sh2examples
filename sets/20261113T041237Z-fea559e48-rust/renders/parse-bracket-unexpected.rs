fn main() {
    if std::path::Path::new(&std::env::var("CONF").unwrap_or_default()).is_file() {
        print!("exists\n");
    } else {
    }
}