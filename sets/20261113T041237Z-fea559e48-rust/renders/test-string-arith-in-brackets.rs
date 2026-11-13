fn main() {
    let a = [10, 20, 30, 40];
    let n = 1;
    if n < (a[1].to_string().trim().parse::<i64>().unwrap_or(0) + a[2].to_string().trim().parse::<i64>().unwrap_or(0)) {
        print!("arith-test-ok\n");
    } else {
        print!("arith-test-bad\n");
    }
    print!("done\n");
}