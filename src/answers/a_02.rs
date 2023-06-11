use std::fs;

pub fn func(){
    let contents = fs::read_to_string("src/input/02.txt")
        .expect("Could not find the file.");
    //println!("Contents: {}", contents);

    let score = contents
        .lines()
        .fold(0, |sum, val| sum + determine_score(val));

    println!("Final score: {}", score);
}

fn determine_score(line: &str) -> u16 {
    //Split it up into tokens and take the first two of them
    let mut actions = line.split_ascii_whitespace();
    let action_1 = standardise_input(actions.next().unwrap());
    let action_2 = standardise_input(actions.next().unwrap());

    let mut _score = 0;

    // Score for each type of move    
    match action_2 {
        "Rock" => { _score = 1 }
        "Paper" => { _score = 2 }
        "Scissors" => { _score = 3 }
        _ => { panic!("Invalid input") }
    }

    // Score for the outcome
    if action_1.eq(action_2) { _score += 3 }
    else {
        if action_2 == "Rock" && action_1 == "Scissors" { _score += 6 }
        else if action_2 == "Scissors" && action_1 == "Paper" { _score += 6 }
        else if action_2 == "Paper" && action_1 == "Rock" { _score += 6 }
    }

    _score
}

fn standardise_input(input: &str) -> &str {
    match input {
        "A" | "X" => { "Rock" }
        "B" | "Y" => { "Paper" }
        "C" | "Z" => { "Scissors" }
        _ => { panic!("Invalid argument for standardise_input()") }
    }
}