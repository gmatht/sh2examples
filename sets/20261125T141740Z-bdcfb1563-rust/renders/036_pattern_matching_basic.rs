use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_regex(text: &str, pat: &str, flags: &str) -> Option<bool> {
    let pat = if flags.contains('i') { format!("(?i){}", pat) } else { pat.to_string() };
    match regex::Regex::new(&pat) {
        Ok(re) => Some(re.is_match(text)),
        Err(_) => None,
    }
}

fn main() -> std::process::ExitCode {
    print!("== [[ pattern and regex ]]\n");
    { let __sh_buf = "file.txt".to_string().ends_with(".txt"); __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        print!("pattern-match\n");
    }
    }
    { let __sh_buf = { let __sh_mapped = __sh_regex(&"file.txt".to_string().to_string(), &"^file\\.[a-z]+$".to_string().to_string(), ""); match __sh_mapped { Some(__sh_mapped) => __sh_mapped, None => { __SH_RC.store(2, Ordering::Relaxed); false } } }; __SH_RC.store(if __sh_buf { 0 } else { 1 }, Ordering::Relaxed); if __sh_buf {
        print!("regex-match\n");
        __SH_RC.store(0, Ordering::Relaxed);
    }
    }
    std::process::ExitCode::from(__SH_RC.load(Ordering::Relaxed) as u8)
}