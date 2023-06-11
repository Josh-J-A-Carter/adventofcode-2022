use std::{fs, ops::RangeInclusive};

pub fn func() {
    let contents = fs::read_to_string("src/input/04.txt")
        .unwrap();

    let score = contents
        .lines()
        .fold(0, |sum, curr_pair| {

            // Split the pair up into two segments
            let mut segments = curr_pair.split(",");

            // Read the segments as ranges
            let range_1 = read_range(segments.next().unwrap());
            let range_2 = read_range(segments.next().unwrap());

            // Check if one containers the other
            if is_contained(&range_1, &range_2) { return sum + 1 }
            if is_contained(&range_2, &range_1) { return sum + 1 }

            // They don't contain each other, so don't increment the counter
            return sum
        });

    println!("Number of ranges including each other: {score}");

}

fn read_range(string: &str) -> RangeInclusive<u32> {
    let values: Vec<u32> = string
        .split("-")
        .map(|str| str.parse::<u32>().unwrap())
        .collect();

    let lo = 0 as usize;
    let hi = values.len() - 1 as usize;

    return values[lo]..=values[hi];
}

/**
 * Does r_1 contain r_2?
 */
fn is_contained(r_1: &RangeInclusive<u32>, r_2: &RangeInclusive<u32>) -> bool {
    if r_1.start() <= r_2.start()
        && r_1.end() >= r_2.end() {
            return true
    }
    
    false
}
