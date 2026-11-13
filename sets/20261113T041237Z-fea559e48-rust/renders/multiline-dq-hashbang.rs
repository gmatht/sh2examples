fn main() {
    print!("#!/bin/sh\nexec 'command' --option \"$@\"\n");
}