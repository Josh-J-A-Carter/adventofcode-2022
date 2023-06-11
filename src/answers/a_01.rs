// use std::env;
use std::fs;

/**
 * Arguments:
 * cargo run -- src/input.txt
 */
pub fn func() {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    // println!("Currently in directory {}", file_path);

    let contents = fs::read_to_string("src/input/01.txt")
        .expect("File not found.");

    //Store the most recent elf's calories (we don't need to remember every single elf, just the ones with most calories!)
    //Store the list of elves with the most calories
    let mut previous_elf = 0;
    let mut most_calories: Vec<u32> = vec![0];

    //Loop through the file
    for line in contents.lines() {

        //If the line is empty, it's a new elf.
        if line.is_empty(){
            //Iterate over the elves with the most calories, and change the ordering if necessary
            for i in 0..most_calories.len() {
                if previous_elf > most_calories.get(i).unwrap().to_owned() {
                    most_calories.insert(i, previous_elf);
                    most_calories.pop();
                    //Break early because we already know it's bigger than the remaining elements
                    break;
                }
            }
            previous_elf = 0;
            continue;
        }

        let current = line.parse::<u32>()
            .expect("Incorrect input in the file - can only count numbers :(");
        previous_elf += current;
    }

    println!("Most calories: {:?}", most_calories);
    println!("So the top {:?} elves have {:?} calories between them", most_calories.len(), most_calories.iter().sum::<u32>());

}


