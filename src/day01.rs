fn main() {
    let file = std::fs::read_to_string("src/day01in.txt").expect("File not found!");
    let lines: Vec<i64> = file.split("\n").into_iter()
        .map(|i| i.trim().parse().expect("Can't parse as int"))
        .collect();

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}

fn part1(lines: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for val in lines {
        sum += calculate_fuel(*val, false);
    }
    sum
}

fn part2(lines: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for val in lines {
        sum += calculate_fuel(*val, true);
    }
    sum
}

fn calculate_fuel(val: i64, recursive: bool) -> i64 {
    let mut sum = val / 3 - 2;
    if recursive && sum > 0 {
        let recursive_sum = calculate_fuel(sum, recursive);
        if recursive_sum > 0 {
            sum += recursive_sum;
        }
    }
    sum
}