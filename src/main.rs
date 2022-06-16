use std::env;
use gleephs::input::*;

fn main() {
    let mut arguments = env::args();

    match arguments.len() {
        0 | 1 => {
            println!("No arguments were passed. Exiting.");
            return;
        },
        _ => {
            let mut hash = arguments.nth(1); // Skip the first argument

            while let Some(content) = hash {
                if is_hex(&content) {
                    let glyph_input = squash(&content);
                    
                    if let Ok(number) = glyph_input {
                        println!("Raw input: {:x}", number);
                        let mut grid: [[Node; 4]; 4] = Node::grid_from_u64(&number);

                        for node in 0..16 {
                            println!("The value for node ({},{}) is {}", (node / 4), (node % 4), &grid[(node / 4)][(node % 4)].value);
                        }
                    }
                }

                hash = arguments.next();
            }
        }
    }
}
