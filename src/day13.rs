fn main() {
    let file = std::fs::read_to_string("src/day13in.txt").expect("File not found!");
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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
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

fn part1(instructions: &std::collections::HashMap<usize, i128>) -> i128 {
    let memory = &mut instructions.clone();
    let mut game = std::collections::HashMap::<Position, Tile>::new();
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;

    run_simulation(memory, |io| {
        if let IODirection::Out(val) = io {
            if i == 0 {
                x = val as i32;
            } else if i == 1 {
                y = val as i32;
            } else {
                let tile = match val {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => Tile::Paddle,
                    4 => Tile::Ball,
                    _ => panic!("Unknown output {}", val)
                };
                game.insert(Position { x, y }, tile);
            }
            i = (i + 1) % 3;
        } else {
            panic!("not supposed to input!");
        }
        0
    });
    let mut sum = 0;
    for entry in game {
        if entry.1 == Tile::Block {
            sum += 1
        }
    }
    sum
}

fn print_grid(grid: &std::collections::HashMap::<Position, Tile>, max_x: &i32, max_y: &i32) {
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            print!("{}", match *grid.get(&Position { x, y }).unwrap() {
                Tile::Empty => " ",
                Tile::Wall => "W",
                Tile::Block => "©",
                Tile::Paddle => "P",
                Tile::Ball => "B"
            })
        }
        println!();
    }
}

fn part2(instructions: &std::collections::HashMap<usize, i128>) -> i128 {
    let memory = &mut instructions.clone();
    memory.insert(0, 2);
    let mut game = std::collections::HashMap::<Position, Tile>::new();
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut ball_pos = Position { x: 0, y: 0 };
    let mut paddle_pos = Position { x: 0, y: 0 };
    let mut score = 0;

    run_simulation(memory, |io| {
        if let IODirection::Out(val) = io {
            if i == 0 {
                x = val as i32;
                max_x = if x > max_x { x } else { max_x }
            } else if i == 1 {
                y = val as i32;
                max_y = if y > max_y { y } else { max_y }
            } else {
                if x == -1 && y == 0 {
                    score = val;
                    print_grid(&game, &max_x, &max_y);
                    println!("score: {}", val);
                } else {
                    let tile = match val {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        _ => panic!("Unknown output {}", val)
                    };
                    if tile == Tile::Ball {
                        ball_pos = Position { x, y };
                    } else if tile == Tile::Paddle {
                        paddle_pos = Position { x, y };
                    }
                    game.insert(Position { x, y }, tile);
                }
            }
            i = (i + 1) % 3;
            0
        } else {
            ball_pos.x.cmp(&paddle_pos.x) as i128
        }
    });

    score
}

