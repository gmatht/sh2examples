static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    if __SH_ARGV.lock().unwrap().len().to_string().trim().parse::<i64>().unwrap_or(0) < 2 {
        print!("One\n");
        print!("Two\n");
    } else {
    }
}