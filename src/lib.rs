pub mod input {

    pub fn is_hex(input: &str) -> bool { // We expect a 128-bit hexadecimal number
        match input.len() {
            32 => {
                for character in input.chars() {
                    if let Some(_) = character.to_digit(16) {
                        continue;
                    } else {
                        return false;
                    }
                }
                true
            },
            _ => false
        }
    }

    pub fn squash(input: &str) -> Result<u64, &str> {
        let halves: [&str; 2] = [&input[0..(input.len()/2)], &input[(input.len()/2)..]];

        let halves_hex = (u64::from_str_radix(halves[0], 16), u64::from_str_radix(halves[1], 16));

        if let (Ok(a), Ok(b)) = halves_hex {
            return Ok(a^b);
        } else {
            return Err("Error parsing input.");
        }
    }

    #[derive(Copy, Clone)]
    pub struct Node {
        pub value: u64,
        pub snake: bool
    }

    impl Node {
        fn new(value: u64) -> Node {
            Node {
                value,
                snake: false
            }
        }
    }

    impl Node {
        pub fn grid_from_u64(input: &u64) -> [[Node; 4]; 4] {
            let mut out_grid: [[Node; 4]; 4] = [[Node::new(0); 4]; 4];
            for shift in 0..16 {
                out_grid[shift / 4][shift % 4] = Node::new((input & (0xf << shift*4)) >> shift*4);
            }
            out_grid
        }
    }
}

pub mod snakes {
    use crate::input::*;

    pub fn crawl(grid: &mut [[Node; 4]; 4], glyph: &mut [[char; 9]; 9], seed: &u64) -> u64 { // Returns how many unconnected nodes are left after the crawl
        let start = scan_grid(grid);
        // Find the next step: while let
    }

    fn scan_grid(grid: &[[Node; 4]; 4]) -> [u64; 2] { // Returns the coordinates of the next starting point
        let mut highest_coords = [0, 0];
        let mut highest_value = grid[0][0].value; // Start with a real value: it's possible the highest value is 0.

        for node in 0..16 {
            if grid[node / 4][node % 4].snake == false && grid[node / 4][node % 4].value > highest_value {
                highest_coords = [node as u64 / 4, node as u64 % 4];
                highest_value = grid[node / 4][node % 4].value;
            } else {
                continue
            }
        }
        highest_coords
    }

    fn sniff_neighbors(grid: &[[Node; 4]; 4], start: &[u64; 2]) -> Option<[u64; 2]> { // Returns the coordinates for the next step or nothing.
        // Return the co-ordinates for the next node, if any.
        // Rules: the next node is the immediately adjoining (up, right, down, left) node with an equal value, or
        // the immedaitely adjoining node with the closest lower value.
        // In case of a tie, the following order is used: up > right > down > left.
    }
}

// ░ ▒ ▓