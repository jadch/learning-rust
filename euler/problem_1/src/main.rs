// https://projecteuler.net/problem=1

/*
    <p>If we list all the natural numbers below $10$ that are multiples of $3$ or $5$, we get $3, 5, 6$ and $9$. The sum of these multiples is $23$.</p>
    <p>Find the sum of all the multiples of $3$ or $5$ below $1000$.</p>
*/

// Correct answer = 233'168

fn main() {
    let sum = compute_sum_of_multiples_below_max(3, 1000)
        + compute_sum_of_multiples_below_max(5, 1000)
        - compute_sum_of_multiples_below_max(3 * 5, 1000);
    println!("total sum is {}", sum);
}

// Based on summing an Arithmetic Progression https://en.wikipedia.org/wiki/Arithmetic_progression
fn compute_sum_of_multiples_below_max(multiple: i32, max: i32) -> i32 {
    let max_iterations: i32 = (max - 1) / multiple;
    println!("max_iterations for {} is {}", multiple, max_iterations);

    let sum_of_arithmetic_progression =
        (max_iterations * (multiple + max_iterations * multiple)) / 2;
    println!(
        "sum_of_arithmetic_progression for {} is {}",
        multiple, sum_of_arithmetic_progression
    );

    sum_of_arithmetic_progression
}
