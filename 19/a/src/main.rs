use std::env;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

fn digit_to_mode(d: i32) -> Mode {
    match d {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => {
            panic!("unexpected mode digit: {}", d);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Opcode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
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
        9 => Opcode::AdjustRelativeBase,
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

#[derive(Debug, PartialEq, Clone)]
enum ReturnState {
    NeedMoreInput,
    ProducedOutput,
    Break,
}

#[derive(Debug, PartialEq, Clone)]
struct ProgramState {
    program: Vec<i64>,
    return_state: ReturnState,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    pc: usize,
    input_counter: usize,
    relative_base: i64,
}

fn run_program(s: &mut ProgramState) {
    let mut counter = 0;

    while s.pc < s.program.len() && counter < 1000000 {
        counter += 1;

        let opcode = s.program[s.pc];

        // println!("s.pc {:?} opcode {:?}", s.pc, opcode);

        let (op, mode3, mode2, mode1) = parse_op(opcode as usize);

        let val1 = s.program[s.pc + 1];
        let val2 = s.program[s.pc + 2];
        let val3 = s.program[s.pc + 3];

        // println!(
        //     "op: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, val1: {:?}, val2: {:?}, val3: {:?}, a: {}, b: {}",
        //     op, mode1, mode2, mode3, val1, val2, val3, a, b
        // );

        match op {
            Opcode::Addition => {
                if mode3 == Mode::Immediate {
                    panic!("illegal immediate mode");
                }

                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };

                let b = match mode2 {
                    Mode::Position => s.program[val2 as usize],
                    Mode::Immediate => val2,
                    Mode::Relative => s.program[(s.relative_base + val2) as usize],
                };

                match mode3 {
                    Mode::Position => s.program[val3 as usize] = a + b,
                    Mode::Immediate => panic!("illegal immediate mode"),
                    Mode::Relative => s.program[(s.relative_base + val3) as usize] = a + b,
                };

                s.pc += 4;
            }
            Opcode::Multiplication => {
                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };

                let b = match mode2 {
                    Mode::Position => s.program[val2 as usize],
                    Mode::Immediate => val2,
                    Mode::Relative => s.program[(s.relative_base + val2) as usize],
                };

                match mode3 {
                    Mode::Position => s.program[val3 as usize] = a * b,
                    Mode::Immediate => panic!("illegal immediate mode"),
                    Mode::Relative => s.program[(s.relative_base + val3) as usize] = a * b,
                };

                s.pc += 4;
            }
            Opcode::Input => {
                if s.inputs.len() <= s.input_counter {
                    s.return_state = ReturnState::NeedMoreInput;
                    return;
                }

                match mode1 {
                    Mode::Position => s.program[val1 as usize] = s.inputs[s.input_counter],
                    Mode::Immediate => panic!("illegal immediate mode"),
                    Mode::Relative => {
                        s.program[(s.relative_base + val1) as usize] = s.inputs[s.input_counter]
                    }
                };

                s.input_counter += 1;

                s.pc += 2;
            }
            Opcode::Output => {
                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };
                s.outputs.push(a);

                s.pc += 2;

                s.return_state = ReturnState::ProducedOutput;
                return;
            }
            Opcode::JumpIfTrue => {
                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };

                let b = match mode2 {
                    Mode::Position => s.program[val2 as usize],
                    Mode::Immediate => val2,
                    Mode::Relative => s.program[(s.relative_base + val2) as usize],
                };

                if a != 0 {
                    s.pc = b as usize;
                } else {
                    s.pc += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };

                let b = match mode2 {
                    Mode::Position => s.program[val2 as usize],
                    Mode::Immediate => val2,
                    Mode::Relative => s.program[(s.relative_base + val2) as usize],
                };

                if a == 0 {
                    s.pc = b as usize;
                } else {
                    s.pc += 3;
                }
            }
            Opcode::LessThan => {
                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };

                let b = match mode2 {
                    Mode::Position => s.program[val2 as usize],
                    Mode::Immediate => val2,
                    Mode::Relative => s.program[(s.relative_base + val2) as usize],
                };

                let c_index = match mode3 {
                    Mode::Position => val3 as usize,
                    Mode::Immediate => panic!("illegal immediate mode"),
                    Mode::Relative => (s.relative_base + val3) as usize,
                };

                if a < b {
                    s.program[c_index] = 1;
                } else {
                    s.program[c_index] = 0;
                }

                s.pc += 4;
            }
            Opcode::Equals => {
                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };

                let b = match mode2 {
                    Mode::Position => s.program[val2 as usize],
                    Mode::Immediate => val2,
                    Mode::Relative => s.program[(s.relative_base + val2) as usize],
                };

                let c_index = match mode3 {
                    Mode::Position => val3 as usize,
                    Mode::Immediate => panic!("illegal immediate mode"),
                    Mode::Relative => (s.relative_base + val3) as usize,
                };

                if a == b {
                    s.program[c_index] = 1;
                } else {
                    s.program[c_index] = 0;
                }

                s.pc += 4;
            }
            Opcode::AdjustRelativeBase => {
                let a = match mode1 {
                    Mode::Position => s.program[val1 as usize],
                    Mode::Immediate => val1,
                    Mode::Relative => s.program[(s.relative_base + val1) as usize],
                };

                s.relative_base += a;

                s.pc += 2;
            }
            Opcode::Break => {
                s.return_state = ReturnState::Break;
                return;
            }
        }
    }

    if counter >= 1000000 {
        s.return_state = ReturnState::Break;
    }
}

fn parse_program(input: &str) -> Vec<i64> {
    let mut program: Vec<i64> = input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    program.resize(1000000, 0);

    program
}

#[derive(Debug, PartialEq, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let mut map = Vec::new();

    let mut res = 0;

    for y in 0..50 {
        let mut map_row = Vec::new();
        for x in 0..50 {
            let mut program = ProgramState {
                program: parse_program(&input),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            program.inputs.push(x);
            program.inputs.push(y);

            run_program(&mut program);

            if program.return_state == ReturnState::ProducedOutput {
                match program.outputs.last().unwrap() {
                    0 => {
                        map_row.push('.');
                    }
                    1 => {
                        map_row.push('#');
                        res += 1;
                    }
                    _ => {
                        panic!("unexpected");
                    }
                }
            } else {
                panic!("unexpected ({:?})", program.return_state);
            }
        }
        map.push(map_row);
    }

    for y in 0..50 {
        for x in 0..50 {
            print!("{}", map[y][x]);
        }
        println!();
    }

    println!("res: {}", res);
}
