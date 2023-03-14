fn main() {
    let file = std::fs::read_to_string("src/day07in.txt").expect("File not found!");
    let instructions: Vec<i64> = file.split(',').into_iter()
        .map(|i| i.trim().parse().expect("Can't parse as int"))
        .collect();
    println!("part 1: {}", part1(&instructions));
    println!("part 2: {}", part2(&instructions));
}

// inverted from problem, mode = true means load from memory
struct Instruction {
    op: i64,
    mode1: bool,
    mode2: bool,
}

fn parse_instruction(val: i64) -> Instruction {
    let str = format!("{:05}", val);
    Instruction {
        op: (str[3..5]).parse().unwrap(),
        mode1: (str[2..3]).parse::<i64>().unwrap() == 0,
        mode2: (str[1..2]).parse::<i64>().unwrap() == 0,
    }
}

fn run_simulation(memory: &mut Vec<i64>, phase: i64, input: i64) -> i64 {
    let mut i = 0;
    let mut instruction = parse_instruction(memory[i]);
    let mut last_output = 0;
    let mut input_count = 0;
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
            memory[dest] = if input_count == 0 { phase } else { input };
            input_count += 1;
            i += 2;
        } else if instruction.op == 4 {
            let output = if instruction.mode1 { memory[memory[i + 1] as usize] } else { memory[i + 1] };
            last_output = output;
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

struct AmpState<'a> {
    memory: &'a mut Vec<i64>,
    phase: i64,
    input: i64,
    instruction: usize,
    use_phase: bool,
}

fn run_looping_simulation(state: &mut AmpState) -> (i64, StopCondition) {
    let mut instruction = parse_instruction(state.memory[state.instruction]);
    while instruction.op != 99 {
        if instruction.op < 3 {
            let x = if instruction.mode1 { state.memory[state.memory[state.instruction + 1] as usize] } else { state.memory[state.instruction + 1] };
            let y = if instruction.mode2 { state.memory[state.memory[state.instruction + 2] as usize] } else { state.memory[state.instruction + 2] };
            let dest = state.memory[state.instruction + 3] as usize;
            if instruction.op == 1 {
                state.memory[dest] = x + y;
            } else {
                state.memory[dest] = x * y;
            }
            state.instruction += 4;
        } else if instruction.op == 3 {
            // input
            let dest = state.memory[state.instruction + 1] as usize;
            state.memory[dest] = if state.use_phase { state.phase } else { state.input };
            state.use_phase = false;
            state.instruction += 2;
        } else if instruction.op == 4 {
            let output = if instruction.mode1 { state.memory[state.memory[state.instruction + 1] as usize] } else { state.memory[state.instruction + 1] };
            state.instruction += 2;
            return (output, StopCondition::Output);
        } else if instruction.op == 5 { // bnez
            let x = if instruction.mode1 { state.memory[state.memory[state.instruction + 1] as usize] } else { state.memory[state.instruction + 1] };
            let y = if instruction.mode2 { state.memory[state.memory[state.instruction + 2] as usize] } else { state.memory[state.instruction + 2] };
            if x == 0 {
                state.instruction += 3;
            } else {
                state.instruction = y as usize;
            }
        } else if instruction.op == 6 { // beqz
            let x = if instruction.mode1 { state.memory[state.memory[state.instruction + 1] as usize] } else { state.memory[state.instruction + 1] };
            let y = if instruction.mode2 { state.memory[state.memory[state.instruction + 2] as usize] } else { state.memory[state.instruction + 2] };
            if x != 0 {
                state.instruction += 3;
            } else {
                state.instruction = y as usize;
            }
        } else if instruction.op == 7 { // lt
            let x = if instruction.mode1 { state.memory[state.memory[state.instruction + 1] as usize] } else { state.memory[state.instruction + 1] };
            let y = if instruction.mode2 { state.memory[state.memory[state.instruction + 2] as usize] } else { state.memory[state.instruction + 2] };
            let dest = state.memory[state.instruction + 3] as usize;
            if x < y {
                state.memory[dest] = 1;
            } else {
                state.memory[dest] = 0;
            }
            state.instruction += 4;
        } else if instruction.op == 8 { // lt
            let x = if instruction.mode1 { state.memory[state.memory[state.instruction + 1] as usize] } else { state.memory[state.instruction + 1] };
            let y = if instruction.mode2 { state.memory[state.memory[state.instruction + 2] as usize] } else { state.memory[state.instruction + 2] };
            let dest = state.memory[state.instruction + 3] as usize;
            if x == y {
                state.memory[dest] = 1;
            } else {
                state.memory[dest] = 0;
            }
            state.instruction += 4;
        }
        instruction = parse_instruction(state.memory[state.instruction]);
    }
    (0, StopCondition::Halt)
}

enum StopCondition {
    Output,
    Halt,
}

// stolen from https://rosettacode.org/wiki/Determine_if_a_string_has_all_unique_characters#Rust
fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn part1(instructions: &Vec<i64>) -> i64 {
    let mut max = 0;
    for x in 1000..100000 {
        let x_str = x.to_string();
        if x < 10000 && x_str.contains('0') {
            continue;
        }

        if let None = unique(x_str.as_str()) {
            let a = (x % 100000 - x % 10000) / 10000;
            let b = (x % 10000 - x % 1000) / 1000;
            let c = (x % 1000 - x % 100) / 100;
            let d = (x % 100 - x % 10) / 10;
            let e = (x % 10 - x % 1) / 1;
            if a < 5 && b < 5 && c < 5 && d < 5 && e < 5 {
                let result_a = run_simulation(&mut instructions.clone(), a, 0);
                let result_b = run_simulation(&mut instructions.clone(), b, result_a);
                let result_c = run_simulation(&mut instructions.clone(), c, result_b);
                let result_d = run_simulation(&mut instructions.clone(), d, result_c);
                let result_e = run_simulation(&mut instructions.clone(), e, result_d);
                // let result = run_simulation(memory, a, b, c, d, e);
                if result_e > max {
                    max = result_e;
                }
            }
        }
    }
    max
}

fn part2(instructions: &Vec<i64>) -> i64 {
    let mut max = 0;
    for x in 1000..100000 {
        let x_str = x.to_string();

        if let None = unique(x_str.as_str()) {
            let a = (x % 100000 - x % 10000) / 10000;
            let b = (x % 10000 - x % 1000) / 1000;
            let c = (x % 1000 - x % 100) / 100;
            let d = (x % 100 - x % 10) / 10;
            let e = (x % 10 - x % 1) / 1;
            if a >= 5 && b >= 5 && c >= 5 && d >= 5 && e >= 5 {
                let mut a_state = AmpState {
                    memory: &mut instructions.clone(),
                    phase: a,
                    input: 0,
                    instruction: 0,
                    use_phase: true,
                };
                let a_stop = run_looping_simulation(&mut a_state).0;
                let mut b_state = AmpState {
                    memory: &mut instructions.clone(),
                    phase: b,
                    input: a_stop,
                    instruction: 0,
                    use_phase: true,
                };
                let b_stop = run_looping_simulation(&mut b_state).0;
                let mut c_state = AmpState {
                    memory: &mut instructions.clone(),
                    phase: c,
                    input: b_stop,
                    instruction: 0,
                    use_phase: true,
                };
                let c_stop = run_looping_simulation(&mut c_state).0;
                let mut d_state = AmpState {
                    memory: &mut instructions.clone(),
                    phase: d,
                    input: c_stop,
                    instruction: 0,
                    use_phase: true,
                };
                let d_stop = run_looping_simulation(&mut d_state).0;
                let mut e_state = AmpState {
                    memory: &mut instructions.clone(),
                    phase: e,
                    input: d_stop,
                    instruction: 0,
                    use_phase: true,
                };
                let mut e_stop = run_looping_simulation(&mut e_state);
                let mut last_e_output = e_stop.0;
                while let StopCondition::Output = e_stop.1 {
                    last_e_output = e_stop.0;
                    a_state.input = e_stop.0;
                    b_state.input = run_looping_simulation(&mut a_state).0;
                    c_state.input = run_looping_simulation(&mut b_state).0;
                    d_state.input = run_looping_simulation(&mut c_state).0;
                    e_state.input = run_looping_simulation(&mut d_state).0;
                    e_stop = run_looping_simulation(&mut e_state);
                }
                if last_e_output > max {
                    max = last_e_output;
                }
            }
        }
    }
    max
}
