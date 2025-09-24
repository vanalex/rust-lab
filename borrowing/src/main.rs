fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.push(6);
    let my_int = 10;
    let my_string = String::from("hello, world");
    own_vector(my_vec);
    own_integer(my_int);
    own_string(my_string);


}

fn own_vector(mut vector: Vec<i32>){
    vector.push(10);
    println!("vector => {:?}", vector);
}

fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}