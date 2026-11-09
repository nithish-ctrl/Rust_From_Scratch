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

    guess_random_number();
    
}


fn guess_random_number(){
    println!("Lets play a Guessing Game!");

    let mut number_guesses = String::new();

    println!("How many guesses would you like to have?");
    io::stdin()
        .read_line(&mut number_guesses)
        .expect("Error reading input.");

    let max_num: u8 = number_guesses.trim().parse().expect("Please enter a valid number");


    let mut numbers_list : Vec<u32>= Vec::new();
    for _ in 0..max_num {
        let mut guessed = String::new();
        println!("Please input your guess : ");
        io::stdin()
            .read_line(&mut guessed)
            .expect("Failed to read line");
        numbers_list.push(guessed.trim().parse().expect("Enter a valid guess"));
    }
    println!("The numbers guessed are : {:?}", numbers_list);
}   