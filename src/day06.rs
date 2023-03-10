use std::cmp::Ordering;

fn main() {
    let file = std::fs::read_to_string("src/day06in.txt").expect("File not found!");
    let lines: Vec<&str> = file.split("\n").into_iter()
        .collect();

    let com = Planet {
        parent_name: "",
        child_names: vec![],
    };

    let mut planets = std::collections::HashMap::new();
    planets.insert("COM", com);
    for x in &lines {
        let split_line: Vec<&str> = (*x).split(")").into_iter().collect();
        let star = split_line[0];
        let planet = split_line[1];

        if !planets.contains_key(star) {
            let new_star = Planet {
                parent_name: "",
                child_names: vec![planet],
            };
            planets.insert(star, new_star);
        } else {
            let star_obj = planets.get_mut(star).unwrap();
            (*star_obj).child_names.push(planet);
        }

        if planets.contains_key(planet) {
            let mut planet_obj = planets.get_mut(planet).unwrap();
            (*planet_obj).parent_name = star;
        } else {
            let new_planet = Planet {
                parent_name: star,
                child_names: vec![],
            };
            planets.insert(planet, new_planet);
        }
    }

    println!("part1: {}", part1(&planets));
    println!("part2: {}", part2(&planets));
}

struct Planet<'a> {
    parent_name: &'a str,
    child_names: Vec<&'a str>,
}

fn part1(planets: &std::collections::HashMap<&str, Planet>) -> i32 {
    let mut sum = 0;
    for p in planets {
        let mut cur = p.1;
        while planets.contains_key(cur.parent_name) {
            sum += 1;
            cur = planets.get(cur.parent_name).unwrap();
        }
    }
    sum
}

fn part2(planets: &std::collections::HashMap<&str, Planet>) -> i32 {
    // find path for SAN
    // find path for YOU
    // find first intersection, add two tails
    // YOU -> COM - A - B - C
    // SAN -> COM - A - D - E
    // PATH = C - B - A - D - E
    let mut me_path = vec![];
    let mut san_path = vec![];

    let mut cur = planets.get("YOU").unwrap();
    while planets.contains_key(cur.parent_name) {
        me_path.push(cur.parent_name);
        cur = planets.get(cur.parent_name).unwrap();
    }

    cur = planets.get("SAN").unwrap();
    while planets.contains_key(cur.parent_name) {
        san_path.push(cur.parent_name);
        cur = planets.get(cur.parent_name).unwrap();
    }
    let mut san_id = 0;
    loop {
        let san_pos = san_path.get(san_id).unwrap();
        for me_index in 0..me_path.len() {
            let me = me_path.get(me_index).unwrap();
            if let Ordering::Equal = (*me).cmp(*san_pos) {
                return me_index as i32 + san_id as i32;
            }
        }

        san_id += 1;
        if san_id >= san_path.len() {
            panic!("shared path not found");
        }
    }
}