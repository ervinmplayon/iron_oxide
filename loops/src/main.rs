fn main() {
    let mut count = 0;

    // Loop labels to disambiguate between multiple loops
    // If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop
    // that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 
    // Loop labels must begin with a single quote.

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10; 
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // innermost loop
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
    println!("===========================================");
    println!("WHILE LOOP");
    while_loopz();
    println!("===========================================");
    inefficient_loop();
    println!("===========================================");
    for_loop_frfr();
    println!("===========================================");
    for_loop_liftoff();
}

fn while_loopz() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Range - provided by the standard library, generates all numbers in sequence starting from one number and ending before another number. 
// rev() - reverses the range
fn for_loop_liftoff() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// This code is error prone. The program could panic if the index value or test condition is incorrect. 
// Its also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop. 
fn inefficient_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]); // formatting is different due to presence of index

        index += 1;
    }
}

fn for_loop_frfr() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
