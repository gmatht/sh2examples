fn main() {
    let __sh_case0 = std::env::var("prev").unwrap_or_default();
    if { let __t: &str = {&__sh_case0}; __t == "foo|bar" } {
        print!("matched\n");
    } else {
    }
}