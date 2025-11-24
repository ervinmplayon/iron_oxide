fn main() {
    println!("Hello, world!");
    another_function(5);

    let x = return_5(); // would be the same as let x = 5;
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The new value of x is {x}");

    let y = {
        let x = 3; // a statement
        x + 1 // an expression
    };
    println!("The value of y is: {y}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn return_5() -> i32 {
    5 // an expression whose value we want to return
}

fn plus_one(x: i32) -> i32 {
    x + 1   // Also an expression. Putting a ; in here would turn this into a statement. 
            // Statements do not evaluate to a value, which is expressed by (), the unit type. 
            // And therefore , nothing is returned, which contradicts the function definition and results in an error. 
}
