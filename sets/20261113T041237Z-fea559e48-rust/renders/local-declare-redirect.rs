fn main() {
    let __sh_case0 = "x".to_string();
    if { let __t: &str = {&__sh_case0}; __t == "pattern" } {
        {
        let _ = std::io::Write::write_all(&mut std::fs::File::create("/dev/null").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "".as_bytes());
        }
    } else {
    }
}