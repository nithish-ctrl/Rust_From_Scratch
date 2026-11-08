use std::io;
use std::cmp::Ordering;

fn main() {
    let correct_number = rand::random_range(1..=10);
    println!("The correct number is : {}", correct_number);
    println!("Lets play a guessing game!");

    loop {
    println!("Please input your guess : ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed : {}", guess);

    // let guess: u32 = guess.trim().parse().expect("Please enter a number"); This would work but it would panic if the user enters a non-number. So we will use match to handle that case.
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(e) => {
            println!("Error with parsing the number, try again : {}.", e);
            continue;
        }
    };

    /* 
    let message = if guess < correct_number {
        String::from("You guessed too low.")
    } else if guess > correct_number {
        String::from("You guessed too high.")
    } else { 
        String::from("You guessed correctly!")
    };
    */

    let message = match guess.cmp(&correct_number){
        Ordering::Less => String::from("You guessed too Low."),
        Ordering::Greater => String::from("You guesses too high."),
        Ordering::Equal => {
            println!("You guessed correctly!");
            break;
        }
    };
    println!("{message}");
    }
}
