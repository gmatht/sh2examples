static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

fn main() {
    println!("{}", f());
    println!("{}", f());
}

fn f() -> String {
    let __v = "f-called".to_string(); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}