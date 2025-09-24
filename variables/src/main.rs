fn main() {
    let message = "This is a string";
    println!("the message is {}", message);
    // message = "" This does not compile
    let mut mutable_message = "This is a mutable string";
    println!("the mutable message is {}", mutable_message);
    mutable_message = "This is a new string";
    println!("the mutable message is {}", mutable_message);
}
