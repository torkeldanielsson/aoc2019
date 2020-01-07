use std::env;
use std::fs;

fn main() {
    /*
        let input = "deal into new stack
    cut -2
    deal with increment 7
    cut 8
    cut -4
    deal with increment 7
    cut 3
    deal with increment 9
    deal with increment 3
    cut -1";
    */

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let mut stack: Vec<i32> = (0..10007).collect();

    println!("{:?}", stack);

    for line in input.lines() {
        if line.contains("stack") {
            println!("deal into new stack");
            stack.reverse();
        } else if line.contains("increment") {
            let num_tmp: Vec<&str> = line.split(" ").skip(3).take(1).collect();
            let num = num_tmp[0].parse::<i32>().unwrap();
            println!("deal with increment {}", num);
            let mut pos: i32 = 0;
            let old_stack = stack.clone();
            for n in old_stack {
                stack[pos as usize] = n;
                pos = (pos + num) % stack.len() as i32;
            }
        } else if line.contains("cut") {
            let num_tmp: Vec<&str> = line.split(" ").skip(1).take(1).collect();
            let mut num = num_tmp[0].parse::<i32>().unwrap();
            println!("cut {}", num);
            if num < 0 {
                num = stack.len() as i32 + num;
            }

            let tmp = stack[0..(num as usize)].to_owned();
            stack = stack[(num as usize)..stack.len()].to_owned();
            stack.extend(tmp);
        }
    }
    for i in 0..stack.len() {
        if stack[i] == 2019 {
            println!("{}", i);
        }
    }
}
