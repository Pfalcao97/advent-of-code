use std::fs;

fn main() {
    
    let contents = fs::read_to_string("input.txt")
                                .expect("input.txt not found."); 

    let packet_size:usize = 14;

    let mut chars_vectors:Vec<char> = contents.chars().take(packet_size).collect();

    for c in contents.chars().skip(packet_size).enumerate() {

        let mut auxiliary_vector = chars_vectors.clone();

        auxiliary_vector.sort();
        auxiliary_vector.dedup();

        if auxiliary_vector.len() < packet_size {
            chars_vectors.remove(0);
            chars_vectors.push(c.1);
        } else {
            println!("Found in position: {}", c.0 + packet_size);
            break
        };
    };

    println!("{:?}", chars_vectors);

}