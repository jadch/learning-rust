mod problem_5;
mod problem_10;

fn main() {
    println!("computing solution...");

    let solution = problem_10::summation_of_primes(2000000);
    println!("solution: {}", solution);
}
