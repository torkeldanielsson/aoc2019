#[derive(Debug, PartialEq, Clone)]
struct Lookup {
    index: usize,
    multiplier: i32,
}

fn main() {
    // let input = "80871224585914546619083218645595";
    // let input = "19617804207202209144916044189917";
    // let input = "69317163492948606335995924319873";
    let input = "59704176224151213770484189932636989396016853707543672704688031159981571127975101449262562108536062222616286393177775420275833561490214618092338108958319534766917790598728831388012618201701341130599267905059417956666371111749252733037090364984971914108277005170417001289652084308389839318318592713462923155468396822247189750655575623017333088246364350280299985979331660143758996484413769438651303748536351772868104792161361952505811489060546839032499706132682563962136170941039904873411038529684473891392104152677551989278815089949043159200373061921992851799948057507078358356630228490883482290389217471790233756775862302710944760078623023456856105493";

    let mut numbers: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();

    let mut lookup_tables: Vec<Vec<Lookup>> = Vec::new();

    for position in 1..=numbers.len() {
        let mut index: i32 = -1;

        let mut lookups: Vec<Lookup> = Vec::new();

        for i in 0..(numbers.len() + 1) {
            if index >= 0 {
                let z = i % (4 * position);
                let phase = z / position;
                match phase {
                    1 => {
                        lookups.push(Lookup {
                            index: index as usize,
                            multiplier: 1,
                        });
                    }
                    3 => {
                        lookups.push(Lookup {
                            index: index as usize,
                            multiplier: -1,
                        });
                    }
                    _ => {}
                }
            }
            index += 1;
        }
        lookup_tables.push(lookups);
    }

    // for (i, lookups) in lookup_tables.iter().enumerate() {
    //     println!("gen {}: {:?}", i, lookups);
    // }

    for _ in 0..100 {
        let old_numbers = numbers;
        numbers = Vec::new();

        for position in 0..old_numbers.len() {
            let mut v = 0;

            for lookup in &lookup_tables[position] {
                v += lookup.multiplier * old_numbers[lookup.index];
            }

            numbers.push(v.abs() % 10);
        }
    }

    {
        for i in 0..8 {
            print!("{}", numbers[i]);
        }
        println!("");
    }
}
