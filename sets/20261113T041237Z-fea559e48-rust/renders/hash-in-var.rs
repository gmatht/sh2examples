fn main() {
    let x = { let __v = {&std::env::var("y").unwrap_or_default()}.to_string(); match __v.find(" ") { Some(__i) => __v[__i + " ".len()..].to_string(), None => __v } };
    println!("{}", x);
}