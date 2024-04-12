use std::fs;

fn main() {
    
    let  contents = fs::read_to_string("input.txt")
                                .expect("input.txt not found."); 


    let mut counter:i16 = 0;
    for line in contents.lines() {

        let groups:Vec<&str> = line.split(',').collect();

        let first:Vec<i16> = groups[0]
                                .split('-')
                                .collect::<Vec<&str>>()
                                .iter()
                                .filter_map(|s| s.parse().ok())
                                .collect();

        let second:Vec<i16> = groups[1]
                                .split('-')
                                .collect::<Vec<&str>>()
                                .iter()
                                .filter_map(|s| s.parse().ok())
                                .collect();



        if first[0] >= second[0] && first[1] <= second[1] {     
            counter += 1;              
        };


        if first[0] < second[0] && first[1] >= second[1] {
            counter += 1;
        } else if first[0] == second[0]  && first[1] > second[1] {
            counter += 1;
        };
    };

    println!("Counter: {}", counter);
}