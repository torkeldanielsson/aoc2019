use ncurses::*;
use rand::Rng;
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

#[derive(Debug, PartialEq, Clone)]
enum Material {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    pos: Vec2,
    material: Material,
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    map: Vec<Location>,
    score: i32,
    program: ProgramState,
    ball_x: i32,
    paddle_x: i32,
    ball_y: i32,
    old_ball_y: i32,
}

fn print_state(state: &State, vec_len: usize, counter: i64) {
    let mut max = Vec2 {
        x: -999999,
        y: -9999999,
    };
    let mut min = Vec2 {
        x: 999999,
        y: 9999999,
    };
    for pi in &state.map {
        if max.x < pi.pos.x {
            max.x = pi.pos.x;
        }
        if max.y < pi.pos.y {
            max.y = pi.pos.y;
        }
        if min.x > pi.pos.x {
            min.x = pi.pos.x;
        }
        if min.y > pi.pos.y {
            min.y = pi.pos.y;
        }
    }

    clear();
    for pi in &state.map {
        mvaddch(
            pi.pos.y,
            pi.pos.x,
            match pi.material {
                Material::Empty => ' ' as u64,
                Material::Wall => '#' as u64,
                Material::Block => '*' as u64,
                Material::Paddle => '_' as u64,
                Material::Ball => 'o' as u64,
            },
        );
    }

    mvaddstr(max.y + 2, max.x, &format!("{}", state.score));
    mvaddstr(max.y + 4, max.x, &format!("{}", vec_len));
    mvaddstr(max.y + 6, max.x, &format!("{}", counter));
    refresh();
}

fn process_output(state: &mut State) -> bool {
    let mut bounce_detected = false;

    let x = state.program.outputs[state.program.outputs.len() - 3];
    let y = state.program.outputs[state.program.outputs.len() - 2];

    if x == -1 && y == 0 {
        state.score = state.program.outputs[state.program.outputs.len() - 1] as i32;
    } else {
        let material = match state.program.outputs[state.program.outputs.len() - 1] {
            0 => Material::Empty,
            1 => Material::Wall,
            2 => Material::Block,
            3 => Material::Paddle,
            4 => Material::Ball,
            _ => panic!("unexpected"),
        };

        if material == Material::Ball {
            if state.old_ball_y < state.ball_y && (y as i32) < state.ball_y {
                bounce_detected = true;
            }

            state.ball_x = x as i32;
            state.old_ball_y = state.ball_y;
            state.ball_y = y as i32;
        }

        if material == Material::Paddle {
            state.paddle_x = x as i32;
        }

        let mut found = false;
        for mat in &mut state.map {
            if mat.pos.x == x as i32 && mat.pos.y == y as i32 {
                mat.material = material.clone();
                found = true;
            }
        }
        if !found {
            state.map.push(Location {
                pos: Vec2 {
                    x: x as i32,
                    y: y as i32,
                },
                material: material,
            });
        }
    }

    return bounce_detected;
}

fn main() {
    let mut counter = 0;

    let mut rng = rand::thread_rng();

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let mut state = State {
        map: Vec::new(),
        score: 0,
        program: ProgramState {
            program: parse_program(&input),
            return_state: ReturnState::ProducedOutput,
            inputs: vec![],
            outputs: vec![],
            pc: 0,
            input_counter: 0,
            relative_base: 0,
        },
        ball_x: 0,
        paddle_x: 0,
        ball_y: 0,
        old_ball_y: 0,
    };

    initscr();
    noecho();

    let mut old_states: Vec<State> = Vec::new();

    while state.program.return_state != ReturnState::Break
        && state.program.return_state != ReturnState::NeedMoreInput
    {
        run_program(&mut state.program);

        if state.program.return_state == ReturnState::ProducedOutput
            && state.program.outputs.len() % 3 == 0
            && state.program.outputs.len() != 0
        {
            process_output(&mut state);
        }
    }

    print_state(&state, old_states.len(), 0);

    old_states.push(state.clone());

    loop {
        while state.program.return_state != ReturnState::Break {
            run_program(&mut state.program);

            if state.program.return_state == ReturnState::NeedMoreInput {
                let r = rng.gen_range(0, 20);
                if r < 10 {
                    state.program.inputs.push(rng.gen_range(-1, 2));
                } else if r < 13 {
                    state.program.inputs.push(-1);
                    state.program.inputs.push(-1);
                    state.program.inputs.push(-1);
                    state.program.inputs.push(-1);
                } else if r < 16 {
                    state.program.inputs.push(1);
                    state.program.inputs.push(1);
                    state.program.inputs.push(1);
                    state.program.inputs.push(1);
                } else {
                    if state.ball_x < state.paddle_x {
                        state.program.inputs.push(-1);
                        state.program.inputs.push(-1);
                        state.program.inputs.push(-1);
                        state.program.inputs.push(-1);
                        state.program.inputs.push(-1);
                    } else {
                        state.program.inputs.push(1);
                        state.program.inputs.push(1);
                        state.program.inputs.push(1);
                        state.program.inputs.push(1);
                        state.program.inputs.push(1);
                    }
                }
            }

            if state.program.return_state == ReturnState::ProducedOutput
                && state.program.outputs.len() % 3 == 0
                && state.program.outputs.len() != 0
            {
                if process_output(&mut state) {
                    print_state(&state, old_states.len(), counter);
                    old_states.push(state.clone());
                    break;
                }
            }

            if counter % 10000 == 0 {
                print_state(&state, old_states.len(), counter);
            }
            counter += 1;
        }

        if state.program.return_state == ReturnState::Break {
            state = old_states.last().unwrap().clone();

            if state.score < 18000 {
                if old_states.len() > 1 && rng.gen_range(-15, 2) > 0 {
                    old_states.pop();
                }
                if old_states.len() > 20 && rng.gen_range(-500, 2) > 0 {
                    old_states.pop();
                    old_states.pop();
                    old_states.pop();
                    old_states.pop();
                    old_states.pop();
                    old_states.pop();
                }
            }

            state = old_states.last().unwrap().clone();
        }
    }

    print_state(&state, old_states.len(), 0);

    getch();

    endwin();
}
