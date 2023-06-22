use std::{fs, collections::VecDeque};

pub fn func() {
    let contents = fs::read_to_string("src/input/11.txt").expect("Unable to read file");

    let mut monkeys = parse_contents(contents);

    // We run the simultation for 20 rounds
    for _ in 1 ..= 20 {
        for monkey in 0 .. monkeys.len() {
            inspect(&mut monkeys, monkey)
        }
    }

    // Then we check how many inspections each monkey has done
    let mut highest_num = vec![0; 2];
    for monkey in 0 .. monkeys.len() {
        let curr_total = monkeys[monkey].total_inspections;
        println!("Monkey {monkey} has done {} inspections", curr_total);
        for i in 0 .. highest_num.len() {
            if curr_total > highest_num[i] {
                highest_num.insert(i, curr_total);
                highest_num.pop();
                break
            }
        }
    }
    let combined_score = highest_num.iter().fold(1, |sum, val| sum * val);
    println!("\nThe multiplicative score of the most active monkeys is {combined_score}");

}

fn parse_contents(contents: String) -> Vec<Monkey> {

    let mut monkeys = vec![];

    let mut iter = contents.lines();

    loop {
        match iter.next() {
            // No more input; end the loop
            None => { break }
            // In this case, we know the input is in order so I already know the index number
            Some(_) => {}
        }

        // Then collect and process the starting items
        let mut items = VecDeque::new();
        let token_stream = iter.next().expect("Ran out of input during initialisation").replace(",", "");
        let tokens: Vec<&str> = token_stream.split(' ').collect();
        // Parse the integers and put them in the items queue
        for token in tokens.iter() {
            match token.parse::<u32>() {
                Err(_) => { continue }
                Ok(value) => { items.push_back(value) }
            }
        }

        // Read in the operation and operand
        let mut operation = Operation::Empty;
        let mut operand = 0;
        let input = iter.next().expect("Ran out of input during initialisation");
        let tokens: Vec<&str> = input.split(' ').collect();

        if input.contains("* old") { operation = Operation::Square }
        else if input.contains('*') { operation = Operation::Multiply }
        else if input.contains('+') { operation = Operation::Add }

        for token in tokens {
            match token.parse::<u32>() {
                Err(_) => { continue }
                Ok(value) => { operand = value }
            }
        }

        // Read in the divisor test
        let mut divide_test = 0;
        let tokens: Vec<&str> = iter.next().expect("Ran out of input during initialisation").split(' ').collect();
        for token in tokens {
            match token.parse::<u32>() {
                Err(_) => { continue }
                Ok(value) => { divide_test = value }
            }
        }

        // Read in the first recipient
        let mut recipient_1 = 0;
        let tokens: Vec<&str> = iter.next().expect("Ran out of input during initialisation").split(' ').collect();
        for token in tokens {
            match token.parse::<usize>() {
                Err(_) => { continue }
                Ok(value) => { recipient_1 = value }
            }
        }

        // Read in the second recipient
        let mut recipient_2 = 0;
        let tokens: Vec<&str> = iter.next().expect("Ran out of input during initialisation").split(' ').collect();
        for token in tokens {
            match token.parse::<usize>() {
                Err(_) => { continue }
                Ok(value) => { recipient_2 = value }
            }
        }
        // Skip the next line as it is empty
        iter.next();

        // Finally, store these values in the vector
        monkeys.push(Monkey {
            items: items,
            operation: operation,
            operand: operand,
            divide_test: divide_test,
            recipients: (recipient_1, recipient_2),
            total_inspections: 0
        });
    }

    monkeys
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    operand: u32,
    divide_test: u32,
    recipients: (usize, usize),
    total_inspections: u32
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Square,
    Empty
}

fn inspect(monkeys: &mut Vec<Monkey>, inspector: usize) {

    loop {
        // Try get another element
        match monkeys[inspector].items.pop_front() {
            // The queue is empty, so this monkey has finished
            None => { break }
            // This monkey still has items to inspect
            Some(item) => {
                monkeys[inspector].total_inspections += 1;

                // Apply the operation
                let mut current = item;
                match &monkeys[inspector].operation {
                    Operation::Add => { current += monkeys[inspector].operand }
                    Operation::Multiply => { current *= monkeys[inspector].operand }
                    Operation::Square => { current *= current }
                    Operation::Empty => { panic!("Uninitialised 'operation' field during inspect()") }
                }
                // 'Worry level' divided by 3
                current /= 3;

                // Check which monkey to pass this to
                let mut recipient = monkeys[inspector].recipients.1;
                if monkeys[inspector].divide_test == 0 { panic!("'divide_test' uninitialised; tried to divide by zero!") }
                if current % monkeys[inspector].divide_test == 0 { recipient = monkeys[inspector].recipients.0 }
                monkeys[recipient].items.push_back(current);
            }
        }
    }

}
