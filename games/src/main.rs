// Filename: main.rs
// By: Younchul Cho
// Date: January / 2021
// Description:
//  	The game program build by Rust
// 	    It shows the list of game, and the user will select on of these.

use std::io;

// Import the game files
mod guess_game;
mod hangman_game;

// Start point
fn main() {
    loop {
        let select = menu();
        
        match select {
            0 => {
                println!("Good Bye!!\n");
                break;
            }
            1 => guess_game::game(),
            2 => hangman_game::game(),
            _ => println!("The game is not ready yet!!"),
        }
    }
}


// Show the menu, and the user can select one of that
fn menu() -> u32 {
    let mut select = String::new();

    println!("\nWelcom to the Game World!!");
    println!("0. Exit the Game World.");
    println!("1. Guess game");
    println!("2. Hangman game");
    println!("Select the Game =>");
    
    io::stdin().read_line(&mut select).expect("Failed to read line.");
    let select:u32 = select.trim().parse().expect("Type a number!");

    return select;
}