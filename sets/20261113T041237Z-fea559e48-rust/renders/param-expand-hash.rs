fn main() {
    #[allow(non_snake_case)] let MAXWAIT = "20 10";
    if { let __v = {&MAXWAIT}.to_string(); match __v.rfind(" ") { Some(__i) => __v[..__i].to_string(), None => __v } }.trim().parse::<i64>().unwrap_or(0) > { let __v = {&MAXWAIT}.to_string(); match __v.find(" ") { Some(__i) => __v[__i + " ".len()..].to_string(), None => __v } }.trim().parse::<i64>().unwrap_or(0) {
        print!("compare done\n");
    } else {
    }
}