use std::collections::HashSet;
use std::fs;

fn split_to_tuple(input: &str) -> (String, String) {
    let v: Vec<&str> = input.split(')').collect();
    if v.len() != 2 {
        panic!("wrong split len")
    }
    (v[0].to_owned(), v[1].to_owned())
}

fn count_links(input: &str) -> i32 {
    let orbit_info: Vec<(String, String)> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| split_to_tuple(s))
        .collect();

    let mut bodies = HashSet::new();

    for oi in &orbit_info {
        bodies.insert(&oi.0);
        bodies.insert(&oi.1);
    }

    let mut sum = 0;

    for b in &bodies {
        let mut orbit_count = 0;

        let mut seen_right = true;

        let mut var_body = b.to_owned();

        while seen_right {
            seen_right = false;

            for oi in &orbit_info {
                if &oi.1 == var_body {
                    seen_right = true;
                    var_body = &oi.0;
                    orbit_count += 1;
                }
            }
        }

        //println!("{:?}, count {}", b, orbit_count);
        sum += orbit_count;
    }

    sum
}

fn main() {
    let filename = "../input.txt";
    let contents = fs::read_to_string(filename).expect("error reading file");

    println!("result: {}", count_links(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let test_input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
            let expected_result = 42;

            let result = count_links(test_input);
            assert_eq!(expected_result, result);
        }
    }
}
