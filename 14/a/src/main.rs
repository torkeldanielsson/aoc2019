use rand::Rng;

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

fn recursive_exchanger(rules: &Vec<Rule>, mut required: Vec<Produce>) -> i32 {
    let mut rng = rand::thread_rng();

    loop {
        let mut only_ore_left = true;
        for req in &required {
            if req.id != "ORE" {
                only_ore_left = false;
                break;
            }
        }

        if only_ore_left {
            let mut ore_count = 0;
            for req in required {
                ore_count += req.num;
            }
            return ore_count;
        }

        let mut more_to_change = true;
        while more_to_change {
            more_to_change = false;

            {
                let required_original = required.clone();
                required = Vec::new();

                for req in required_original {
                    let mut changed = false;
                    for rule in rules {
                        if rule.output.id == req.id {
                            let mut multiplier = 1;
                            while multiplier * rule.output.num < req.num {
                                multiplier += 1;
                            }
                            if multiplier * rule.output.num == req.num {
                                more_to_change = true;
                                changed = true;
                                for input in &rule.input {
                                    required.push(Produce {
                                        id: input.id.clone(),
                                        num: input.num * multiplier,
                                    });
                                }
                            }
                        }
                    }

                    if !changed {
                        required.push(req);
                    }
                }
            }

            {
                let required_original = required.clone();
                required = Vec::new();

                for req in &required_original {
                    let mut is_in_req = false;
                    for r in &required {
                        if r.id == req.id {
                            is_in_req = true;
                        }
                    }
                    if !is_in_req {
                        let mut sum_req: Produce = req.clone();
                        sum_req.num = 0;
                        for req_2 in &required_original {
                            if req_2.id == req.id {
                                sum_req.num += req_2.num;
                            }
                        }
                        required.push(sum_req);
                    }
                }
            }
        }

        {
            let mut count = 0;
            for r in &required {
                if r.id != "ORE" {
                    count += 1;
                }
            }

            if count > 0 {
                let required_original = required.clone();
                required = Vec::new();

                let chosen = rng.gen_range(0, count);

                let mut i = 0;
                for req in &required_original {
                    if req.id != "ORE" {
                        if i == chosen {
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
                        } else {
                            required.push(req.clone());
                        }

                        i += 1;
                    } else {
                        required.push(req.clone());
                    }
                }

                // println!("{:?} => {:?}", required_original, required);
            }
        }
    }
}

fn get_ore_count(rules: &Vec<Rule>) -> i32 {
    let mut required: Vec<Produce> = Vec::new();

    for rule in rules {
        if rule.output.id == "FUEL" {
            required = rule.input.clone();
        }
    }

    let mut lowest_res = std::i32::MAX;

    for _ in 0..1000000000 {
        let r = recursive_exchanger(rules, required.clone());
        if lowest_res > r {
            lowest_res = r;
            println!("lowest: {}", lowest_res);
        }
    }

    return lowest_res;
}

fn main() {
    // let input_text = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
    // let input_text = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
    // let input_text = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    // let input_text = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF";
    // let input_text = "171 ORE => 8 CNZTR\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n5 BHXH, 4 VRPVC => 5 LTCX";
    let input_text = "12 VJWQR, 1 QTBC => 6 BGXJV\n12 BGTMN, 2 DRKQR => 2 TVSF\n2 FTFK => 2 THNDN\n13 LKRTN, 7 MDPN, 12 NZKQZ => 5 LPWPD\n1 HDKX, 3 DWZS, 1 RCBQS => 1 DCRK\n14 ZCMF => 6 ZKHLC\n3 ZFVH, 2 ZCMF, 1 SCJG, 1 LQWJ, 4 BGBJ, 1 NHPR, 3 VKZFJ, 7 FWFXZ => 4 QVJMP\n11 TNMLB => 7 NVDCR\n1 LPWPD, 1 BGBJ => 2 SCJG\n3 DFCVF, 1 QGSN => 6 PQXG\n1 BGXJV, 1 THNDN => 4 BCQN\n3 LKRTN => 9 MDPN\n2 THNDN, 13 RCKZ, 10 FQSLN => 8 VKZFJ\n4 LBCZ, 9 LWHS => 1 FQSLN\n6 WSRVZ => 9 TNMLB\n8 FQSLN, 14 JQRF, 4 BGTMN => 5 QGSN\n4 ZCMF, 4 PLSM, 2 ZHTX => 8 TDHPM\n2 RSKC, 10 SHBC, 8 MDPN => 6 FMSZ\n2 VJWQR => 2 FPTV\n12 DRKQR => 6 NHPR\n35 QJLF, 22 BGTMN, 11 VJWTR, 1 QVJMP, 8 LQWJ, 1 TWLC, 16 NXZCH, 18 THKF, 42 JBLM => 1 FUEL\n2 BGTMN, 4 XJKN => 8 ZCMF\n4 TVSF => 3 RSKC\n7 HRWS, 1 TVSF => 3 ZHTX\n134 ORE => 4 WSRVZ\n1 VKZFJ, 1 TWLC, 4 ZHTX, 5 THNDN, 12 PLVN, 1 ZFXNP, 1 PQXG, 6 CWHV => 7 VJWTR\n20 XJKN, 1 LCKW, 3 NZKQZ => 7 HDKX\n1 LPWPD => 8 RCKZ\n4 RCBQS, 1 NVDCR => 5 BGBJ\n8 BGXJV => 4 BGTMN\n13 QBDX, 16 BGXJV => 6 NZKQZ\n2 LPWPD => 3 DRKQR\n4 QBDX => 7 XJKN\n12 LCKW, 9 NVDCR => 3 RCBQS\n142 ORE => 3 QBDX\n1 WXHJF => 1 XKDJ\n2 RSKC => 2 CWHV\n2 ZHTX, 1 ZFXNP => 6 JQRF\n1 FTFK, 1 TVSF, 1 QBDX => 2 JBLM\n1 TDHPM, 14 NHPR, 3 QPSF => 5 ZFVH\n3 GDTPC, 1 ZKHLC => 8 ZFXNP\n5 DWZS => 3 LQWJ\n1 FTFK, 4 LBCZ, 13 NHPR => 1 FWFXZ\n1 RCBQS, 12 SHBC => 9 FTFK\n1 WSRVZ, 1 XKDJ => 5 LKRTN\n2 BGTMN, 1 MDPN => 5 PLSM\n2 BGXJV, 17 XKDJ, 4 FPTV => 9 LCKW\n148 ORE => 2 QTBC\n110 ORE => 2 VJWQR\n42 ZFXNP, 15 RCKZ, 8 GDTPC => 3 QJLF\n13 HRWS => 4 GDTPC\n34 HRWS => 4 DFCVF\n2 VKZFJ, 2 NHPR, 16 PLVN, 1 QPSF, 13 LBCZ, 4 DCRK, 10 LWHS => 7 NXZCH\n3 CWHV, 1 THNDN => 7 LWHS\n1 BGXJV, 2 QBDX => 5 DWZS\n9 LQWJ => 8 QPSF\n21 BCQN, 3 FMSZ, 1 RSKC => 5 THKF\n118 ORE => 6 WXHJF\n11 FMSZ => 9 TWLC\n28 PLSM => 5 SHBC\n1 ZKHLC, 23 SCJG => 7 LBCZ\n17 DWZS, 16 THNDN => 9 PLVN\n7 HDKX => 9 HRWS";

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
