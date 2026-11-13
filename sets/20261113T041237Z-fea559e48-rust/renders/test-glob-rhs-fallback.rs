fn main() {
    if "abc" == "a*" {
        print!("bad\n");
    } else {
        print!("literal-star-nomatch\n");
    }
    if "xyz" == "a*" {
        print!("bad\n");
    } else {
        print!("const-star-nomatch\n");
    }
    let pat = "a*";
    if "abc".to_string() == pat {
        print!("bad\n");
    } else {
        print!("var-star-nomatch\n");
    }
    if "abc" == "a*" {
        print!("bad\n");
    } else {
        print!("db-bracket-nomatch\n");
    }
    if "abc" == "[ab]*" {
        print!("bad\n");
    } else {
        print!("bracket-literal-nomatch\n");
    }
}