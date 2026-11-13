fn main() {
    let filename = "my_file.txt";
    let basename = { let __v = {&filename}.to_string(); match __v.rfind(".") { Some(__i) => __v[..__i].to_string(), None => __v } };
    let extension = { let __v = {&filename}.to_string(); match __v.rfind(".") { Some(__i) => __v[__i + ".".len()..].to_string(), None => __v } };
    println!("{}", format!("Basename: {}", basename.clone()));
    println!("{}", format!("Extension: {}", extension.clone()));
    println!("{}", format!("Uppercase: {}", "MY_FILE.TXT".to_string()));
    println!("{}", format!("Lowercase: {}", "my_file.txt".to_string()));
}