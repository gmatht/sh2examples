use std::sync::atomic::Ordering;

static __SH_RC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

static __SH_OUT: std::sync::Mutex<Option<Vec<u8>>> = std::sync::Mutex::new(None);
thread_local! { static __SH_OUTFILE_TL: std::cell::RefCell<Option<std::fs::File>> = const { std::cell::RefCell::new(None) }; }
static __SH_STDIN: std::sync::Mutex<Option<Box<dyn std::io::Read + Send>>> = std::sync::Mutex::new(None);
static __SH_STDIN_PATH: std::sync::Mutex<Option<std::path::PathBuf>> = std::sync::Mutex::new(None);

static __SH_PIPESTATUS: std::sync::Mutex<Vec<i32>> = std::sync::Mutex::new(Vec::new());

static __SH_ARITH_WORD_FAIL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! { #[allow(non_upper_case_globals)] static __SHV_longline: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) }; }

fn __sh_q_printf(s: &str) -> String {
    if s.is_empty() { return "''".to_string(); }
    let mut ansi = false;
    for c in s.chars() {
        if (c as u32) < 32 || c as u32 == 127 { ansi = true; }
    }
    if ansi {
        // control chars — bash switches to $'...' ANSI-C quoting
        let mut body = String::new();
        for c in s.chars() {
            match c {
                '\n' => body.push_str("\\n"),
                '\t' => body.push_str("\\t"),
                '\r' => body.push_str("\\r"),
                '\'' => body.push_str("\\'"),
                '\\' => body.push_str("\\\\"),
                c if (c as u32) < 32 || c as u32 == 127 => body.push_str(&format!("\\x{:02x}", c as u32)),
                _ => body.push(c),
            }
        }
        return format!("$'{}'", body);
    }
    // plain word — backslash-escape the shell metachars bash %q escapes
    let mut o = String::new();
    for c in s.chars() {
        match c {
            ' ' | '\'' | '\\' | '"' | '$' | '`' => { o.push('\\'); o.push(c); }
            _ => o.push(c),
        }
    }
    o
}

fn __sh_echo_esc(s: &str) -> String {
    let mut o = String::new();
    let ch: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < ch.len() {
        if ch[i] == '\\' && i + 1 < ch.len() {
            i += 1;
            match ch[i] {
                'n' => o.push('\n'), 't' => o.push('\t'), 'r' => o.push('\r'),
                'a' => o.push('\x07'), 'b' => o.push('\x08'), 'f' => o.push('\x0c'),
                'v' => o.push('\x0b'), 'e' => o.push('\x1b'), '\\' => o.push('\\'),
                'c' => return o,
                '0'..='7' => {
                    let mut n = 0; let mut k = 0;
                    while k < 3 && i + k < ch.len() && ch[i + k].is_ascii_digit() && ch[i + k] <= '7' {
                        n = n * 8 + (ch[i + k] as u32 - '0' as u32); k += 1;
                    }
                    o.push(char::from_u32(n & 0xff).unwrap_or('\0'));
                    i += k - 1;
                }
                'x' => {
                    let mut n = 0; let mut k = 0;
                    while k < 2 && i + 1 + k < ch.len() && ch[i + 1 + k].is_ascii_hexdigit() {
                        n = n * 16 + ch[i + 1 + k].to_digit(16).unwrap(); k += 1;
                    }
                    if k > 0 { o.push(char::from_u32(n).unwrap_or('\0')); i += k; }
                    else { o.push('x'); }
                }
                c => { o.push('\\'); o.push(c); }
            }
            i += 1;
        } else { o.push(ch[i]); i += 1; }
    }
    o
}

fn __sh_printf_b(s: &str) -> String { __sh_echo_esc(s) }

fn __sh_atoi(s: &str) -> i64 {
    let t = s.trim();
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        return i64::from_str_radix(h, 16).unwrap_or(0);
    }
    t.parse::<i64>().unwrap_or(0)
}

fn __sh_atou(s: &str) -> u64 {
    let t = s.trim();
    if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        return u64::from_str_radix(h, 16).unwrap_or(0);
    }
    t.parse::<u64>().unwrap_or(0)
}

fn __sh_atof(s: &str) -> f64 {
    s.trim().parse().unwrap_or(0.0)
}

fn __sh_printf(fmt: &str, args: &[String]) -> String {
    let mut out = String::new();
    let mut ai = 0usize;
    loop {
        let start_ai = ai;
        let ch: Vec<char> = fmt.chars().collect();
        let mut i = 0;
        while i < ch.len() {
            let c = ch[i];
            if c == '\\' && i + 1 < ch.len() {
                i += 1;
                match ch[i] {
                    'n' => out.push('\n'), 't' => out.push('\t'), 'r' => out.push('\r'),
                    'a' => out.push('\x07'), 'b' => out.push('\x08'), 'f' => out.push('\x0c'),
                    'v' => out.push('\x0b'), 'e' => out.push('\x1b'), '\\' => out.push('\\'),
                    'c' => return out,
                    '0'..='7' => {
                        let mut n = 0; let mut k = 0;
                        while k < 3 && i + k < ch.len() && ch[i + k].is_ascii_digit() && ch[i + k] <= '7' {
                            n = n * 8 + (ch[i + k] as u32 - '0' as u32); k += 1;
                        }
                        out.push(char::from_u32(n & 0xff).unwrap_or('\0'));
                        i += k - 1;
                    }
                    'x' => {
                        let mut n = 0; let mut k = 0;
                        while k < 2 && i + 1 + k < ch.len() && ch[i + 1 + k].is_ascii_hexdigit() {
                            n = n * 16 + ch[i + 1 + k].to_digit(16).unwrap(); k += 1;
                        }
                        if k > 0 { out.push(char::from_u32(n).unwrap_or('\0')); i += k; }
                        else { out.push('x'); }
                    }
                    _ => { out.push('\\'); out.push(ch[i]); }
                }
                i += 1;
                continue;
            }
            if c == '%' && i + 1 < ch.len() {
                let mut j = i + 1;
                let mut flags = String::new();
                while j < ch.len() && "-+ 0#".contains(ch[j]) { flags.push(ch[j]); j += 1; }
                let mut width = String::new();
                while j < ch.len() && ch[j].is_ascii_digit() { width.push(ch[j]); j += 1; }
                let mut prec: Option<String> = None;
                if j < ch.len() && ch[j] == '.' {
                    j += 1;
                    let mut p = String::new();
                    while j < ch.len() && ch[j].is_ascii_digit() { p.push(ch[j]); j += 1; }
                    prec = Some(p);
                }
                // C LENGTH MODIFIERS (`%lld`, `%zu`, `%hhd`) — skip them:
                // an unconsumed 'l' leaked into the output verbatim
                // (corpus-c 008_sized_ints printed "4000000000ld")
                while j < ch.len() && "lhLzjt".contains(ch[j]) { j += 1; }
                if j >= ch.len() { out.push('%'); break; }
                let conv = ch[j];
                i = j + 1;
                if conv == '%' { out.push('%'); continue; }
                let arg = args.get(ai).cloned().unwrap_or_default();
                ai += 1;
                let w: usize = width.parse().unwrap_or(0);
                let left = flags.contains('-');
                let zero = flags.contains('0');
                let p = prec.as_ref().map(|s| s.parse().unwrap_or(0)).unwrap_or(-1);
                let mut piece = String::new();
                match conv {
                    's' => {
                        piece = arg.clone();
                        if p >= 0 { piece = piece.chars().take(p as usize).collect(); }
                    }
                    'b' => {
                        piece = __sh_printf_b(&arg);
                        if p >= 0 { piece = piece.chars().take(p as usize).collect(); }
                    }
                    'q' => { piece = __sh_q_printf(&arg); }
                    'c' => { piece = arg.chars().next().map(|c| c.to_string()).unwrap_or_default(); }
                    'd' | 'i' => { piece = format!("{}", __sh_atoi(&arg)); }
                    'u' => { piece = format!("{}", __sh_atou(&arg)); }
                    'o' => { piece = format!("{:o}", __sh_atou(&arg)); }
                    'x' => { piece = format!("{:x}", __sh_atou(&arg)); }
                    'X' => { piece = format!("{:X}", __sh_atou(&arg)); }
                    'f' | 'F' => {
                        let fv = __sh_atof(&arg);
                        let p2 = if p < 0 { 6 } else { p as usize };
                        piece = format!("{:.*}", p2, fv);
                    }
                    'e' | 'E' => {
                        let fv = __sh_atof(&arg);
                        let p2 = if p < 0 { 6 } else { p as usize };
                        piece = format!("{:.*e}", p2, fv);
                        if conv == 'E' { piece = piece.to_uppercase(); }
                    }
                    'g' | 'G' => {
                        let fv = __sh_atof(&arg);
                        let p2 = if p < 0 { 6 } else { p as usize };
                        piece = format!("{:.*}", p2, fv);
                    }
                    _ => { piece = arg.clone(); }
                }
                if w > piece.chars().count() {
                    let pad = w - piece.chars().count();
                    if left { piece = format!("{}{}", piece, " ".repeat(pad)); }
                    else if zero { piece = format!("{}{}", "0".repeat(pad), piece); }
                    else { piece = format!("{}{}", " ".repeat(pad), piece); }
                }
                out.push_str(&piece);
                continue;
            }
            out.push(c);
            i += 1;
        }
        if ai >= args.len() || ai == start_ai { break; }
    }
    out
}

fn __sh_capture_rc(cmd: &str) -> (String, i32) {
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    c.stdout(std::process::Stdio::piped());
    c.stdin(std::process::Stdio::inherit());
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

fn __sh_spawn(cmd: &str, input: Option<&[u8]>) -> i32 {
    // a shell-out inside a native stage inherits the stage's stdin
    // (`echo x | grep -f <(…)` — the grep reads the echo's buffer)
    let mut input = input.map(|d| d.to_vec());
    let stdin_path = __SH_STDIN_PATH.lock().unwrap().clone();
    if input.is_none() && stdin_path.is_none() {
        if let Some(mut r) = __SH_STDIN.lock().unwrap().take() {
            let mut d = Vec::new();
            let _ = std::io::Read::read_to_end(&mut r, &mut d);
            input = Some(d);
        }
    }
    let want_out = __SH_OUT.lock().unwrap().is_some() || __SH_OUTFILE_TL.with(|v| v.borrow().is_some());
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(cmd);
    if !want_out {
        // the child inherits fd 1 raw — flush Rust's buffered stdout
        // first so prints BEFORE the child hit the pipe first
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }
    if want_out { c.stdout(std::process::Stdio::piped()); }
    if input.is_none() {
        if let Some(p) = &stdin_path {
            // the child must see the REAL device fd (`tty < /dev/pts/5`
            // needs isatty, which a byte pipe lacks)
            if let Ok(f) = std::fs::File::open(p) {
                c.stdin(std::process::Stdio::from(f));
            }
        }
    }
    if input.is_some() { c.stdin(std::process::Stdio::piped()); }
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return 1 };
    if let Some(data) = input {
        if let Some(mut si) = ch.stdin.take() { let _ = std::io::Write::write_all(&mut si, &data); }
    }
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    if !out.is_empty() {
        let mut used = false;
        __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &out); used = true; } });
        if !used {
            if let Some(b) = __SH_OUT.lock().unwrap().as_mut() {
                b.extend_from_slice(&out);
            } else {
                let _ = std::io::Write::write_all(&mut std::io::stdout(), &out);
            }
        }
    }
    rc
}

fn __sh_grepmatches(text: &str, pat: &str, flags: &str) -> (String, i32) {
    let mut opts = String::from("-o");
    if flags.contains('i') { opts.push('i'); }
    if flags.contains('E') { opts.push('E'); }
    let cmd = format!("printf '%s' \"$1\" | grep {opts} \"$2\"");
    let mut c = std::process::Command::new("bash");
    c.arg("-c").arg(&cmd).arg("sh").arg(text).arg(pat);
    c.stdout(std::process::Stdio::piped());
    let mut ch = match c.spawn() { Ok(x) => x, Err(_) => return (String::new(), 1) };
    let mut out = Vec::new();
    if let Some(mut so) = ch.stdout.take() {
        let _ = std::io::Read::read_to_end(&mut so, &mut out);
    }
    let rc = ch.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);
    let mut s = String::from_utf8_lossy(&out).to_string();
    while s.ends_with('\n') { s.pop(); }
    (s, rc)
}

type __ShCapState = (Option<Vec<u8>>, Option<Box<dyn std::io::Read + Send>>, Option<std::fs::File>);

#[allow(non_upper_case_globals)]
fn main() {
    println!("{}", "== Basic grep parameters ==".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    if !({
let mut __sh_pbuf0: Vec<u8> = Vec::new();
__SH_PIPESTATUS.lock().unwrap().clear();
{
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"text with pattern\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf0 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
__SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-i").arg("PATTERN").stdin(std::process::Stdio::piped()).spawn() { Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf0); } __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1) }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
__SH_RC.load(Ordering::SeqCst) == 0
} && {
print!("{}", "  -i match: OK\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("{}", "  -i match: FAIL\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    let count = {
    let __cap0_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap0_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap0_in = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf2: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout3 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"line1\nline2\nline3\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf2 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout3);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("grep").arg("-v").arg("line2").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("wc").arg("-l").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf2); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap0_in;
    let __cap0_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap0_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap0_outf);
    let mut __s = String::from_utf8_lossy(&__cap0_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("  -v count: {} (expected 2)", count.clone()));
    let mut matched = {
    let __cap1_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap1_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap1_in = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf4: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout5 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"match\nno match\nmatch again\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf4 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout5);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-c").arg("match").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn() {Ok(mut __cc) => { if let Some(mut __ci) = __cc.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf4); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap1_in;
    let __cap1_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap1_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap1_outf);
    let mut __s = String::from_utf8_lossy(&__cap1_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("  -c count: {} (expected 2)", matched.clone()));
    println!("{}", "== Context parameters ==".to_string());
    let mut __sh_pbuf6: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"line1\nline2\nTARGET\nline4\nline5\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf6 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-A' '2' 'TARGET'".to_string(), "'/tmp/grep_out.txt'".to_string()), Some(&__sh_pbuf6)), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", format!("  -A 2 lines: {} (expected 3)", { let (__c, __r) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'".to_string(), "'/tmp/grep_out.txt'".to_string())); __SH_RC.store(__r, Ordering::SeqCst); __c }));
    let mut __sh_pbuf8: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"line1\nline2\nTARGET\nline4\nline5\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf8 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-B' '2' 'TARGET'".to_string(), "'/tmp/grep_out.txt'".to_string()), Some(&__sh_pbuf8)), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", format!("  -B 2 lines: {} (expected 3)", { let (__c, __r) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'".to_string(), "'/tmp/grep_out.txt'".to_string())); __SH_RC.store(__r, Ordering::SeqCst); __c }));
    let mut __sh_pbuf10: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"line1\nline2\nTARGET\nline4\nline5\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf10 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-C' '1' 'TARGET'".to_string(), "'/tmp/grep_out.txt'".to_string()), Some(&__sh_pbuf10)), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", format!("  -C 1 lines: {} (expected 3)", { let (__c, __r) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'".to_string(), "'/tmp/grep_out.txt'".to_string())); __SH_RC.store(__r, Ordering::SeqCst); __c }));
    println!("{}", "== File handling parameters ==".to_string());
    {
    __SH_RC.store(0, Ordering::SeqCst);
    let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/grep_file.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "content\n".as_bytes());
    }
    if !({ let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-c").arg("content").arg("/tmp/grep_file.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } && {
print!("{}", "  -c file: OK\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("{}", "  -c file: FAIL\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if !({ let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-l").arg("content").arg("/tmp/grep_file.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } && {
print!("{}", "  -l: found\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("{}", "  -l: not found\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    if !({ let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("grep").arg("-L").arg("nonexistent").arg("/tmp/grep_file.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } && {
print!("{}", "  -L: not found (correct)\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("{}", "  -L: found (wrong)\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", "== Output formatting parameters ==".to_string());
    matched = {
    let __cap2_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap2_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap2_in = __SH_STDIN.lock().unwrap().take();
    let (__m, __r) = __sh_grepmatches("text with pattern in it", "pattern", "");
    if !__m.is_empty() { { let __w: &[Vec<String>] = {&[vec![__m]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } }; }
    __SH_RC.store(__r, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap2_in;
    let __cap2_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap2_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap2_outf);
    let mut __s = String::from_utf8_lossy(&__cap2_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    __SH_RC.store(0, Ordering::SeqCst);
    println!("{}", format!("  -o match: '{}' (expected 'pattern')", matched.clone()));
    let lineno = {
    let __cap3_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap3_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap3_in = __SH_STDIN.lock().unwrap().take();
    let mut __sh_pbuf12: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __sh_oldout13 = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"text with pattern in it\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf12 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __sh_oldout13);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); let __rs0 = std::process::Command::new("grep").arg("-n").arg("pattern").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn(); match __rs0 { Ok(mut __p0) => { let __rs1 = { let __x = std::process::Stdio::from(__p0.stdout.take().unwrap()); std::process::Command::new("cut").arg("-d").arg(":").arg("-f").arg("1").stdin(__x).stdout(std::process::Stdio::piped()).spawn() }; match __rs1 { Ok(mut __cc) => { if let Some(mut __ci) = __p0.stdin.take() { let _ = std::io::Write::write_all(&mut __ci, &__sh_pbuf12); } let mut __co: Vec<u8> = Vec::new(); if let Some(mut __cs) = __cc.stdout.take() { let _ = std::io::Read::read_to_end(&mut __cs, &mut __co); } let __cr = __cc.wait().map(|s| s.code().unwrap_or(1)).unwrap_or(1);  let _ = __p0.wait(); if !__co.is_empty() { let mut __cu = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, &__co); __cu = true; } }); if !__cu { if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(&__co); } else { let _ = std::io::Write::write_all(&mut std::io::stdout(), &__co); } } } __cr }, Err(e) => {  let _ = __p0.wait();if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Err(e) => { if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } } }, Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap3_in;
    let __cap3_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap3_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap3_outf);
    let mut __s = String::from_utf8_lossy(&__cap3_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("  -n line: {} (expected 1)", lineno.clone()));
    println!("{}", "== Recursive parameters ==".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    if { let __r = { let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("mkdir").arg("-p").arg("/tmp/grep_sub").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }; __SH_RC.store(__r, Ordering::SeqCst); __r == 0 } {
        {
        __SH_RC.store(0, Ordering::SeqCst);
        let _ = std::io::Write::write_all(&mut std::fs::File::create("/tmp/grep_sub/file.txt").unwrap_or_else(|_| std::fs::File::open("/dev/null").unwrap()), "subfile content\n".as_bytes());
        }
    }
    let found = {
    let __cap4_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap4_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap4_in = __SH_STDIN.lock().unwrap().take();
    __SH_PIPESTATUS.lock().unwrap().clear();
    __SH_RC.store(__sh_spawn(&format!("{} | {}", format!("{} 2> {}", "'grep' '-r' 'subfile' '/tmp/grep_sub'".to_string(), "'/dev/null'".to_string()), "'wc' '-l'".to_string()), Some(&[])), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    *__SH_STDIN.lock().unwrap() = __cap4_in;
    let __cap4_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap4_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap4_outf);
    let mut __s = String::from_utf8_lossy(&__cap4_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
};
    println!("{}", format!("  -r recursive: {} files matched (expected 1)", found.clone()));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-r").arg("-f").arg("/tmp/grep_sub").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    println!("{}", "== Line length parameters ==".to_string());
    __SH_RC.store(0, Ordering::SeqCst);
    __SHV_longline.with(|v| *v.borrow_mut() = {
    let __cap5_outf = __SH_OUTFILE_TL.with(|v| v.borrow_mut().take());
    let __cap5_out = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    let __cap5_in = __SH_STDIN.lock().unwrap().take();
    { let __w: &[Vec<String>] = {&[vec![__sh_printf("a%.0s", &vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "10".to_string(), "11".to_string(), "12".to_string(), "13".to_string(), "14".to_string(), "15".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "19".to_string(), "20".to_string(), "21".to_string(), "22".to_string(), "23".to_string(), "24".to_string(), "25".to_string(), "26".to_string(), "27".to_string(), "28".to_string(), "29".to_string(), "30".to_string(), "31".to_string(), "32".to_string(), "33".to_string(), "34".to_string(), "35".to_string(), "36".to_string(), "37".to_string(), "38".to_string(), "39".to_string(), "40".to_string(), "41".to_string(), "42".to_string(), "43".to_string(), "44".to_string(), "45".to_string(), "46".to_string(), "47".to_string(), "48".to_string(), "49".to_string(), "50".to_string(), "51".to_string(), "52".to_string(), "53".to_string(), "54".to_string(), "55".to_string(), "56".to_string(), "57".to_string(), "58".to_string(), "59".to_string(), "60".to_string(), "61".to_string(), "62".to_string(), "63".to_string(), "64".to_string(), "65".to_string(), "66".to_string(), "67".to_string(), "68".to_string(), "69".to_string(), "70".to_string(), "71".to_string(), "72".to_string(), "73".to_string(), "74".to_string(), "75".to_string(), "76".to_string(), "77".to_string(), "78".to_string(), "79".to_string(), "80".to_string(), "81".to_string(), "82".to_string(), "83".to_string(), "84".to_string(), "85".to_string(), "86".to_string(), "87".to_string(), "88".to_string(), "89".to_string(), "90".to_string(), "91".to_string(), "92".to_string(), "93".to_string(), "94".to_string(), "95".to_string(), "96".to_string(), "97".to_string(), "98".to_string(), "99".to_string(), "100".to_string(), "101".to_string(), "102".to_string(), "103".to_string(), "104".to_string(), "105".to_string(), "106".to_string(), "107".to_string(), "108".to_string(), "109".to_string(), "110".to_string(), "111".to_string(), "112".to_string(), "113".to_string(), "114".to_string(), "115".to_string(), "116".to_string(), "117".to_string(), "118".to_string(), "119".to_string(), "120".to_string(), "121".to_string(), "122".to_string(), "123".to_string(), "124".to_string(), "125".to_string(), "126".to_string(), "127".to_string(), "128".to_string(), "129".to_string(), "130".to_string(), "131".to_string(), "132".to_string(), "133".to_string(), "134".to_string(), "135".to_string(), "136".to_string(), "137".to_string(), "138".to_string(), "139".to_string(), "140".to_string(), "141".to_string(), "142".to_string(), "143".to_string(), "144".to_string(), "145".to_string(), "146".to_string(), "147".to_string(), "148".to_string(), "149".to_string(), "150".to_string(), "151".to_string(), "152".to_string(), "153".to_string(), "154".to_string(), "155".to_string(), "156".to_string(), "157".to_string(), "158".to_string(), "159".to_string(), "160".to_string(), "161".to_string(), "162".to_string(), "163".to_string(), "164".to_string(), "165".to_string(), "166".to_string(), "167".to_string(), "168".to_string(), "169".to_string(), "170".to_string(), "171".to_string(), "172".to_string(), "173".to_string(), "174".to_string(), "175".to_string(), "176".to_string(), "177".to_string(), "178".to_string(), "179".to_string(), "180".to_string(), "181".to_string(), "182".to_string(), "183".to_string(), "184".to_string(), "185".to_string(), "186".to_string(), "187".to_string(), "188".to_string(), "189".to_string(), "190".to_string(), "191".to_string(), "192".to_string(), "193".to_string(), "194".to_string(), "195".to_string(), "196".to_string(), "197".to_string(), "198".to_string(), "199".to_string(), "200".to_string()])]]}; let __nl = {false}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    *__SH_STDIN.lock().unwrap() = __cap5_in;
    let __cap5_buf = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __cap5_out;
    __SH_OUTFILE_TL.with(|v| *v.borrow_mut() = __cap5_outf);
    let mut __s = String::from_utf8_lossy(&__cap5_buf).to_string();
    while __s.ends_with('\n') { __s.pop(); }
    __s
});
    if !({
let mut __sh_pbuf15: Vec<u8> = Vec::new();
__SH_PIPESTATUS.lock().unwrap().clear();
{
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __w: &[Vec<String>] = {&[vec![__SHV_longline.with(|v| v.borrow().clone())]]}; let __nl = {true}; let _ = {false}; if __SH_ARITH_WORD_FAIL.swap(false, Ordering::SeqCst) { } else { let mut __s = String::new(); let mut __f = true; for __ww in __w { for __x in __ww { if !__f { __s.push(' '); } __f = false; __s.push_str(__x); } } if __nl { __s.push('\n'); } let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __s.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__s.as_bytes()); } else { print!("{}", __s); } } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf15 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
__SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-m' '1' 'a'".to_string(), "'/dev/null'".to_string()), Some(&__sh_pbuf15)), Ordering::SeqCst);
__SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
__SH_RC.load(Ordering::SeqCst) == 0
} && {
print!("{}", "  -m 1 (long line): OK\n");
__SH_RC.store(0, Ordering::SeqCst);
__SH_RC.load(Ordering::SeqCst) == 0
}) {
        print!("{}", "  -m 1 (long line): FAIL\n");
        __SH_RC.store(0, Ordering::SeqCst);
    }
    println!("{}", "== Word-regexp and line-regexp parameters ==".to_string());
    let mut __sh_pbuf17: Vec<u8> = Vec::new();
    __SH_PIPESTATUS.lock().unwrap().clear();
    {
    let __old = __SH_OUT.lock().unwrap().take();
    *__SH_OUT.lock().unwrap() = Some(Vec::new());
    { let __t: &str = {"foo\nfoobar\nbar\n"}; let mut __u = false; __SH_OUTFILE_TL.with(|v| { if let Some(f) = v.borrow_mut().as_mut() { let _ = std::io::Write::write_all(f, __t.as_bytes()); __u = true; } }); if __u { } else if let Some(b) = __SH_OUT.lock().unwrap().as_mut() { b.extend_from_slice(__t.as_bytes()); } else { print!("{}", __t); } };
    __SH_RC.store(0, Ordering::SeqCst);
    __sh_pbuf17 = __SH_OUT.lock().unwrap().take().unwrap();
    *__SH_OUT.lock().unwrap() = __old;
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
}
    __SH_RC.store(__sh_spawn(&format!("{} > {}", "'grep' '-w' 'foo'".to_string(), "'/tmp/grep_out.txt'".to_string()), Some(&__sh_pbuf17)), Ordering::SeqCst);
    __SH_PIPESTATUS.lock().unwrap().push(__SH_RC.load(Ordering::SeqCst));
    println!("{}", format!("  -w word match lines: {} (expected 1)", { let (__c, __r) = __sh_capture_rc(&format!("{} < {}", "'wc' '-l'".to_string(), "'/tmp/grep_out.txt'".to_string())); __SH_RC.store(__r, Ordering::SeqCst); __c }));
    __SH_RC.store({ let _ = std::io::Write::flush(&mut std::io::stdout()); match std::process::Command::new("rm").arg("-f").arg("/tmp/grep_file.txt").arg("/tmp/grep_out.txt").status() { Ok(s) => s.code().unwrap_or(1), Err(e) => if e.kind() == std::io::ErrorKind::NotFound { 127 } else { 126 } } }, Ordering::SeqCst);
    std::process::exit(__SH_RC.load(Ordering::SeqCst));
}