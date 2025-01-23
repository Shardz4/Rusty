fn main() {
    let string1 = String::from("abcd");{
    let string2 = String::from("xyz");
    let result = longesst(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}
}
fn longesst<'a>(x:&'a str, y:&'a str) -> &'a str {
    // This means that x and y will have the same generic lifetime length. They don't actually change the lifetime but create a relationship between the lifetime of multiple references. 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
