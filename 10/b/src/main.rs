#[derive(Debug, PartialEq, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn distance_between(a: &Vec2, b: &Vec2) -> f32 {
    (((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)) as f32).sqrt()
}

fn parse_map(input_text: &str) -> Vec<Vec2> {
    let mut positions: Vec<Vec2> = Vec::new();

    let mut y: i32 = 0;
    for line in input_text.lines() {
        let mut x: i32 = 0;
        let line_vec: Vec<char> = line.chars().collect();
        for c in line_vec {
            if c == '#' {
                positions.push(Vec2 { x, y });
            }
            x += 1;
        }
        y += 1;
    }

    positions
}

fn zap_asteroids(asteroids: &mut Vec<Vec2>) -> Vec<Vec2> {
    asteroids.to_vec()
}

fn main() {
    println!("test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let input_text = "#..\n.#.\n..#";
            let mut asteroid_map = parse_map(input_text);

            let removed_asteroids = zap_asteroids(&mut asteroid_map);
        }
    }
}
