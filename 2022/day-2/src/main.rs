use std::fs;
use std::collections::HashMap;

fn main() {
    
    let  contents = fs::read_to_string("input1.txt")
                                .expect("input.txt not found."); 

    let choice_score = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let translator = HashMap::from([
        ("A","X"),
        ("B","Y"),
        ("C","Z"),
    ]);

    let winner = HashMap::from([
        ("X", "Y"),
        ("Y", "Z"),
        ("Z", "X"),
    ]);

    let mut score = 0;
    for line in contents.lines(){
        
        let play: Vec<&str> = line.split(" ").collect();

        let my_play:&str = play[1];
        let opponent_play:&str = match translator.get(play[0]){
            Some(n) => n,
            None => panic!()
        };

        match choice_score.get(my_play) {
            Some(n) => score += n,
            None => println!("Error on parsing my hand.")
        };

        let winning_hand:&str = match winner.get(opponent_play){
            Some(n) => n,
            None => panic!()
        };

        println!("{}vs{} (winning hand is {}).", my_play, opponent_play, winning_hand);

        if my_play == winning_hand {
            score += 6
        } else if my_play == opponent_play {
            score += 3
        }
    }

    println!("Final score: {}.", score);
}

