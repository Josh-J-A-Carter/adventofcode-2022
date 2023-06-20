use std::{fs, rc::Rc, cell::RefCell, str::Lines};


pub fn func() {

    let contents = fs::read_to_string("src/input/07.txt").unwrap();

    // iterator for all the contents
    let mut iter = contents.lines();

    // create the tree
    let root = create_tree(&mut iter);

    // find the total size of all the folders below a certain size threshold
    let total_size = traverse_max(root.clone(), 100_000);
    println!("Total size: {}", total_size);

    // find the smallest folder that has at least 'required_space' size
    let free_space = 70_000_000 - root.borrow().size;
    let required_space = 30_000_000 - free_space;
    let smallest_folder = traverse_min(root.clone(), required_space);
    println!("Minimum folder size that fits the criterion: {}", smallest_folder);

}

/**
 * Takes a Lines iterator and returns a reference to the root directory
 */
fn create_tree(iter: &mut Lines) -> Rc<RefCell<Folder>> {
    let mut current = Rc::new(RefCell::new(Folder{
        name: "/".to_owned(),
        parent: None,
        subfolders: Vec::new(),
        size: 0
    }));

    // We'll want to know the root node in the future
    let root = current.clone();

    // Ignore the first command! It's a cd command with no ls command before it
    iter.next();

    // go through all the contents
    loop {
        // get the next command. Break if empty, or unwrap
        let next = iter.next();
        if next == None { break }
        let current_command = next.unwrap();
        //println!("CURRENT DIR: {}", current.borrow().to_string());
        //println!("{current_command}");

        // We assume this is a command - not output from previous command
        let tokens: Vec<&str> = current_command.split(' ').collect();
        match tokens[1] {
            // if it's a cd, we need to know which directory to switch into
            "cd" => {
                // Need to create a temporary binding to hold the potential subdirectory
                // Otherwise, we reassign 'current' while it has been borrowed.
                let mut next_current = Option::None;
                for dir in current.borrow().subfolders.iter() {
                    if tokens[2] == (*dir.borrow()).name {
                        next_current = Option::Some(dir.clone());
                        //Stop looping
                        break
                    }
                }
                match next_current {
                    Some(folder) => { current = folder }
                    None => {
                        // May have run 'cd ..', so need to account for this!
                        // Panic if we cd out of the root directory
                        // Otherwise, just go into the parent directory
                        if tokens[2] == ".." {
                            let mut _next_parent = None;
                            match &current.borrow().parent {
                                None => { panic!("Tried to cd .. out of the file system!") }
                                Some(parent) => { _next_parent = Some(parent.clone()) }
                            }
                            match _next_parent {
                                Some(folder) => { current = folder }
                                None => {}
                            }
                        }
                    }
                }
            }

            // List all the subdirectories and files here
            // We want to collect the next few lines of input, and turn them into files/folders
            "ls" => {
                for input in iter.clone() {
                    
                    // Split this line of input into tokens
                    let curr_tokens: Vec<&str> = input.split(' ').collect();
                    match curr_tokens[0] {
                        // Stop when we get to an actual command
                        "$" => { break }
                        // If it's a directory, slap it in the subdirectories vector
                        "dir" => {
                            current.borrow_mut().subfolders.push(Rc::new(RefCell::new(Folder{
                                name: curr_tokens[1].to_owned(),
                                parent: Option::Some(current.clone()),
                                subfolders: Vec::new(),
                                size: 0
                            })));
                        }

                        // Otherwise, it's a number for a file
                        num => {
                            // Add this file to the directory
                            let size = num.parse::<u32>().unwrap();
                            
                            // we've made the file, now we just need to add the size to its parents
                            let mut node = current.clone();
                            loop {
                                // Increase the size
                                node.borrow_mut().size += size;
                                // Then get the next parent and store it in 'node'
                                let mut _next_node = None;
                                match &node.borrow().parent {
                                    None => { break }
                                    Some(parent) => { _next_node = Some(parent.clone()) }
                                }
                                match &_next_node {
                                    Some(folder) => { node = folder.clone() }
                                    None => { break }
                                }
                            }
                        }
                    }
                }
            }

            // This isn't a command we care about; probably just some input
            _ => { continue }
        }
    }
    return root
}


/**
 * Recursively traverse the folders to see which ones fit within the size limit
 * 
 * root - the node to start looking at, all children will be recursively checked too
 * max_size - the maximum size allowed for a folder to be included (inclusive)
 * 
 * Returns the total size of all the folders that fit this criterion
 */
fn traverse_max(root: Rc<RefCell<Folder>>, max_size: u32) -> u32 {
    let binding = root.borrow();
    let mut total_size = 0;
    if binding.size <= max_size {
        //println!("Match: {} with {}", binding.name, binding.size);
        total_size += binding.size;
    }

    // Recurse on all the children
    // The base case is implicit; if there are no children, then there is no recursive call
    for subdir in binding.subfolders.iter() {
        total_size += traverse_max(subdir.clone(), max_size);        
    }

    return total_size
}

/**
 * Recursively traverse the tree to find the smallest directory that is at least equal to min_size
 * 
 * root - the node to start looking at, all children will be recursively checked too
 * min_size - the minimum size allowed for a folder to be be deleted (inclusive)
 * 
 * Returns only the smallest folder which has size >= min_size
 */
fn traverse_min(root: Rc<RefCell<Folder>>, min_size: u32) -> u32 {
    let binding = root.borrow();
    // Return early if this directory is too small
    // Don't check children, since they'll be too small as well!
    if binding.size < min_size { return 0 }

    //println!("Match: {} with {}", binding.name, binding.size);
    let mut curr_min = binding.size;

    // Recurse on all the children
    // The base case is implicit; if there are no children, then there is no recursive call
    for subdir in binding.subfolders.iter() {
        let val = traverse_min(subdir.clone(), min_size);

        // If this yields a smaller result than curr_min
        // and that result is still above the threshold, then update curr_min
        if val >= min_size && val < curr_min {
            curr_min = val;
        }
    }

    return curr_min
}

struct Folder {
    name: String, 
    parent: Option<Rc<RefCell<Folder>>>,
    subfolders: Vec<Rc<RefCell<Folder>>>,
    size: u32
}