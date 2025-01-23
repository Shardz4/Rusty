use std::fmt::Display;
fn longest_with_an_announcement<'a,T>(
    x: &'a str,
    y: &'a str,
    ann:T,
) -> &'a str
where T: Display,
// Here we are bounding Generic reference T.
 {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {}
