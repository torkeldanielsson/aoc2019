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
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
enum Material {
    Empty,
    Scaffold,
    Robot,
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    program: ProgramState,
    robot_pos: Vec2,
    robot_dir: Direction,
    map: Vec<Vec<Material>>,
}

#[derive(Debug, PartialEq, Clone)]
enum MoveType {
    Move,
    TurnLeft,
    TurnRight,
}

#[derive(Debug, PartialEq, Clone)]
struct Movement {
    move_type: MoveType,
    dist: i32,
}

fn print_map(map: &Vec<Vec<Material>>, robot_pos: &Vec2) {
    for y in 0..map.len() {
        for x in 0..map.last().unwrap().len() {
            if robot_pos.x == x as i32 && robot_pos.y == y as i32 {
                print!("O");
            } else {
                print!(
                    "{}",
                    match map[y][x] {
                        Material::Empty => ' ',
                        Material::Scaffold => '#',
                        Material::Robot => 'O',
                    }
                );
            }
        }
        println!("");
    }
}

fn get_move_str(movements: &Vec<Movement>) -> String {
    let mut move_str = String::new();

    for m in movements {
        if m.move_type == MoveType::Move {
            move_str = format!("{}{}", move_str, m.dist);
        } else {
            move_str = format!(
                "{}{}",
                move_str,
                match m.move_type {
                    MoveType::TurnLeft => "L",
                    MoveType::TurnRight => "R",
                    _ => panic!("cant happen"),
                }
            );
        }
        move_str = format!("{},", move_str);
    }
    move_str
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = fs::read_to_string(filename).expect("error reading file");

    let mut state = State {
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
        robot_dir: Direction::Up,
        map: Vec::new(),
    };

    while state.program.return_state == ReturnState::ProducedOutput {
        run_program(&mut state.program);
    }

    let output_as_string = state
        .program
        .outputs
        .iter()
        .map(|n| ((*n as u8) as char))
        .collect::<String>();
    let output_lines: Vec<String> = output_as_string
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect();

    for line in output_lines {
        let mut arr: Vec<Material> = Vec::new();

        arr.push(Material::Empty);

        for c in line.chars() {
            arr.push(match c {
                '.' => Material::Empty,
                '#' => Material::Scaffold,
                _ => Material::Robot,
            });
        }

        if state.map.len() == 0 {
            state.map.push(vec![Material::Empty; arr.len()]);
        }

        state.map.push(arr);
    }
    state
        .map
        .push(vec![Material::Empty; state.map.last().unwrap().len()]);

    {
        let mut robot_count = 0;
        for y in 0..state.map.len() {
            for x in 0..state.map.last().unwrap().len() {
                if state.map[y][x] == Material::Robot {
                    robot_count += 1;
                    state.robot_pos.x = x as i32;
                    state.robot_pos.y = y as i32;
                    state.map[y][x] = Material::Scaffold;
                }
            }
        }
        if robot_count != 1 {
            panic!("unexpected");
        }
    }

    print_map(&state.map, &state.robot_pos);

    {
        let mut p1_res = 0;
        for y in 1..state.map.len() - 1 {
            for x in 1..state.map.last().unwrap().len() - 1 {
                if state.map[y][x] == Material::Scaffold
                    && state.map[y + 1][x] == Material::Scaffold
                    && state.map[y][x + 1] == Material::Scaffold
                    && state.map[y - 1][x] == Material::Scaffold
                    && state.map[y][x - 1] == Material::Scaffold
                {
                    p1_res += x * y;
                }
            }
        }
        println!("p1: {}", p1_res);
    }

    let mut movements: Vec<Movement> = Vec::new();
    {
        movements.push(Movement {
            move_type: MoveType::TurnLeft,
            dist: 0,
        });
        state.robot_dir = Direction::Left;

        let mut done = false;
        while !done {
            let mut accumulated_moves = 0;
            let (dx, dy) = match state.robot_dir {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            while state.map[(state.robot_pos.y + dy) as usize][(state.robot_pos.x + dx) as usize]
                == Material::Scaffold
            {
                accumulated_moves += 1;
                state.robot_pos.x += dx;
                state.robot_pos.y += dy;

                println!(
                    "state.robot_pos: {:?}, {}, {}, {}, {}",
                    state.robot_pos,
                    (state.robot_pos.y + dy),
                    (state.robot_pos.y + dy) as usize,
                    (state.robot_pos.x + dx),
                    (state.robot_pos.x + dx) as usize
                );
                print_map(&state.map, &state.robot_pos);
            }
            movements.push(Movement {
                move_type: MoveType::Move,
                dist: accumulated_moves,
            });

            let up =
                state.map[(state.robot_pos.y - 1) as usize][(state.robot_pos.x) as usize].clone();
            let down =
                state.map[(state.robot_pos.y + 1) as usize][(state.robot_pos.x) as usize].clone();
            let left =
                state.map[(state.robot_pos.y) as usize][(state.robot_pos.x - 1) as usize].clone();
            let right =
                state.map[(state.robot_pos.y) as usize][(state.robot_pos.x + 1) as usize].clone();

            match (&up, &down, &left, &right) {
                (Material::Scaffold, Material::Empty, Material::Empty, Material::Empty)
                | (Material::Empty, Material::Scaffold, Material::Empty, Material::Empty)
                | (Material::Empty, Material::Empty, Material::Scaffold, Material::Empty)
                | (Material::Empty, Material::Empty, Material::Empty, Material::Scaffold) => {
                    done = true;
                }
                _ => {}
            }

            if !done {
                match (&state.robot_dir, &up, &down, &left, &right) {
                    (
                        Direction::Up,
                        Material::Empty,
                        Material::Scaffold,
                        Material::Scaffold,
                        Material::Empty,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnLeft,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Left;
                    }
                    (
                        Direction::Up,
                        Material::Empty,
                        Material::Scaffold,
                        Material::Empty,
                        Material::Scaffold,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnRight,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Right;
                    }
                    (
                        Direction::Down,
                        Material::Scaffold,
                        Material::Empty,
                        Material::Scaffold,
                        Material::Empty,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnRight,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Left;
                    }
                    (
                        Direction::Down,
                        Material::Scaffold,
                        Material::Empty,
                        Material::Empty,
                        Material::Scaffold,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnLeft,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Right;
                    }
                    (
                        Direction::Left,
                        Material::Scaffold,
                        Material::Empty,
                        Material::Empty,
                        Material::Scaffold,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnRight,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Up;
                    }
                    (
                        Direction::Left,
                        Material::Empty,
                        Material::Scaffold,
                        Material::Empty,
                        Material::Scaffold,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnLeft,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Down;
                    }
                    (
                        Direction::Right,
                        Material::Scaffold,
                        Material::Empty,
                        Material::Scaffold,
                        Material::Empty,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnLeft,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Up;
                    }
                    (
                        Direction::Right,
                        Material::Empty,
                        Material::Scaffold,
                        Material::Scaffold,
                        Material::Empty,
                    ) => {
                        movements.push(Movement {
                            move_type: MoveType::TurnRight,
                            dist: 0,
                        });
                        state.robot_dir = Direction::Down;
                    }
                    _ => {}
                }
            }
        }

        for m in &movements {
            if m.move_type == MoveType::Move {
                print!("{},", m.dist);
            } else {
                print!(
                    "{},",
                    match m.move_type {
                        MoveType::TurnLeft => "L",
                        MoveType::TurnRight => "R",
                        _ => panic!("cant happen"),
                    }
                );
            }
        }
        println!("");
    }

    {
        let filename_2 = &args[2];

        let input_2 = fs::read_to_string(filename_2).expect("error reading file");

        state.program = ProgramState {
            program: parse_program(&input_2),
            return_state: ReturnState::ProducedOutput,
            inputs: vec![],
            outputs: vec![],
            pc: 0,
            input_counter: 0,
            relative_base: 0,
        };

        let mut movements_res_order = String::new();
        let mut movements_res_a: Vec<Movement> = Vec::new();
        let mut movements_res_b: Vec<Movement> = Vec::new();
        let mut movements_res_c: Vec<Movement> = Vec::new();

        'outer: for a_len in 2..10 {
            let mut movements_tmp_a = movements.clone();

            let movement_a: Vec<Movement> = movements_tmp_a[0..a_len].to_owned();
            movements_tmp_a = movements_tmp_a[a_len..].to_owned();

            let mut order_str_a = "A".to_string();
            {
                let mut found_match = true;
                while found_match {
                    found_match = false;

                    if movements_tmp_a.len() >= a_len {
                        let mut got_match = true;
                        for i in 0..a_len {
                            if movements_tmp_a[i] != movement_a[i] {
                                got_match = false;
                            }
                        }
                        if got_match {
                            found_match = true;
                            order_str_a.push_str(",A");
                            movements_tmp_a = movements_tmp_a[a_len..].to_owned();
                        }
                    }
                }
            }

            for b_len in 2..10 {
                let mut movements_tmp_b = movements_tmp_a.clone();

                let movement_b: Vec<Movement> = movements_tmp_b[0..b_len].to_owned();
                movements_tmp_b = movements_tmp_b[b_len..].to_owned();

                let mut order_str_b = order_str_a.clone();
                order_str_b.push_str(",B");

                {
                    let mut found_match = true;
                    while found_match {
                        found_match = false;

                        if movements_tmp_b.len() >= a_len {
                            let mut got_match = true;
                            for i in 0..a_len {
                                if movements_tmp_b[i] != movement_a[i] {
                                    got_match = false;
                                }
                            }
                            if got_match {
                                found_match = true;
                                order_str_b.push_str(",A");
                                movements_tmp_b = movements_tmp_b[a_len..].to_owned();
                            }
                        }

                        if movements_tmp_b.len() >= b_len {
                            let mut got_match = true;
                            for i in 0..b_len {
                                if movements_tmp_b[i] != movement_b[i] {
                                    got_match = false;
                                }
                            }
                            if got_match {
                                found_match = true;
                                order_str_b.push_str(",B");
                                movements_tmp_b = movements_tmp_b[b_len..].to_owned();
                            }
                        }
                    }
                }

                for c_len in 2..10 {
                    let mut movements_tmp_c = movements_tmp_b.clone();

                    let movement_c: Vec<Movement> = movements_tmp_c[0..c_len].to_owned();
                    movements_tmp_c = movements_tmp_c[c_len..].to_owned();

                    let mut order_str_c = order_str_b.clone();
                    order_str_c.push_str(",C");
                    {
                        let mut found_match = true;
                        while found_match {
                            found_match = false;

                            if movements_tmp_c.len() >= a_len {
                                let mut got_match = true;
                                for i in 0..a_len {
                                    if movements_tmp_c[i] != movement_a[i] {
                                        got_match = false;
                                    }
                                }
                                if got_match {
                                    found_match = true;
                                    order_str_c.push_str(",A");
                                    movements_tmp_c = movements_tmp_c[a_len..].to_owned();
                                }
                            }

                            if movements_tmp_c.len() >= b_len {
                                let mut got_match = true;
                                for i in 0..b_len {
                                    if movements_tmp_c[i] != movement_b[i] {
                                        got_match = false;
                                    }
                                }
                                if got_match {
                                    found_match = true;
                                    order_str_c.push_str(",B");
                                    movements_tmp_c = movements_tmp_c[b_len..].to_owned();
                                }
                            }

                            if movements_tmp_c.len() >= c_len {
                                let mut got_match = true;
                                for i in 0..c_len {
                                    if movements_tmp_c[i] != movement_c[i] {
                                        got_match = false;
                                    }
                                }
                                if got_match {
                                    found_match = true;
                                    order_str_c.push_str(",C");
                                    movements_tmp_c = movements_tmp_c[c_len..].to_owned();
                                }
                            }
                        }
                    }

                    if order_str_c != "A,B,C" {
                        println!(
                            "{}, {} {} {}",
                            order_str_c,
                            get_move_str(&movement_a),
                            get_move_str(&movement_b),
                            get_move_str(&movement_c)
                        );
                    }

                    if movements_tmp_c.len() == 0 {
                        movements_res_order = order_str_c;
                        movements_res_a = movement_a.clone();
                        movements_res_b = movement_b.clone();
                        movements_res_c = movement_c.clone();

                        println!("SUCCESS!");
                        break 'outer;
                    }
                }
            }
        }

        let mut movement_str_a = get_move_str(&movements_res_a);
        let mut movement_str_b = get_move_str(&movements_res_b);
        let mut movement_str_c = get_move_str(&movements_res_c);

        println!("");

        println!("{}", get_move_str(&movements));
        {
            let mut reconstructed_move_str = String::new();
            let somethignasdf: Vec<String> = movements_res_order
                .split(",")
                .map(|s| s.to_string())
                .collect();
            for p in somethignasdf.join("").chars() {
                match p {
                    'A' => reconstructed_move_str.push_str(&movement_str_a),
                    'B' => reconstructed_move_str.push_str(&movement_str_b),
                    'C' => reconstructed_move_str.push_str(&movement_str_c),
                    _ => {}
                }
                reconstructed_move_str.pop();
                reconstructed_move_str.push_str("#");
            }
            println!("{}", reconstructed_move_str);
        }

        movement_str_a.pop();
        movement_str_b.pop();
        movement_str_c.pop();

        movements_res_order.push_str("\n");
        movement_str_a.push_str("\n");
        movement_str_b.push_str("\n");
        movement_str_c.push_str("\n");

        println!(
            "{:?}, {:?} {:?} {:?}",
            movements_res_order, movement_str_a, movement_str_b, movement_str_c
        );

        for n in movements_res_order.chars().map(|c| c as i64) {
            state.program.inputs.push(n);
        }

        for n in movement_str_a.chars().map(|c| c as i64) {
            state.program.inputs.push(n);
        }

        for n in movement_str_b.chars().map(|c| c as i64) {
            state.program.inputs.push(n);
        }

        for n in movement_str_c.chars().map(|c| c as i64) {
            state.program.inputs.push(n);
        }

        for n in "n\n".chars().map(|c| c as i64) {
            state.program.inputs.push(n);
        }

        loop {
            run_program(&mut state.program);

            if state.program.return_state != ReturnState::ProducedOutput {
                break;
            }

            if state.program.outputs.len() > 0 && *state.program.outputs.last().unwrap() == 10 {
                let output_as_string = state
                    .program
                    .outputs
                    .iter()
                    .map(|n| ((*n as u8) as char))
                    .collect::<String>();
                let output_lines: Vec<String> = output_as_string
                    .lines()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_owned())
                    .collect();

                for line in output_lines {
                    println!("{}", line);
                }

                state.program.outputs = vec![];
            }
        }

        println!(
            "last output value: {}",
            state.program.outputs.last().unwrap()
        );
    }
}
