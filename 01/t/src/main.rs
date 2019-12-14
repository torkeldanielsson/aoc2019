use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("error reading file");

    {
        let mut sum = 0;

        for line in contents.lines() {
            let my_int = line.parse::<i32>().unwrap();
            let val = (my_int / 3) - 2;
            sum += val;
        }

        println!("part 1: {}", sum);
    }

    {
        let mut total_sum = 0;

        for line in contents.lines() {
            let my_int = line.parse::<i32>().unwrap();
            let mut val = (my_int / 3) - 2;
            while val > 0 {
                total_sum += val;
                val = (val / 3) - 2;
            }
        }

        println!("part 2: {}", total_sum);
    }
}
