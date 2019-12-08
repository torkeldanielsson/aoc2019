use std::env;
use std::fs;
use std::io::Write;

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

#[derive(Debug, PartialEq)]
enum ReturnState {
    Error,
    NeedMoreInput,
    ProducedOutput,
    Break,
}

struct ProgramState {
    program: Vec<i32>,
    return_state: ReturnState,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
    pc: usize,
    input_counter: usize,
}

fn run_program(s: &mut ProgramState) {
    while s.pc < s.program.len() {
        let opcode = s.program[s.pc];

        // println!("s.pc {:?} opcode {:?}", s.pc, opcode);

        let (op, mode3, mode2, mode1) = parse_op(opcode as usize);

        let val1 = s.program[s.pc + 1];
        let val2 = s.program[s.pc + 2];
        let val3 = s.program[s.pc + 3];

        // println!(
        //     "op: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, val1: {:?}, val2: {:?}, val3: {:?}",
        //     op, mode1, mode2, mode3, val1, val2, val3
        // );

        match op {
            Opcode::Addition => {
                let a;
                if mode1 == Mode::Position {
                    a = s.program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = s.program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                s.program[val3 as usize] = a + b;

                s.pc += 4;
            }
            Opcode::Multiplication => {
                let a;
                if mode1 == Mode::Position {
                    a = s.program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = s.program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                s.program[val3 as usize] = a * b;

                s.pc += 4;
            }
            Opcode::Input => {
                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                if s.inputs.len() <= s.input_counter {
                    s.return_state = ReturnState::NeedMoreInput;
                    return;
                }

                s.program[val1 as usize] = s.inputs[s.input_counter];
                s.input_counter += 1;

                s.pc += 2;
            }
            Opcode::Output => {
                if mode1 == Mode::Position {
                    s.outputs.push(s.program[val1 as usize])
                } else {
                    s.outputs.push(val1)
                }

                s.pc += 2;

                s.return_state = ReturnState::ProducedOutput;
                return;
            }
            Opcode::JumpIfTrue => {
                let a;
                if mode1 == Mode::Position {
                    a = s.program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = s.program[val2 as usize];
                } else {
                    b = val2;
                }

                if a != 0 {
                    s.pc = b as usize;
                } else {
                    s.pc += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let a;
                if mode1 == Mode::Position {
                    a = s.program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = s.program[val2 as usize];
                } else {
                    b = val2;
                }

                if a == 0 {
                    s.pc = b as usize;
                } else {
                    s.pc += 3;
                }
            }
            Opcode::LessThan => {
                let a;
                if mode1 == Mode::Position {
                    a = s.program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = s.program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                if a < b {
                    s.program[val3 as usize] = 1;
                } else {
                    s.program[val3 as usize] = 0;
                }

                s.pc += 4;
            }
            Opcode::Equals => {
                let a;
                if mode1 == Mode::Position {
                    a = s.program[val1 as usize];
                } else {
                    a = val1;
                }

                let b;
                if mode2 == Mode::Position {
                    b = s.program[val2 as usize];
                } else {
                    b = val2;
                }

                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                if a == b {
                    s.program[val3 as usize] = 1;
                } else {
                    s.program[val3 as usize] = 0;
                }

                s.pc += 4;
            }
            Opcode::Break => {
                s.return_state = ReturnState::Break;
                return;
            }
        }
    }
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

fn find_combinations(phases_left: &[i32]) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    // println!("in: {:?}", phases_left);

    if phases_left.len() == 1 {
        res.push(vec![phases_left[0]]);

        return res;
    }

    for phase in phases_left {
        let mut child_phases = Vec::new();
        for p in phases_left {
            if p != phase {
                child_phases.push(*p);
            }
        }

        // println!("phase: {}, child_phases: {:?}", phase, child_phases);

        let mut cr = find_combinations(&child_phases);

        for c in &mut cr {
            c.reverse();
            c.push(*phase);
            c.reverse();

            res.push((*c).clone());
        }
    }

    return res;
}

fn find_optimal_config(
    phases_input: &[i32],
    program: &[i32],
    input_signal: i32,
) -> (Vec<i32>, i32) {
    let combinations = find_combinations(phases_input);

    let mut res = (vec![], input_signal);

    for combination in &combinations {
        let mut amp_programs: Vec<ProgramState> = Vec::new();

        for i in 0..5 {
            amp_programs.push(ProgramState {
                program: program.to_owned(),
                return_state: ReturnState::Error,
                inputs: vec![combination[i]],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
            });
        }

        let mut last_res = 0;
        let mut done = false;

        while !done {
            for i in 0..5 {
                amp_programs[i].inputs.push(last_res);

                run_program(&mut amp_programs[i]);

                if amp_programs[i].return_state != ReturnState::ProducedOutput {
                    println!("return state: {:?}", amp_programs[i].return_state);
                    done = true;
                }

                last_res = *amp_programs[i].outputs.last().unwrap();

                println!("{}, {}: {:?}", i, last_res, amp_programs[i].return_state);
                std::io::stdout().flush().unwrap();
            }
        }

        println!("combination: {:?}, result: {}", combination, last_res);
        std::io::stdout().flush().unwrap();

        if res.0.len() == 0 {
            res.0 = combination.clone();
            res.1 = last_res
        }
        if res.1 < last_res {
            res.0 = combination.clone();
            res.1 = last_res
        }
    }

    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let program = parse_program(&input);

    let (order, best_val) = find_optimal_config(&vec![5, 6, 7, 8, 9], &program, 0);

    println!("result: {:?}, {:?}", order, best_val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
            let program = parse_program(&input);

            let optimal_res = find_optimal_config(&vec![5, 6, 7, 8, 9], &program, 0);

            println!("{:?}", optimal_res);

            let expected_result = 139629729;
            assert_eq!(expected_result, optimal_res.1);
        }
    }
}
