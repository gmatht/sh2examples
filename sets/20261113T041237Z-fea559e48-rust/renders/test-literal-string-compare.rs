fn main() {
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
    if !("hello".to_string() != "hello".to_string()) {
        print!("var-eq\n");
    }
    if "hello".to_string() != "other".to_string() {
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
}