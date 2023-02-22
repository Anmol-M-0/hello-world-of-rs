use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Please guess a number");
    let secret_num = gen_secret_number();
    let max_tries = 3;
    let mut try_number = 0;
    while try_number < max_tries {
        println!("try number {}", (try_number + 1));
        try_number += 1;
        let status = compare_guess(input_guess(), secret_num);
        match status {
            Ordering::Less => println!("The provided number is less than the answer."),
            Ordering::Equal => println!("The provided number is equal to the answer."),
            Ordering::Greater => println!("The provided number is greater than the answer"),
        }
        if status == Ordering::Equal {
            println!("You win");
            return;
        }
    }
    println!("You lose, thank you for playing our game!");
}

fn read_line() -> (String, usize) {
    let mut input: String = String::new();
    let buf: usize = io::stdin()
                .read_line(&mut input)
                .expect("falied to read the stdin");
    return (input, buf);
}

fn atoi(a: String) -> i32 {
    a.parse::<i32>()
        .expect("invalid input: please provide an integer string")
}

fn gen_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=10000)
}

fn input_guess() -> i32 {
    atoi(read_line().0.trim().to_string())
}

fn compare_guess(guess: i32, ans: i32) -> std::cmp::Ordering {
    guess.cmp(&ans)
}