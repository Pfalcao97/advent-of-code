use std::fs;

fn main() {
    
    let  contents = fs::read_to_string("input.txt")
                                .expect("input.txt not found."); 

    let number_of_lines = contents.lines().count();
    println!("Number of lines: {}", number_of_lines);

    let mut counter:u8 = 0;
    for line in contents.lines() {

        let groups:Vec<&str> = line.split(',').collect();

        let first:Vec<u8> = groups[0]
                                .split('-')
                                .collect::<Vec<&str>>()
                                .iter()
                                .filter_map(|s| s.parse().ok())
                                .collect();

        let second:Vec<u8> = groups[1]
                                .split('-')
                                .collect::<Vec<&str>>()
                                .iter()
                                .filter_map(|s| s.parse().ok())
                                .collect();

        if first[1] < second[0] || first[0] > second[1] || second[1] < first[0] || second[0] > first[1] {
            // I've found it easier to subtract those that don't overlap than to think on edge cases that did.
            counter += 1;
        }
    };

    println!("Counter: {}", counter);
    println!("Groups that don't overlap: {}", number_of_lines - counter as usize);
}