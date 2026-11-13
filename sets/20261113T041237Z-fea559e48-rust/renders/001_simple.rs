fn main() {
    println!("{}", "Hello, World!".to_string());
    if std::path::Path::new("test.txt").is_file() {
        println!("{}", "File exists".to_string());
    }
    for i in [1, 2, 3, 4, 5] {
        println!("{}", { let __si = i.to_string(); let __st = __si.strip_suffix('\n').unwrap_or(&__si); if __st.len() >= 6 && (__st.starts_with("list#") || __st.starts_with("obj#")) && __st[5..].parse::<usize>().is_ok() { vec![__si.to_string()] } else { __si.split(|c| c == ' ' || c == '\t' || c == '\n') .filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<String>>() } }.join(" "));
    }
}