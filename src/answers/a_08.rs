use std::fs;

pub fn func() {
    let contents = fs::read_to_string("src/input/08.txt").expect("Could not find the file.");

    let grid = read_as_grid(contents);

    let mut num_visible_trees = 0 as u32;
    let mut max_scenic_score = 0 as u32;

    for row in 0 .. grid.len() {
        for col in 0 .. grid[0].len() {
            if is_visible(&grid, row, col) { num_visible_trees += 1 }

            let scenic_score = scenic_score(&grid, row, col);
            if scenic_score > max_scenic_score { max_scenic_score = scenic_score }
        }
    }

    println!("{} trees are visible", num_visible_trees);
    println!("{} is the max scenic score", max_scenic_score);
}

/**
 * Is the tree at the specified position covered by surrounding trees?
 * 
 * Takes the grid of trees, as well as the row and column of the tree to check
 * 
 * Returns true if the specified tree is not covered by a taller tree from each direction
 */
fn is_visible(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    // If the position is in the first row or column, it is visible
    if row <= 0 || col <= 0 { return true }
    // If the position is in the last row or column, it is visible
    if row >= grid.len() - 1 || col >= grid[0].len() - 1 { return true }
    
    // We want to check if there are any trees from the left that are taller, or equally as tall
    // We only need ONE tree to be taller or equally sized in order for the tree to be covered
    let mut is_covered = false;
    for col_2 in 0 .. col {
        if grid[row][col_2] >= grid[row][col] {
            is_covered = true;
            break
        }
    }
    if !is_covered { return true }

    // Check on the right side
    let mut is_covered = false;
    for col_2 in col+1 .. grid[0].len() {
        if grid[row][col_2] >= grid[row][col] {
            is_covered = true;
            break
        }
    }
    if !is_covered { return true }

    // Check from above
    let mut is_covered = false;
    for row_2 in 0 .. row {
        if grid[row_2][col] >= grid[row][col] {
            is_covered = true;
            break
        }
    }
    if !is_covered { return true }

    // Check from below
    let mut is_covered = false;
    for row_2 in row+1 .. grid.len() {
        if grid[row_2][col] >= grid[row][col] {
            is_covered = true;
            break
        }
    }
    if !is_covered { return true }

    // Otherwise, it is not visible
    false
}

/**
 * Find the scenic score of the specified tree
 * 
 * Takes the grid of trees, as well as the row and column of the tree to check
 * 
 * Returns the scenic score of the tree at this position
 */
fn scenic_score(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    // Need to avoid overflow errors
    if row <= 0 || col <= 0 { return 0 }
    if row >= grid.len() - 1 || col >= grid[0].len() - 1 { return 0 }

    let mut left_distance = 0 as u32;
    for col_2 in (0 .. col).rev() {
        left_distance += 1;
        if grid[row][col_2] >= grid[row][col] { break }
    }

    // Check on the right side
    let mut right_distance = 0 as u32;
    for col_2 in col+1 .. grid[0].len() {
        right_distance += 1;
        if grid[row][col_2] >= grid[row][col] { break }
    }

    // Check from above
    let mut up_distance = 0 as u32;
    for row_2 in (0 .. row).rev() {
        up_distance += 1;
        if grid[row_2][col] >= grid[row][col] { break }
    }

    // Check from below
    let mut down_distance = 0 as u32;
    for row_2 in row+1 .. grid.len() {
        down_distance += 1;
        if grid[row_2][col] >= grid[row][col] { break }
    }

    // Otherwise, it is not visible
    left_distance * right_distance * up_distance * down_distance
}

fn read_as_grid(contents: String) -> Vec<Vec<u32>> {
    // Since the dimensions are dynamic, we have to use the heap's memory
    let mut grid = vec![];

    // For each line in the file,
    for line in contents.lines() {
        // Take each character in this line and map it to an integer
        // Store these mappings as a vector, and push it to the grid vector.
        grid.push(line.chars().map(|c| c.to_digit(10).expect("Unable to parse")).collect());
    }

    grid
}
