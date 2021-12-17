use std::io;

fn main() {
    println!("Try guessing the number!");
    println!("start by inputting your guess....");

    let mut user_guess = String::new();

    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

    println!("You guessed {}", user_guess);
}
