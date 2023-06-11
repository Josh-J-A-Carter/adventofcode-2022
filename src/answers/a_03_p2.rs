use std::fs;

pub fn func(){
    let contents = fs::read_to_string("src/input/03.txt");

    
    let binding = contents
        .unwrap();
    
    let mut lines = binding.lines();

    let mut score = 0 as u32;
    
    loop {
        let elf_1 = match lines.next() {
            Some(s) => { s }
            None => { break; }
        };
        let elf_2 = lines.next().unwrap();
        let elf_3 = lines.next().unwrap();

        score += get_badge(elf_1, elf_2, elf_3);
    }
    
    println!("Total score {score}")
    
}

fn get_badge(elf_1: &str, elf_2: &str, elf_3: &str) -> u32 {
    for c_1 in elf_1.chars() {
        for c_2 in elf_2.chars() {
            for c_3 in elf_3.chars() {
                if c_1 == c_2 && c_2 == c_3 {
                    let mut val = 1 + c_1.to_ascii_uppercase() as u32 - 'A' as u32;
                    if c_1.is_ascii_uppercase() { val += 26 }
                    return val
                }
            }
        }
    }

    panic!("No matching item");
}