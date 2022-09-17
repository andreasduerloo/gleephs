use std::env;
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
                run(&content);
                hash = arguments.next();
            }            
        }
    }
}
