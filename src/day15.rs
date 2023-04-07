fn main() {
    let file = std::fs::read_to_string("src/day15in.txt").expect("File not found!");
    let raw_instructions: Vec<i128> = file.split(',').into_iter()
        .map(|i| i.trim().parse().expect("Can't parse as int"))
        .collect();
    let mut instructions = std::collections::HashMap::new();
    let mut i = 0_usize;
    for instruction in raw_instructions.into_iter() {
        instructions.insert(i, instruction as i128);
        i += 1;
    }
    let mut memory = instructions.clone();
    println!("part 1: {}", part1(&mut memory));
    println!("part 2: {}", part2(&mut memory));
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Node {
    pos: Position,
    parent: Position,
    parent_dir: DroidDirection,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}) ({:?}) {:?}", self.pos, self.parent, self.parent_dir)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum DroidDirection {
    NORTH(i32),
    SOUTH(i32),
    WEST(i32),
    EAST(i32),
}

#[derive(PartialEq, Eq)]
enum PathDir {
    Forward,
    TurningBack,
    Reverse,
}

fn translate_pos(pos: &Position, dir: DroidDirection) -> Position {
    let mut new_pos = pos.clone();
    match dir {
        DroidDirection::NORTH(_) => new_pos.y -= 1,
        DroidDirection::SOUTH(_) => new_pos.y += 1,
        DroidDirection::WEST(_) => new_pos.x -= 1,
        DroidDirection::EAST(_) => new_pos.x += 1,
    }
    new_pos
}

fn north() -> DroidDirection {
    DroidDirection::NORTH(1)
}

fn south() -> DroidDirection {
    DroidDirection::SOUTH(2)
}

fn west() -> DroidDirection {
    DroidDirection::WEST(3)
}

fn east() -> DroidDirection {
    DroidDirection::EAST(4)
}

fn next(dir: &DroidDirection) -> Option<DroidDirection> {
    match dir {
        DroidDirection::NORTH(_) => Some(south()),
        DroidDirection::SOUTH(_) => Some(west()),
        DroidDirection::WEST(_) => Some(east()),
        DroidDirection::EAST(_) => None
    }
}

fn reverse(dir: &DroidDirection) -> DroidDirection {
    match dir {
        DroidDirection::NORTH(_) => south(),
        DroidDirection::SOUTH(_) => north(),
        DroidDirection::WEST(_) => east(),
        DroidDirection::EAST(_) => west()
    }
}

fn part1(memory: &mut std::collections::HashMap<usize, i128>) -> i64 {
    let mut steps = None;
    run(memory, |len| {
        steps = Some(len);
        true
    }, |_| {});
    steps.unwrap() as i64
}

fn part2(memory: &mut std::collections::HashMap<usize, i128>) -> i64 {
    let mut steps = None;
    run(memory, |_| false, |dist| {
        if let Some(prev) = steps {
            if prev < dist {
                steps = Some(dist);
            }
        } else {
            steps = Some(dist);
        }
    });
    steps.unwrap() as i64
}

fn add_surrounding_to_unseen(parent: &Position, unseen_paths: &mut Vec<Vec<Node>>, unseen_positions: &mut std::collections::HashSet<Position>, seen: &mut std::collections::HashMap<Position, Option<Vec<Node>>>) {
    let mut dir = Some(north());
    while dir != None {
        let parent_path = seen.get(&parent).unwrap().as_ref();
        let pos = translate_pos(parent, dir.unwrap());
        let mut new_path = parent_path.cloned().unwrap();
        if !seen.contains_key(&pos) {
            if !unseen_positions.contains(&pos) {
                // println!("adding to unseen path ends at {:?}", pos);
                new_path.push(Node {
                    pos,
                    parent: *parent,
                    parent_dir: reverse(&dir.unwrap()),
                });
                unseen_paths.insert(0, new_path);
                if !unseen_positions.insert(pos.clone()) {
                    panic!("adding to unseen twice!");
                }
            } else {
                print!("");
            }
        }
        dir = next(&dir.unwrap());
    }
    unseen_paths.sort_by(|a, b| b.len().cmp(&a.len()));
}

fn run<T, V>(memory: &mut std::collections::HashMap<usize, i128>, mut found_oxygen: T, mut found_path: V)
    where T: FnMut(usize) -> bool, V: FnMut(usize) {
    let mut seen: std::collections::HashMap<Position, Option<Vec<Node>>> = std::collections::HashMap::new();
    let mut cur_pos = Position { x: 0, y: 0 };
    seen.insert(cur_pos, Some(vec![Node { pos: cur_pos, parent: cur_pos, parent_dir: north() }]));
    let mut unseen_paths = vec![];
    let mut unseen_positions = std::collections::HashSet::new();
    add_surrounding_to_unseen(&cur_pos, &mut unseen_paths, &mut unseen_positions, &mut seen);
    let mut dir_to_prev = north();
    let mut cur_path = unseen_paths.pop().unwrap();
    let mut path_index = 0;
    let mut path_dir = PathDir::Forward;
    let mut should_stop = false;

    run_simulation(memory, |io| {
        if let IODirection::Out(val) = io {
            if val == 2 {
                if found_oxygen(cur_path.len() - 1) {
                    should_stop = true;
                    unseen_positions.clear();
                }
            }

            if val == 2 || val == 1 {
                if let PathDir::TurningBack = path_dir {
                    unseen_positions.remove(&cur_pos);
                    // we reached the end of the path and found an open space, add all to unseen and then go back to start
                    if let Some(_) = seen.insert(cur_pos, Some(cur_path.clone())) {
                        panic!("seeing the same open space again monkaS!");
                    }
                    add_surrounding_to_unseen(&cur_pos, &mut unseen_paths, &mut unseen_positions, &mut seen);
                    if unseen_positions.len() == 0 {
                        should_stop = true;
                    } else {
                        found_path(cur_path.len());
                    }
                    path_dir = PathDir::Reverse;
                }
            } else {
                let has_node_been_seen = seen.get(&cur_pos);
                if let Some(val) = has_node_been_seen {
                    if let Some(_) = val {
                        panic!("Wall was reported but node was seen before in same position. {:?}", cur_pos);
                    }
                } else {
                    unseen_positions.remove(&cur_pos);
                    if let Some(_) = seen.insert(cur_pos, None) {
                        panic!("seeing the same wall again monkaS!");
                    } // new wall
                    cur_pos = translate_pos(&cur_pos, cur_path[path_index].parent_dir);
                    path_index -= 1;
                }

                if let PathDir::TurningBack = path_dir {
                    // we reached the end of the path and found a wall, go back to start
                    path_dir = PathDir::Reverse;
                } else {
                    panic!("wtf");
                }
            }
            IOResponse::None
        } else {
            let moving_to = if path_dir == PathDir::Forward {
                if path_index as i32 == cur_path.len() as i32 - 2_i32 {
                    path_dir = PathDir::TurningBack;
                    // handling in input above based on whether we hit wall or open space
                }
                path_index += 1;
                let next = cur_path[path_index];
                cur_pos = next.pos;
                dir_to_prev = next.parent_dir;
                reverse(&next.parent_dir)
            } else {
                if path_index == 0 {
                    if should_stop || unseen_paths.len() == 0 {
                        path_dir = PathDir::Forward;
                        north()
                    } else {
                        cur_path = unseen_paths.pop().unwrap();
                        if path_index as i32 == cur_path.len() as i32 - 2_i32 {
                            path_dir = PathDir::TurningBack;
                        } else {
                            path_dir = PathDir::Forward;
                        }
                        path_index += 1;
                        let next = cur_path[path_index];
                        cur_pos = next.pos;
                        dir_to_prev = next.parent_dir;
                        reverse(&next.parent_dir)
                    }
                } else {
                    let cur = cur_path[path_index];
                    path_index -= 1;
                    let next = cur_path[path_index];
                    cur_pos = next.pos;

                    dir_to_prev = reverse(&cur.parent_dir);
                    cur.parent_dir
                }
            };
            let dir = match moving_to {
                DroidDirection::NORTH(dir_val) => dir_val,
                DroidDirection::SOUTH(dir_val) => dir_val,
                DroidDirection::WEST(dir_val) => dir_val,
                DroidDirection::EAST(dir_val) => dir_val
            } as i128;
            if should_stop {
                IOResponse::Halt(dir)
            } else {
                IOResponse::Output(dir)
            }
        }
    });
}

/* IntCode computer FailFish */

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

#[derive(Eq, PartialEq)]
enum IODirection {
    In,
    Out(i128),
}

#[derive(Eq, PartialEq)]
enum IOResponse {
    None,
    Output(i128),
    Halt(i128),
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

fn run_simulation<T>(memory: &mut std::collections::HashMap<usize, i128>, mut io: T)
    where T: FnMut(IODirection) -> IOResponse,
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
            if let IOResponse::Halt(_) = input {
                break;
            } else if let IOResponse::Output(val) = input {
                memory.insert(dest as usize, val);
            } else {
                panic!("Cannot return None from an input command.")
            }
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
