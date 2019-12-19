#[derive(Debug, PartialEq, Clone)]
struct Produce {
    id: String,
    num: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    output: Produce,
    input: Vec<Produce>,
}

fn parse_produce(input: &str) -> Produce {
    let parts: Vec<&str> = input.split(' ').map(|s| s.trim()).collect();

    if parts.len() != 2 {
        panic!("why is length not 2? {}, {:?}", parts.len(), parts);
    }

    return Produce {
        id: parts[1].to_string(),
        num: parts[0].parse::<i32>().unwrap(),
    };
}

fn parse_input_to_rule_list(input: &str) -> Vec<Rule> {
    let mut res: Vec<Rule> = Vec::new();

    for line in input.lines() {
        let main_parts: Vec<&str> = line.split("=>").map(|s| s.trim()).collect();

        if main_parts.len() != 2 {
            panic!(
                "why is length not 2? {}, {:?}",
                main_parts.len(),
                main_parts
            );
        }

        let output = parse_produce(main_parts[1]);

        let mut procude_inputs: Vec<Produce> = Vec::new();
        let inputs: Vec<&str> = main_parts[0].split(',').map(|s| s.trim()).collect();
        for input_produce in inputs {
            procude_inputs.push(parse_produce(input_produce));
        }

        res.push(Rule {
            output: output,
            input: procude_inputs,
        });
    }

    return res;
}

fn get_ore_count(rules: &Vec<Rule>) -> i32 {
    let mut required: Vec<Produce> = Vec::new();

    for rule in rules {
        if rule.output.id == "FUEL" {
            required = rule.input.clone();
        }
    }

    println!("{:?}", required);

    let mut non_ore_left = true;
    while non_ore_left {
        non_ore_left = false;

        {
            let required_before = required.clone();

            required = Vec::new();

            for req in &required_before {
                if req.id == "ORE" {
                    required.push(req.to_owned());
                } else {
                    non_ore_left = true;

                    for rule in rules {
                        if rule.output.id == req.id {
                            let mut multiplier = 1;
                            while multiplier * rule.output.num < req.num {
                                multiplier += 1;
                            }
                            for input in &rule.input {
                                required.push(Produce {
                                    id: input.id.clone(),
                                    num: input.num * multiplier,
                                });
                            }
                        }
                    }
                }
            }
        }

        println!("     {:?}", required);

        {
            let required_before = required.clone();

            required = Vec::new();
            for (req_prim_i, req_prim) in required_before.iter().enumerate() {
                let mut is_in_req = false;
                for r in &required {
                    if r.id == req_prim.id {
                        is_in_req = true;
                    }
                }
                if !is_in_req {
                    let mut sum_req: Produce = req_prim.clone();
                    for (i, r) in required_before.iter().enumerate() {
                        if req_prim_i != i && r.id == req_prim.id {
                            sum_req.num += r.num;
                        }
                    }
                    required.push(sum_req);
                }
            }
        }
        println!("{:?}", required);
    }

    let mut ore_count = 0;
    for req in required {
        ore_count += req.num;
    }

    return ore_count;
}

fn main() {
    let input_text = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
    let rules = parse_input_to_rule_list(input_text);

    // println!("{:?}", rules);

    let ore_count = get_ore_count(&rules);

    println!("{:?}", ore_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let input_text = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
            let rules = parse_input_to_rule_list(input_text);

            println!("{:?}", rules);

            let expected_pos = Vec2 { x: 1, y: 1 };
            let expected_visible = 2;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
    }
}
