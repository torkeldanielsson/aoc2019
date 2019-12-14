#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    pos: Vec3,
    velocity: Vec3,
}

fn main() {
    let mut moons: Vec<Moon> = Vec::new();
    moons.push(Moon {
        pos: Vec3 {
            x: -3,
            y: 10,
            z: -1,
        },
        velocity: Vec3 { x: 0, y: 0, z: 0 },
    });
    moons.push(Moon {
        pos: Vec3 {
            x: -12,
            y: -10,
            z: -5,
        },
        velocity: Vec3 { x: 0, y: 0, z: 0 },
    });
    moons.push(Moon {
        pos: Vec3 { x: -9, y: 0, z: 10 },
        velocity: Vec3 { x: 0, y: 0, z: 0 },
    });
    moons.push(Moon {
        pos: Vec3 { x: 7, y: -5, z: -3 },
        velocity: Vec3 { x: 0, y: 0, z: 0 },
    });

    for t in 0..1001 {
        let mut energy_sum = 0;
        for i in 0..4 {
            moons[i].pos.x += moons[i].velocity.x;
            moons[i].pos.y += moons[i].velocity.y;
            moons[i].pos.z += moons[i].velocity.z;

            let energy = (moons[i].pos.x.abs() + moons[i].pos.y.abs() + moons[i].pos.z.abs())
                * (moons[i].velocity.x.abs()
                    + moons[i].velocity.y.abs()
                    + moons[i].velocity.z.abs());

            energy_sum += energy;

            println!("t{}: moon {}: {:?}, energy: {}", t, i, moons[i], energy);
        }
        println!("energy sum: {}", energy_sum);

        for i in 0..4 {
            for j in (i + 1)..4 {
                if moons[i].pos.x < moons[j].pos.x {
                    moons[i].velocity.x += 1;
                    moons[j].velocity.x -= 1;
                }
                if moons[i].pos.x > moons[j].pos.x {
                    moons[i].velocity.x -= 1;
                    moons[j].velocity.x += 1;
                }
                if moons[i].pos.y < moons[j].pos.y {
                    moons[i].velocity.y += 1;
                    moons[j].velocity.y -= 1;
                }
                if moons[i].pos.y > moons[j].pos.y {
                    moons[i].velocity.y -= 1;
                    moons[j].velocity.y += 1;
                }
                if moons[i].pos.z < moons[j].pos.z {
                    moons[i].velocity.z += 1;
                    moons[j].velocity.z -= 1;
                }
                if moons[i].pos.z > moons[j].pos.z {
                    moons[i].velocity.z -= 1;
                    moons[j].velocity.z += 1;
                }
            }
        }
    }
}
