use std::fs;

fn split_to_tuple(input: &str) -> (String, String) {
    let v: Vec<&str> = input.split(')').collect();
    if v.len() != 2 {
        panic!("wrong split len")
    }
    (v[0].to_owned(), v[1].to_owned())
}

fn get_link_list(orbit_info: &Vec<(String, String)>, source: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let mut seen_right = true;

    let mut var_body = source;

    while seen_right {
        seen_right = false;

        for oi in orbit_info {
            if &oi.1 == var_body {
                seen_right = true;
                var_body = &oi.0;
                res.push(oi.0.to_owned());
            }
        }
    }

    res
}

fn count_orbits_between(input: &str, src: &str, dst: &str) -> i32 {
    let orbit_info: Vec<(String, String)> = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| split_to_tuple(s))
        .collect();

    let src_to_com = get_link_list(&orbit_info, src);
    let dst_to_com = get_link_list(&orbit_info, dst);

    println!("{:?}", src_to_com);
    println!("{:?}", dst_to_com);

    let mut res = 0;

    'outer: for b in &src_to_com {
        for bb in &dst_to_com {
            if b == bb {
                break 'outer;
            }
        }
        res += 1;
    }

    'outer2: for b in &dst_to_com {
        for bb in &src_to_com {
            if b == bb {
                break 'outer2;
            }
        }
        res += 1;
    }

    res
}

fn main() {
    let filename = "../input.txt";
    let input = fs::read_to_string(filename).expect("error reading file");

    println!("result: {}", count_orbits_between(&input, "YOU", "SAN"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let test_input =
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
            let expected_result = 4;

            let result = count_orbits_between(&test_input, "YOU", "SAN");
            assert_eq!(expected_result, result);
        }
    }
}
