#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut moons: Vec<Vec3> = Vec::new();
    moons.push(Vec3 { x: -1, y: 0, z: 2 });
    moons.push(Vec3 {
        x: 2,
        y: -10,
        z: -7,
    });
    moons.push(Vec3 { x: 4, y: -8, z: 9 });
    moons.push(Vec3 { x: 3, y: 5, z: -1 });

    println!("moons: {:?}", moons);
}
