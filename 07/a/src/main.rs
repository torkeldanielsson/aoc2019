use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

fn digit_to_mode(d: i32) -> Mode {
    match d {
        0 => Mode::Position,
        1 => Mode::Immediate,
        _ => {
            panic!("unexpected mode digit: {}", d);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Break,
}

fn digit_to_opcode(d: i32) -> Opcode {
    match d {
        1 => Opcode::Addition,
        2 => Opcode::Multiplication,
        3 => Opcode::Input,
        4 => Opcode::Output,
        5 => Opcode::JumpIfTrue,
        6 => Opcode::JumpIfFalse,
        7 => Opcode::LessThan,
        8 => Opcode::Equals,
        99 => Opcode::Break,
        _ => {
            panic!("Illegal opcode {:?}", d);
        }
    }
}

fn parse_op(n: usize) -> (Opcode, Mode, Mode, Mode) {
    fn x_inner(n: usize, xs: &mut Vec<usize>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);

    xs.reverse();

    while xs.len() < 5 {
        xs.push(0);
    }

    xs.reverse();

    (
        digit_to_opcode((xs[3] * 10 + xs[4]) as i32),
        digit_to_mode(xs[0] as i32),
        digit_to_mode(xs[1] as i32),
        digit_to_mode(xs[2] as i32),
    )
}

fn run_program(program_in: &[i32], inputs: &[i32]) -> Vec<i32> {
    let mut program: Vec<i32> = program_in.to_owned();

    let mut res: Vec<i32> = Vec::new();

    let mut pc = 0;

    let mut input_counter = 0;

    while pc < program.len() {
        let opcode = program[pc];

        // println!("pc {:?} opcode {:?}", pc, opcode);

        let (op, mode3, mode2, mode1) = parse_op(opcode as usize);

        let val1 = program[pc + 1];
        let val2 = program[pc + 2];
        let val3 = program[pc + 3];

        // println!(
        //     "op: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, val1: {:?}, val2: {:?}, val3: {:?}",
        //     op, mode1, mode2, mode3, val1, val2, val3
        // );

        match op {
            Opcode::Addition => {
                let a;
                if mode1 == Mode::Position {
                    a = program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                program[val3 as usize] = a + b;

                pc += 4;
            }
            Opcode::Multiplication => {
                let a;
                if mode1 == Mode::Position {
                    a = program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                program[val3 as usize] = a * b;

                pc += 4;
            }
            Opcode::Input => {
                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }
                program[val1 as usize] = inputs[input_counter];
                input_counter += 1;

                pc += 2;
            }
            Opcode::Output => {
                if mode1 == Mode::Position {
                    res.push(program[val1 as usize])
                } else {
                    res.push(val1)
                }
                pc += 2;
            }
            Opcode::JumpIfTrue => {
                let a;
                if mode1 == Mode::Position {
                    a = program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = program[val2 as usize];
                } else {
                    b = val2;
                }

                if a != 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let a;
                if mode1 == Mode::Position {
                    a = program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = program[val2 as usize];
                } else {
                    b = val2;
                }

                if a == 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            Opcode::LessThan => {
                let a;
                if mode1 == Mode::Position {
                    a = program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                if a < b {
                    program[val3 as usize] = 1;
                } else {
                    program[val3 as usize] = 0;
                }

                pc += 4;
            }
            Opcode::Equals => {
                let a;
                if mode1 == Mode::Position {
                    a = program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                if a == b {
                    program[val3 as usize] = 1;
                } else {
                    program[val3 as usize] = 0;
                }

                pc += 4;
            }
            Opcode::Break => {
                break;
            }
        }
    }

    return res;
}

fn parse_program(input: &str) -> Vec<i32> {
    let mut program: Vec<i32> = input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    program.push(0);
    program.push(0);
    program.push(0);

    program
}

fn find_optimal_config(phases_left: &[i32], program: &[i32], input_signal: i32) -> (Vec<i32>, i32) {
    let mut res = (vec![], input_signal);

    for phase in phases_left {
        let signals = run_program(program, &vec![*phase, input_signal]);
        let signal = signals[0];

        let mut child_phases = Vec::new();
        for p in phases_left {
            if p != phase {
                child_phases.push(*p);
            }
        }

        let mut cr = find_optimal_config(&child_phases, program, signal);

        cr.0.reverse();
        cr.0.push(*phase);
        cr.0.reverse();

        if res.0.len() == 0 {
            res = cr.clone();
        }
        if res.1 < cr.1 {
            res = cr;
        }
    }

    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let program = parse_program(&input);
    let optimal_res = find_optimal_config(&vec![0, 1, 2, 3, 4], &program, 0);
    println!("res {:?}", optimal_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
            let program = parse_program(&input);
            let expected_result = 999;

            let result = run_program(&program, &vec![7]);
            assert_eq!(expected_result, result[0]);
        }

        {
            let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
            let program = parse_program(&input);
            let expected_result = 1000;

            let result = run_program(&program, &vec![8]);
            assert_eq!(expected_result, result[0]);
        }

        {
            let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
            let program = parse_program(&input);
            let expected_result = 1001;

            let result = run_program(&program, &vec![9]);
            assert_eq!(expected_result, result[0]);
        }

        {
            let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
            let program = parse_program(&input);

            let optimal_res = find_optimal_config(&vec![0, 1, 2, 3, 4], &program, 0);

            println!("{:?}", optimal_res);

            let expected_result = 43210;
            assert_eq!(expected_result, optimal_res.1);
        }

        {
            let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
            let program = parse_program(&input);

            let optimal_res = find_optimal_config(&vec![0, 1, 2, 3, 4], &program, 0);

            println!("{:?}", optimal_res);

            let expected_result = 54321;
            assert_eq!(expected_result, optimal_res.1);
        }

        {
            let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
            let program = parse_program(&input);

            let optimal_res = find_optimal_config(&vec![0, 1, 2, 3, 4], &program, 0);

            println!("{:?}", optimal_res);

            let expected_result = 65210;
            assert_eq!(expected_result, optimal_res.1);
        }
    }
}
