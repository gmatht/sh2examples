use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_arg: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

#[allow(non_upper_case_globals)]
fn main() {
    #[allow(unused_assignments)] #[allow(non_snake_case)] let mut X_SET = 0;
    __SH_RC.store(0, Ordering::SeqCst);
    let __sh_items0 = [__SH_ARGV.lock().unwrap().clone()].concat();
    let mut __sh_i1: usize = 0;
    loop {
    if !(__sh_i1 < __sh_items0.len()) { break; }
        __SHV_arg.with(|v| *v.borrow_mut() = __sh_items0[__sh_i1].clone());
        if {
let mut __sh_pbuf2: Vec<u8> = Vec::new();
__SH_PIPESTATUS.lock().unwrap().clear();
{
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __w: &[Vec<String>] = {&[vec![__SHV_arg.with(|v| v.borrow().clone())]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-q").arg("-e").arg("^-[a-zA-Z0-9]*x").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf2); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
__SH_RC.load(Ordering::SeqCst) == 0
} {
            X_SET = 1;
        }
    __sh_i1 += 1;
    }
    println!("{}", X_SET.to_string());
    __SH_RC.store(0, Ordering::SeqCst);
}