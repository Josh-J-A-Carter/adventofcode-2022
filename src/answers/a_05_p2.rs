use std::fs;
use std::collections::HashMap;

pub fn func() {

    // Load in the containers and store as a hash map
    let _raw_stacks = fs::read_to_string("src/input/05_stacks.txt").unwrap();

    let mut containers = get_containers(&_raw_stacks);

    display(&containers);

    // Load in the raw operations
    let operations = fs::read_to_string("src/input/05.txt").unwrap();

    for line in operations.lines() {
        // Format the line, removing
        let mut _input = line
            .replace("move", "")
            .replace("from", "")
            .replace("to", "");
        
        let mut args = Vec::new();

        for arg in _input.split(' ') {
            // We don't want whitespaces as arguments
            let formatted = arg.trim();
            if formatted.is_empty() { continue }
            args.push(formatted.parse::<u32>().unwrap());
        }

        // The number of times to repeat the operation
        let mut to_move: Vec<String> = Vec::new();
        // Pop the correct number from args[1] onto the vector
        for _ in 0..args[0] { to_move.push(containers.get_mut(&args[1]).unwrap().pop().unwrap()) }
        for _ in 0..args[0] { containers.get_mut(&args[2]).unwrap().push(to_move.pop().unwrap()) }
    }

    // Print the new state of the container
    display(&containers);

    print_pattern(&containers);

}

/**
 * Get all the containers loaded onto a hash map.
 * Keys are integers
 * Entries are vectors of strings
 */
fn get_containers(raw_stacks: &str) -> HashMap<u32, Vec<String>> {
    //Get all the keys into a vector of integers
    let keys: Vec<u32> = raw_stacks
        .lines()
        .last()
        .unwrap()
        .split("   ") // there are three white spaces between each key
        .map(|str| {
            let val = str.trim(); // trim any remaining white space
            val.parse::<u32>().unwrap()
        })
        .collect();

    // Each container is 4 characters along from the previous one
    const CONTAINER_LEN: usize = 4;

    // Containers
    let mut containers = HashMap::new();

    // Take each column / key (1-9)
    for key in keys.iter() {

        // Stack to hold the values
        // Containers will initially be in reverse order!
        let mut stack: Vec<String> = Vec::new();

        // Read through each line
        for line in raw_stacks.lines() {
            // The lowest character we want to look at for this iteration
            // Assuming the keys start at 0.
            let lo = (key - 1) as usize * CONTAINER_LEN;
            // The highest character we want to look at for this iteration
            // Remove 1 so we don't go out of bounds
            let hi = key.to_owned() as usize * CONTAINER_LEN - 1;

            // Read the container for this iteration, and trim whitespace
            let curr_container = line[lo..hi].trim().to_owned();
            // If it's empty, go to the next iteration...
            if curr_container.is_empty() { continue }

            stack.push(curr_container);
        }

        // Remove the last element because this is the key
        stack.pop();
        // Reverse it (it's a stack!)
        stack.reverse();
        // Put the stack into the hash map
        containers.insert(key.to_owned(), stack.to_vec());
    }

    return containers;

}

/**
 * Display the entire set of containers
 */
fn display(containers: &HashMap<u32, Vec<String>>) {
    // Diagnostics to show that the stacks are as expected
    let mut keys: Vec<&u32> = containers.keys().collect();
    // Sort the keys so that they are in order
    keys.sort();
    // Print the corresponding entries for each key
    println!();
    keys.iter().for_each(|key| println!("{key}: {:?}", containers.get(key).unwrap()));
}

/**
 * Print the last element on each crate
 */
fn print_pattern(containers: &HashMap<u32, Vec<String>>) {
    // Diagnostics to show that the stacks are as expected
    let mut keys: Vec<&u32> = containers.keys().collect();
    // Sort the keys so that they are in order
    keys.sort();
    // Print the corresponding entries for each key
    println!("\nPattern:");
    keys.iter().for_each(|key| print!("{}", containers.get(key).unwrap().last().unwrap()));
}