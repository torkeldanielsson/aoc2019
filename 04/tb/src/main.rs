fn split_to_digits(n: usize) -> Vec<usize> {
    fn x_inner(n: usize, xs: &mut Vec<usize>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

fn is_ok(d: usize) -> bool {
    let digits = split_to_digits(d);

    let mut found_double = false;
    let mut no_decreases = true;

    for (i, digit1) in digits.iter().enumerate() {
        if i < digits.len() - 1 && digit1 == &digits[i + 1] {
            let mut found_double_here = true;
            if i < digits.len() - 2 && digit1 == &digits[i + 2] {
                found_double_here = false;
            }
            if i > 0 && digit1 == &digits[i - 1] {
                found_double_here = false;
            }

            if found_double_here {
                found_double = true;
            }
        }
        for digit2 in &digits[(i + 1)..] {
            if digit2 < digit1 {
                no_decreases = false
            }
        }
    }

    return found_double && no_decreases;
}

fn is_ok_test(d: usize) {
    if is_ok(d) {
        println!("ok: {:?}", d);
    } else {
        println!("not ok: {:?}", d);
    }
}

fn main() {
    is_ok_test(112233);
    is_ok_test(122345);
    is_ok_test(111123);
    is_ok_test(135679);
    is_ok_test(135677);
    is_ok_test(133679);
    is_ok_test(134669);
    is_ok_test(223450);
    is_ok_test(123789);
    is_ok_test(123444);
    is_ok_test(111122);

    let mut count = 0;

    for d in 125730..=579381 {
        if is_ok(d) {
            count += 1;
        }
    }

    println!("ok: {:?}", count);
}
