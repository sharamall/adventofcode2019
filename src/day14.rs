fn main() {
    let file = std::fs::read_to_string("src/day14in.txt").expect("File not found!");
    let mut name_to_reaction =
        file.split("\n").into_iter()
            .fold(std::collections::HashMap::new(), |mut map, line| {
                let mut split = line.split(" => ");
                let reagents_str = split.next().unwrap();
                let product_str = split.next().unwrap();
                let mut reagents = vec![];

                for reagent in reagents_str.split(", ").into_iter() {
                    let mut reagent_split = reagent.split(" ");
                    let count = reagent_split.next().unwrap().trim().parse().unwrap();
                    let name = reagent_split.next().unwrap().trim();
                    reagents.push(Reagent {
                        count,
                        name: String::from(name),
                    });
                }
                let mut product_split = product_str.split(" ");
                let count = product_split.next().unwrap().trim().parse().unwrap();
                let name = product_split.next().unwrap().trim();

                let reaction = Reaction {
                    count,
                    reagents,
                };

                map.insert(name, reaction);

                map
            });

    name_to_reaction.insert("ORE", Reaction {
        reagents: vec![],
        count: 1,
    });
    println!("part 1: {}", part1(&name_to_reaction));
    println!("part 2: {}", part2(&name_to_reaction));
}

fn find_required_amounts_recursively<'a>(name_to_reaction: &'a std::collections::HashMap<&'a str, Reaction>, needed: &mut std::collections::HashMap<&'a str, (i64, i64)>, r: &Reagent, multiplier: i64) {
    if r.name != "ORE" {
        let reaction = name_to_reaction.get(r.name.as_str()).unwrap();

        for reagent in &reaction.reagents {
            let mut cur_required = if let Some(val) = needed.get(reagent.name.as_str()) { val.clone() } else { (0, 0) };
            let new_need = reagent.count.clone() * multiplier;

            let mut additional_needed = 0;
            if cur_required.0 + new_need > cur_required.1 {
                let num_needed = cur_required.0 + new_need - cur_required.1;
                let mut result = num_needed / name_to_reaction.get(reagent.name.as_str()).unwrap().count;
                let remainder = num_needed % name_to_reaction.get(reagent.name.as_str()).unwrap().count;
                if remainder > 0 {
                    result += 1;
                }
                cur_required.1 += result * name_to_reaction.get(reagent.name.as_str()).unwrap().count;
                additional_needed += result;
            }
            cur_required.0 += new_need;
            needed.insert(&reagent.name, cur_required);
            find_required_amounts_recursively(name_to_reaction, needed, reagent, additional_needed);
        }
    }
}

fn part1(name_to_reaction: &std::collections::HashMap<&str, Reaction>) -> i64 {
    let mut num_needed = std::collections::HashMap::new();
    num_needed.insert("FUEL", (1, 1));
    find_required_amounts_recursively(name_to_reaction, &mut num_needed, &Reagent { name: String::from("FUEL"), count: 1 }, 1);
    return num_needed.get("ORE").unwrap().clone().0;
}

fn part2(name_to_reaction: &std::collections::HashMap<&str, Reaction>) -> i64 {
    let mut num_needed = std::collections::HashMap::new();
    num_needed.insert("FUEL", (1, 1));
    let mut loops = 0;
    let reagent = Reagent { name: String::from("FUEL"), count: loops };
    let mut ore_needed_for_fuel = 0;
    while ore_needed_for_fuel < 1_000_000_000_000 {
        find_required_amounts_recursively(name_to_reaction, &mut num_needed, &reagent, 1);
        ore_needed_for_fuel = num_needed.get("ORE").unwrap().clone().0;
        loops += 1;
    }

    loops - 1
}

#[derive(Debug)]
struct Reagent {
    count: i64,
    name: String,
}

#[derive(Debug)]
struct Reaction {
    reagents: Vec<Reagent>,
    count: i64,
}