fn main() {
    let file = std::fs::read_to_string("src/day11in.txt").expect("File not found!");
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
    part2(&memory);
    // println!("part 2: {}", part2(&memory));
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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
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
    if let Some(val) = memory.get(&i) {
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

enum IODirection {
    In,
    Out(i128),
}

enum RobotDirection {
    North,
    West,
    East,
    South,
}

fn run_simulation<T>(memory: &mut std::collections::HashMap<usize, i128>, mut io: T)
    where T: FnMut(IODirection) -> i128,
{
    let mut i = 0_usize;
    let mut relative_addr = 0_i128;
    let mut instruction = parse_instruction(*memory.get(&i).unwrap());
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
            let input = io(IODirection::In);
            memory.insert(dest as usize, input);
            i += 2;
        } else if instruction.op == 4 {
            let output = get_value_from_memory(memory, i + 1, instruction.mode1, relative_addr);
            io(IODirection::Out(output));
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
}

fn paint_surface(memory: &mut std::collections::HashMap<usize, i128>, surface: &mut std::collections::HashMap::<Position, bool>) {
    let mut cur_pos = Position {
        x: 0,
        y: 0,
    };
    let mut step = 0;
    let mut direction = RobotDirection::North;

    run_simulation(memory, |dir| {
        if let IODirection::Out(out) = dir {
            if step == 0 {
                // paint
                if out > 1 || out < 0 {
                    panic!("Invalid val {}", out);
                }
                surface.insert(cur_pos.clone(), if out == 0 { false } else { true });
                step = 1;
            } else {

                // move
                direction = match direction {
                    RobotDirection::North => if out == 0 { RobotDirection::West } else { RobotDirection::East }
                    RobotDirection::West => if out == 0 { RobotDirection::South } else { RobotDirection::North }
                    RobotDirection::East => if out == 0 { RobotDirection::North } else { RobotDirection::South }
                    RobotDirection::South => if out == 0 { RobotDirection::East } else { RobotDirection::West }
                };
                match direction {
                    RobotDirection::North => { cur_pos.y -= 1; }
                    RobotDirection::West => { cur_pos.x -= 1; }
                    RobotDirection::East => { cur_pos.x += 1; }
                    RobotDirection::South => { cur_pos.y += 1; }
                }
                step = 0;
            }
            0 // ignored by caller
        } else {
            let val = surface.get(&cur_pos).unwrap_or_else(|| &false);
            if *val {
                1
            } else {
                0
            }
        }
    });
}

fn part1(instructions: &std::collections::HashMap<usize, i128>) -> i128 {
    let memory = &mut instructions.clone();
    let mut surface = std::collections::HashMap::<Position, bool>::new();
    paint_surface(memory, &mut surface);
    surface.len() as i128
}

fn part2(instructions: &std::collections::HashMap<usize, i128>) {
    let memory = &mut instructions.clone();
    let mut surface = std::collections::HashMap::<Position, bool>::new();
    surface.insert(Position { x: 0, y: 0 }, true);
    paint_surface(memory, &mut surface);

    let mut min = Position {
        x: 0,
        y: 0,
    };
    let mut max = Position {
        x: 0,
        y: 0,
    };
    for tuple in &surface {
        let pos = tuple.0;
        if pos.x < min.x {
            min.x = pos.x;
        }
        if pos.y < min.y {
            min.y = pos.y;
        }
        if pos.x > max.x {
            max.x = pos.x;
        }
        if pos.y > max.y {
            max.y = pos.y;
        }
    }
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if *surface.get(&Position { x, y }).unwrap_or_else(|| &false) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}