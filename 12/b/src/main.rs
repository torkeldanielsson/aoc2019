#[derive(Debug, PartialEq, Clone)]
struct State {
    pos: Vec<i32>,
    velocity: Vec<i32>,
}

fn find_repeat_period(input: Vec<i32>) -> i32 {
    let mut previous_states: Vec<State> = Vec::new();
    let mut state = State {
        pos: input,
        velocity: vec![0, 0, 0, 0],
    };

    let mut loop_detected: bool = false;

    let mut t = 0;

    while !loop_detected {
        if state.velocity[0] == 0
            && state.velocity[1] == 0
            && state.velocity[2] == 0
            && state.velocity[3] == 0
        {
            for previous_state in &previous_states {
                if &state == previous_state {
                    loop_detected = true;
                }
            }
            previous_states.push(state.clone());
        }

        for i in 0..4 {
            state.pos[i] += state.velocity[i];
        }
        // println!("t: {}, state: {:?}", t, state);

        for i in 0..4 {
            for j in (i + 1)..4 {
                if state.pos[i] < state.pos[j] {
                    state.velocity[i] += 1;
                    state.velocity[j] -= 1;
                }
                if state.pos[i] > state.pos[j] {
                    state.velocity[i] -= 1;
                    state.velocity[j] += 1;
                }
            }
        }

        t += 1;
    }
    t - 1
}

fn main() {
    // let x_vals = vec![-1, 2, 4, 3];
    // let y_vals = vec![0, -10, -8, 5];
    // let z_vals = vec![2, -7, 8, -1];

    // let x_vals = vec![-8, 5, 2, 9];
    // let y_vals = vec![-10, 5, -7, -8];
    // let z_vals = vec![0, 10, 3, -3];

    let x_vals = vec![-3, -12, -9, 7];
    let y_vals = vec![10, -10, 0, -5];
    let z_vals = vec![-1, -5, 10, -3];

    let x_period = find_repeat_period(x_vals) as i64;
    println!("x period: {}", x_period);

    let y_period = find_repeat_period(y_vals) as i64;
    println!("y period: {}", y_period);

    let z_period = find_repeat_period(z_vals) as i64;
    println!("z period: {}", z_period);

    let mut largest_period = x_period;
    if y_period > largest_period {
        largest_period = y_period;
    }
    if z_period > largest_period {
        largest_period = z_period;
    }

    let mut res: i64 = 1;
    while res % x_period != 0 || res % y_period != 0 || res % z_period != 0 {
        res += largest_period;
    }
    println!("common period: {}", res);
    println!("...or: {}", x_period * y_period * z_period);
}
