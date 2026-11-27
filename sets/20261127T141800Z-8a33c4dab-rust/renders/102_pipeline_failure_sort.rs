use std::sync::atomic::Ordering;

fn main() -> std::process::ExitCode {
    print!("Sort test:\n");
    println!("{}", { let __sh_text: String = "c\nb\na".to_string(); let mut __sh_lines: Vec<String> = if __sh_text.is_empty() { Vec::new() } else { let mut __sh_collected: Vec<String> = __sh_text.split('\n').map(|__sh_line| __sh_line.to_string()).collect(); if __sh_text.ends_with('\n') { __sh_collected.pop(); } __sh_collected }; let __sort_key = |__sh_line: &String| -> String { __sh_line.clone() }; __sh_lines.sort_by(|__sh_left, __sh_right| __sort_key(__sh_left).cmp(&__sort_key(__sh_right)));  __sh_lines.join("\n") });
    print!("---\n");
    println!("{}", { let __sh_text: String = "3\n1\n2".to_string(); let mut __sh_lines: Vec<String> = if __sh_text.is_empty() { Vec::new() } else { let mut __sh_collected: Vec<String> = __sh_text.split('\n').map(|__sh_line| __sh_line.to_string()).collect(); if __sh_text.ends_with('\n') { __sh_collected.pop(); } __sh_collected }; let __sort_key = |__sh_line: &String| -> String { __sh_line.clone() }; __sh_lines.sort_by(|__sh_left, __sh_right| { let (__sh_state, __sh_put) = (__sort_key(__sh_left).trim().parse::<f64>().unwrap_or(0.0), __sort_key(__sh_right).trim().parse::<f64>().unwrap_or(0.0)); __sh_state.partial_cmp(&__sh_put).unwrap_or(std::cmp::Ordering::Equal) });  __sh_lines.join("\n") });
    print!("done\n");
    std::process::ExitCode::SUCCESS
}