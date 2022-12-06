use std::str::FromStr;
use std::char::from_digit;
use std::num::ParseIntError;

pub fn run() {
    let input = std::fs::read_to_string("./day5/input.txt").expect("Should be data");
    let groups: Vec<&str> = input.split("\n\n").collect();

    part_one(&groups);
    part_two(&groups);
}

fn part_one(groups: &Vec<&str>) {
    let mut stacks = generate_stacks(&groups[0]);
    let commands = generate_commands(&groups[1]);
    for command in commands.iter() {
        move_one(&mut stacks, command);
    }

    for stack in stacks.iter() {
        print!("{}", stack[stack.len() - 1]);
    }
}

fn part_two(groups: &Vec<&str>) {
    let mut stacks = generate_stacks(&groups[0]);
    let commands = generate_commands(&groups[1]);

    for command in commands.iter() {
        move_all(&mut stacks, command);
    }

    println!("\nPart two:");
    for stack in stacks.iter() {
        print!("{}", stack[stack.len() - 1]);
    }
    println!("\n");
}

fn generate_stacks(raw_stacks: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<&str> = raw_stacks.lines().collect();
    let stacks = lines.pop().unwrap();
    lines.reverse();

    let number_of_stacks: Vec<&str> = stacks.split("   ").collect();

    let mut stack_structs = number_of_stacks.iter().map(|_| Vec::new() ).collect::<Vec<Vec<char>>>();

    for line in lines.iter() {
        for stack in 1..=number_of_stacks.len() {
            let idx = stacks.find(from_digit(stack as u32, 10).unwrap()).unwrap();
            let item = line.chars().collect::<Vec<char>>()[idx];
            if item != ' ' {
                stack_structs[stack - 1].push(item);
            }
        }
    }

    stack_structs
}

fn generate_commands(raw_commands: &str) -> Vec<Command> {
    let lines: Vec<&str> = raw_commands.lines().collect();

    lines.iter().map(|line| line.parse::<Command>().unwrap()).collect::<Vec<Command>>()
}

struct Command {
    amount: u8,
    from: u8,
    to: u8,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        return Ok(Command {
            amount: split[1].parse::<u8>().unwrap(),
            from: split[3].parse::<u8>().unwrap(),
            to: split[5].parse::<u8>().unwrap()
        });
    }
}

fn move_one(stacks: &mut Vec<Vec<char>>, command: &Command) {
    for _ in 0..command.amount {
        let cargo = stacks[(command.from - 1) as usize].pop().unwrap();
        stacks[(command.to - 1) as usize].push(cargo);
    }
}

fn move_all(stacks: &mut Vec<Vec<char>>, command: &Command) {
    let amount = command.amount as usize;
    let from_stack = stacks[(command.from - 1) as usize].clone();
    let from_stack_len = from_stack.len();

    let (new_from, new_addition) = from_stack.split_at(from_stack_len - amount);

    stacks[(command.from - 1) as usize].clear();
    stacks[(command.from - 1) as usize].append(&mut new_from.to_vec());
    stacks[(command.to - 1) as usize].append(&mut new_addition.to_vec());
}
