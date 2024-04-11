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
    let mut racksack_repeat:String = "".to_string();
    for line in contents.lines().enumerate() {
        
        if line.0%3 == 0{
            racksack_repeat = line.1.to_string();
            continue; //skip this iteration
        }

        let mut char_tracker:Vec<char> = Vec::new();
        for c in line.1.chars() {
            if racksack_repeat.contains(c) {
                char_tracker.push(c);
            }
        }

        char_tracker.sort();
        char_tracker.dedup();

        racksack_repeat = String::from_iter(char_tracker);

        if (line.0 + 1)%3 == 0{
            let mut equivalency = racksack_repeat.chars();
            priority_sum += alphabet.get(&equivalency.next().unwrap()).unwrap();
            continue; //skip this iteration
        }
        ;
    };
    println!("Sum of all priorities: {}.", priority_sum);

}