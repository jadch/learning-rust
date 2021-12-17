use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..101);

    println!("Hey {}", secret_num);
    println!("Try guessing the number!");
    println!("start by inputting your guess....");

    loop {
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please input a number !");
                continue;
            }
        };
        println!("You guessed {}", user_guess);

        match user_guess.cmp(&secret_num) {
            Ordering::Less => println!("secret number is higher than {}. Try again", user_guess),
            Ordering::Greater => println!("secret number is lower than {}. Try again", user_guess),
            Ordering::Equal => {
                println!("Woohoo");
                break;
            }
        }
    }
}
