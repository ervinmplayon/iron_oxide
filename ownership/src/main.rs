fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.
    println!("value of x: {x}");

    // return values ============================================================
    let s1 = gives_ownership(); // gives_ownership moves its return valu into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its returned value into s3

    println!("value of s1: {s1}");
    println!("value of s3: {s3}");
    // println!("value of s2: {s2}"); // out of scope

} // Here, s3 goes out of scope and is dropped. s2 was moved, nothing happens, s1 goes out of scope and is dropped

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // some_integer goes out of scope, nothing special happens

fn gives_ownership() -> String {    // gives_ownership will move its
                                    // return value into the function
                                    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                        // moves out to the calling
                                        // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}