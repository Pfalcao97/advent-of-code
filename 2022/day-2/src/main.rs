use std::fs;
use std::collections::HashMap;

// A for Rock
// B for Paper
// C for Scissor

fn main() {
    
    let  contents = fs::read_to_string("input.txt")
                                .expect("input.txt not found."); 

    let choice_score = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
    ]);


    let possible_plays = HashMap::from([
        ("A", ["B", "C"]),
        ("B", ["C", "A"]),
        ("C", ["A", "B"]),
    ]);

    let mut score = 0;
    for line in contents.lines(){
        
        let play: Vec<&str> = line.split(" ").collect();
        let opponent_play:&str = play[0];

        let winning_hand:&str = match possible_plays.get(opponent_play){
            Some(n) => n[0],
            None => panic!()
        };

        let loosing_hand:&str = match possible_plays.get(opponent_play){
            Some(n) => n[1],
            None => panic!()
        };

        let my_play:&str;
        match play[1] {
            "Z" => {
                my_play = winning_hand;
                score += 6;
            },
            "Y" => {
                my_play = opponent_play;
                score += 3;
            },
            "X" => {
                my_play = loosing_hand;
            },
            _ => panic!()
        };

        match choice_score.get(my_play) {
            Some(n) => score += n,
            None => println!("Error on parsing my hand.")
        };
    }

    println!("Final score: {}.", score);
}

