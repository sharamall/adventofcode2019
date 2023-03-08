fn main() {
    let file = std::fs::read_to_string("src/day02in.txt").expect("File not found!");
    let instructions: Vec<usize> = file.split(',').into_iter()
        .map(|i| i.trim().parse().expect("Can't parse as int"))
        .collect();
    println!("part 1 {}", part1(&instructions));
    println!("part 2 {}", part2(&instructions));
}

fn run_simulation(memory: &mut Vec<usize>, pos1: usize, pos2: usize) -> usize {
    let mut i = 0;
    memory[1] = pos1;
    memory[2] = pos2;
    while memory[i] != 99 {
        if memory[i] == 1 {
            let x = memory[i + 1];
            let y = memory[i + 2];
            let dest = memory[i + 3];
            memory[dest] = memory[x] + memory[y];
            i += 4
        } else {
            let x = memory[i + 1];
            let y = memory[i + 2];
            let dest = memory[i + 3];
            memory[dest] = memory[x] * memory[y];
            i += 4
        }
    }
    memory[0]
}

fn part1(instructions: &Vec<usize>) -> usize {
    let memory = &mut instructions.clone();
    run_simulation(memory, 12, 2)
}

fn part2(instructions: &Vec<usize>) -> usize {
    for pos1 in 1..100 {
        for pos2 in 1..100 {
            let memory = &mut instructions.clone();
            let result = run_simulation(memory, pos1, pos2);
            if result == 19690720 {
                return memory[1] * 100 + memory[2];
            }
        }
    }
    panic!("Didn't find a valid result")
}
