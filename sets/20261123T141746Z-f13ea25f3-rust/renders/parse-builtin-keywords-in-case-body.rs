static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    if __sh_arg(0) == "start".to_string() {
        { let mut __sh_value = __SH_ARGV.lock().unwrap(); let __sh_num = (1).min(__sh_value.len() as i64) as usize; __sh_value.drain(0..__sh_num); }
    } else if __sh_arg(0) == "stop".to_string() {
        let __sh_newargv = vec!["stopped".to_string()]; *__SH_ARGV.lock().unwrap() = __sh_newargv;
    } else if __sh_arg(0) == "check".to_string() {
        print!("checking\n");
    } else {
        print!("unknown\n");
    }
    std::process::ExitCode::SUCCESS
}