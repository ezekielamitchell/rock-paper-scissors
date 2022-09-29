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
    let hands = ["rock", "paper", "scissors"];
    let hand = hands[rand::thread_rng().gen_range(0..3)];
    println!("current hand: {}", hand);

    // request user hand
    println!("Choose your hand [rock, paper, scissors]: ");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("invalid input");


    let choice = user_input.trim();

    if hands.contains(&choice) {
    } else {
        println!("error: invalid input");
        return;
    }

    if choice == hand {
        println!("tie game!");
        return;
    } else if (choice == "rock" && hand == "scissors") || (choice == "paper" && hand == "rock") || (choice == "scissors" && hand == "paper") {
        println!("you win!");
        return;
    }

    println!("you lose!");

}