#[derive(Debug, PartialEq, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct VisibleAsteroid {
    pos: Vec2,
    angle: f32,
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

fn zap_asteroids(pos: Vec2, asteroids: &Vec<Vec2>) -> Vec<VisibleAsteroid> {
    let mut visible_asteroids = Vec::new();

    for other in asteroids {
        if !(pos.x == other.x && pos.y == other.y) {
            let pos_to_other = distance_between(&pos, other);
            let mut is_visible = true;

            for occluding in asteroids {
                if !(pos.x == occluding.x && pos.y == occluding.y)
                    && !(occluding.x == other.x && occluding.y == other.y)
                {
                    // println!(
                    //     "pos: {:?}, occluding: {:?}, other: {:?}, ",
                    //     pos, occluding, other
                    // );

                    let pos_to_occluding = distance_between(&pos, occluding);
                    let other_to_occluding = distance_between(other, occluding);

                    if pos_to_occluding < pos_to_other && other_to_occluding < pos_to_other {
                        let angle_pos_to_other =
                            ((pos.y - other.y) as f32).atan2((pos.x - other.x) as f32);
                        let angle_pos_to_occluding =
                            ((pos.y - occluding.y) as f32).atan2((pos.x - occluding.x) as f32);

                        if (angle_pos_to_other - angle_pos_to_occluding).abs() < 0.00001 {
                            // println!(
                            //     "occluded! ({} == {})",
                            //     angle_pos_to_other, angle_pos_to_occluding
                            // );
                            is_visible = false;
                        }
                    }
                }
            }

            if is_visible {
                let x = (other.x - pos.x) as f32;
                let y = -(other.y - pos.y) as f32;
                let atan_angle_pre = y.atan2(x);
                let atan_angle_raw = 0.5 * std::f32::consts::PI - atan_angle_pre;
                let mut atan_angle = atan_angle_raw;
                while atan_angle < 0.0 {
                    atan_angle += 2.0 * std::f32::consts::PI;
                }

                println!("pos: {:?}, other: {:?}, x: {}, y: {}, atan_angle_pre: {}, atan_angle_raw: {}, atan_angle: {}, ",pos,other, x,y,atan_angle_pre,atan_angle_raw,atan_angle);
                visible_asteroids.push(VisibleAsteroid {
                    pos: other.clone(),
                    angle: atan_angle,
                })
            }
        }
    }

    visible_asteroids.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());

    visible_asteroids
}

fn main() {
    let input_text = "..#..###....#####....###........#\n.##.##...#.#.......#......##....#\n#..#..##.#..###...##....#......##\n..####...#..##...####.#.......#.#\n...#.#.....##...#.####.#.###.#..#\n#..#..##.#.#.####.#.###.#.##.....\n#.##...##.....##.#......#.....##.\n.#..##.##.#..#....#...#...#...##.\n.#..#.....###.#..##.###.##.......\n.##...#..#####.#.#......####.....\n..##.#.#.#.###..#...#.#..##.#....\n.....#....#....##.####....#......\n.#..##.#.........#..#......###..#\n#.##....#.#..#.#....#.###...#....\n.##...##..#.#.#...###..#.#.#..###\n.#..##..##...##...#.#.#...#..#.#.\n.#..#..##.##...###.##.#......#...\n...#.....###.....#....#..#....#..\n.#...###..#......#.##.#...#.####.\n....#.##...##.#...#........#.#...\n..#.##....#..#.......##.##.....#.\n.#.#....###.#.#.#.#.#............\n#....####.##....#..###.##.#.#..#.\n......##....#.#.#...#...#..#.....\n...#.#..####.##.#.........###..##\n.......#....#.##.......#.#.###...\n...#..#.#.........#...###......#.\n.#.##.#.#.#.#........#.#.##..#...\n.......#.##.#...........#..#.#...\n.####....##..#..##.#.##.##..##...\n.#.#..###.#..#...#....#.###.#..#.\n............#...#...#.......#.#..\n.........###.#.....#..##..#.##...";
    // let input_text = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
    let mut asteroid_map = parse_map(input_text);

    let mut zap_count = 0;
    loop {
        let zap_list = zap_asteroids(Vec2 { x: 27, y: 19 }, &asteroid_map);
        for z in zap_list {
            asteroid_map.retain(|x| x != &z.pos);
            zap_count += 1;
            println!("{}: {:?}", zap_count, z);
            if zap_count == 200 {
                println!("nr 200: {:?}", z.pos);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        {
            let input_text = "#..\n.#.\n..#";
            let mut asteroid_map = parse_map(input_text);

            let removed_asteroids = zap_asteroids(Vec2 { x: 27, y: 19 }, asteroid_map);
        }
    }
}
