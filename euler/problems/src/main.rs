mod problem_3;

fn main() {
    println!("computing solution...");

    let solution = problem_3::largest_prime_factor(600851475143, 1);
    println!("solution: {}", solution);
}
