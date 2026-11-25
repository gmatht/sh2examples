use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn __sh_out_bytes(s: &str) {
    use std::io::Write;
    let mut buf: Vec<u8> = Vec::with_capacity(s.len());
    for c in s.chars() {
        let u = c as u32;
        if (0xF800..=0xF8FF).contains(&u) { buf.push((u - 0xF800) as u8); }
        else { let mut b = [0u8; 4]; buf.extend_from_slice(c.encode_utf8(&mut b).as_bytes()); }
    }
    let _ = std::io::stdout().lock().write_all(&buf);
}

fn main() -> std::process::ExitCode {
    print!("Test non-UTF-8:\n");
    __sh_out_bytes("accnt\n");
    __SH_RC.store(0, Ordering::Relaxed);
    println!("exit: {}", __SH_RC.load(Ordering::Relaxed));
    std::process::ExitCode::SUCCESS
}