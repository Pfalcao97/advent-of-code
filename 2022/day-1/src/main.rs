use std::fs;

fn main() {
    
    let  contents = fs::read_to_string("input1.txt")
                                .expect("input.txt not found."); 

    let mut aux = 0;
    let mut food_sum = Vec::new();
    for line in contents.lines(){
        //println!("{}", line);
        if line.is_empty(){
            //println!("{}", aux);
            food_sum.push(aux);
            aux = 0;
        } else {
            match line.parse::<i32>() {
                Ok(n) => aux = aux + n,
                Err(e) => println!("Couldn't parse: {}", e)
            };
        }
    }

    // After EOF, there's no line break, so all food
    // must be summed to last elf.
    food_sum.push(aux);
    food_sum.sort();

    match food_sum.last() {
        Some(n) => println!("Most food: {}", n),
        None => println!("No value found.")
    };
    
}

