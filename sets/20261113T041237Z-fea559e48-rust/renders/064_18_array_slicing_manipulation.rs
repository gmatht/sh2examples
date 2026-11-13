thread_local! { #[allow(non_upper_case_globals)] static __SHV_first_half: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_last_half: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_middle: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) }; }
thread_local! { #[allow(non_upper_case_globals)] static __SHV_numbers: std::cell::RefCell<Vec<i64>> = const { std::cell::RefCell::new(Vec::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    __SHV_numbers.with(|v| *v.borrow_mut() = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    __SHV_middle.with(|v| *v.borrow_mut() = vec![{ let __v = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __s = 3usize.min(__v.len()); let __e = (__s + 4usize).min(__v.len()); __v[__s..__e].join(" ") }]);
    __SHV_first_half.with(|v| *v.borrow_mut() = vec![{ let __v = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __s = 0usize.min(__v.len()); let __e = (__s + 5usize).min(__v.len()); __v[__s..__e].join(" ") }]);
    __SHV_last_half.with(|v| *v.borrow_mut() = vec![{ let __v = __SHV_numbers.with(|v| v.borrow().iter().map(|n| n.to_string()).collect::<Vec<String>>()); let __s = 5usize.min(__v.len()); __v[__s..].join(" ") }]);
    println!("{}", format!("Middle: {}", __SHV_middle.with(|v| v.borrow().clone()).join(" ")));
    println!("{}", format!("First half: {}", __SHV_first_half.with(|v| v.borrow().clone()).join(" ")));
    println!("{}", format!("Last half: {}", __SHV_last_half.with(|v| v.borrow().clone()).join(" ")));
}