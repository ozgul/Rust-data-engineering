/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue..
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;
use std::io;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    println!("Initial fruit salad: {:?}", fruit);

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    println!("Shuffled fruit salad: {:?}", fruit);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    let new_fruit_front: String = String::new();
    let mut new_fruit_front: String = new_fruit_front.trim().to_string();

    println!("Enter a fruit to be added to the front of the queue");
    io::stdin()
        .read_line(&mut new_fruit_front)
        .expect("Failed to read line");
    fruit.push_front(&mut new_fruit_front);

    let new_fruit_back: String = String::new();
    let mut new_fruit_back: String = new_fruit_back.trim().to_string();
    println!("Enter a fruit to be added to the end of the queue ");
    io::stdin()
        .read_line(&mut new_fruit_back)
        .expect("Failed to read line");
    fruit.push_back(&mut new_fruit_back);

    if let Some(back) = fruit.pop_back() {
        println!("Removed from the back: {}", back);
    }

    if let Some(front) = fruit.pop_front() {
        println!("Removed from the front: {}", front);
    }

    let fruit: Vec<_> = fruit.into_iter().collect();

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
