use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_RET: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

fn main() {
    println!("{}", { let __a: Vec<String> = vec!["--long-option=value with spaces".to_string(), "--array-option".to_string(), "item1".to_string(), "item2".to_string(), "item3".to_string(), "--flag".to_string(), "positional argument".to_string(), { let __v = std::env::var("var").unwrap_or_default(); if __v.is_empty() { "default".to_string() } else { __v.to_string() } }, {
    __SH_RC.store(0, Ordering::SeqCst);
    "computed".to_string()
}]; complex_function(&__a) });
}

fn complex_function(__argv: &[String]) -> String {
    let __v = format!("fn args: {}", __argv.to_vec().join(" ")); *__SH_RET.lock().unwrap() = __v.clone(); return __v;
}