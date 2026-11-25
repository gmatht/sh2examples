use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    if std::path::Path::new("/var/lib/foo").is_dir() {
        {
            let mut __sh_status: i32 = 0;
            let err_text = |__e: &std::io::Error| -> String { match __e.kind() {
                std::io::ErrorKind::NotFound => "No such file or directory".into(),
                std::io::ErrorKind::PermissionDenied => "Permission denied".into(),
                std::io::ErrorKind::AlreadyExists => "File exists".into(),
                std::io::ErrorKind::IsADirectory => "Is a directory".into(),
                std::io::ErrorKind::NotADirectory => "Not a directory".into(),
                _ => format!("{}", __e) } };
            for __sh_path in &vec![format!("{}/??\\:??\\:??", "/var/lib/foo")] {
                match std::fs::symlink_metadata(__sh_path) {
                    Ok(__sh_mapped) => {
                if __sh_mapped.is_dir() {
                    eprintln!("rm: cannot remove '{}': Is a directory", __sh_path);
                    __sh_status = 1;
                    continue;
                }
                let __sh_res = std::fs::remove_file(__sh_path);
                        if let Err(__sh_err) = __sh_res {
                            if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                                eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                                __sh_status = 1;
                            }
                        }
                    }
                    Err(__sh_err) => {
                            if __sh_err.kind() == std::io::ErrorKind::NotFound { /* -f: a missing path is not an error */ } else {
                                eprintln!("rm: cannot remove '{}': {}", __sh_path, err_text(&__sh_err));
                                __sh_status = 1;
                            }
                    }
                }
            }
            __SH_RC.store(__sh_status, Ordering::Relaxed);
        }
    } else {
    }
    print!("f=[/var/lib/foo]\n");
    std::process::ExitCode::SUCCESS
}