fn main() {
    let number = 19;

    if number < 10 {
        println!("number is less than 10");
    } else if number == 10 {
        println!("number is equal to 10");
    } else {
        println!("number is bigger than 10");
    }

    // using if in a let statement
    let other_value = if number < 10 { 0 } else { 100 };
    println!("other_value has value {}", other_value);
}
