fn main() {
    for i in [1, 2, 3] {
        if !(!(i != 2)) {
            continue;
        }
        println!("{}", i);
    }
}