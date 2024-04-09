use std::fs;

fn main() {
    
    let  contents = fs::read_to_string("input.txt")
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
    food_sum.reverse();

    // Solution for first part:
    match food_sum.first() {
        Some(n) => println!("Most calories: {}", n),
        None => println!("No value found.")
    };

    println!("Top three elves: {:?}", &food_sum[..3]);
    let top_three = &food_sum[..3];
    let mut total_calories = 0;

    for t in top_three.iter(){
        total_calories = total_calories + t;
    }

    println!("Sum of calories for top three: {}", total_calories);
}

