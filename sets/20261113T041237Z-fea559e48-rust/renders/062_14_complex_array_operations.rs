thread_local! { #[allow(non_upper_case_globals)] static __SHV_array: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "Testing complex array operations...".to_string());
    __SHV_array.with(|v| *v.borrow_mut() = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()]);
    __SHV_array.with(|v| { let mut b = v.borrow_mut(); for __w in vec!["item4".to_string()] { b.push(__w); } });
    println!("{}", format!("Array: {}", __SHV_array.with(|v| v.borrow().clone()).join(" ")));
    println!("{}", format!("Length: {}", __SHV_array.with(|v| v.borrow().len()).to_string()));
    println!("{}", format!("First item: {}", __SHV_array.with(|v| v.borrow().get(0usize).cloned().unwrap_or_default())));
}