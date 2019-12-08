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

    let chunks = input.chunks(layer_stride as usize);

    for i in 0..layer_stride {
        if i != 0 && (i % layer_dims.0) == 0 {
            print!("\n");
        }
        for l in 0..(input.len() / layer_stride as usize) {
            let coord: usize = l * layer_stride as usize + i as usize;
            if input[coord] == 1 {
                print!("#");
                break;
            }
            if input[coord] == 0 {
                print!("-");
                break;
            }
        }
        print!(" ");
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
