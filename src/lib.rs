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
        pub snake: bool,
        pub checked: bool
    }

    impl Node {
        fn new(value: u64) -> Node {
            Node {
                value,
                snake: false,
                checked: false
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