static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() {
    if { let __t: &str = {&__SH_ARGV.lock().unwrap().join(" ")}; __t.contains("--view") } {
        print!("view mode\n");
    } else {
    }
}