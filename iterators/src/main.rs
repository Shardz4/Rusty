#[derive(PartialEq, Debug)]
struct Shoe {
    style : u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
 #[test]
 fn filter_by_size () {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe{
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        }
    ]

 let in_my_size = shoes_in_my_size(shoes,10);
 assert_eq!(
     in_my_size,
     vec![
         Shoe {
             size: 10,
             style: String::from("sneaker"),
         },
         Shoe {
             size: 10,
             style: String::from("boot"),
         },
     ]

        )
    }

fn main {

}