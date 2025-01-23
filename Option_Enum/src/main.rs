
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = None;
    let sum = x+y.unwrap_or(0);
    } 