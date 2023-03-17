fn main() {
    let file = std::fs::read_to_string("src/day10in.txt").expect("File not found!");
    let raw_input: Vec<&str> = file.split('\n').into_iter()
        .map(|i| i.trim())
        .collect();
    let mut asteroids = vec![];
    let mut x;
    let mut y = 0;
    for line in raw_input {
        x = 0;
        for c in line.chars() {
            if c == '#' {
                asteroids.push(Asteroid {
                    x,
                    y,
                });
            }
            x += 1;
        }
        y += 1;
    }

    println!("part 1: {}", part1(&asteroids));
    println!("part 2: {}", part2(&asteroids));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Asteroid {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Ray {
    angle: f32,
    dir: bool,
}

impl Eq for Ray {}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.angle.to_bits() == other.angle.to_bits() &&
            self.dir == other.dir
    }
}

impl std::hash::Hash for Ray {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.angle.to_bits().hash(state);
        self.dir.hash(state);
    }
}

fn find_asteroid(grid: &Vec<Asteroid>) -> (&Asteroid, std::collections::HashMap<Ray, Vec<Asteroid>>) {
    let mut max = 0;
    let mut max_asteroid = None;
    for asteroid in grid {
        let mut los_set: std::collections::HashMap<Ray, Vec<Asteroid>> = std::collections::HashMap::new();

        for other in grid {
            if asteroid != other {
                let y_delta = other.y as f32 - asteroid.y as f32;
                let x_delta = other.x as f32 - asteroid.x as f32;
                let dir = if asteroid.x == other.x {
                    asteroid.y > other.y
                } else {
                    asteroid.x > other.x
                };
                let mut ray = Ray {
                    dir,
                    angle: (-y_delta).atan2(x_delta), // negative y because y axis is reversed, positive down instead of up
                };
                if ray.angle < 0.0 {
                    ray.angle += std::f32::consts::PI * 2.0;
                }

                if los_set.contains_key(&ray) {
                    los_set.get_mut(&ray).unwrap().push(other.clone());
                } else {
                    los_set.insert(ray, vec![other.clone()]);
                }
            }
        }
        if los_set.len() > max {
            max = los_set.len();
            max_asteroid = Some((asteroid, los_set));
        }
    }

    max_asteroid.unwrap()
}

fn part1(grid: &Vec<Asteroid>) -> i32 {
    find_asteroid(grid).1.len() as i32
}

fn distance(root: &Asteroid, other: &Asteroid) -> f32 {
    let dist_squared = (other.x - root.x) *
        (other.x - root.x) + (other.y - root.y) * (other.y - root.y);
    (dist_squared as f32).sqrt()
}

fn part2(grid: &Vec<Asteroid>) -> i32 {
    let result = find_asteroid(grid);

    let mut ordered_angles = vec![];
    for mut tuple in result.1.into_iter() {
        tuple.1.sort_by(|a, b| {
            let a_dist = distance(result.0, a);
            let b_dist = distance(result.0, b);
            let cmp = a_dist.partial_cmp(&b_dist);
            cmp.unwrap()
        });
        ordered_angles.push(tuple);
    }
    ordered_angles.sort_by(|a, b| a.0.angle.partial_cmp(&b.0.angle).unwrap());
    let mut i = 0;

    let pi_halves = std::f32::consts::PI / 2.0;
    while ordered_angles.get(i).unwrap().0.angle <= pi_halves {
        i += 1;
    }
    let mut destroyed = 0;
    let mut last_destroyed = None;
    i -= 1;
    while destroyed != 200 {
        let asteroids = &mut ordered_angles.get_mut(i).unwrap().1;
        if asteroids.len() > 0 {
            let removed = asteroids.remove(0);
            last_destroyed = Some(removed);
            destroyed += 1;
        }
        i = if i == 0 { ordered_angles.len() - 1 } else { i - 1 };
    }

    last_destroyed.unwrap().x * 100 + last_destroyed.unwrap().y
}