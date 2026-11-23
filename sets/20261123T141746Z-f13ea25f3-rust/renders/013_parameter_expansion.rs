thread_local! { #[allow(non_upper_case_globals)] static __SHV_maybe: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    print!("== Case modification in parameter expansion ==\n");
    let name = "world".to_string();
    println!("{}", name.clone().to_uppercase());
    println!("{}", name.clone().to_lowercase());
    print!("World\n");
    print!("== Advanced parameter expansion ==\n");
    let path = "/tmp/013_param_expansion_file.txt".to_string();
    println!("{}", { let __sh_str: String = path.clone().to_string(); let __sh_delim: &str = "/"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[__sh_begin + __sh_delim.len()..].to_string(), None => __sh_str } });
    println!("{}", { let __sh_str: String = path.clone().to_string(); let __sh_delim: &str = "/"; match __sh_str.rfind(__sh_delim) { Some(__sh_begin) => __sh_str[..__sh_begin].to_string(), None => __sh_str } });
    print!("aXXa\n");
    print!("== More parameter expansion ==\n");
    print!(" world\n");
    print!("hello \n");
    print!("hell0 w0rld\n");
    print!("== Default values ==\n");
    __SHV_maybe.with(|v| *v.borrow_mut() = String::new());
    println!("{}", { let __sh_value = __SHV_maybe.with(|v| v.borrow().clone()); if __sh_value.is_empty() { "default".to_string() } else { __sh_value.to_string() } });
    println!("{}", { let __sh_value = __SHV_maybe.with(|v| v.borrow().clone()); if __sh_value.is_empty() { let __sh_default = "default".to_string(); __SHV_maybe.with(|v| *v.borrow_mut() = __sh_default.clone()); __sh_default.clone() } else { __sh_value } });
    println!("{}", { let __sh_value = __SHV_maybe.with(|v| v.borrow().clone()); if __sh_value.is_empty() { eprintln!("sh2: {}: {}", "maybe", "error".to_string()); std::process::exit(1); } __sh_value });
    std::process::ExitCode::SUCCESS
}