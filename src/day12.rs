use num::Integer;

fn main() {
    let file = std::fs::read_to_string("src/day12in.txt").expect("File not found!");
    let r = regex::Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    let mut moons = vec![];
    let mut i = 0;
    for line in file.split('\n').into_iter() {
        let caps = r.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let y = caps.get(2).unwrap().as_str().parse().unwrap();
        let z = caps.get(3).unwrap().as_str().parse().unwrap();
        moons.push(Moon {
            id: i,
            pos: Vector3i32 { x, y, z },
            vel: Vector3i32 { x: 0, y: 0, z: 0 },
        });
        i += 1;
    }

    println!("part 1: {}", part1(&moons));
    println!("part 2: {}", part2(&moons));
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Vector3i32 {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::Add for Vector3i32 {
    type Output = Vector3i32;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3i32 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}


#[derive(Copy, Clone, Eq, Debug)]
struct Moon {
    id: i32,
    pos: Vector3i32,
    vel: Vector3i32,
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.vel == other.vel
    }
}

impl std::hash::Hash for Moon {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.vel.hash(state);
    }
}

fn energy(moon: &Moon) -> i32 {
    (
        moon.pos.x.abs() +
            moon.pos.y.abs() +
            moon.pos.z.abs()
    ) * (
        moon.vel.x.abs() +
            moon.vel.y.abs() +
            moon.vel.z.abs()
    )
}

fn part1(orig_moons: &Vec<Moon>) -> i32 {
    let mut moons = orig_moons.clone();
    let moon_count = moons.len();
    for _ in 0..1000 {
        for moon_index in 0..moon_count {
            for other_index in 0..moon_count {
                let moon = moons.get(moon_index).unwrap();
                let other = moons.get(other_index).unwrap();
                if moon.id != other.id {
                    let new_vel = Vector3i32 {
                        x: moon.vel.x - moon.pos.x.cmp(&other.pos.x) as i32,
                        y: moon.vel.y - moon.pos.y.cmp(&other.pos.y) as i32,
                        z: moon.vel.z - moon.pos.z.cmp(&other.pos.z) as i32,
                    };
                    moons[moon_index].vel = new_vel
                }
            }
        }
        for moon_index in 0..moon_count {
            let new_pos = moons[moon_index].pos + moons[moon_index].vel;
            moons[moon_index].pos = new_pos;
        }
    }
    let mut total_energy = 0;
    for moon_index in 0..moon_count {
        total_energy += energy(moons.get(moon_index).unwrap());
    }

    total_energy
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct MoonStateKey {
    moon1: Moon,
    moon2: Moon,
    moon3: Moon,
    moon4: Moon,
}

fn part2(orig_moons: &Vec<Moon>) -> u128 {
    let mut moons = orig_moons.clone();
    let moon_count = moons.len();
    let mut x_states = std::collections::HashMap::new();
    let mut y_states = std::collections::HashMap::new();
    let mut z_states = std::collections::HashMap::new();

    let mut x_repeat = None;
    let mut y_repeat = None;
    let mut z_repeat = None;

    let mut i = 0_u128;

    while x_repeat == None || y_repeat == None || z_repeat == None {
        if x_repeat == None {
            let x_key = (
                moons.get(0).unwrap().pos.x,
                moons.get(1).unwrap().pos.x,
                moons.get(2).unwrap().pos.x,
                moons.get(3).unwrap().pos.x,
                moons.get(0).unwrap().vel.x,
                moons.get(1).unwrap().vel.x,
                moons.get(2).unwrap().vel.x,
                moons.get(3).unwrap().vel.x
            );
            if x_states.contains_key(&x_key) {
                x_repeat = Some(i);
            } else {
                x_states.insert(x_key, 0);
            }
        }
        if y_repeat == None {
            let y_key = (
                moons.get(0).unwrap().pos.y,
                moons.get(1).unwrap().pos.y,
                moons.get(2).unwrap().pos.y,
                moons.get(3).unwrap().pos.y,
                moons.get(0).unwrap().vel.y,
                moons.get(1).unwrap().vel.y,
                moons.get(2).unwrap().vel.y,
                moons.get(3).unwrap().vel.y
            );
            if y_states.contains_key(&y_key) {
                y_repeat = Some(i);
            } else {
                y_states.insert(y_key, 0);
            }
        }
        if z_repeat == None {
            let z_kez = (
                moons.get(0).unwrap().pos.z,
                moons.get(1).unwrap().pos.z,
                moons.get(2).unwrap().pos.z,
                moons.get(3).unwrap().pos.z,
                moons.get(0).unwrap().vel.z,
                moons.get(1).unwrap().vel.z,
                moons.get(2).unwrap().vel.z,
                moons.get(3).unwrap().vel.z
            );
            if z_states.contains_key(&z_kez) {
                z_repeat = Some(i);
            } else {
                z_states.insert(z_kez, 0);
            }
        }

        for moon_index in 0..moon_count {
            for other_index in 0..moon_count {
                if moon_index != other_index {
                    let moon = moons.get(moon_index).unwrap();
                    let other = moons.get(other_index).unwrap();
                    let new_vel = Vector3i32 {
                        x: moon.vel.x - moon.pos.x.cmp(&other.pos.x) as i32,
                        y: moon.vel.y - moon.pos.y.cmp(&other.pos.y) as i32,
                        z: moon.vel.z - moon.pos.z.cmp(&other.pos.z) as i32,
                    };
                    moons[moon_index].vel = new_vel
                }
            }
        }
        for moon_index in 0..moon_count {
            let new_pos = moons[moon_index].pos + moons[moon_index].vel;
            moons[moon_index].pos = new_pos;
        }

        i += 1;
    }

    x_repeat.unwrap().lcm(&y_repeat.unwrap()).lcm(&z_repeat.unwrap())
}