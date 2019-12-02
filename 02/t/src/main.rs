use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut numbers: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    if filename.contains("input.txt") {
        println!("replacing because input");
        numbers[1] = 12;
        numbers[2] = 2;
    }

    let mut index = 0;

    while index < numbers.len() {
        if numbers[index] == 99 {
            println!("numbers[0] = {}", numbers[0]);
            break;
        }

        let a = numbers[index + 1] as usize;
        let b = numbers[index + 2] as usize;
        let o = numbers[index + 3] as usize;

        if numbers[index] == 1 {
            numbers[o] = numbers[a] + numbers[b];
        } else if numbers[index] == 2 {
            numbers[o] = numbers[a] * numbers[b];
        } else {
            println!("unexpected: numbers[{}] = {}", index, numbers[index]);
        }

        index += 4;
    }

    for number in numbers {
        print!("{:?},", number);
    }
    println!("");
}
