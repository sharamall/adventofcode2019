fn main() {
    let file = std::fs::read_to_string("src/day03in.txt").expect("File not found!");
    let input: Vec<Vec<&str>> = file.split('\n').into_iter()
        .map(|i|
            i.split(",").into_iter().collect()
        )
        .collect();
    let (part1, part2) = run(&input[0], &input[1]);
    println!("part 1 {}", part1);
    println!("part 2 {}", part2);
}

#[derive(PartialEq, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

fn calc_intermediate_positions(dir: &str, dist: i32, x: &mut i32, y: &mut i32) -> Vec<Vector2> {
    let mut visited: Vec<Vector2> = vec![];
    match dir {
        "R" => {
            for new_x in (*x + 1)..=(*x + dist) {
                let new_pos = Vector2 {
                    x: new_x,
                    y: *y,
                };
                visited.push(new_pos);
            }
            *x += dist;
        }
        "L" => {
            for new_x in ((*x - dist)..=(*x - 1)).rev() {
                let new_pos = Vector2 {
                    x: new_x,
                    y: *y,
                };
                visited.push(new_pos);
            }
            *x -= dist;
        }
        "U" => {
            for new_y in (*y + 1)..=(*y + dist) {
                let new_pos = Vector2 {
                    x: *x,
                    y: new_y,
                };
                visited.push(new_pos);
            }
            *y += dist;
        }
        "D" => {
            for new_y in ((*y - dist)..=(*y - 1)).rev() {
                let new_pos = Vector2 {
                    x: *x,
                    y: new_y,
                };
                visited.push(new_pos);
            }
            *y -= dist;
        }
        _ => panic!("Unknown dir {}", dir)
    }
    visited
}

fn run(str1: &Vec<&str>, str2: &Vec<&str>) -> (i32, i32) {
    let mut visited = std::collections::HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut step = 1;
    let mut max_distance = 10000000_i32;
    let mut steps = 10000000_i32;
    for instruction in str1 {
        let dir = &instruction[0..1];
        let dist: i32 = (&instruction[1..]).trim().parse().expect("Couldn't parse");
        let positions = calc_intermediate_positions(dir, dist, &mut x, &mut y);
        for pos in positions {
            visited.insert(pos, step);
            step += 1
        }
    }
    x = 0;
    y = 0;
    step = 1;
    for instruction in str2 {
        let dir = &instruction[0..1];
        let dist: i32 = (&instruction[1..]).trim().parse().expect("Couldn't parse");
        let positions = calc_intermediate_positions(dir, dist, &mut x, &mut y);
        for pos in positions {
            if visited.contains_key(&pos) {
                let manhattan_dist = pos.x.abs() + pos.y.abs();
                if manhattan_dist < max_distance {
                    max_distance = manhattan_dist
                }
                let combined_steps = step + visited[&pos];
                if combined_steps < steps {
                    steps = combined_steps
                }
            }
            step += 1;
        }
    }

    (max_distance, steps)
}