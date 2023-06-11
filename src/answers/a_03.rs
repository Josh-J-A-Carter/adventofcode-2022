use std::fs;

pub fn func(){
    let contents = fs::read_to_string("src/input/03.txt");

    
    let score = contents
        .unwrap()
        // for each line
        .lines()
        // apply closure
        .fold(0, |sum: u32, line| {
            // split in half
            let lo = 0;
            let mid = line.len() / 2;
            let hi = line.len();
            
            let compart_1 = &line[lo..mid];
            let compart_2 = &line[mid..hi];
                
            // for each char in first half,
            for char_1 in compart_1.chars() {
                // for each char in second half
                for char_2 in compart_2.chars() {
                        // matches ? return score plus the previous sum
                        if char_1 == char_2 {
                            let mut val = 1 + char_1.to_ascii_uppercase() as u32 - 'A' as u32;
                            if char_1.is_ascii_uppercase() { val += 26 }
                            return sum + val
                        }
                    }
                }

                panic!("Didn't find two matching items for this rucksack");
            });

    println!("Total score {score}")

}