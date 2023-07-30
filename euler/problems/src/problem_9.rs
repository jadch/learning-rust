// https://projecteuler.net/problem=9

/*
A Pythagorean triplet is a set of three natural numbers, a < b < c,
for which a^2 + b^2 = c^2.

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product a bc.

SOLUTION: Starting with a^2 + b^2 = c^2 we can write all pythagorean triplets in terms of 
arbitrary natural numbers m and n:

    a = 2mn
    b = m^2 - n^2
    c = m^2 + n^2  (m or n cannot be higher than 32, otherwise c alone > 1000)

ANSWER: 31875000
*/

use num::PrimInt;

pub fn special_pythagorean_triplet() -> i32 {
    for m in 1..=32 {
        for n in 1..=32 {
            let a = 2 * m * n;
            let b = m.pow(2) - n.pow(2);
            let c = m.pow(2) + n.pow(2);

            if a + b + c == 1000 {
                println!("a: {} - b: {} - c: {}", a, b, c);
                return a * b * c;
            }
        }
    }
    return 0;
}