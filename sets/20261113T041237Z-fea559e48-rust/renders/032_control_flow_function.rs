static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn main() {
    println!("{}", greet("World".to_string().to_string()));
}

fn greet(p0: String) -> String {
    let __v = format!("Hello, {}!", p0.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}