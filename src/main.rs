use std::env;
use gleephs;

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
                if gleephs::input::is_hex(&content) {
                    let glyph_input = gleephs::input::squash(&content);
                    
                    if let Ok(number) = glyph_input {
                        println!("Raw input: {:x}", number);
                    }
                }

                hash = arguments.next();
            }
        }
    }
}
