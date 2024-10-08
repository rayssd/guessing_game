use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn set_upper_bound() -> u32 {
    let upper_bound: u32 = loop {
        let mut upper_bound = String::new();
        println!("Put in the upper range");
        io::stdin()
            .read_line(&mut upper_bound)
            .expect("Failed to read input");

        match upper_bound.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };
    return upper_bound;
}

fn main() {
    println!("WELCOME TO THE GUESSING GAME!!\n");

    let mut upper_bound: u32 = set_upper_bound();
    let mut lower_bound: u32 = 1;

    println!("The upper bound is {}", upper_bound);

    let secret_number = rand::thread_rng().gen_range(1..=upper_bound);

    // println!("The secret number is {secret_number}");

    loop {
        // let mut out_of_bound: bool = false;
        let mut guess = String::new();

        // guess has to be declared inside the loop because the subsequent
        // shaowing changes the type
        println!("Guess the number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                lower_bound = if guess < lower_bound {
                    println!("Your guess is out of lower bound.");
                    lower_bound
                } else {
                    guess
                };
                println!("{lower_bound} to {}", &upper_bound);
            }
            Ordering::Greater => {
                upper_bound = if guess > upper_bound {
                    println!("Your guess is out of the upper bound.");
                    upper_bound
                } else {
                    guess
                };
                println!("{} to {upper_bound}", &lower_bound);
            }
            Ordering::Equal => {
                println!("You've got it!");
                break;
            }
        }
    }
}
