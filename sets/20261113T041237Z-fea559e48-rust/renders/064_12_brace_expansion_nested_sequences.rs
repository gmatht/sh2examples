use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() {
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mkdir").arg("-p").arg("project/src/main/java").arg("project/src/main/resources").arg("project/src/test/java").arg("project/src/test/resources").arg("project/docs/api").arg("project/docs/user").arg("project/build/classes").arg("project/build/lib").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    print!("Project structure created\n");
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("done: {}", __SH_RC.load(Ordering::SeqCst).to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}