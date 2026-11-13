fn main() {
    println!("{}", { let __v = {&std::env::var("PATH").unwrap_or_default()}.to_string(); match __v.find(":") { Some(__i) => __v[..__i].to_string(), None => __v } });
}