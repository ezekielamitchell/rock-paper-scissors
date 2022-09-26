#[allow(unused_variables)]
#[allow(unused_imports)]

use std::io;
use rand::Rng;

fn main() {
    greeting();
    game();
}

fn greeting() {
    println!("--- Rock-Paper-Scissors ---");
}

fn game() {

    // generate computer hand
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..3);

    let hands = vec!["rock", "paper", "scissors"];
    let hand = hands[random_number];
    println!("current hand: {}", hand);

    // request user hand
    println!("Enter your hand:");

    for h in &hands {
        println!("{}", h);
    }

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("invalid input");


    let choice: &str = user_input.trim();

    if hands.contains(&choice) {
        if choice == hand {
            println!("its a tie!");
        }
    } else {
        println!("invalid input!");
        return;
    }





}