use std::fs;
use std::collections::HashMap;

fn create_stacks(stack_vector:&mut Vec<(usize, char)>) -> HashMap<usize, Vec<char>> {

    let qty_columns = stack_vector.last().unwrap().0;
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    for _i in 0..qty_columns {
        stack_vector.pop();
    };

    let clean_stacks:Vec<&(usize, char)> = stack_vector.iter().filter(|x| x.1 != ' ').collect();

    for letter_box in clean_stacks.iter().rev() {

        if !stacks.contains_key(&letter_box.0) {
            stacks.insert(letter_box.0, vec![letter_box.1]);
        } else {
            stacks.get_mut(&letter_box.0).unwrap().append(&mut vec![letter_box.1]);
        };
    };

    stacks
}

fn main() {
    
    let  contents = fs::read_to_string("input.txt")
                                .expect("input.txt not found."); 

    let parts:Vec<&str> = contents.split("\n\n").collect();

    let mut stacks_aux = Vec::new();
    for line in parts[0].lines() {
        for c in line.chars().enumerate() {
            if (c.0 + 3) % 4 == 0 {
                stacks_aux.push(((c.0 + 3)/4, c.1));
            };
        };
    };

    let mut stacks = create_stacks(&mut stacks_aux);

    for line in parts[1].lines() {

        let parsed_moves:Vec<&str> = line.split(" from ").collect();

        let qty:usize = parsed_moves[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let from:usize = parsed_moves[1].split(" to ").collect::<Vec<&str>>()[0].parse().unwrap();
        let to:usize = parsed_moves[1].split(" to ").collect::<Vec<&str>>()[1].parse().unwrap();

        let mut start = stacks.get(&from).unwrap().clone();

        let mut aux = Vec::new();
        if start.len() <= (qty - 1) {
            aux = start.clone();
        } else {
            aux = start.split_off(start.len() - qty);
        }

        stacks.insert(from, start.to_vec());

        let mut finish = stacks.get(&to).unwrap().clone();
        finish.append(&mut aux);

        stacks.insert(to, finish.to_vec());
         
    }

    for k in stacks.keys() {
        println!("Last item from {}: {}", k, stacks.get(k).unwrap().last().unwrap());
    }
}