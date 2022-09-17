use std::env;
use gleephs::input::*;
use gleephs::glyphs::*;

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
                    let glyph_input = fold(&content);
                    
                    if let Ok(number) = glyph_input {
                        let mut squashed_num = squash(&number);
                        let mut neighbor_mask = count_neighbors(&squashed_num);
                        while neighbor_mask != 0 {
                            squashed_num = flip_bits(&squashed_num, &neighbor_mask);
                            neighbor_mask = count_neighbors(&squashed_num);
                        }
                        draw_glyph(&squashed_num);
                    }
                }
                hash = arguments.next();
            }            
        }
    }
}
