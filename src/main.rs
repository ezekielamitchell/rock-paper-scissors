// use std::thread::sleep;

#[allow(unused_variables)]

fn main() {
    greeting();
}

fn greeting() {
    println!("--- Rock-Paper-Scissors ---");
    game();
}

fn game() {
    let hands = vec!["rock", "paper", "scissors"];
    for hand in hands {
        println!("{}", hand);
    }
}
