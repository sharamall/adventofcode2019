fn main() {
    let file = std::fs::read_to_string("src/day05in.txt").expect("File not found!");
    let instructions: Vec<i32> = file.split(',').into_iter()
        .map(|i| i.trim().parse().expect("Can't parse as int"))
        .collect();
    println!("part 1: {}", part1(&instructions));
    println!("part 2: {}", part2(&instructions));
}

// inverted from problem, mode = true means load from memory
struct Instruction {
    op: i32,
    mode1: bool,
    mode2: bool
}

fn parse_instruction(val: i32) -> Instruction {
    let str = format!("{:05}", val);
    Instruction {
        op: (str[3..5]).parse().unwrap(),
        mode1: (str[2..3]).parse::<i32>().unwrap() == 0,
        mode2: (str[1..2]).parse::<i32>().unwrap() == 0,
    }
}

fn run_simulation(memory: &mut Vec<i32>, input: i32) -> i32 {
    let mut i = 0;
    let mut instruction = parse_instruction(memory[i]);
    let mut last_output = 0;
    while instruction.op != 99 {
        if instruction.op < 3 {
            let x = if instruction.mode1 { memory[memory[i + 1] as usize] } else { memory[i + 1] };
            let y = if instruction.mode2 { memory[memory[i + 2] as usize] } else { memory[i + 2] };
            let dest = memory[i + 3] as usize;
            if instruction.op == 1 {
                memory[dest] = x + y;
            } else {
                memory[dest] = x * y;
            }
            i += 4;
        } else if instruction.op == 3 {
            // input
            let dest = memory[i + 1] as usize;
            memory[dest] = input;
            i += 2;
        } else if instruction.op == 4 {
            let output = if instruction.mode1 { memory[memory[i + 1] as usize] } else { memory[i + 1] };
            if last_output != 0 {
                panic!("Overwriting a non-zero output");
            } else {
                last_output = output;
            }
            i += 2;
        } else if instruction.op == 5 { // bnez
            let x = if instruction.mode1 { memory[memory[i + 1] as usize] } else { memory[i + 1] };
            let y = if instruction.mode2 { memory[memory[i + 2] as usize] } else { memory[i + 2] };
            if x == 0 {
                i += 3;
            } else {
                i = y as usize;
            }
        } else if instruction.op == 6 { // beqz
            let x = if instruction.mode1 { memory[memory[i + 1] as usize] } else { memory[i + 1] };
            let y = if instruction.mode2 { memory[memory[i + 2] as usize] } else { memory[i + 2] };
            if x != 0 {
                i += 3;
            } else {
                i = y as usize;
            }
        } else if instruction.op == 7 { // lt
            let x = if instruction.mode1 { memory[memory[i + 1] as usize] } else { memory[i + 1] };
            let y = if instruction.mode2 { memory[memory[i + 2] as usize] } else { memory[i + 2] };
            let dest = memory[i + 3] as usize;
            if x < y {
                memory[dest] = 1;
            } else {
                memory[dest] = 0;
            }
            i += 4;
        } else if instruction.op == 8 { // lt
            let x = if instruction.mode1 { memory[memory[i + 1] as usize] } else { memory[i + 1] };
            let y = if instruction.mode2 { memory[memory[i + 2] as usize] } else { memory[i + 2] };
            let dest = memory[i + 3] as usize;
            if x == y {
                memory[dest] = 1;
            } else {
                memory[dest] = 0;
            }
            i += 4;
        }
        instruction = parse_instruction(memory[i]);
    }
    last_output
}

fn part1(instructions: &Vec<i32>) -> i32 {
    let memory = &mut instructions.clone();
    run_simulation(memory, 1)
}

fn part2(instructions: &Vec<i32>) -> i32 {
    let memory = &mut instructions.clone();
    run_simulation(memory, 5)
}
