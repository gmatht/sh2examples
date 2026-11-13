fn __sh_fowner(s: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    let uid = std::process::Command::new("id").arg("-u").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    std::fs::metadata(s).map(|m| m.uid().to_string() == uid).unwrap_or(false)
}

fn __sh_fgroup(s: &str) -> bool {
    use std::os::unix::fs::MetadataExt;
    let gid = std::process::Command::new("id").arg("-g").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    std::fs::metadata(s).map(|m| m.gid().to_string() == gid).unwrap_or(false)
}

fn __sh_fnewer(s: &str) -> bool {
    // -N: modified after last read — approximate with mtime > atime
    std::fs::metadata(s).map(|m| {
        m.modified().and_then(|mt| m.accessed().map(|at| mt > at)).unwrap_or(false)
    }).unwrap_or(false)
}

fn main() {
    if std::fs::symlink_metadata("/dev/null").map(|m| m.file_type().is_symlink()).unwrap_or(false) {
    }
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o010000) == 0o010000).unwrap_or(false) } {
    }
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o060000) == 0o060000).unwrap_or(false) } {
    }
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o020000) == 0o020000).unwrap_or(false) } {
    }
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o002000) == 0o002000).unwrap_or(false) } {
    }
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o001000) == 0o001000).unwrap_or(false) } {
    }
    if { use std::os::unix::fs::MetadataExt; std::fs::metadata("/dev/null").map(|m| (m.mode() & 0o004000) == 0o004000).unwrap_or(false) } {
    }
    if __sh_fowner("/dev/null") {
    }
    if __sh_fgroup("/dev/null") {
    }
    if __sh_fnewer("/dev/null") {
    }
    if std::fs::metadata("a").and_then(|m| m.modified()).map(|t| t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)).unwrap_or(0) > std::fs::metadata("b").and_then(|m| m.modified()).map(|t| t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)).unwrap_or(0) {
    }
    if std::fs::metadata("a").and_then(|m| m.modified()).map(|t| t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)).unwrap_or(0) < std::fs::metadata("b").and_then(|m| m.modified()).map(|t| t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)).unwrap_or(0) {
    }
    if { use std::os::unix::fs::MetadataExt; match (std::fs::metadata("a"), std::fs::metadata("b")) { (Ok(__a), Ok(__b)) => __a.dev() == __b.dev() && __a.ino() == __b.ino(), _ => false } } {
    }
    print!("{}", "parsed OK\n".to_string());
    println!("{}", format!("status: {}", "ok".to_string()));
}