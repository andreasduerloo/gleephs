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

    pub fn fold(input: &str) -> Result<u64, &str> {
        let halves: [&str; 2] = [&input[0..(input.len()/2)], &input[(input.len()/2)..]];
    
        let halves_hex = (u64::from_str_radix(halves[0], 16), u64::from_str_radix(halves[1], 16));
    
        if let (Ok(a), Ok(b)) = halves_hex {
            return Ok(a^b);
        } else {
            return Err("Error parsing input.");
        }
    }

    pub fn squash(input: &u64) -> u16 {
        let mut output: u64 = 0x0;
        for i in 0..16 {
            output = output | ((input >> 4*i) & 0x1) << i;
        }
        return output as u16;
    }
}

pub mod glyphs {
    use crate::input::*;

    pub fn run(content: &str) {
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
    }

    fn count_neighbors(input: &u16) -> u16 {
        // Returns a bitmask of bits to be flipped because they have too many neighbors
        // This could theoretically end in a loop
        let bitmasks: [u16; 16] = [
            0x12,
            0x25,
            0x4A,
            0x84,
            0x121,
            0x252,
            0x4A4,
            0x848,
            0x1210,
            0x2520,
            0x4A40,
            0x8480,
            0x2100,
            0x5200,
            0xA400,
            0x4800
        ];
    
        let mut output: u16 = 0x0;
    
        for i in 0..16 {
            // Get the value for that bit
            let value = input >> i & 0x1;
    
            match value {
                0 => {
                    let neighbors: u16 = !input & bitmasks[i];
                    match count_bits(&neighbors) {
                        0 | 1 | 2 => {
                            continue
                        },
                        _ => {
                            output = output | 0x1 << i
                        }
                    }
                },
                _ => {
                    let neighbors: u16 = input & bitmasks[i];
                    match count_bits(&neighbors) {
                        0 | 1 | 2 => {
                            continue
                        },
                        _ => {
                            output = output | 0x1 << i
                        }
                    }
                }
            }
        }
        return output;
    }

    fn count_bits(input: &u16) -> u16 {
        let mut output: u16 = 0;
        for i in 0..16 {
            output += input >> i & 0x1;
        }
        return output;
    }

    pub fn flip_bits(input: &u16, mask: &u16) -> u16 {
        let mut output: u16 = 0x0;
        for i in 0..16 {
            if mask >> i == 1 {
                output = output | 0x1 << i & (!(input >> i & 0x1) << i);
            }
            else {
                output = output | input & (0x1 << i);
            }
        }
        return output;
    }

    fn draw_horizontal(input: &u16, offset: usize) {
        let mut output: String = "".to_string();
    
        for i in 0..3 {
            output.push_str("▓▓");
            if input & 0x1 << (offset - i) == (input & 0x1 << offset - (i + 1)) << 1 {
                output.push_str("▓▓");
            }
            else {
                output.push_str("  ");
            }
        }
        output.push_str("▓▓");
        println!("{}", output);
    }

    fn draw_vertical(input: &u16, offset: usize) {
        let mut output: String = "".to_string();

        for i in 0..4 {
            if input & 0x1 << (offset - i) == (input & ( 0x1 << ((offset - 4) - i))) << 4 {
                output.push_str("▓▓");
            }
            else {
                output.push_str("  ");
            }
            output.push_str("  ");
        }
        println!("{}", output);
    }

    fn draw_glyph(input: &u16) {
        draw_horizontal(input, 15);
        draw_vertical(input, 15);
        draw_horizontal(input, 11);
        draw_vertical(input, 11);
        draw_horizontal(input, 7);
        draw_vertical(input, 7);
        draw_horizontal(input, 3);
    }
}

// ░ ▒ ▓