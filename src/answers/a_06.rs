use std::fs;

pub fn func() {
    let contents = fs::read_to_string("src/input/06.txt").unwrap();

    // The length of the packet signal
    const PACKET_LEN: usize = 14;

    // Need to start from the 4th position
    let first_idx = PACKET_LEN;
    // Technically this is the number of bytes! But the input is ASCII so it'll work
    let last_idx = contents.len();

    for i in first_idx .. last_idx {
        let curr_chars: Vec<char> = contents[i - first_idx .. i].chars().collect();

        // println!("{:?}", curr_chars);
        if check_unique(curr_chars) {
            println!("Matching index at {i}");
            break;
        }
        
    }
}

fn check_unique(chars: Vec<char>) -> bool {
    for c in chars.iter() {
        // Count the number of times this character has occurred in the past 4 characters
        let occurences = chars.iter().fold(0, |sum, character| if c == character { sum + 1 } else { sum } );
    
        if occurences > 1 { return false }
    }

    return true;
    
}
