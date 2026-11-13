use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

fn main() {
    let mut __sh_pbuf0: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout3 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let mut __sh_pbuf4: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout5 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"Level 3\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    { let __t: &str = {"Level 4\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    { let __t: &str = {"Still level 4\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf4 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout5);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("Level").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf4); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout3);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("sed").arg("s/Level/Depth/").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf2); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("wc").arg("-l").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf0); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", format!("status: {}", "ok".to_string()));
    __SH_RC.store(0, Ordering::SeqCst);
}