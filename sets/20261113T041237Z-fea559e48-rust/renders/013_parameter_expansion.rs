thread_local! { #[allow(non_upper_case_globals)] static __SHV_maybe: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    print!("== Case modification in parameter expansion ==\n");
    print!("WORLD\n");
    print!("world\n");
    print!("World\n");
    print!("== Advanced parameter expansion ==\n");
    print!("013_param_expansion_file.txt\n");
    print!("/tmp\n");
    print!("aXXa\n");
    print!("== More parameter expansion ==\n");
    print!(" world\n");
    print!("hello \n");
    print!("hell0 w0rld\n");
    print!("== Default values ==\n");
    __SHV_maybe.with(|v| *v.borrow_mut() = String::new());
    println!("{}", { let __v = __SHV_maybe.with(|v| v.borrow().clone()); if __v.is_empty() { "default".to_string() } else { __v.to_string() } });
    println!("{}", { let __v = __SHV_maybe.with(|v| v.borrow().clone()); if __v.is_empty() { let __d = "default".to_string(); __SHV_maybe.with(|v| *v.borrow_mut() = __d.clone()); __d.clone() } else { __v } });
    println!("{}", { let __v = __SHV_maybe.with(|v| v.borrow().clone()); if __v.is_empty() { eprintln!("sh2: {}: {}", "maybe", "error".to_string()); std::process::exit(1); } __v });
}