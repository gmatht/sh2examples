fn main() -> std::process::ExitCode {
    let name = "John Doe";
    println!("Hello {}", "John_Doe");
    println!("Length: {}", (name.len() as i64));
    println!("First: {}", name.chars().skip((0) as usize).take((4) as usize).collect::<String>());
    println!("Last: {}", { let __sh_s = &name; let __sh_o = -3; let __sh_l = i64::MIN; let __sh_ch: Vec<char> = __sh_s.chars().collect(); let __sh_n = __sh_ch.len() as i64; let mut __sh_st = if __sh_o < 0 { __sh_n + __sh_o } else { __sh_o }; if __sh_st < 0 { __sh_st = 0; } if __sh_st > __sh_n { String::new() } else { let mut __sh_cn = if __sh_l == i64::MIN { __sh_n - __sh_st } else if __sh_l < 0 { __sh_n + __sh_l - __sh_st } else { __sh_l }; if __sh_cn < 0 { __sh_cn = 0; } if __sh_st + __sh_cn > __sh_n { __sh_cn = __sh_n - __sh_st; } __sh_ch[__sh_st as usize..(__sh_st + __sh_cn) as usize].iter().collect::<String>() } });
    std::process::ExitCode::SUCCESS
}