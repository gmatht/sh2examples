fn main() -> std::process::ExitCode {
    print!("#!/bin/sh\nexec 'command' --option \"$@\"\n");
    std::process::ExitCode::SUCCESS
}