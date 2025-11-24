fn main() {
    let mut count = 0;
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
}

// Loop labels to disambiguate between multiple loops
// If you have loops within loops, break and continue apply to the innermost loop at that point. You can optionally specify a loop label on a loop
// that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 
// Loop labels must begin with a single quote.

fn while_loopz() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
