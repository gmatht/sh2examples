static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn main() -> std::process::ExitCode {
    let __sh_newargv = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()]; *__SH_ARGV.lock().unwrap() = __sh_newargv;
    { let __sh_w = [{ let __sh_value = __SH_ARGV.lock().unwrap().clone(); let __sh_offset = ((3 - 1).max(0) as usize).min(__sh_value.len()); let __sh_len = if i64::MIN < 0 { __sh_value.len().saturating_sub(__sh_offset) as i64 } else { i64::MIN }; let __sh_stop = ((__sh_offset as i64 + __sh_len).max(__sh_offset as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_offset); __sh_value[__sh_offset..__sh_stop].to_vec() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    std::process::ExitCode::SUCCESS
}