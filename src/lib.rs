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
}