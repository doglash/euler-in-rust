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

    let sum_of_multiples = multiples.iter().sum();
    
    return sum_of_multiples;
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

    let sum_of_fibs = even_fibs.iter().sum();

    return sum_of_fibs;
}

fn problem3() -> u32 {
    let number = 600851475143;
    let max_possible_factor = number / 2;

    // is it a factor
    // possible_factor % 1 == 0
    // 
    // is it prime
    // 
    return "not a number";
}

fn main() {
    let problem1_answer: u32 = problem1();
    println!("Problem One Answer: {}", problem1_answer);

    let problem2_answer: u32 = problem2();
    println!("Problem Two Answer: {}", problem2_answer);

    let problem3_answer: u32 = problem3();
    println!("Problem Three Answer: {}", problem3_answer);
}
