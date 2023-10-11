// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // 字面值
    string("red".to_string()); // 字符串
    string(String::from("hi")); // 字符串
    string("rust is fun!".to_owned()); // 类似 to_string() 也是基于 Clone trait的，可以认为是 clone()
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station")); // 都是字面值
    string_slice(&String::from("abc")[0..1]); // slice
    string_slice("  hello there ".trim()); // to_owned() 或者 to_string() 后才是字符串
    string("Happy Monday!".to_string().replace("Mon", "Tues")); 
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // 返回的是一个新字符串
}
