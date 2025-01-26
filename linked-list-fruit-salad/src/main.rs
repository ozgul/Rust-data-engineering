/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list..
From Coursera course https://www.coursera.org/learn/data-engineering-rust
*/

use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;
use std::io;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    let mut new_fruit_pos: String = String::new();
    // Print out the fruit salad index and items
    for (i, item) in fruit.iter().enumerate() {
        println!("Index {}: {}", i, item);
    }
    println!("Enter the position where the new fruit should be added:");
    let mut position_input = String::new();
    io::stdin()
        .read_line(&mut position_input)
        .expect("Failed to read line");
    let position: usize = position_input
        .trim()
        .parse()
        .expect("Please enter a valid number");
    println!("Enter the  fruit to be added to the linked list");
    io::stdin()
        .read_line(&mut new_fruit_pos)
        .expect("Failed to read line");

    fruit.insert(position, &mut new_fruit_pos);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    let mut remove_fruit: String = String::new();
    println!("Enter the  fruit to be removed from the linked list");
    io::stdin()
        .read_line(&mut remove_fruit)
        .expect("Failed to read line");
    // Check that remove_fruit exists in the linked list
    if fruit.contains(&remove_fruit.trim()) {
        // Find the index of the fruit to be removed
        let index = fruit
            .iter()
            .position(|&x| x == remove_fruit.trim())
            .unwrap();
        // Split the linked list at the index of the fruit to be removed
        let mut split_list = fruit.split_off(index);
        // Remove the fruit from the linked list
        split_list.pop_front();
        // Append the split list to the original linked list
        fruit.append(&mut split_list);
    } else {
        println!("The fruit does not exist in the linked list");
    }
    // choose a random fruit from the fruit salad
    let random_fruit = fruit.iter().choose(&mut rng);
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
