use std::io;
use rand::Rng;

fn main() {
    // Greeting message
    println!("Welcome to the Dice Roller!");

    // Prompt user for the number of sides on the dice
    println!("How many sides should the dice have?");
    let mut sides = String::new();
    io::stdin().read_line(&mut sides).expect("Failed to read line");
    let sides: u32 = match sides.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Prompt user for the number of rolls
    println!("How many times would you like to roll the dice?");
    let mut rolls = String::new();
    io::stdin().read_line(&mut rolls).expect("Failed to read line");
    let rolls: u32 = match rolls.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Roll the dice!
    let mut rng = rand::thread_rng();
    for i in 1..=rolls {
        let outcome = rng.gen_range(1..=sides);
        println!("Roll {}: {}", i, outcome);
    }
}
