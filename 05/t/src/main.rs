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
    Break,
}

fn digit_to_opcode(d: i32) -> Opcode {
    match d {
        1 => Opcode::Addition,
        2 => Opcode::Multiplication,
        3 => Opcode::Input,
        4 => Opcode::Output,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut program: Vec<i32> = contents
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    program.push(0);
    program.push(0);
    program.push(0);

    let mut pc = 0;

    let input = 1;

    while pc < program.len() {
        let opcode = program[pc];

        println!("pc {:?} opcode {:?}", pc, opcode);

        let (op, mode3, mode2, mode1) = parse_op(opcode as usize);

        let val1 = program[pc + 1];
        let val2 = program[pc + 2];
        let val3 = program[pc + 3];

        println!(
            "op: {:?}, mode1: {:?}, mode2: {:?}, mode3: {:?}, val1: {:?}, val2: {:?}, val3: {:?}",
            op, mode1, mode2, mode3, val1, val2, val3
        );

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

                if mode3 == Mode::Position {
                    program[val3 as usize] = a + b;
                } else {
                    panic!("Uhm... Immediate mode for addition?");
                }
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

                if mode3 == Mode::Position {
                    program[val3 as usize] = a * b;
                } else {
                    panic!("Uhm... Immediate mode for multiplication?");
                }
                pc += 4;
            }
            Opcode::Input => {
                if mode1 == Mode::Position {
                    program[val1 as usize] = input;
                } else {
                    panic!("Uhm... Immediate mode for input?");
                }
                pc += 2;
            }
            Opcode::Output => {
                if mode1 == Mode::Position {
                    println!("output: {}", program[val1 as usize]);
                } else {
                    println!("output: {}", val1);
                }
                pc += 2;
            }
            Opcode::Break => {
                break;
            }
        }
    }

    for number in program {
        print!("{:?},", number);
    }
    println!("");
}
