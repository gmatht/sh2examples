fn __sh_uid() -> u32 {
    std::fs::read_to_string("/proc/self/status").ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("Uid:"))
            .and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok()))
        .unwrap_or(0)
}

fn __sh_gid() -> u32 {
    std::fs::read_to_string("/proc/self/status").ok()
        .and_then(|s| s.lines().find(|l| l.starts_with("Gid:"))
            .and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok()))
        .unwrap_or(0)
}

fn __sh_access(p: &str, want: u32) -> bool {
    use std::os::unix::fs::MetadataExt;
    let Ok(md) = std::fs::metadata(p) else { return false };
    if __sh_uid() == 0 {
        return want & 1 == 0 || md.mode() & 0o111 != 0;
    }
    let mode = md.mode();
    let bits = if md.uid() == __sh_uid() {
        (mode >> 6) & 7
    } else if md.gid() == __sh_gid() {
        (mode >> 3) & 7
    } else {
        mode & 7
    };
    bits & want != 0
}

fn main() -> std::process::ExitCode {
    if std::path::Path::new(&std::env::var("file").unwrap_or_default()).is_file() && __sh_access(&std::env::var("file").unwrap_or_default(), 4) {
        print!("readable\n");
    } else {
    }
    std::process::ExitCode::SUCCESS
}