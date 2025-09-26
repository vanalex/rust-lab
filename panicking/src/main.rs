fn looping_and_panicking(nubers: Vec<i32>) {
    for num in nubers {
        if num < 0 {
            panic!("Boom!");
        }
    }
}

fn main() {
    looping_and_panicking(vec![1, 2, 3, -4, 5]);
}
