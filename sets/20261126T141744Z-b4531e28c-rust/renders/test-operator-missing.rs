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

fn __sh_fowner(s: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    let uid = __sh_uid();
    std::fs::metadata(s).map(|m| m.uid() == uid).unwrap_or(false)
}

fn __sh_fgroup(s: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    let gid = __sh_gid();
    std::fs::metadata(s).map(|m| m.gid() == gid).unwrap_or(false)
}

fn main() -> std::process::ExitCode {
    if std::fs::symlink_metadata("/dev/null").map(|__sh_mapped| __sh_mapped.file_type().is_symlink()).unwrap_or(false) {
    }
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o010000) == 0o010000 }).unwrap_or(false) {
    }
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o060000) == 0o060000 }).unwrap_or(false) {
    }
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o020000) == 0o020000 }).unwrap_or(false) {
    }
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o002000) == 0o002000 }).unwrap_or(false) {
    }
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o001000) == 0o001000 }).unwrap_or(false) {
    }
    if std::fs::metadata("/dev/null").map(|__sh_mapped| { use std::os::unix::fs::MetadataExt; (__sh_mapped.mode() & 0o004000) == 0o004000 }).unwrap_or(false) {
    }
    if __sh_fowner("/dev/null") {
    }
    if __sh_fgroup("/dev/null") {
    }
    if { match std::fs::metadata("/dev/null") { Ok(__sh_m) => match (__sh_m.modified(), __sh_m.accessed()) { (Ok(__sh_mt), Ok(__sh_at)) => __sh_mt > __sh_at, _ => false }, Err(_) => false } } {
    }
    if { let __sh_a = std::fs::metadata("a").and_then(|m| m.modified()).ok(); let __sh_b = std::fs::metadata("b").and_then(|m| m.modified()).ok(); match (__sh_a, __sh_b) { (Some(__sh_a), Some(__sh_b)) => __sh_a > __sh_b, (Some(_), None) => true, _ => false } } {
    }
    if { let __sh_a = std::fs::metadata("a").and_then(|m| m.modified()).ok(); let __sh_b = std::fs::metadata("b").and_then(|m| m.modified()).ok(); match (__sh_a, __sh_b) { (Some(__sh_a), Some(__sh_b)) => __sh_a < __sh_b, (None, Some(_)) => true, _ => false } } {
    }
    if { use std::os::unix::fs::MetadataExt; match (std::fs::metadata("a"), std::fs::metadata("b")) { (Ok(__sh_a), Ok(__sh_b)) => __sh_a.dev() == __sh_b.dev() && __sh_a.ino() == __sh_b.ino(), _ => false } } {
    }
    print!("parsed OK\n");
    println!("status: {}", "ok");
    std::process::ExitCode::SUCCESS
}