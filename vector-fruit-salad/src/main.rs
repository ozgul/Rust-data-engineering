/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
From Coursera course https://www.coursera.org/learn/data-engineering-rust
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::io;

fn main() {
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    let new_fruit = String::new();
    let mut new_fruit = new_fruit.trim().to_string();
    // Ask the user to enter a fruit
    println!("Enter a fruit:");
    io::stdin()
        .read_line(&mut new_fruit)
        .expect("Failed to read line");
    fruit.push(&new_fruit);
    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // choose a random fruit from the fruit salad
    let random_fruit: Option<&&str> = fruit.choose(&mut rng);
    println!("Random fruit: {}", random_fruit.unwrap());

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
