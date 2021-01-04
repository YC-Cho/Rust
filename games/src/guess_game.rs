// Filename: guess_game.rs
// By: Younchul Cho
// Date: January / 2021
// Description:
//  	The guessing game program build by Rust
// 	    The system creat the number between 1 and 100 randomly.
//      The user have three chance to guessing the random number.

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;


pub fn game() {
    println!("Welcom to the guessing game!");
    let mut min = 1;
    let mut max = 100;
    let mut count = 1;
    const MAX_ATTEMPT:u32 = 3;

    // Create the secret number
    let secret_number = rand::thread_rng().gen_range(min, max+1);
    println!("Secret Number: {}", secret_number);
    
    // Looping until the user got random number
    // Or reached the maximum attempt time
    loop {
        // Get the guessing number from the user
        println!("\nPlease input your guess...between {} {}", min, max);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        println!("You guessed: {}", guess);

        // Change the data type from string to unsigned integer
        let guess:u32 = guess.trim().parse().expect("Type a number!");
        
        // Check the value is right range
        if (guess >= min) && (guess <= max)
        {
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!");
                    min = guess + 1;
                },
                Ordering::Greater => {
                    println!("Too big");
                    max = guess - 1;
                },
                Ordering::Equal => {
                    println!("You win!");
                    println!("Bye Bye!!\n");
                    break;
                },
            };

            // Check the attempt reached max number
            if count == MAX_ATTEMPT
            {
                println!("You loss!");
                println!("Bye Bye!!\n");
                break;
            }
            println!("It is your {}'s attempt!", count);
            println!("You have {} more time to get right number!", MAX_ATTEMPT - count);

            // Increase the counting number
            count += 1;
        }
        else if guess < min
        {
            println!("You guess too lower value. Try again!");
        }
        else if guess > max
        {
            println!("You guess too high value. Try again!");
        }
    }
}