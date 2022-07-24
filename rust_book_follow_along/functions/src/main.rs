fn main() {
    println!("Hello, world!");
    another_function(function_that_returns_five(), 's');
}

fn another_function(x: i32, unit: char) {
    println!("The value is {}{}", x, unit);
}

// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function
fn function_that_returns_five() -> i32 {
    5
}
