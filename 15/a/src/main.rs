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
    OxygenTank,
}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    pos: Vec2,
    material: Material,
    dist: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    map: Vec<Location>,
    program: ProgramState,
    robot_pos: Vec2,
}

fn print_state(state: &State, dist: i32) {
    let mut max = Vec2 {
        x: std::i32::MIN,
        y: std::i32::MIN,
    };
    let mut min = Vec2 {
        x: std::i32::MAX,
        y: std::i32::MAX,
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

    for p in &state.map {
        mvaddch(
            max.y - p.pos.y,
            p.pos.x - min.x,
            match p.material {
                Material::Empty => ' ' as u64,
                Material::Wall => '#' as u64,
                Material::OxygenTank => '!' as u64,
            },
        );
    }

    mvaddch(
        max.y - state.robot_pos.y,
        state.robot_pos.x - min.x,
        'O' as u64,
    );

    mvaddstr(
        max.y - min.y + 6,
        0,
        &format!("size: {} x {}", max.x - min.x, max.y - min.y),
    );

    mvaddstr(max.y - min.y + 8, 0, &format!("dist: {}", dist));

    refresh();
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn set_material(pos: Vec2, map: &mut Vec<Location>, material: Material) {
    for l in &mut *map {
        if l.pos.x == pos.x && l.pos.y == pos.y {
            l.material = material;
            return;
        }
    }
    map.push(Location {
        pos: pos,
        material: material,
        dist: 0,
    });
}

fn main() {
    let mut count: i64 = 0;

    let mut rng = rand::thread_rng();

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let mut state = State {
        map: Vec::new(),
        program: ProgramState {
            program: parse_program(&input),
            return_state: ReturnState::ProducedOutput,
            inputs: vec![],
            outputs: vec![],
            pc: 0,
            input_counter: 0,
            relative_base: 0,
        },
        robot_pos: Vec2 { x: 0, y: 0 },
    };

    state.map.push(Location {
        pos: Vec2 { x: 0, y: 0 },
        material: Material::Empty,
        dist: 0,
    });

    initscr();
    noecho();

    run_program(&mut state.program);

    while state.program.return_state != ReturnState::Break {
        while state.program.return_state != ReturnState::NeedMoreInput {
            run_program(&mut state.program);
        }

        let dir = match rng.gen_range(0, 4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("unexpected"),
        };

        state.program.inputs.push(match dir {
            Direction::Up => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        });

        run_program(&mut state.program);

        while state.program.return_state != ReturnState::ProducedOutput {
            run_program(&mut state.program);
        }

        let mut dx = 0;
        let mut dy = 0;
        match dir {
            Direction::Up => {
                dy = 1;
            }
            Direction::Down => {
                dy = -1;
            }
            Direction::Left => {
                dx = 1;
            }
            Direction::Right => {
                dx = -1;
            }
        }

        match state.program.outputs.last().unwrap() {
            0 => {
                set_material(
                    Vec2 {
                        x: state.robot_pos.x + dx,
                        y: state.robot_pos.y + dy,
                    },
                    &mut state.map,
                    Material::Wall,
                );
            }
            1 => {
                state.robot_pos.x += dx;
                state.robot_pos.y += dy;
                set_material(
                    Vec2 {
                        x: state.robot_pos.x,
                        y: state.robot_pos.y,
                    },
                    &mut state.map,
                    Material::Empty,
                );
            }
            2 => {
                state.robot_pos.x += dx;
                state.robot_pos.y += dy;
                set_material(
                    Vec2 {
                        x: state.robot_pos.x,
                        y: state.robot_pos.y,
                    },
                    &mut state.map,
                    Material::OxygenTank,
                );
            }
            _ => panic!("unexpected"),
        }

        count += 1;
        if count % 100000 == 0 {
            for loc in &mut state.map {
                loc.dist = -1;
                if loc.pos.x == 0 && loc.pos.y == 0 {
                    loc.dist = 0;
                }
            }

            let mut empty_left = true;

            while empty_left {
                empty_left = false;

                let original_map = state.map.clone();

                for loc in &mut state.map {
                    if loc.material == Material::Empty || loc.material == Material::OxygenTank {
                        for l2 in &original_map {
                            if l2.pos.x == loc.pos.x && l2.pos.y + 1 == loc.pos.y {
                                if l2.dist >= 0 && (loc.dist == -1 || loc.dist > l2.dist + 1) {
                                    loc.dist = l2.dist + 1;
                                    empty_left = true;
                                }
                            }
                            if l2.pos.x == loc.pos.x && l2.pos.y - 1 == loc.pos.y {
                                if l2.dist >= 0 && (loc.dist == -1 || loc.dist > l2.dist + 1) {
                                    loc.dist = l2.dist + 1;
                                    empty_left = true;
                                }
                            }
                            if l2.pos.x + 1 == loc.pos.x && l2.pos.y == loc.pos.y {
                                if l2.dist >= 0 && (loc.dist == -1 || loc.dist > l2.dist + 1) {
                                    loc.dist = l2.dist + 1;
                                    empty_left = true;
                                }
                            }
                            if l2.pos.x - 1 == loc.pos.x && l2.pos.y == loc.pos.y {
                                if l2.dist >= 0 && (loc.dist == -1 || loc.dist > l2.dist + 1) {
                                    loc.dist = l2.dist + 1;
                                    empty_left = true;
                                }
                            }
                        }
                    }
                }
            }
            for loc in &state.map {
                if loc.material == Material::OxygenTank {
                    print_state(&state, loc.dist);
                }
            }
        }
    }

    print_state(&state, 0);

    getch();

    endwin();
}
