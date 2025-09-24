struct Greet{}

impl Greet {
    fn hello() {
        println!("Hello, world!");
    }
}

fn main() {
    println!("Hello, world!");
    Greet::hello();
}
