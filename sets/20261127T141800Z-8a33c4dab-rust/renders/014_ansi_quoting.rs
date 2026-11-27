fn main() -> std::process::ExitCode {
    print!("== ANSI-C quoting ==\n");
    print!("line1\nline2\tTabbed\n");
    print!("== Escape sequences ==\n");
    print!("bell\x07\n");
    print!("backspace\x08\n");
    print!("formfeed\x0c\n");
    print!("newline\n\n");
    print!("carriage\rreturn\n");
    print!("tab\tseparated\n");
    print!("vertical\x0btab\n");
    print!("== Unicode and hex ==\n");
    print!("Hello\n");
    print!("Hello\n");
    print!("== Practical examples ==\n");
    print!("Name       Age        City\n");
    print!("John       25         NYC\n");
    print!("Jane       30         LA\n");
    std::process::ExitCode::SUCCESS
}