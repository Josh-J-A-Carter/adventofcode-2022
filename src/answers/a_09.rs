use std::fs;
use std::collections::HashSet;


pub fn func() {

    let contents = fs::read_to_string("src/input/09.txt").expect("Could not read the file");

    // The points to be tracking as they move through space
    let mut head = Point{x: 0, y: 0};
    let mut tail = Point{x: 0, y: 0};

    // Store the positions we have already visited
    let mut visited_positions: HashSet<Point> = HashSet::new();
    visited_positions.insert(tail.clone());

    // Iterate through all the commands, updating positions as necessary
    for line in contents.lines() {
        // read in the current instruction
        let tokens: Vec<&str> = line.split_whitespace().collect();
        // Repeat this command 'num_iterations' times
        let num_iterations = tokens[1].parse::<i32>().expect("Unable to parse string as an integer");
        for _ in 0 .. num_iterations {

            // Debugging print statements
            //print_board(&head, &tail);
            //println!("{}", tokens[0]);

            // Update the head's position
            match tokens[0] {
                "U" => { head.y -= 1 }
                "D" => { head.y += 1 }
                "L" => { head.x -= 1 }
                "R" => { head.x += 1 }
                _ => { panic!("Unknown command") }
            }

            // Update the tail's position
            tail = next_tail_pos(&head, &tail);

            // Add this position to the list of visited positions
            visited_positions.insert(tail.clone());

        }
    }

    println!("The tail visited {} positions", visited_positions.len());
}


fn print_board(head: &Point, tail: &Point) {
    for y in -25 .. 25 {
        for x in -25 .. 25 {
            if head.x == x && head.y == y { print!("H") }
            else if tail.x == x && tail.y == y { print!("T") }
            else { print!(".") }
        }
        println!();
    }
    println!();
}

fn next_tail_pos(head: &Point, tail: &Point) -> Point {

    let diff_x = (head.x - tail.x).abs();
    let diff_y = (head.y - tail.y).abs();

    // Tail is already in an allowed position
    if diff_x <= 1 && diff_y <= 1 { return tail.clone() }

    // Otherwise, we want to move the tail *towards* the head
    // That means finding if the difference for each axis is positive or negative
    // We then normalise this so that it is either +1 or -1, by dividing by its absolute value
    let x_direction;
    let y_direction;
    if diff_x > 0 { x_direction = (head.x - tail.x) / diff_x }
    else { x_direction = 0 }

    if diff_y > 0 { y_direction = (head.y - tail.y) / diff_y }
    else { y_direction = 0 }

    // We want this to have specific behaviour if there is only a difference on one of the axes
    // I.e. if we have a change in x OR y, but not both
    if diff_x > 1 && diff_y < 1 { return Point{x: tail.x + x_direction, y: tail.y} }
    if diff_x < 1 && diff_y > 1 { return Point{x: tail.x, y: tail.y + y_direction} }

    // Otherwise, we have a change in both x and y, so we have to 
    Point{x: tail.x + x_direction, y: tail.y + y_direction}
}


#[derive(Eq, PartialEq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32
}