fn main() {
    if { let __t: &str = {&std::env::var("var").unwrap_or_default()}; !__t.is_empty() } {
        print!("has chars\n");
    } else {
    }
}