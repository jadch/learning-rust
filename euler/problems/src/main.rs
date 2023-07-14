mod problem_5;
mod problem_7;

fn main() {
    println!("computing solution...");

    let solution = problem_7::nth_prime(10001);
    println!("solution: {}", solution);
}
