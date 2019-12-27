use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Clone)]
struct Loc {
    val: char,
    dist: i32,
}

fn recursive_fill(
    map: &mut Vec<Vec<Loc>>,
    cur_x: usize,
    cur_y: usize,
    keys_and_doors: &Vec<char>,
    lowest_res: i32,
) {
    let offsets: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];

    let d = map[cur_y][cur_x].dist;

    if d > lowest_res {
        return;
    }

    for (dx, dy) in &offsets {
        let iy = ((cur_y as i32) + dy) as usize;
        let ix = ((cur_x as i32) + dx) as usize;

        let d2 = map[iy][ix].dist;

        if (map[iy][ix].val == '.' || keys_and_doors.contains(&map[iy][ix].val))
            && (d2 == -1 || d2 > d + 1)
        {
            map[iy][ix].dist = d + 1;
            recursive_fill(map, ix, iy, keys_and_doors, lowest_res);
        }
    }
}

fn recursive_search(
    mut map: Vec<Vec<Loc>>,
    cur_x: usize,
    cur_y: usize,
    keys_and_doors: Vec<char>,
    steps: i32,
    lowest_res: &mut i32,
) -> i32 {
    let offsets: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];

    let mut all_done = true;
    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if !keys_and_doors.contains(&map[y][x].val)
                && ((map[y][x].val as u8 >= 'a' as u8 && map[y][x].val as u8 <= 'z' as u8)
                    || (map[y][x].val as u8 >= 'A' as u8 && map[y][x].val as u8 <= 'Z' as u8))
            {
                all_done = false;
            }
        }
    }
    if all_done {
        return steps;
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[y][x].dist = -1;
        }
    }

    map[cur_y][cur_x].dist = steps;

    recursive_fill(&mut map, cur_x, cur_y, &keys_and_doors, *lowest_res);

    struct PossiblePath {
        x: usize,
        y: usize,
        keys_and_doors: Vec<char>,
        steps: i32,
    }

    let mut possible_paths: Vec<PossiblePath> = Vec::new();

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if !keys_and_doors.contains(&map[y][x].val)
                && (map[y][x].val as u8 >= 'a' as u8 && map[y][x].val as u8 <= 'z' as u8)
            {
                if map[y][x].dist != -1 {
                    println!("{:?}", keys_and_doors);
                    panic!("unexpected {}", map[y][x].val);
                }

                for (dx, dy) in &offsets {
                    let iy = ((y as i32) + dy) as usize;
                    let ix = ((x as i32) + dx) as usize;
                    let d2 = map[iy][ix].dist;
                    if d2 != -1 && d2 + 1 < *lowest_res {
                        let mut modified_keys_and_doors = keys_and_doors.clone();
                        modified_keys_and_doors.push(map[y][x].val);
                        modified_keys_and_doors.push((map[y][x].val as u8 - 32) as char);

                        possible_paths.push(PossiblePath {
                            x: x,
                            y: y,
                            keys_and_doors: modified_keys_and_doors,
                            steps: d2 + 1,
                        });
                    }
                }
            }
        }
    }

    let mut rng = rand::thread_rng();

    possible_paths.shuffle(&mut rng);

    for possible_path in possible_paths {
        let recursive_res = recursive_search(
            map.clone(),
            possible_path.x,
            possible_path.y,
            possible_path.keys_and_doors,
            possible_path.steps,
            lowest_res,
        );
        if recursive_res < *lowest_res {
            *lowest_res = recursive_res;
            println!("lowest_res: {}", *lowest_res);
        }
    }

    //let mut prstr = String::new();
    //prstr = format!("lowest_res: {}\n", lowest_res);
    //prstr = format!("{}{:?}\n", prstr, keys_and_doors);
    //for (y, ml) in map.iter().enumerate() {
    //    for (x, p) in ml.iter().enumerate() {
    //        if x == cur_x && y == cur_y {
    //            prstr.push_str("@");
    //        } else if keys_and_doors.contains(&p.val) {
    //            prstr.push_str(".");
    //        } else {
    //            prstr = format!("{}{}", prstr, p.val);
    //        }
    //    }
    //    prstr.push_str("\n");
    //}
    //println!("{}", prstr);
    //for (y, ml) in map.iter().enumerate() {
    //    for (x, p) in ml.iter().enumerate() {
    //        print!("{}", p.dist);
    //    }
    //    println!();
    //}

    return *lowest_res;
}

fn main() {
    // let input = "#########\n#b.A.@.a#\n#########";
    // let input = "########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################";
    // let input = "########################\n#...............b.C.D.f#\n#.######################\n#.....@.a.B.c.d.A.e.F.g#\n########################";
    // let input = "#################\n#i.G..c...e..H.p#\n########.########\n#j.A..b...f..D.o#\n########@########\n#k.E..a...g..B.n#\n########.########\n#l.F..d...h..C.m#\n#################";
    // let input = "########################\n#@..............ac.GI.b#\n###d#e#f################\n###A#B#C################\n###g#h#i################\n########################";
    let input = "#################################################################################\n#.#.......#...........#...#............p#.....#.#...#...........#..w............#\n#.#.#.###.#.#.#####.###.#.#########.###.#.###.#.#.#.#.#######.#.#######.#######.#\n#...#.#...#.#.#...#.#...#.........#...#.#.#.#.#...#...#.....#.#.....#...#.......#\n#.###.#.#####.#.#.###.###########.###U#.#.#.#.#.#######.###.#.#####.#.###.#######\n#.#.#.#.......#.#...#...#.......#.#...#.#.#.#.#...#.......#.#...#.#.#...#.#.....#\n#J#.#.#########.###.#.#.#.#######.#.###.#.#.#.#####.#######.#.#.#.#.#.#.###.###.#\n#...#.........#...#.#.#.#.#.....#.#.#...#.#.#.......#.#.....#.#.#.#.#.#...#.#.#.#\n###.#########.###.#.###.#.#.###.#.#.#.###.#.#########.#.#######.#.#.#####.#.#.#.#\n#...#...#...#.#...#.#...#...#...#.#.#...#.....#.....#.#...#...#.#...#.L.#.#.#.#o#\n#.###.###.#.#.#.###.#.#######.#.#.#.#########.#.###.#.###.#.#.#.###.#.#.#.#.#.#.#\n#...#...#.#...#.#.....#...#...#.#.#.E...#...#...#.#.#...#...#.#...#...#.#...#.#.#\n###.###.#.#####.#######.#.#.###.#.#.###.###.#####.#.###.#####.#.#.#####.#.###.#.#\n#.....#.#...#...#...#...#...#...#.#.#...#...#.....#.#...#...#.#.#.#.....#.T..d#.#\n#####.#.###.#.###.#.###.#####.###.###.#.#.#.#.###.#.#.#.###.#.#.#.###.#########.#\n#...#...#...#.....#...#.....#z..#.....#.#.#...#...#.#.#.#...#.#.#...#...........#\n#.#.#####.#.#########.#.###.###########.#.#######.#.#.#.#.#.#.#.###.###########.#\n#.#...#...#...#.#.....#.#...#.........#.#.#.....#.#.#.#...#.#.#.#...#...#...#...#\n#.###.#.#####.#.#.#####.#.###.#.#######.#.#.###.###.#.#####.#.###.###.#.#.#.#.###\n#.#...#.#...#.#.#.#.....#.....#...#.....#.#...#.....#.....#.#...#.#...#...#...#.#\n#.###.#.#.###.#.#.#####X#########.#.#####.###.#######.###.#.###V#.#.#########.#.#\n#.S.#...#...#...#...#.#.#...#...#.#.....#..b#.....#.#...#.#.#...#.#.....#...#...#\n#.#.#####.#.###.###.#.#.#.###.#.#.#####.#.#.#####.#.#.###.#.#####.#######.#.#####\n#.#.#.....#...#...#...#.#.....#.#.....#.#.#.....#.#.#.#...#.....#.........#s....#\n#.#.#.#######I###.#####.#######.#####.#.#.#######.#.#.#.#########.#############.#\n#.#.#.#...#.....#.....#.#...#.......#.#.#.#.....#.#...#.#.......#.#...#.......#.#\n###.#.###.#.#########.#.#.#.#.#####.#.#.#.#.###.#.#####.#.#####.#.#.#.###.###.#.#\n#...#.#...#...#...#...#...#...#...#.#...#.#...#.#.#...#.#...#...#...#.....#m#.#.#\n#.###.#.#####.#.#.#.###########.#.#.#####.###.#.#.#.#.#.###.#.#############.#.#.#\n#...#.#.....#...#.#.#.#.........#.#.#...#...#.#.#...#.#.....#.................#.#\n###.#.###.#.#####.#.#.#.#########.#.#.#.#.#.#.#.#####.#########################.#\n#...#.....#...#.#...#.#.#.#.......#.#.#.#.#...#.....#.....#...................#.#\n#.###########.#.#####.#G#.#.#####.#.#.#.#.#########.#####.#.#################.#.#\n#.........#...#.......#...#.#.....#...#.#.........#.....#.#.#...............#...#\n#.#######.#.###.#########.#.#.#########.#########.#.#####.#.#.###########.#.###.#\n#q#.#.....#.....#.....#..k#r#.#.........#.#.......#.....#...#...#.......#.#...#.#\n#.#.#.###########.###.#.###.###.#########.#.###########.#######.#.#.###.#.#####.#\n#...#.#...#.......#.#.#.#.#...#.#.....#.#.#.#.....#...#.........#.#.#..g#.C.#...#\n###.#.#.#.#.#######.#.#.#.###.#.#.###.#.#.#.#.###.#.#.#############.#.#####.#.###\n#...#...#.......A..c#.......#.....#...........#.....#.......R.......#.....#.....#\n#######################################.@.#######################################\n#.....#.....#...#................f....#.................F...#.........#.........#\n#.###.###.###.#.#.###.###############.#.#.#########.#.#####.#.###.###.#.#######.#\n#.#.......#...#...#...#.....#.......#...#...#.....#.#.#.#...#...#...#.#.Q.#...#.#\n#.#########.#######.###.###.#.#.#######.#####.###.###.#.#.#####.###.#####.#.###.#\n#.........#...#.......#.#.#.#.#.....#...#.....#.......#.#.....#.#...#.......#...#\n#.#######.###.#.#######.#.#.#######.#.###.#############.#####.#.#.###.#######.#.#\n#.Ot#.#.....#.#...#...#.#.#...#.....#...#...#...#.......#.....#.#.......#.#...#.#\n###.#.#.#####.#####.#.#.#.###.#.###.###.#.#.#.#.#####.#.#.#############.#.#.###.#\n#.#.#.#.....#.......#.....#...#.#.#.#...#.#...#.....#.#.#.#..a......#.....#.#...#\n#.#.#.#####.###############.###.#.#.#.###.#########.#.#.#.#####.###.#######.#.###\n#.#.#.....#...........#.....#...#.#.#.#.#.#.......#.#.#.#...#...#.......#...#...#\n#.#.###.#####.#####.###.#####.###.#.#.#.###.#####.#.#.#.###.#.#########.#.#####.#\n#.#.....#.....#.#...#...#.....#...#.#...#...#...#...#.#...#.#.....#.......#.....#\n#.###.###.#####.#.###.###.#####.#.#.###.#.###.#.#####.#.###.#.###K###############\n#.....#...#...#.....#.#.....#...#...#...#.....#.#.....#.#...#.#...#.....#.....N.#\n#.#####.###.###.#####.#####.#.#######.###.#####.###.#####.###.#.#.#.###.#.#####.#\n#.#.....#...#...#.....#.....#.....#.#.#.#.....#...#.....#...#.#.#.#...#...#...#.#\n#.#.#####.#.#.###.#####.#########.#.#.#.#####.###.#.###.###.###.#.###.#####.###.#\n#.#.....#.#...#.#.#.........#.....#.#.#.#...#.#.#.#...#.........#...#.#.....#...#\n#.#####M#.#####.#.#####.###.#####.#.#.#.#.#.#.#.#.###################.#.#.###.###\n#...#...#.#.....#.....#...#.....#.#.#.#.#.#.#.#.#.#.........#..y..H...#.#.....#.#\n###.#.###.#.###.#########.#####.#.#.#.#.#.#.#.#.#.#.#######.###########.#######.#\n#.#.#...#.#h#...#.......#.#.#..x#...#...#.#.#.#.#...#.......#.........#...#.....#\n#.#.###.#.###.#.#.#####.#.#.#.#.#######.#.#.#.#.#####.#####.#.###.###.###.#.###.#\n#...#...#.#...#.#.#...#.#...#.#.#...#.B.#.#.#...#...#.#...#.#...#...#...#...#.#.#\n#.###.###.#.###.#.#.###.###.#.###.#.#.###.#.###.#.#.#.#.#.#.#######.###.#####.#.#\n#.#...#.#.#...#...#.......#.#.....#.#...#.#...#.#.#.#.#.#.#...#.....#...#.....#.#\n###.###.#.#.#.#####.#######.#######.###.#.#####.#.#.#.###.###.#.#######.#.###.#.#\n#...#.#...#.#.#...#.Y...#...#.....#.....#.......#.#.#...#.....#.#.....#.#...#...#\n#.###.#.###.#.#.#.#######.###.###.#######.#########.###.#####.#.#.###.#####.#####\n#.#...D.#v..#.#.#.#.......#...#...#...#.#...#...#.....#.....#.#.#.#.#.....#...#.#\n#.###.###.###.#.#.#.#######.###.#.#.#.#.###.#.#Z#.#########.#.#.#.#.#####.###.#.#\n#...#...#.#...#.#...#...#.....#.#.#.#...#.#...#.#...#.......#...#.#...#.....#...#\n#.#.###.###W#.#.#####.###.#####.#.#.#####.#####.#.#.#.#####.#####.#.#P#.#.#####.#\n#.#...#.....#.#.....#.......#...#.#..l..#.....#.#.#.#.#.....#.....#.#.#.#.#...#i#\n#.###.#######.#####.#.#######.###.#####.#.###.#.#.#.#.#######.#####.###.###.#.#.#\n#...#...#....n#.....#...#..u..#...#.....#...#.#.#.#.#.....#...#.....#...#...#...#\n###.###.#######.#########.#####.###.###.#.#.###.###.#####.#.###.###.#.###.#######\n#.....#..e................#.........#...#.#.............#.......#...#........j..#\n#################################################################################";

    let mut map: Vec<Vec<Loc>> = Vec::new();

    for input_line in input.lines() {
        let line: Vec<Loc> = input_line
            .chars()
            .map(|c| Loc { val: c, dist: -1 })
            .collect();
        map.push(line);
    }

    let mut cur_x = 0;
    let mut cur_y = 0;

    for (y, ml) in map.iter().enumerate() {
        for (x, p) in ml.iter().enumerate() {
            // print!("{}", p.val);
            if p.val == '@' {
                cur_x = x;
                cur_y = y;
            }
        }
        // println!();
    }

    let mut keys_and_doors: Vec<char> = Vec::new();
    keys_and_doors.push('@');

    let mut lowest_res = std::i32::MAX;
    let res = recursive_search(map, cur_x, cur_y, keys_and_doors, 0, &mut lowest_res);

    println!("res: {}", res);
}
