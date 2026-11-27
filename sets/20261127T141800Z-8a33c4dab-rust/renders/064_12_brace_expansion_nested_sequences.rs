use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    {
        let mut __sh_status: i32 = 0;
        for __sh_path in &vec!["project/src/main/java".to_string(), "project/src/main/resources".to_string(), "project/src/test/java".to_string(), "project/src/test/resources".to_string(), "project/docs/api".to_string(), "project/docs/user".to_string(), "project/build/classes".to_string(), "project/build/lib".to_string()] {
            let __sh_res = if true { std::fs::create_dir_all(__sh_path) } else { std::fs::create_dir(__sh_path) };
            match __sh_res {
                Ok(_) => {
                    if false { println!("mkdir: created directory '{}'", __sh_path); }
                }
                Err(__sh_err) => {
                    // MEASURED shapes, which are NOT one format: this host's
                    // mkdir names the path on `File exists` and omits it on
                    // `Permission denied` / `No such file or directory`.
                    match __sh_err.kind() {
                        std::io::ErrorKind::AlreadyExists => eprintln!("mkdir: {}: File exists", __sh_path),
                        std::io::ErrorKind::PermissionDenied => eprintln!("mkdir: Permission denied"),
                        std::io::ErrorKind::NotFound => eprintln!("mkdir: No such file or directory"),
                        _ => eprintln!("mkdir: {}: {}", __sh_path, __sh_err),
                    }
                    __sh_status = 1;
                }
            }
        }
        __SH_RC.store(__sh_status, Ordering::Relaxed);
    }
    print!("Project structure created\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}