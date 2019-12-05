use std::env;
use std::fs;

#[derive(Debug)]
struct Line {
    x: i32,
    y: i32,
    horizontal: bool,
    dist: i32,
    original: String,
}

fn parse_line(line: &str) -> Vec<Line> {
    let segments: Vec<&str> = line
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let mut res: Vec<Line> = vec![];

    let mut x = 0;
    let mut y = 0;

    for segment in segments {
        let ch = segment.chars().next().unwrap();
        let dist = (&segment[1..]).parse::<i32>().unwrap();
        match ch {
            'U' => {
                res.push(Line {
                    x: x,
                    y: y,
                    horizontal: false,
                    dist: dist,
                    original: segment.to_owned(),
                });
                y += dist;
            }
            'D' => {
                res.push(Line {
                    x: x,
                    y: y,
                    horizontal: false,
                    dist: -dist,
                    original: segment.to_owned(),
                });
                y -= dist;
            }
            'L' => {
                res.push(Line {
                    x: x,
                    y: y,
                    horizontal: true,
                    dist: -dist,
                    original: segment.to_owned(),
                });
                x -= dist;
            }
            'R' => {
                res.push(Line {
                    x: x,
                    y: y,
                    horizontal: true,
                    dist: dist,
                    original: segment.to_owned(),
                });
                x += dist;
            }
            _ => {}
        }
    }

    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("error reading file");

    let mut lines = contents.lines();

    let first_lines = parse_line(lines.next().unwrap());
    let second_lines = parse_line(lines.next().unwrap());

    /*
        for line in &first_lines {
            println!("L1: {:?}", line);
        }
        for line in &second_lines {
            println!("L2: {:?}", line);
        }
    */

    let mut min = std::i32::MAX;

    for (i, line1) in first_lines.iter().enumerate() {
        for (j, line2) in second_lines.iter().enumerate() {
            if line1.horizontal != line2.horizontal {
                let horizontal;
                let vertical;

                if line1.horizontal {
                    horizontal = line1;
                    vertical = line2;
                } else {
                    horizontal = line2;
                    vertical = line1;
                }

                let vert_max = std::cmp::max(vertical.y, vertical.y + vertical.dist);
                let vert_min = std::cmp::min(vertical.y, vertical.y + vertical.dist);
                let horiz_max = std::cmp::max(horizontal.x + horizontal.dist, horizontal.x);
                let horiz_min = std::cmp::min(horizontal.x + horizontal.dist, horizontal.x);

                if horizontal.y >= vert_min
                    && horizontal.y <= vert_max
                    && vertical.x >= horiz_min
                    && vertical.x <= horiz_max
                {
                    //println!("L1: {:?}", line1);
                    //println!("L2: {:?}", line2);
                    println!("hit at x: {} y: {}", horizontal.y, vertical.x);

                    if horizontal.y != 0
                        && vertical.x != 0
                        && (horizontal.y.abs() + vertical.x.abs()) < min
                    {
                        min = horizontal.y.abs() + vertical.x.abs();
                    }
                }
            }
        }
    }

    println!("res: {:?}", min);
}
