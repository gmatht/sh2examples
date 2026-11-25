static __SH_ARGV: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn __sh_cat(ws: &[Vec<String>]) -> Vec<String> {
    ws.concat()
}

fn __sh_arg(i: usize) -> String {
    __SH_ARGV.lock().unwrap().get(i).cloned().unwrap_or_default()
}

fn main() -> std::process::ExitCode {
    let __sh_newargv = __sh_cat(&[vec![__sh_arg(0)], vec![std::env::var("tempfile").unwrap_or_default()], { let __sh_value = __SH_ARGV.lock().unwrap().clone(); let __sh_offset = ((3 - 1).max(0) as usize).min(__sh_value.len()); let __sh_len = if i64::MIN < 0 { __sh_value.len().saturating_sub(__sh_offset) as i64 } else { i64::MIN }; let __sh_stop = ((__sh_offset as i64 + __sh_len).max(__sh_offset as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_offset); __sh_value[__sh_offset..__sh_stop].to_vec() }]); *__SH_ARGV.lock().unwrap() = __sh_newargv;
    { let __sh_w = [{ let __sh_value = __SH_ARGV.lock().unwrap().clone(); let __sh_offset = ((1 - 1).max(0) as usize).min(__sh_value.len()); let __sh_len = if i64::MIN < 0 { __sh_value.len().saturating_sub(__sh_offset) as i64 } else { i64::MIN }; let __sh_stop = ((__sh_offset as i64 + __sh_len).max(__sh_offset as i64)).min(__sh_value.len() as i64) as usize; let __sh_stop = __sh_stop.max(__sh_offset); __sh_value[__sh_offset..__sh_stop].to_vec() }].concat(); if !__sh_w.is_empty() { print!("{}", __sh_w.join(" ")); } println!(); }
    std::process::ExitCode::SUCCESS
}