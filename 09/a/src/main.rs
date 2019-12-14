use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum ReturnState {
    Error,
    NeedMoreInput,
    ProducedOutput,
    Break,
}

#[derive(Debug)]
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
    while s.pc < s.program.len() {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let mut program = ProgramState {
        program: parse_program(&input),
        return_state: ReturnState::Error,
        inputs: vec![],
        outputs: vec![],
        pc: 0,
        input_counter: 0,
        relative_base: 0,
    };

    run_program(&mut program);

    println!("{:?}", program);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let input = "1,0,0,0,99";
            let mut program = ProgramState {
                program: parse_program(&input),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            let expected_result = [2, 0, 0, 0, 99];
            assert_eq!(
                program.program[0..5]
                    .iter()
                    .zip(&expected_result)
                    .filter(|&(a, b)| a == b)
                    .count(),
                expected_result.len()
            );
        }
        {
            let input = "2,3,0,3,99";
            let mut program = ProgramState {
                program: parse_program(&input),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            let expected_result = [2, 3, 0, 6, 99];
            assert_eq!(
                program.program[0..5]
                    .iter()
                    .zip(&expected_result)
                    .filter(|&(a, b)| a == b)
                    .count(),
                expected_result.len()
            );
        }
        {
            let input = "2,4,4,5,99,0";
            let mut program = ProgramState {
                program: parse_program(&input),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            let expected_result = [2, 4, 4, 5, 99, 9801];
            assert_eq!(
                program.program[0..expected_result.len()]
                    .iter()
                    .zip(&expected_result)
                    .filter(|&(a, b)| a == b)
                    .count(),
                expected_result.len()
            );
        }
        {
            let input_txt = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
            let mut program = ProgramState {
                program: parse_program(&input_txt),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![1],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            assert_eq!(program.outputs[0], 1);
        }
        {
            let input_txt = "3,9,8,9,10,9,4,9,99,-1,8";
            let mut program = ProgramState {
                program: parse_program(&input_txt),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![7, 8, 9],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            run_program(&mut program);

            println!(
                "outputs: {:?}, program counter: {:?}",
                &program.outputs, program.pc
            );

            program.pc = 0;
            program.program = parse_program(&input_txt);
            run_program(&mut program);

            program.pc = 0;
            program.program = parse_program(&input_txt);
            run_program(&mut program);

            let expected_result = [0, 1, 0];
            assert_eq!(
                program
                    .outputs
                    .iter()
                    .zip(&expected_result)
                    .filter(|&(a, b)| a == b)
                    .count(),
                expected_result.len()
            );
        }
        {
            let input_txt = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
            let mut program = ProgramState {
                program: parse_program(&input_txt),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![7, 8, 9],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            run_program(&mut program);

            println!(
                "outputs: {:?}, program counter: {:?}",
                &program.outputs, program.pc
            );

            program.pc = 0;
            program.program = parse_program(&input_txt);
            run_program(&mut program);
            program.pc = 0;
            program.program = parse_program(&input_txt);
            run_program(&mut program);

            println!(
                "outputs: {:?}, program counter: {:?}",
                &program.outputs, program.pc
            );

            let expected_result = [999, 1000, 1001];
            assert_eq!(
                program
                    .outputs
                    .iter()
                    .zip(&expected_result)
                    .filter(|&(a, b)| a == b)
                    .count(),
                expected_result.len()
            );
        }
        {
            let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
            let mut program = ProgramState {
                program: parse_program(&input),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            println!("{:?}", program.outputs);

            let expected_result = [
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ];
            assert_eq!(
                program
                    .outputs
                    .iter()
                    .zip(&expected_result)
                    .filter(|&(a, b)| a == b)
                    .count(),
                expected_result.len()
            );
        }
        {
            let input_txt = "1102,34915192,34915192,7,4,7,99,0";
            let mut program = ProgramState {
                program: parse_program(&input_txt),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![1],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            assert_eq!(program.outputs[0], 1219070632396864);
        }
        {
            let input_txt = "104,1125899906842624,99";
            let mut program = ProgramState {
                program: parse_program(&input_txt),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![1],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            assert_eq!(program.outputs[0], 1125899906842624);
        }
        {
            let input_txt = "1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,1,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,493,1024,1102,1,38,1015,1101,20,0,1011,1101,0,509,1026,1101,0,32,1018,1101,0,333,1022,1102,1,0,1020,1101,326,0,1023,1101,0,33,1010,1101,21,0,1016,1101,25,0,1004,1102,28,1,1008,1102,1,506,1027,1102,488,1,1025,1101,0,27,1013,1101,1,0,1021,1101,0,34,1019,1101,607,0,1028,1102,1,23,1003,1102,26,1,1007,1102,29,1,1009,1101,31,0,1000,1102,37,1,1012,1101,30,0,1005,1101,602,0,1029,1101,36,0,1002,1102,1,22,1001,1102,1,35,1014,1102,24,1,1006,1102,39,1,1017,109,4,21102,40,1,6,1008,1010,40,63,1005,63,203,4,187,1106,0,207,1001,64,1,64,1002,64,2,64,109,13,1206,3,221,4,213,1106,0,225,1001,64,1,64,1002,64,2,64,109,-5,1208,-9,22,63,1005,63,241,1106,0,247,4,231,1001,64,1,64,1002,64,2,64,109,-5,21107,41,40,3,1005,1010,263,1106,0,269,4,253,1001,64,1,64,1002,64,2,64,109,-1,1202,3,1,63,1008,63,29,63,1005,63,295,4,275,1001,64,1,64,1106,0,295,1002,64,2,64,109,16,21108,42,42,-8,1005,1014,313,4,301,1105,1,317,1001,64,1,64,1002,64,2,64,109,-4,2105,1,5,1001,64,1,64,1105,1,335,4,323,1002,64,2,64,109,-5,1207,-4,28,63,1005,63,355,1001,64,1,64,1105,1,357,4,341,1002,64,2,64,109,2,21102,43,1,-1,1008,1014,45,63,1005,63,377,1106,0,383,4,363,1001,64,1,64,1002,64,2,64,109,-10,1208,-3,36,63,1005,63,401,4,389,1106,0,405,1001,64,1,64,1002,64,2,64,109,6,21107,44,45,1,1005,1012,423,4,411,1105,1,427,1001,64,1,64,1002,64,2,64,109,4,21101,45,0,3,1008,1018,45,63,1005,63,453,4,433,1001,64,1,64,1105,1,453,1002,64,2,64,109,-23,2101,0,10,63,1008,63,36,63,1005,63,475,4,459,1106,0,479,1001,64,1,64,1002,64,2,64,109,26,2105,1,6,4,485,1105,1,497,1001,64,1,64,1002,64,2,64,109,4,2106,0,5,1105,1,515,4,503,1001,64,1,64,1002,64,2,64,109,-25,1201,10,0,63,1008,63,26,63,1005,63,537,4,521,1105,1,541,1001,64,1,64,1002,64,2,64,109,18,21101,46,0,-1,1008,1014,43,63,1005,63,565,1001,64,1,64,1106,0,567,4,547,1002,64,2,64,109,-6,1201,-4,0,63,1008,63,33,63,1005,63,587,1105,1,593,4,573,1001,64,1,64,1002,64,2,64,109,22,2106,0,-3,4,599,1105,1,611,1001,64,1,64,1002,64,2,64,109,-28,2102,1,-2,63,1008,63,22,63,1005,63,633,4,617,1105,1,637,1001,64,1,64,1002,64,2,64,109,-1,21108,47,44,9,1005,1011,653,1105,1,659,4,643,1001,64,1,64,1002,64,2,64,109,10,2107,24,-8,63,1005,63,681,4,665,1001,64,1,64,1105,1,681,1002,64,2,64,109,-11,2107,31,4,63,1005,63,697,1106,0,703,4,687,1001,64,1,64,1002,64,2,64,109,8,2101,0,-8,63,1008,63,23,63,1005,63,727,1001,64,1,64,1105,1,729,4,709,1002,64,2,64,109,-16,2108,21,10,63,1005,63,749,1001,64,1,64,1106,0,751,4,735,1002,64,2,64,109,17,2108,36,-8,63,1005,63,769,4,757,1105,1,773,1001,64,1,64,1002,64,2,64,109,-10,1207,1,23,63,1005,63,791,4,779,1105,1,795,1001,64,1,64,1002,64,2,64,109,-3,2102,1,6,63,1008,63,22,63,1005,63,815,1106,0,821,4,801,1001,64,1,64,1002,64,2,64,109,16,1205,7,837,1001,64,1,64,1105,1,839,4,827,1002,64,2,64,109,-5,1202,0,1,63,1008,63,30,63,1005,63,863,1001,64,1,64,1106,0,865,4,845,1002,64,2,64,109,4,1205,9,883,4,871,1001,64,1,64,1106,0,883,1002,64,2,64,109,16,1206,-7,899,1001,64,1,64,1106,0,901,4,889,4,64,99,21102,1,27,1,21101,915,0,0,1105,1,922,21201,1,47633,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,942,1,0,1105,1,922,22102,1,1,-1,21201,-2,-3,1,21101,957,0,0,1106,0,922,22201,1,-1,-2,1105,1,968,22101,0,-2,-2,109,-3,2106,0,0";
            let mut program = ProgramState {
                program: parse_program(&input_txt),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![1],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            assert_eq!(program.outputs[0], 3345854957);
        }
        {
            let input_txt = "1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,1,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,493,1024,1102,1,38,1015,1101,20,0,1011,1101,0,509,1026,1101,0,32,1018,1101,0,333,1022,1102,1,0,1020,1101,326,0,1023,1101,0,33,1010,1101,21,0,1016,1101,25,0,1004,1102,28,1,1008,1102,1,506,1027,1102,488,1,1025,1101,0,27,1013,1101,1,0,1021,1101,0,34,1019,1101,607,0,1028,1102,1,23,1003,1102,26,1,1007,1102,29,1,1009,1101,31,0,1000,1102,37,1,1012,1101,30,0,1005,1101,602,0,1029,1101,36,0,1002,1102,1,22,1001,1102,1,35,1014,1102,24,1,1006,1102,39,1,1017,109,4,21102,40,1,6,1008,1010,40,63,1005,63,203,4,187,1106,0,207,1001,64,1,64,1002,64,2,64,109,13,1206,3,221,4,213,1106,0,225,1001,64,1,64,1002,64,2,64,109,-5,1208,-9,22,63,1005,63,241,1106,0,247,4,231,1001,64,1,64,1002,64,2,64,109,-5,21107,41,40,3,1005,1010,263,1106,0,269,4,253,1001,64,1,64,1002,64,2,64,109,-1,1202,3,1,63,1008,63,29,63,1005,63,295,4,275,1001,64,1,64,1106,0,295,1002,64,2,64,109,16,21108,42,42,-8,1005,1014,313,4,301,1105,1,317,1001,64,1,64,1002,64,2,64,109,-4,2105,1,5,1001,64,1,64,1105,1,335,4,323,1002,64,2,64,109,-5,1207,-4,28,63,1005,63,355,1001,64,1,64,1105,1,357,4,341,1002,64,2,64,109,2,21102,43,1,-1,1008,1014,45,63,1005,63,377,1106,0,383,4,363,1001,64,1,64,1002,64,2,64,109,-10,1208,-3,36,63,1005,63,401,4,389,1106,0,405,1001,64,1,64,1002,64,2,64,109,6,21107,44,45,1,1005,1012,423,4,411,1105,1,427,1001,64,1,64,1002,64,2,64,109,4,21101,45,0,3,1008,1018,45,63,1005,63,453,4,433,1001,64,1,64,1105,1,453,1002,64,2,64,109,-23,2101,0,10,63,1008,63,36,63,1005,63,475,4,459,1106,0,479,1001,64,1,64,1002,64,2,64,109,26,2105,1,6,4,485,1105,1,497,1001,64,1,64,1002,64,2,64,109,4,2106,0,5,1105,1,515,4,503,1001,64,1,64,1002,64,2,64,109,-25,1201,10,0,63,1008,63,26,63,1005,63,537,4,521,1105,1,541,1001,64,1,64,1002,64,2,64,109,18,21101,46,0,-1,1008,1014,43,63,1005,63,565,1001,64,1,64,1106,0,567,4,547,1002,64,2,64,109,-6,1201,-4,0,63,1008,63,33,63,1005,63,587,1105,1,593,4,573,1001,64,1,64,1002,64,2,64,109,22,2106,0,-3,4,599,1105,1,611,1001,64,1,64,1002,64,2,64,109,-28,2102,1,-2,63,1008,63,22,63,1005,63,633,4,617,1105,1,637,1001,64,1,64,1002,64,2,64,109,-1,21108,47,44,9,1005,1011,653,1105,1,659,4,643,1001,64,1,64,1002,64,2,64,109,10,2107,24,-8,63,1005,63,681,4,665,1001,64,1,64,1105,1,681,1002,64,2,64,109,-11,2107,31,4,63,1005,63,697,1106,0,703,4,687,1001,64,1,64,1002,64,2,64,109,8,2101,0,-8,63,1008,63,23,63,1005,63,727,1001,64,1,64,1105,1,729,4,709,1002,64,2,64,109,-16,2108,21,10,63,1005,63,749,1001,64,1,64,1106,0,751,4,735,1002,64,2,64,109,17,2108,36,-8,63,1005,63,769,4,757,1105,1,773,1001,64,1,64,1002,64,2,64,109,-10,1207,1,23,63,1005,63,791,4,779,1105,1,795,1001,64,1,64,1002,64,2,64,109,-3,2102,1,6,63,1008,63,22,63,1005,63,815,1106,0,821,4,801,1001,64,1,64,1002,64,2,64,109,16,1205,7,837,1001,64,1,64,1105,1,839,4,827,1002,64,2,64,109,-5,1202,0,1,63,1008,63,30,63,1005,63,863,1001,64,1,64,1106,0,865,4,845,1002,64,2,64,109,4,1205,9,883,4,871,1001,64,1,64,1106,0,883,1002,64,2,64,109,16,1206,-7,899,1001,64,1,64,1106,0,901,4,889,4,64,99,21102,1,27,1,21101,915,0,0,1105,1,922,21201,1,47633,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,942,1,0,1105,1,922,22102,1,1,-1,21201,-2,-3,1,21101,957,0,0,1106,0,922,22201,1,-1,-2,1105,1,968,22101,0,-2,-2,109,-3,2106,0,0";
            let mut program = ProgramState {
                program: parse_program(&input_txt),
                return_state: ReturnState::ProducedOutput,
                inputs: vec![2],
                outputs: vec![],
                pc: 0,
                input_counter: 0,
                relative_base: 0,
            };

            while program.return_state == ReturnState::ProducedOutput {
                run_program(&mut program);
            }

            assert_eq!(program.outputs[0], 68938);
        }
    }
}
