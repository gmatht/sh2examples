fn main() {
    let s = "abc:def:ghi";
    println!("{}", format!("short-prefix={}", { let __v = {&s}.to_string(); match __v.find(":") { Some(__i) => __v[__i + ":".len()..].to_string(), None => __v } }));
    println!("{}", format!("long-prefix={}", { let __v = {&s}.to_string(); match __v.rfind(":") { Some(__i) => __v[__i + ":".len()..].to_string(), None => __v } }));
    println!("{}", format!("short-suffix={}", { let __v = {&s}.to_string(); match __v.rfind(":") { Some(__i) => __v[..__i].to_string(), None => __v } }));
    println!("{}", format!("long-suffix={}", { let __v = {&s}.to_string(); match __v.find(":") { Some(__i) => __v[..__i].to_string(), None => __v } }));
}