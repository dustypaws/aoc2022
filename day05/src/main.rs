use std::{collections::VecDeque, fs::read_to_string, vec};

const STACK_STEP_WIDTH: usize = 4;

fn main() {
    let file = read_to_string("input.txt").expect("Unable to read input");

    let mut stack = read_stack(&file);

    let instruction_start = find_data_divider(&file) + 1;

    /*
     * Part 1
     */

    for line in file.lines().skip(instruction_start) {
        let (amount, from, to) = decode_instructions(line);
        stack_exec_step(&mut stack, amount, from - 1, to - 1);
    }

    // Collect last items of each stack
    let mut last_items: Vec<String> = vec![];
    for s in stack {
        let last = s.clone().pop_back().expect("Unable to retrieve last item");
        last_items.push(last.to_string());
    }
    let joined = last_items.concat();
    println!("Part 1: {joined}");

    /*
     * Part 2
     */

    // Reset stack for part 2...yeah, it's ugly...
    let mut stack = read_stack(&file);

    for line in file.lines().skip(instruction_start) {
        let (amount, from, to) = decode_instructions(line);
        stack_preserving_order(&mut stack, amount, from - 1, to - 1);
    }

    // Collect last items of each stack
    let mut last_items: Vec<String> = vec![];
    for s in stack {
        let last = s.clone().pop_back().expect("Unable to retrieve last item");
        last_items.push(last.to_string());
    }
    let joined = last_items.concat();
    println!("Part 2: {joined}");
}

fn stack_preserving_order(stack: &mut Vec<VecDeque<char>>, amount: usize, from: usize, to: usize) {
    let mut tmp_cache: Vec<char> = vec![];
    for _ in 0..amount {
        tmp_cache.push(stack[from].pop_back().expect("Failed to pop from stack"));
    }
    for c in tmp_cache.iter().rev() {
        stack[to].push_back(*c);
    }
}
fn stack_exec_step(stack: &mut Vec<VecDeque<char>>, amount: usize, from: usize, to: usize) {
    for _ in 0..amount {
        let f = stack[from].pop_back().expect("Failed to pop from stack");
        stack[to].push_back(f);
    }
}

fn decode_instructions(inst: &str) -> (usize, usize, usize) {
    let insts = inst.split(' ').collect::<Vec<_>>();

    let amount = insts[1].parse().expect("Unable to parse amount");
    let from = insts[3].parse().expect("Unable to parse from");
    let to = insts[5].parse().expect("Unable to parse to");

    (amount, from, to)
}

fn find_data_divider(file: &String) -> usize {
    let mut index_loc = 0;
    for (i, l) in file.lines().enumerate() {
        if l.is_empty() {
            index_loc = i;
            break;
        }
    }
    index_loc
}

fn read_stack(file: &String) -> Vec<VecDeque<char>> {
    let input: Vec<&str> = file.lines().collect();

    // "- 1" to remove the column numbers which aren't necessary.
    let index_loc = find_data_divider(file) - 1;

    let mut stack: Vec<VecDeque<char>> = vec![];
    for l in input[0..index_loc].iter().rev() {
        let mut cur_stack = l
            .chars()
            .map(|c| if c == '[' || c == ']' { ' ' } else { c })
            .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
            .collect::<VecDeque<char>>();

        // Remove the 1st element as it's empty and messes with step-width
        cur_stack.pop_front();
        stack.push(cur_stack);
    }

    let mut final_stack: Vec<VecDeque<char>> = vec![VecDeque::new(); stack.len() + 1];
    for s in stack {
        for (y, c) in s.iter().step_by(STACK_STEP_WIDTH).enumerate() {
            if !c.is_whitespace() {
                final_stack[y].push_back(*c);
            }
        }
    }

    final_stack
}
