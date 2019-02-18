use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num;

fn main() {
    println!("Guess the number!");

    let secret_number = seed_solution();
    let mut num_guesses = 0;

    loop {
        num_guesses += 1;

        let guess = next_guess();
        println!("You guess: {}", guess);

        let guess: i32 = match parse_answer(guess) {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Score: {}", num_guesses);
                break;
            }
        }
    }
}

fn seed_solution() -> i32 {
    rand::thread_rng().gen_range(1, 101)
}

fn parse_answer(input: String) -> Result<i32, num::ParseIntError> {
    input.trim().parse()
}

fn next_guess() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}
