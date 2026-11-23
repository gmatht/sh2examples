use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn main() -> std::process::ExitCode {
    print!("ax/pz ax/qz ayz\n");
    print!("ad bd cd\n");
    print!("src/main/a src/main/b src/test/a src/test/b docs/a docs/b\n");
    print!("file.txt file.bak file.orig\n");
    print!("prexpost preypost\n");
    print!("source.bat source.c\n");
    print!("pre");
    print!("xpost ypost\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("done: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}