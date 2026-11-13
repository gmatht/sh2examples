fn main() {
    if !("hello".to_string() != "hello".to_string()) {
        print!("greeting\n");
    } else if !("hello".to_string() != "bye".to_string()) {
        print!("farewell\n");
    } else {
        print!("unknown\n");
    }
}