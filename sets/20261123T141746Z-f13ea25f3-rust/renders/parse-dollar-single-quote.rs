thread_local! { #[allow(non_upper_case_globals)] static __SHV_IFS: std::cell::RefCell<String> = std::cell::RefCell::new(" \t\n".to_string()); }

#[allow(non_upper_case_globals)]
fn main() -> std::process::ExitCode {
    __SHV_IFS.with(|v| *v.borrow_mut() = "\n\t".to_string());
    print!("IFS value (hex)=");
    print!("$'\\n\\t'");
    print!("\n");
    std::process::ExitCode::SUCCESS
}