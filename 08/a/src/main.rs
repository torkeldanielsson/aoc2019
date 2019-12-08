use std::env;
use std::fs;

fn parse_image(input_str: &str, layer_dims: (i32, i32)) -> i32 {
    let input: Vec<i32> = input_str
        .to_owned()
        .trim()
        .chars()
        .map(|c| (c as i32 - '0' as i32))
        .collect();

    let layer_stride = layer_dims.0 * layer_dims.1;

    let mut lowest_nr_of_zeroes = std::u32::MAX;
    let mut res = 0;

    for chunk in input.chunks(layer_stride as usize) {
        let mut nr_of_zeroes = 0;
        let mut nr_of_ones = 0;
        let mut nr_of_twos = 0;
        for d in chunk {
            if *d == 0 {
                nr_of_zeroes += 1;
            }
            if *d == 1 {
                nr_of_ones += 1;
            }
            if *d == 2 {
                nr_of_twos += 1;
            }
        }
        if nr_of_zeroes < lowest_nr_of_zeroes {
            lowest_nr_of_zeroes = nr_of_zeroes;
            res = nr_of_ones * nr_of_twos;
            println!("{:?}", chunk);
        }
    }

    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let res = parse_image(&input, (25, 6));

    println!("result: {:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let input = "123456789012";
            let res = parse_image(&input, (3, 2));
            let expected_result = 1;
            assert_eq!(expected_result, res);
        }
    }
}
