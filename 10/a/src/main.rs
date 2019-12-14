#[derive(Debug, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn distance_between(a: &Vec2, b: &Vec2) -> f32 {
    (((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)) as f32).sqrt()
}

fn find_best_pos(input_text: &str) -> (Vec2, i32) {
    let mut positions: Vec<Vec2> = Vec::new();
    let mut visible: Vec<i32> = Vec::new();

    let mut y: i32 = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input_text.lines() {
        let mut x: i32 = 0;
        let line_vec: Vec<char> = line.chars().collect();
        for c in line_vec {
            print!("{}", c);
            if c == '#' {
                positions.push(Vec2 { x, y });
            }
            x += 1;
            if x > max_x {
                max_x = x;
            }
        }
        println!("{:?}", positions);
        y += 1;
        if y > max_y {
            max_y = y;
        }
    }

    let mut best_pos: Vec2 = Vec2 { x: -1, y: -1 };
    let mut best_pos_num_visible: i32 = 0;

    for pos in &positions {
        let mut num_visible = 0;

        for other in &positions {
            if !(pos.x == other.x && pos.y == other.y) {
                let pos_to_other = distance_between(pos, other);
                let mut is_visible = true;

                for occluding in &positions {
                    if !(pos.x == occluding.x && pos.y == occluding.y)
                        && !(occluding.x == other.x && occluding.y == other.y)
                    {
                        // println!(
                        //     "pos: {:?}, occluding: {:?}, other: {:?}, ",
                        //     pos, occluding, other
                        // );

                        let pos_to_occluding = distance_between(pos, occluding);
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
                    num_visible += 1;
                }
            }
        }

        visible.push(num_visible);

        if num_visible > best_pos_num_visible {
            best_pos.x = pos.x;
            best_pos.y = pos.y;
            best_pos_num_visible = num_visible;
        }
    }

    // for y in 0..max_y {
    //     for x in 0..max_x {
    //         let mut got_count = false;
    //
    //         for (i, pos) in positions.iter().enumerate() {
    //             if (x as i32) == (pos.x as i32) && (y as i32) == (pos.y as i32) {
    //                 print!("{}", visible[i]);
    //                 got_count = true;
    //             }
    //         }
    //
    //         if !got_count {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    // println!(
    //     "best_pos: {:?}, best_pos_num_visible: {}",
    //     best_pos, best_pos_num_visible
    // );

    (best_pos, best_pos_num_visible)
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
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 1, y: 1 };
            let expected_visible = 2;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
        {
            let input_text = ".#..#\n.....\n#####\n....#\n...##";
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 3, y: 4 };
            let expected_visible = 8;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
        {
            let input_text = ".#..#\n.....\n#####\n....#\n...##";
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 3, y: 4 };
            let expected_visible = 8;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
        {
            let input_text = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 5, y: 8 };
            let expected_visible = 33;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
        {
            let input_text = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.";
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 1, y: 2 };
            let expected_visible = 35;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
        {
            let input_text = ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..";
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 6, y: 3 };
            let expected_visible = 41;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
        {
            let input_text = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 11, y: 13 };
            let expected_visible = 210;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
        {
            let input_text = "..#..###....#####....###........#\n.##.##...#.#.......#......##....#\n#..#..##.#..###...##....#......##\n..####...#..##...####.#.......#.#\n...#.#.....##...#.####.#.###.#..#\n#..#..##.#.#.####.#.###.#.##.....\n#.##...##.....##.#......#.....##.\n.#..##.##.#..#....#...#...#...##.\n.#..#.....###.#..##.###.##.......\n.##...#..#####.#.#......####.....\n..##.#.#.#.###..#...#.#..##.#....\n.....#....#....##.####....#......\n.#..##.#.........#..#......###..#\n#.##....#.#..#.#....#.###...#....\n.##...##..#.#.#...###..#.#.#..###\n.#..##..##...##...#.#.#...#..#.#.\n.#..#..##.##...###.##.#......#...\n...#.....###.....#....#..#....#..\n.#...###..#......#.##.#...#.####.\n....#.##...##.#...#........#.#...\n..#.##....#..#.......##.##.....#.\n.#.#....###.#.#.#.#.#............\n#....####.##....#..###.##.#.#..#.\n......##....#.#.#...#...#..#.....\n...#.#..####.##.#.........###..##\n.......#....#.##.......#.#.###...\n...#..#.#.........#...###......#.\n.#.##.#.#.#.#........#.#.##..#...\n.......#.##.#...........#..#.#...\n.####....##..#..##.#.##.##..##...\n.#.#..###.#..#...#....#.###.#..#.\n............#...#...#.......#.#..\n.........###.#.....#..##..#.##...";
            let (pos, visible) = find_best_pos(input_text);

            let expected_pos = Vec2 { x: 27, y: 19 };
            let expected_visible = 314;
            assert_eq!(pos, expected_pos);
            assert_eq!(visible, expected_visible);
        }
    }
}
