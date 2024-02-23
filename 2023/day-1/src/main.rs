use std::fs;
use regex::Regex;

fn main() {
    
    let  contents = fs::read_to_string("input.txt")
                                .expect("input.txt not found."); 

    let numbers:Vec<u16> = contents
        .lines()
        .map(|line| extractor(line))
        .collect();

    println!("Result is: {}", numbers.iter().sum::<u16>());
}


fn extractor(phrase: &str) -> u16 {

    let mut s = String::from("");

    // I'm having a hard time capturing cases where the end of a match
    // coincides with the begining of another, like in:
    // twoeightnfhdbndrtzltninehdtkheight4eightwolpr 
    // which ignored the last number (two), in favor of the "eight".

    let numbers:Vec<String> = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine")
                            .unwrap()
                            .find_iter(phrase)
                            .map(|m| m.as_str().to_lowercase())
                            .map(|num_str| match num_str.as_str() {
                                "one" => String::from("1"),
                                "two" => String::from("2"),
                                "three" => String::from("3"),
                                "four" => String::from("4"),
                                "five" => String::from("5"),
                                "six" => String::from("6"),
                                "seven" => String::from("7"),
                                "eight" => String::from("8"),
                                "nine" => String::from("9"),
                                _ => String::from(num_str)
                            })
                            .collect();
                        
    let first = numbers[0].chars().next()
                                .expect("Empty string.");

    let last = numbers.last().expect("No numbers found.")
                                    .chars().next().expect("Empty string.");

    s.push(first);
    s.push(last);

    let result = s.parse::<u16>().unwrap();
    println!("{} -> {}", phrase, result);
    result
}