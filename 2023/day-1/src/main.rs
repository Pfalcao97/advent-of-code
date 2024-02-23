use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    
    let  contents = fs::read_to_string("fake-input.txt")
                                .expect("input.txt not found."); 

    let numbers:Vec<u16> = contents
        .lines()
        .map(|line| extractor(line))
        .collect();

    println!("Result is: {}", numbers.iter().sum::<u16>());
}


fn extractor(phrase: &str) -> u16 {

    let mut s = String::from("");

    for n in Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")
                            .unwrap()
                            .find_iter(phrase)
                            .map(|m| m.as_str().to_lowercase())
                            .map(|num_str| num_str) {
     println!("{}",n);
                            }

    let numbers: Vec<&str> = Vec::new();
    //let numbers:Vec<&str> = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")
    //                        .unwrap()
    //                        .find_iter(phrase)
    //                        .map(|m| m.as_str().to_lowercase())
    //                        .map(|num_str| text_2_number.get(&num_str).expect("Not a number.").as_str())
    //                        .collect();
                        
    let first = numbers[0].chars().next()
                                .expect("Empty string.");

    println!("{}", first);
    let last = numbers.last().expect("No numbers found.")
                                    .chars().next().expect("Empty string.");

    s.push(first);
    s.push(last);

    let result = s.parse::<u16>().unwrap();
    println!("{} -> {}", phrase, result);
    result
}
