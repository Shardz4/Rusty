use std::thread;
use std::time::Duration;

// 
fn stimulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn main() {

    struct Cacher <T>
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
}
impl <T> Cacher <T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32){
let mut cached_result = Cacher::new ;|num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
if intensity < 25 {
    println!("Today, do {} pushups!", cached_result.value);
    println!("Next, do {} situps!", cached_result.value);
}
else {
    if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    }
    else {
        println!(
            "Today, run for {} minutes!",
            cached_result.value
        );
    }
}
}