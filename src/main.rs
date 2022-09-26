#[allow(unused_variables)]
#[allow(unused_imports)]

// use std::io;
// use std::io::prelude::*;
// use std::fs::File;
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
    println!("Choose your hand:");

    let mut number = 1;
    for h in hands {
        println!("{} : {}", number, h);
        number += 1;
    }


}