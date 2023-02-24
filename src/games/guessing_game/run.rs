use rand::Rng;
use std::{cmp::Ordering, io};

pub fn run(max_tries: i32, is_debug: bool) {
    println!("Please guess a number");
    let secret_num = gen_secret_number();
    debug_msgs(secret_num, is_debug);
    let mut try_number = 0;
    while try_number < max_tries {
        println!("try number {}", (try_number + 1));
        try_number += 1;
        let status = compare_guess(input_guess(), secret_num);
        describe_status(status);
        if is_game_won(status) {
            println!("{}", game_won_message());
            return;
        }
    }
    println!("{}", game_lost_message());
}

fn debug_msgs(secret_num: i32, is_debug: bool) {
    if is_debug {
        println!("{}", secret_num)
    };
}

fn describe_status(status: Ordering) {
    match status {
        Ordering::Less => println!("The provided number is less than the answer."),
        Ordering::Equal => println!("The provided number is equal to the answer."),
        Ordering::Greater => println!("The provided number is greater than the answer"),
    }
}

fn is_game_won(status: Ordering) -> bool {
    match status {
        Ordering::Less => false,
        Ordering::Equal => true,
        Ordering::Greater => false,
    }
}

fn game_won_message() -> &'static str {
    "You win"
}

fn game_lost_message() -> &'static str {
    "You lose, thank you for playing our game!"
}

fn read_line() -> String {
    let mut input: String = String::new();
    let res = io::stdin().read_line(&mut input);
    match res {
        Err(_) => "0".to_string(),
        Ok(_) => input,
    }
}

fn atoi(a: String) -> i32 {
    match a.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            let random_num = gen_secret_number();
            println!("Invalid number, defaulting to a random number : {random_num}.");
            random_num
        }
    }
}

fn gen_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=10000)
}

fn input_guess() -> i32 {
    atoi(read_line().trim().to_string())
}

fn compare_guess(guess: i32, ans: i32) -> std::cmp::Ordering {
    guess.cmp(&ans)
}
