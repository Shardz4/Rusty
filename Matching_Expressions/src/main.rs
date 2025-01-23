#[derive(Debug)]
fn main() {
    value_in_cents(Coin::Quarter(State::Alaska));
    let five = Some(5);
    let six = plus_one(five);
    let none= plus_one(None);

    let some_value= Some(3);
    match some_value{
        Some(3) => println!("Three"),
        _=>(),
    }
}
enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    //..
}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}
fn value_in_cents(coin:Coin) -> u8{
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1;
        }
        Coin::Nickel =>5,
        Coin::Dime=> 10,
        Coin::Quarter(State) =>{
        println!("State quarter from{:?}!", state);
        0;
    }
}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

if let Some(3) = some_value{
    println!("Three");
}
}