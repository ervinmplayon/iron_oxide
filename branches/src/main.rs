fn main() {
    let num = 3;

    if num < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    im_a_fkin_genius();
}

fn im_a_fkin_genius() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
