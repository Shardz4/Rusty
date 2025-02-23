struct Point<T,U> {
    x: T,
    y: U,
}

impl <T,U> Clone for Point<T,U> {
    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
        Point{
            x: self.x,
            y: other.y,
        }
}}

fn main() { 
    let p1 = Point { x: 1, y:10.4};
    let p2 = Point{x:"Hello", y:'c'};
    letp3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

