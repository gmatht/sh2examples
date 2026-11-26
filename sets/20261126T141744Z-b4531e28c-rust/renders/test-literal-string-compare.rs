fn main() -> std::process::ExitCode {
    if "a" == "a" {
        print!("eq\n");
    }
    if "a" != "a" {
        print!("bad\n");
    } else {
        print!("ne\n");
    }
    if "a" == "b" {
        print!("bad\n");
    } else {
        print!("not-equal\n");
    }
    if "hello" == "hello" {
        print!("var-eq\n");
    }
    if "hello" != "other" {
        print!("var-ne\n");
    }
    if "10" == "10" {
        print!("num-eq\n");
    }
    if "10" == "010" {
        print!("bad\n");
    } else {
        print!("num-string-distinct\n");
    }
    std::process::ExitCode::SUCCESS
}