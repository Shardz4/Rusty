struct ImportantExercept<'a> {
    part: &'a str,
}
// it is saying that the struct cannot outlive the reference passed into part
impl<'a> ImportantExercept<'a> {
    fn announce_and_return_part(&'a self, announcement: &str) -> &'a str {
        // Although we don't need to specify this 'a according to rule 3 of lifetimes.use
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExercept { part: first_sentence };
}