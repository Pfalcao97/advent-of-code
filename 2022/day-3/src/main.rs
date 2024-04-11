use std::fs;
use std::collections::HashMap;

fn main() {
    
    let  contents = fs::read_to_string("input.txt")
                                .expect("input.txt not found."); 

    let alphabet_aux = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().enumerate();
    
    let mut alphabet = HashMap::new();
    for letter in alphabet_aux {
        alphabet.insert(
            letter.1, 
            letter.0 + 1 // The priority starts at 1, but enumeration starts at 0.
        );
    };
    

    let mut priority_sum = 0;
    for line in contents.lines() {

        let size_of_line = line.chars().count();
        let (first, second) = line.split_at(size_of_line/2);
        
        let mut aux:Vec<char> = Vec::new();
        for c in first.chars() {
            if second.contains(c) {
                aux.push(c);
            };
        };

        aux.sort();   // the vector is only really duplicated using
        aux.dedup();  // .dedup() if its sorted first.

        for c in aux.into_iter() {
            priority_sum += alphabet.get(&c).unwrap();
        }
    };

    println!("Sum of all priorities: {}.", priority_sum);
}