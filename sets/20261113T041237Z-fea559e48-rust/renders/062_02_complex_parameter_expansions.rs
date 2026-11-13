fn main() {
    print!("Testing complex parameter expansions...\n");
    let complex_var = "hello world";
    println!("{}", { let __v = {&complex_var}.to_string(); match __v.find("o") { Some(__i) => __v[__i + "o".len()..].to_string(), None => __v } });
    println!("{}", { let __v = {&complex_var}.to_string(); match __v.rfind("o") { Some(__i) => __v[__i + "o".len()..].to_string(), None => __v } });
    println!("{}", { let __v = {&complex_var}.to_string(); match __v.rfind("o") { Some(__i) => __v[..__i].to_string(), None => __v } });
    println!("{}", { let __v = {&complex_var}.to_string(); match __v.find("o") { Some(__i) => __v[..__i].to_string(), None => __v } });
}