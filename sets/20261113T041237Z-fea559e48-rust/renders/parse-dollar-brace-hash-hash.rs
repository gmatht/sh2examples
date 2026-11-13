fn main() {
    let p = "/usr/local/bin/example";
    if { let __v = {&p}.to_string(); match __v.rfind("/") { Some(__i) => __v[__i + "/".len()..].to_string(), None => __v } } == "example".to_string() {
        print!("it is example\n");
    } else {
    }
}