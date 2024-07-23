use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    //variables
    // let first = "jorge lucas";
    // let last = "dark";
    // let data = [1,2,3,4,5];

    // println!(" Hello, {first} {}", last.to_uppercase());
    // println!("{data:?}" );

    //Processing a Guess
    // println!("Hey, wha´s your name?");
    // let mut name = String::new();
    // io::stdin().read_line(&mut name).expect("Failed to read line");
    // println!("Hello {}, Welcome!", name.trim())

    // let correct = rand::thread_rng().gen_range(1..=10);
    // // let mut guess = String::new();

    // println!("correct: {correct}");
    // println!("Hey, guess a number 1-10");

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Error reading input");

    // let guess: u32 = guess.trim().parse().expect("Error with parse");
    // let mut message = if correct < guess {
    //     String::from("You guessed too high.")
    // } else if correct > guess {
    //     String::from("You guessed too low")
    // } else {
    //     String::from("you guessed CORRENT")
    // };

    // println!("{message}");

    //=================MATCH (like when in KOTLIN)====================

    // let message = match guess.cmp(&correct) {
    //     Ordering::Greater => "You guessed too high.",
    //     Ordering::Less => "You guessed too low",
    //     Ordering::Equal => "you guessed CORRENT",
    // };
    // println!("{message}");

    //=====================LOOP=========================
    // let correct = rand::thread_rng().gen_range(1..=10);

    // println!("correct: {correct}");
    // println!("Hey, guess a number 1-10");

    // loop {
    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Error reading input");

    //     // Error handling
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(e) => {
    //             println!("Error with parse, try again, {e}");
    //             continue;
    //         }
    //     };

    //     match guess.cmp(&correct) {
    //         Ordering::Greater => println!("You guessed too high."),
    //         Ordering::Less => println!("You guessed too low"),
    //         Ordering::Equal => {
    //             println!("you guessed CORRENT");
    //             break;
    //         }
    //     };
    // }

    // -===================VECTORS=====================

    let mut how_many = String::new();

    println!("Hom many random numbers do you want to guess?");

    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    let num_guesses: u8 = how_many.trim().parse().expect("Error reading input");

    let mut correct = Vec::new();

    for _i in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    println!("{correct:?}");
    println!("Hey, guess a number 1-10");

    let mut guesses_made = 0;

    while guesses_made < num_guesses {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        // Error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again, {e}");
                continue;
            }
        };

        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high."),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("you guessed CORRENT");
                guesses_made += 1;

                if guesses_made < num_guesses{
                    println!("Let´s now try the next number");
                }
            }
        };
    }


    println!("Thanks for playing! The corrent answers were:");
    for item in correct{
        println!("{item}")
    }

}
