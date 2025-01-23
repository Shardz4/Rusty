struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(u32, u32, u32),
}

struct Joint {
    x: i32,
    y: i32,
}

fn  main() {
    let p = Point { x: 0, y: 6 };
    match p {
        Point {x , y: 0} => {
            println!("Point lies on the s axis at {}", x);
        },
        Point {x: 0, y} => {println!("Pount lies on the y axis at {}", y);}
        Point {x: 0, y: 0} => {println!("Point lies on the origin");}

    }

    let msg = Message::ChangeColor(
        Color::HSV(0, 160, 250));
        match msg {
            Message::Quit => println!("Quitting"),
            Message::Move {x,y} => println!("Moving to {}, {}", x,y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Changing color to RGB({}, {}, {})", r, g, b);},
            Mesage::ChangeColor(Color::Hsv(h, s, v)) => {println!("Color changed to HSV({} , {}, {})", h, s, v);}
       
        _=>(),
        }
        let ((feet, inches), Point {a, b} =
    ((5, 10), Point {a: 30, b: 40});
)
}