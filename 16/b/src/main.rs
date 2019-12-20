fn main() {
    // let input = "12345678";
    // let input = "03036732577212944063491565474664";
    // let input = "02935109699940807407585447034323";
    // let input = "03081770884921959731165446850517";
    let input = "59704176224151213770484189932636989396016853707543672704688031159981571127975101449262562108536062222616286393177775420275833561490214618092338108958319534766917790598728831388012618201701341130599267905059417956666371111749252733037090364984971914108277005170417001289652084308389839318318592713462923155468396822247189750655575623017333088246364350280299985979331660143758996484413769438651303748536351772868104792161361952505811489060546839032499706132682563962136170941039904873411038529684473891392104152677551989278815089949043159200373061921992851799948057507078358356630228490883482290389217471790233756775862302710944760078623023456856105493";

    let mut processed_input: String = String::new();

    for _ in 0..10000 {
        processed_input.push_str(&input)
    }

    let mut numbers: Vec<i64> = processed_input
        .chars()
        .map(|c| c.to_string().parse::<i64>().unwrap())
        .collect();

    let mut index = 0;
    for i in 0..7 {
        let mut multipl = 1;
        for _ in 0..i {
            multipl *= 10;
        }
        index += multipl * numbers[6 - i];
    }

    println!("index: {}", index);

    for generation in 0..100 {
        {
            print!("{}: ", generation);
            for i in 0..8 {
                print!("{}", numbers[i]);
            }
            println!("");
        }

        let old_numbers = numbers.clone();

        for n in &mut numbers {
            *n = 0;
        }

        let mut sign = 1;
        for sweep_series in 0..(numbers.len() / 2 + 1) {
            let mut start_pos = sweep_series * 2;

            let mut old_start_pos = 0;
            let mut old_end_pos = 0;
            let mut old_val = -1;

            let mut output_index = 0;

            while start_pos < numbers.len() && output_index < numbers.len() {
                let mut val = 0;
                let end_pos = std::cmp::min(start_pos + output_index + 1, numbers.len());

                if start_pos > old_end_pos || start_pos == 0 {
                    for i in start_pos..end_pos {
                        val += old_numbers[i];
                    }
                } else {
                    val = old_val;
                    for i in old_start_pos..start_pos {
                        val -= old_numbers[i];
                    }
                    for i in old_end_pos..end_pos {
                        val += old_numbers[i];
                    }

                    /*
                    let mut test_val = 0;
                    for i in start_pos..end_pos {
                        test_val += old_numbers[i];
                    }
                    if val != test_val {
                        println!("{} != {}", val, test_val);
                    }
                    */
                }

                old_start_pos = start_pos;
                old_end_pos = end_pos;
                old_val = val;

                numbers[output_index] += sign * val;

                start_pos += 1 + sweep_series * 2;

                output_index += 1;
            }

            sign *= -1;
        }

        for n in &mut numbers {
            *n = n.abs() % 10;
        }
    }

    {
        for i in index..index + 8 {
            print!("{}", numbers[i as usize]);
        }
        println!("");
    }
}
