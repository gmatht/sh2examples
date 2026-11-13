thread_local! { #[allow(non_upper_case_globals)] static __SHV_i: std::cell::Cell<i64> = const { std::cell::Cell::new(0) }; }

#[allow(non_upper_case_globals)]
fn main() {
    #[allow(non_snake_case)] let LIMIT = 10;
    __SHV_i.with(|v| v.set(0));
    while __SHV_i.with(|v| v.get()) < LIMIT {
        print!("{}", format!("{}", __SHV_i.with(|v| v.get()).to_string()));
        __SHV_i.with(|v| v.set(__SHV_i.with(|v| v.get()) + 1));
    }
    print!("\n");
}