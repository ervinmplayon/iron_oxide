fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}")
}

// The issue with the tuple code used in ../ownership/src/main/ calculate_length function is that 
// we have to return the String to the calling function so we can still use the String after the 
// call to calculcate_length. Instead, we can provide a reference to the String value. 
// A reference is like a pointer in that its an address we can follow to access the data stored at
// that address.
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of 
  // what it refers to, the String is not dropped. 