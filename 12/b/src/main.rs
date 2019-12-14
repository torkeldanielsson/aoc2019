use std::collections::HashSet;

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

fn get_prime_factors(n_in: i64) -> Vec<i64> {
    let mut n = n_in;
    let mut res: Vec<i64> = Vec::new();

    while n % 2 == 0 {
        res.push(2);
        n = n / 2;
    }

    for i in 3..=((n as f64).sqrt() as i64) {
        // While i divides n, print i and divide n
        while n % i == 0 {
            res.push(i);
            n = n / i;
        }
    }

    if n > 2 {
        res.push(n);
    }
    res
}

fn count_occurrence(p: &i64, v: &Vec<i64>) -> i64 {
    let mut res = 0;
    for pv in v {
        if p == pv {
            res += 1;
        }
    }
    res
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
    //println!("x period: {}", x_period);

    let y_period = find_repeat_period(y_vals) as i64;
    //println!("y period: {}", y_period);

    let z_period = find_repeat_period(z_vals) as i64;
    //println!("z period: {}", z_period);

    let x_factors = get_prime_factors(x_period);
    let y_factors = get_prime_factors(y_period);
    let z_factors = get_prime_factors(z_period);

    let mut primes = HashSet::new();
    for p in &x_factors {
        primes.insert(p);
    }
    for p in &y_factors {
        primes.insert(p);
    }
    for p in &z_factors {
        primes.insert(p);
    }

    let mut res = 1;

    for p in primes {
        let mut max_count = 1;

        if count_occurrence(&p, &x_factors) > max_count {
            max_count = count_occurrence(&p, &x_factors);
        }
        if count_occurrence(&p, &y_factors) > max_count {
            max_count = count_occurrence(&p, &y_factors);
        }
        if count_occurrence(&p, &z_factors) > max_count {
            max_count = count_occurrence(&p, &z_factors);
        }
        for _ in 0..max_count {
            res *= p;
        }
    }
    println!("res: {}", res);
}
