fn main() {
    let file = std::fs::read_to_string("src/day08in.txt").unwrap();
    let mut raw: Vec<&str> = file.split("")
        .into_iter()
        .map(|i| i.trim())
        .collect();
    raw.retain(|i| *i != "");
    let input: Vec<i32> = raw
        .into_iter()
        .map(|i| i.trim().parse().unwrap())
        .collect();
    println!("part 1: {}", part1(&input));
    println!("part 2:");
    part2(&input);
}

fn part1(input: &Vec<i32>) -> i32 {
    let width = 25;
    let height = 6;
    let pixels_per_layer = width * height;
    let layers = input.len() / pixels_per_layer;
    let mut zero_count_per_layer = vec![];
    let mut one_count_per_layer = vec![];
    let mut two_count_per_layer = vec![];
    for _ in 0..layers {
        zero_count_per_layer.push(0);
        one_count_per_layer.push(0);
        two_count_per_layer.push(0);
    }
    for l in 0..layers {
        for i in 0..pixels_per_layer {
            match *(input.get(l * pixels_per_layer + i).unwrap()) {
                0 => {
                    zero_count_per_layer[l] += 1;
                }
                1 => {
                    one_count_per_layer[l] += 1;
                }
                2 => {
                    two_count_per_layer[l] += 1;
                }
                _ => {}
            }
        }
    }
    let mut min = pixels_per_layer;
    let mut min_layer = 0;
    for l in 0..layers {
        if zero_count_per_layer[l] < min {
            min = zero_count_per_layer[l];
            min_layer = l;
        }
    }
    one_count_per_layer[min_layer] * two_count_per_layer[min_layer]
}


fn part2(input: &Vec<i32>) {
    let width = 25;
    let height = 6;
    let pixels_per_layer = width * height;
    let layers = input.len() / pixels_per_layer;
    let mut output = vec![];
    for _ in 0..pixels_per_layer {
        output.push(2);
    }
    for l in 0..layers {
        for i in 0..pixels_per_layer {
            match *(input.get(l * pixels_per_layer + i).unwrap()) {
                0 => {
                    if output[i] == 2 {
                        output[i] = 0;
                    }
                }
                1 => {
                    if output[i] == 2 {
                        output[i] = 1;
                    }
                }
                _ => {}
            }
        }
    }
    for i in 0..pixels_per_layer {
        if i % width == 0 {
            println!();
        }
        if output[i] == 1 {
            print!("{}", output[i]);
        } else {
            print!(" ");
        }
    }
}


