fn main() {
    let file = std::fs::read_to_string("src/day09in.txt").expect("File not found!");
    let instructions: Vec<i128> = file.split(',').into_iter()
        .map(|i| i.trim().parse().expect("Can't parse as int"))
        .collect();
    let mut memory = std::collections::HashMap::new();
    let mut i = 0_usize;
    for instruction in instructions.into_iter() {
        memory.insert(i, instruction as i128);
        i += 1;
    }
    println!("part 1: {}", part1(&memory));
    println!("part 2: {}", part2(&memory));
}

// inverted from problem, mode = true means load from memory
struct Instruction {
    op: i32,
    mode1: InstructionMode,
    mode2: InstructionMode,
    mode3: InstructionMode,
}

enum InstructionMode {
    Immediate,
    AbsoluteAddress,
    RelativeAddress,
}

fn parse_instruction(val: i128) -> Instruction {
    let str = format!("{:05}", val);
    let mode1 = (str[2..3]).parse::<i32>().unwrap();
    let mode2 = (str[1..2]).parse::<i32>().unwrap();
    let mode3 = (str[0..1]).parse::<i32>().unwrap();

    Instruction {
        op: (str[3..5]).parse().unwrap(),
        mode1: match mode1 {
            0 => InstructionMode::AbsoluteAddress,
            1 => InstructionMode::Immediate,
            2 => InstructionMode::RelativeAddress,
            _ => panic!("Unknown instruction mode {}", mode1)
        },
        mode2: match mode2 {
            0 => InstructionMode::AbsoluteAddress,
            1 => InstructionMode::Immediate,
            2 => InstructionMode::RelativeAddress,
            _ => panic!("Unknown instruction mode {}", mode2)
        },
        mode3: match mode3 {
            0 => InstructionMode::AbsoluteAddress,
            1 => InstructionMode::Immediate,
            2 => InstructionMode::RelativeAddress,
            _ => panic!("Unknown instruction mode {}", mode3)
        },
    }
}

fn get_value_from_memory(memory: &std::collections::HashMap<usize, i128>, i: usize, mode: InstructionMode, relative_addr: i128) -> i128 {
    if let Some(val) = memory.get(&(i as usize)) {
        match mode {
            InstructionMode::Immediate => {
                val.clone()
            }
            InstructionMode::AbsoluteAddress => {
                get_value_from_memory(memory, val.clone() as usize, InstructionMode::Immediate, relative_addr)
            }
            InstructionMode::RelativeAddress => {
                get_value_from_memory(memory, (val.clone() + relative_addr) as usize, InstructionMode::Immediate, relative_addr)
            }
        }
    } else {
        0
    }
}

fn run_simulation(memory: &mut std::collections::HashMap<usize, i128>, input: i128) -> Vec<i128> {
    let mut i = 0_usize;
    let mut relative_addr = 0_i128;
    let mut instruction = parse_instruction(*memory.get(&i).unwrap());
    let mut outputs = vec![];
    while instruction.op != 99 {
        if instruction.op < 3 {
            let x = get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            let y = get_value_from_memory(memory, i + 2, instruction.mode2, relative_addr);
            let mut dest = get_value_from_memory(memory, i + 3, InstructionMode::Immediate, relative_addr);
            if let InstructionMode::RelativeAddress = instruction.mode3 {
                dest += relative_addr;
            }
            if instruction.op == 1 {
                memory.insert(dest as usize, x + y);
            } else {
                memory.insert(dest as usize, x * y);
            }
            i += 4;
        } else if instruction.op == 3 {
            // input
            let mut dest = get_value_from_memory(memory, i + 1, InstructionMode::Immediate, relative_addr);
            if let InstructionMode::RelativeAddress = instruction.mode1 {
                dest += relative_addr;
            }
            memory.insert(dest as usize, input);
            i += 2;
        } else if instruction.op == 4 {
            let output = get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            outputs.push(output);
            i += 2;
        } else if instruction.op == 5 { // bnez
            let x = get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            let y = get_value_from_memory(memory, i + 2, instruction.mode2, relative_addr);
            if x == 0 {
                i += 3;
            } else {
                i = y as usize;
            }
        } else if instruction.op == 6 { // beqz
            let x = get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            let y = get_value_from_memory(memory, i + 2, instruction.mode2, relative_addr);

            if x != 0 {
                i += 3;
            } else {
                i = y as usize;
            }
        } else if instruction.op == 7 { // lt
            let x = get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            let y = get_value_from_memory(memory, i + 2, instruction.mode2, relative_addr);
            let mut dest = get_value_from_memory(memory, i + 3, InstructionMode::Immediate, relative_addr);
            if let InstructionMode::RelativeAddress = instruction.mode3 {
                dest += relative_addr;
            }
            if x < y {
                memory.insert(dest as usize, 1);
            } else {
                memory.insert(dest as usize, 0);
            }
            i += 4;
        } else if instruction.op == 8 { // lt
            let x = get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            let y = get_value_from_memory(memory, i + 2, instruction.mode2, relative_addr);
            let mut dest = get_value_from_memory(memory, i + 3, InstructionMode::Immediate, relative_addr);
            if let InstructionMode::RelativeAddress = instruction.mode3 {
                dest += relative_addr;
            }
            if x == y {
                memory.insert(dest as usize, 1);
            } else {
                memory.insert(dest as usize, 0);
            }
            i += 4;
        } else if instruction.op == 9 {
            relative_addr += get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            i += 2;
        }
        instruction = parse_instruction(*memory.get(&i).unwrap());
    }
    outputs
}

fn part1(instructions: &std::collections::HashMap<usize, i128>) -> i128 {
    let memory = &mut instructions.clone();
    let result = run_simulation(memory, 1);
    result[result.len() - 1]
}

fn part2(instructions: &std::collections::HashMap<usize, i128>) -> i128 {
    let memory = &mut instructions.clone();
    let result = run_simulation(memory, 2);
    result[result.len() - 1]
}