use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);
}

fn f() {
    println!("Hello from another thread");

    let id = thread::current().id();
    println!("this is my thread id: {id:?}");
}


