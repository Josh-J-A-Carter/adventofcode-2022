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
    let action_1 = opponent_move(actions.next().unwrap());
    let action_2 = my_move(action_1, actions.next().unwrap());

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

/**
 * The move that the opponent is taking, standardised to Rock, Paper, Scissors.
 */
fn opponent_move(input: &str) -> &str {
    match input {
        "A" => { "Rock" }
        "B" => { "Paper" }
        "C" => { "Scissors" }
        _ => { panic!("Invalid argument for standardise_input()") }
    }
}

/**
 * The move that I am taking in response of my opponent, standardised to Rock, Paper Scissors
 * 
 * win_lose_draw:
 * "X" => win
 * "Y" => draw
 * "Z" => lose
 */
fn my_move<'a>(opponent: &'a str, win_lose_draw: &'a str) -> &'a str {
    if win_lose_draw == "Z" {
        match opponent {
            "Rock" => { "Paper" }
            "Paper" => { "Scissors" }
            "Scissors" => { "Rock" }
            _ => { panic!("Invalid arguments for my_move()") }
        }
    }
    else if win_lose_draw == "Y" { opponent.clone() }
    else {
        match opponent {
            "Rock" => { "Scissors" }
            "Paper" => { "Rock" }
            "Scissors" => { "Paper" }
            _ => { panic!("Invalid arguments for my_move()") }
        }
    }
}