fn main() {
    println!("{}", { let __sp = std::env::var("HOME").unwrap_or_default(); __sp.strip_suffix("/").unwrap_or(&__sp).to_string() });
}