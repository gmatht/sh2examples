fn main() {
    #[allow(non_snake_case)] let A = { let __v = {&std::env::var("B").unwrap_or_default()}.to_string(); match __v.find(".") { Some(__i) => __v[..__i].to_string(), None => __v } };
    #[allow(non_snake_case)] let C = { let __v = {&std::env::var("D").unwrap_or_default()}.to_string(); match __v.find("_") { Some(__i) => __v[__i + "_".len()..].to_string(), None => __v } };
    println!("{}", format!("{} {}", A.clone(), C.clone()));
}