fn main() {
    let path = "/usr/local/bin";
    println!("{}", { let __v = {&path}.to_string(); match __v.find("/") { Some(__i) => __v[..__i].to_string(), None => __v } });
    println!("{}", { let __v = {&path}.to_string(); match __v.find("/") { Some(__i) => __v[__i + "/".len()..].to_string(), None => __v } });
}