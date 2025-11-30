fn main() {
    println!("Hello, world!");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}