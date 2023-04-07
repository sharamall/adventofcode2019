fn main() {
    let file = std::fs::read_to_string("src/day16in.txt").expect("File not found!");
    let mut raw: Vec<&str> = file.split("")
        .into_iter()
        .map(|i| i.trim())
        .collect();
    raw.retain(|i| *i != "");
    let input: Vec<i32> = raw
        .into_iter()
        .map(|i| i.trim().parse().unwrap())
        .collect();
    println!("part 1 {}", part1(&mut input.clone()));
    println!("part 2 {}", part2(&mut input.clone()));
}


fn calc_coefficient(output_digit_index: i32, input_digit_index: i32) -> i32 {
    let scale = output_digit_index + 1;
    let index = ((input_digit_index + 1) / scale) % 4 as i32;

    match index {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => panic!("Out of bounds index {}", index)
    }
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut prev = input.clone();
    let mut next = input.clone();
    const PHASES: i32 = 100;
    let offset = 0;

    for _ in 0..PHASES {
        let mut output_i = prev.len() - 1;
        loop {
            let mut sum = 0;
            for input_i in output_i..prev.len() {
                let cur_val = prev[input_i];
                if cur_val != 0 {
                    sum += calc_coefficient(output_i as i32, input_i as i32) * cur_val;
                }
            }
            next[output_i] = sum.abs() % 10;

            if output_i == 0 {
                break;
            }
            output_i -= 1;
        }
        for i in 0..next.len() {
            prev[i] = next[i]
        }

        print!("");
    }
    let mut output = 0;
    for i in 0_usize..8 {
        let power = (7 - i) as u32;
        let multiplier = 10_i32.pow(power);
        output += multiplier * (prev[i + offset])
    }

    output
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut offset = 0_usize;
    for i in 0_usize..7 {
        let power = (6 - i) as u32;
        let multiplier = 10_i32.pow(power);
        offset += (multiplier * input[i]) as usize
    }
    let mut prev = vec![];
    prev.reserve(input.len() * 10000 - offset);

    for repeat in 0..10000 {
        for i in 0..input.len() {
            if input.len() * repeat + i >= offset {
                prev.push(input[i]);
            }
        }
    }
    let mut next = prev.clone();
    const PHASES: i32 = 100;

    for _ in 0..PHASES {
        let mut output_i = prev.len() - 1;
        let mut sum = 0_i32;
        loop {
            sum += prev[output_i];
            next[output_i] = sum.abs() % 10;

            if output_i == 0 {
                break;
            }
            output_i -= 1;
        }
        std::mem::swap(&mut prev, &mut next);
    }
    let mut output = 0;
    for i in 0_usize..8 {
        let power = (7 - i) as u32;
        let multiplier = 10_i32.pow(power);
        output += multiplier * (prev[i])
    }

    output
}

#[test]
fn calc_coefficient_test() {
    assert_eq!(calc_coefficient(0, 0), 1);
    assert_eq!(calc_coefficient(0, 1), 0);
    assert_eq!(calc_coefficient(0, 2), -1);
    assert_eq!(calc_coefficient(0, 3), 0);
    assert_eq!(calc_coefficient(0, 4), 1);
    assert_eq!(calc_coefficient(0, 5), 0);
    assert_eq!(calc_coefficient(0, 6), -1);
    assert_eq!(calc_coefficient(0, 7), 0);

    assert_eq!(calc_coefficient(1, 0), 0);
    assert_eq!(calc_coefficient(1, 1), 1);
    assert_eq!(calc_coefficient(1, 2), 1);
    assert_eq!(calc_coefficient(1, 3), 0);
    assert_eq!(calc_coefficient(1, 4), 0);
    assert_eq!(calc_coefficient(1, 5), -1);
    assert_eq!(calc_coefficient(1, 6), -1);
    assert_eq!(calc_coefficient(1, 7), 0);

    assert_eq!(calc_coefficient(2, 0), 0);
    assert_eq!(calc_coefficient(2, 1), 0);
    assert_eq!(calc_coefficient(2, 2), 1);
    assert_eq!(calc_coefficient(2, 3), 1);
    assert_eq!(calc_coefficient(2, 4), 1);
    assert_eq!(calc_coefficient(2, 5), 0);
    assert_eq!(calc_coefficient(2, 6), 0);
    assert_eq!(calc_coefficient(2, 7), 0);
}