use std::fs;
use std::collections::HashSet;

pub fn func() {

    let contents = fs::read_to_string("src/input/09.txt").expect("Could not read the file");

    // The points to be tracking as they move through space
    let mut knots = vec![Point{x: 0, y: 0}; 10];

    // Store the positions we have already visited
    let mut visited_positions: HashSet<Point> = HashSet::new();
    visited_positions.insert(knots.last().unwrap().clone());

    // Iterate through all the commands, updating positions as necessary
    for line in contents.lines() {
        // read in the current instruction
        let tokens: Vec<&str> = line.split_whitespace().collect();
        // Repeat this command 'num_iterations' times
        let num_iterations = tokens[1].parse::<i32>().expect("Unable to parse string as an integer");
        for _ in 0 .. num_iterations {
            // Update the head's position
            let head = knots.first_mut().unwrap();
            match tokens[0] {
                "U" => { head.y -= 1 }
                "D" => { head.y += 1 }
                "L" => { head.x -= 1 }
                "R" => { head.x += 1 }
                _ => { panic!("Unknown command") }
            }

            // Update all the other knots, using the previous knot as the head
            for i in 1 .. knots.len() { knots[i] = next_tail_pos(&knots[i-1], &knots[i]) }

            // Add this position to the list of visited positions
            let tail = knots.last().unwrap();
            visited_positions.insert(tail.clone());
        }
    }

    println!("The tail visited {} positions", visited_positions.len());
}


/**
 * Find the next position of the tail, in reference to the specified head
 */
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