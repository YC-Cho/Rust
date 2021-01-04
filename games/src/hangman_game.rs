extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io;
// use std::iter::FromIterator;


fn read_words() -> String{
    // Get the words from the file
    let mut file = File::open("words.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    // Remove the double qutation(") and split the words by the comma(,)
    contents = contents.replace('"', "");
    let split = contents.split(',');
    
    // Get the random numbers
    let random_number:u32 = rand::thread_rng().gen_range(0, 2643);
    
    // Get the answer word
    let mut answer = " ";
    let mut count = 0;
    // let mut collect = Vec::new();   

    for s in split
    {
        count += 1;
        if random_number == count
        {   
            answer = s;
            // collect.push(s.to_string().to_uppercase());
            break;
        }
    }

    return answer.to_string().to_uppercase();
    // return collect;
}


fn guess_ready(len:usize) -> String {

    // Ready to get the answer
    let mut count = 0;
    // let mut guess:Vec<String> = Vec::new();
    let mut guess:String = String::from("");
    loop {
        // guess.push('-'.to_string());
        guess += &'-'.to_string();
        count += 1;
        if len == count {
            return guess;
        }
    }
}


fn start_game(answer:String) -> bool {

    const MAX_COUNT:u32 = 10;
    let guessing = guess_ready(answer.capacity());
    let mut count = 0;
    let mut input = String::new();
    let mut collect = Vec::new();
    println!("Answer is {:?} - len {}", answer, answer.len());
    println!("{}", guessing);
    //// Temp   //////
    // let s = String::from_iter(guessing);
    // println!("{}", s);
    //// Temp   //////

    loop {
        input.clear();
        println!("Let's guess:");
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        
        if !(collect.is_empty()) {
            print!("Collect = {:?}", collect);
        }

        // Change the data type from string to unsigned integer
        let mut guess:String = input.trim().parse().expect("Unknown Type!!");
        guess = guess.to_string().to_uppercase();

        count += 1;
        if count == MAX_COUNT {
            println!("The answer is {:?}", answer);
            return false;
        }

        println!("Answer = {:?}, guess = {}", answer, guess);

        if answer.contains(&guess) {
            println!("Mathch!");
            collect.push(guess);
        }
        else {
            println!("Not Match!");
        }
    }
}


pub fn game() {

    // Get the answer word
    let answer = read_words();
    
    let win:bool = start_game(answer);

    match win {
        true => println!("You win!!\n"),
        false => println!("You loss!!\n"),
    }
}