fn main() {
    let file = std::fs::read_to_string("src/day04in.txt").expect("File not found!");
    let input: Vec<i32> = file.split('\n').into_iter()
        .map(|i| i.trim().parse().expect("Couldn't parse int"))
        .collect();


    println!("part 1 {}", part1(input[0], input[1]));
    println!("part 2 {}", part2(input[0], input[1]));
}

fn is_valid_p1(val: &i32) -> bool {
    let str = val.to_string();
    let mut prev: i32 = str[0..1].parse().expect("Unable to parse");
    let mut has_double = false;
    let mut has_decrease = false;
    for i in 1..6 {
        let cur: i32 = str[i..=i].parse().expect("Unable to parse");
        if cur == prev {
            has_double = true
        }
        if cur < prev {
            has_decrease = true
        }
        prev = cur
    }
    has_double && !has_decrease
}

fn part1(min: i32, max: i32) -> i32 {
    let mut sum = 0;
    for val in min..=max {
        if is_valid_p1(&val) {
            sum += 1;
        }
    }
    sum
}

fn is_valid_p2(val: &i32) -> bool {
    let str = val.to_string();
    let mut prev: i32 = str[0..1].parse().expect("Unable to parse");
    let mut has_double = false;
    let mut has_decrease = false;
    let mut match_length = 1;
    for i in 1..6 {
        let cur: i32 = str[i..=i].parse().expect("Unable to parse");
        if cur < prev {
            has_decrease = true;
        }
        if cur == prev {
            match_length += 1;
        } else {
            if match_length == 2 {
                has_double = true;
            }
            match_length = 1;
        }
        prev = cur;
    }
    if match_length == 2 {
        has_double = true;
    }
    has_double && !has_decrease
}

fn part2(min: i32, max: i32) -> i32 {
    let mut sum = 0;
    for val in min..=max {
        if is_valid_p2(&val) {
            sum += 1;
        }
    }
    sum
}