use std::fs;

pub fn func() {
    let contents = fs::read_to_string("src/input/10.txt").expect("Unable to read file");

    let mut cycle = 0 as u32;
    let mut crt_position = 0 as u32;
    let mut register = 1 as i32;
    let mut operation = Operation::new(OperationType::Empty);
    let mut instructions = contents.lines();

    loop {
        // During cycle
        
        // increment the cycles
        cycle += 1;
        
        
        // Draw the current pixel, depending on if the sprite (the register's current value) is within range
        if (crt_position as i32 - register).abs() <= 1 { print!("#") }
        else { print!(".") }
        
        // Go to the next line and reset CRT's position if it's on the next line
        crt_position += 1;
        if cycle % 40 == 0 {
            println!();
            crt_position = 0;
        }
        
        // After the cycle

        // Try load in the next operation if it is currently empty
        match &operation.op {
            OperationType::Empty => {
                match instructions.next() {
                    // No more instructions (EOF)
                    None => { break }

                    Some(input) => {
                        // Read in this instruction and determine which type it is
                        let tokens: Vec<&str> = input.split(' ').collect();
                        match tokens[0] {
                            "noop" => { operation = Operation::new(OperationType::Noop) }
                            "addx" => {
                                let value = tokens[1].parse::<i32>().expect("Unable to parse string to i32");
                                operation = Operation::new(OperationType::Addx {value: value});
                            }
                            _ => { panic!("Undefined instruction") }
                        }
                    }
                }
            }
            // If we already have an operation loaded, then we skip this
            _ => {}
        }

        // Now tell the operation to do its thing
        operation.increment_cycle(&mut register);
    }
}

struct Operation {
    cycles: u8,
    op: OperationType
}

enum OperationType { 
    Empty,
    Noop,
    Addx { value: i32 }
}

impl Operation {
    fn new(op: OperationType) -> Operation {
        Operation{
            cycles: 0,
            op: op
        }
    }

    /**
     * Increment the timer on the current operation, and affect the register
     * 
     * If this operation ends at the end of this cycle, then mutate it to the Empty variant
     */
    fn increment_cycle(&mut self, register: &mut i32) {
        self.cycles += 1;

        // Make sure that instructions actually terminate
        match &self.op {
            OperationType::Noop => {
                if self.cycles >= 1 { self.op = OperationType::Empty }
            }
            OperationType::Addx{value} => {
                if self.cycles >= 2 {
                    *register += value;
                    self.op = OperationType::Empty;
                }
            }
            _ => {}
        }
    }
}