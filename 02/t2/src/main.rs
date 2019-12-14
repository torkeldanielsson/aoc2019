use std::fs;

fn main() {
    let filename = "../input.txt";
    let contents = fs::read_to_string(filename).expect("error reading file");

    let original_program: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let target_value = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut numbers = original_program.clone();

            numbers[1] = noun;
            numbers[2] = verb;

            let mut instruction_pointer = 0;

            let mut result = 0;

            while instruction_pointer < numbers.len() {
                if numbers[instruction_pointer] == 99 {
                    result = numbers[0];
                    break;
                }

                let a = numbers[instruction_pointer + 1] as usize;
                let b = numbers[instruction_pointer + 2] as usize;
                let o = numbers[instruction_pointer + 3] as usize;

                if numbers[instruction_pointer] == 1 {
                    numbers[o] = numbers[a] + numbers[b];
                } else if numbers[instruction_pointer] == 2 {
                    numbers[o] = numbers[a] * numbers[b];
                } else {
                    println!(
                        "unexpected: numbers[{}] = {}",
                        instruction_pointer, numbers[instruction_pointer]
                    );
                }

                instruction_pointer += 4;
            }

            if result == target_value {
                println!("Match! noun: {}, verb: {}", noun, verb);
                return;
            }
        }
    }
}
