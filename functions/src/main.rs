fn main() {
    println!("Hello, world!");
    another_function(5);
    let x = return_5();
    println!("The value of x is: {x}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn return_5() -> i32 {
    5
}
