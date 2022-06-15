use std::env;

fn main() {
    let mut arguments = env::args(); // Type is env::Args

    match arguments.len() {
        0 | 1 => {
            println!("No arguments were passed. Exiting.");
            return;
        },
        _ => {
            // Skip the first argument
            let mut hash = arguments.next(); // Type is Option(String)

            while let Some(content) = hash {
                println!("Argument: {}", content);
                hash = arguments.next();
            }
        }
    }
}
