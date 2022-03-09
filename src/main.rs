fn problem1() -> u32 {
    let three = 3;
    let five = 5;
    let one_thousand = 1000;
    let mut multiples = Vec::new();

    let mut multiplier = 1;
    let mut three_multiple = 3;
    let mut five_multiple = 5;

    while three_multiple < one_thousand || five_multiple < one_thousand {
        if three_multiple < one_thousand && !multiples.contains(&three_multiple) {
            multiples.push(three_multiple);
        }

        if five_multiple < one_thousand && !multiples.contains(&five_multiple) {
            multiples.push(five_multiple);
        }
        
        multiplier += 1;
        three_multiple = multiplier * three;
        five_multiple = multiplier * five;
    }

    multiples.iter().sum()
}

fn problem2() -> u32 {
    let max_fib = 4000000;
    let mut term_one = 1;
    let mut term_two = 2;
    let mut new_term;
    let mut even_fibs = vec![2];

    while term_two < max_fib {
        new_term = term_one + term_two;

        if new_term % 2 == 0 {
            even_fibs.push(new_term);
        }

        term_one = term_two;
        term_two = new_term;
    }

    even_fibs.iter().sum()
}

// fn is_prime(candidate: f64) -> bool {
//     let k_low = (candidate - 1.0) / 6.0;
//     let k_high = (candidate + 1.0) / 6.0;
//     let is_prime_high = possible_prime_high.fract() < f64::EPSILON;
//     let is_prime_low = possible_prime_low.fract() < f64::EPSILON;

//     (candidate - 2.0).abs() < f64::EPSILON || (candidate - 3.0).abs() < f64::EPSILON || is_prime_low || is_prime_high
// }

// fn is_whole(candidate: f64) -> bool {
//     candidate.fract() < f64::EPSILON;
// }

// fn is_factor(candidate:f64,number:f64) -> bool {
//     is_whole(number / candidate)
// }

// fn problem3() -> f64 {
//     let number: f64 = 600851475143.0;
//     let sqrt_number: f64 = f64::sqrt(number);

//     println!("SQRT: {}", sqrt_number);

//     let mut prime_factors = vec![];
//     let mut counter: u64 = 3;
//     let mut found_prime_factor = false;

//     // Find prime factors of number
//     // 
//     // Find list of primes with seive
//     //   Create array of numbers
//     //   iterate through array removing numbers
//     //   
//     // loop through integers to find each possible factor
//     // check each factor is prime
//     while counter <= sqrt_number as u64 && !found_prime_factor {
//         let small_factor = counter as f64;
//         let possible_factor: f64 = number / small_factor;
        
//         println!("factor: {}, possible factor: {}, is_factor: {}", small_factor, possible_factor, is_factor);
        
//         if is_whole(possible_factor) && is_prime(possible_factor) {    
//             prime_factors.push(possible_factor);
//             println!("possible prime factor: {}, i: {}", possible_factor, small_factor);
//             found_prime_factor = true;
//         }

//         counter += 2;
//     }

//     prime_factors.sort_by(|a, b| a.partial_cmp(b).unwrap());

//     return *prime_factors.last().unwrap();
// }

fn is_palindrome(integer: u32) -> bool {
    let mut first_string = integer.to_string();

    let second_string = first_string.split_off(3).chars().rev().collect::<String>();

    println!("first: {}, second: {}", first_string, second_string);

    first_string == second_string
}

fn problem4() -> u32 {
    // Iterate through all nukber to find palindrome
    
    let mut counter_one: u32 = 999;
    let mut counter_two: u32 = 999;
    let mut palindromes = vec![];

    while counter_one > 0 {
        while counter_two > 0 {
            println!("one: {}, two: {}", counter_one, counter_two);
            let number: u32 = counter_one * counter_two;
            
            if number > 99999 && is_palindrome(number) {
                palindromes.push(number);
            }

            counter_two -= 1;
        }
        counter_two = 999;
        counter_one -= 1;
    }

    palindromes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    return *palindromes.last().unwrap();
}

fn main() {
    let problem1_answer: u32 = problem1();
    println!("Problem One Answer: {}", problem1_answer);

    let problem2_answer: u32 = problem2();
    println!("Problem Two Answer: {}", problem2_answer);

    // let problem3_answer: f64 = problem3();
    // println!("Problem Three Answer: {}", problem3_answer);

    let problem4_answer: u32 = problem4();
    println!("Problem Four Answer: {}", problem4_answer);
}
