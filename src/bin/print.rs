fn info<T: std::fmt::Display>(msg: &T) {
    println!("{}", msg);
}

fn main() {
    let a: &str = "Hello";
    info(&a);

    let b: String = "World".to_string();
    info(&b);
}

#[test]
fn test_info() {
    info(&"Hello");
}