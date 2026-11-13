static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    if std::path::Path::new("file.txt").is_file() {
        println!("{}", "File exists".to_string());
    } else {
        println!("{}", "File does not exist".to_string());
    }
    for __sh_lv1 in [1, 2, 3, 4, 5] {
        __SHV_i.with(|v| v.set(__sh_lv1));
        println!("{}", format!("Number: {}", __sh_lv1));
    }
    while __SHV_i.with(|v| v.get()) < 10 {
        println!("{}", format!("Counter: {}", __SHV_i.with(|v| v.get())));
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
    }
    println!("{}", greet("World".to_string().to_string()));
}

fn greet(p0: String) -> String {
    let __v = format!("Hello, {}!", p0.clone()); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}