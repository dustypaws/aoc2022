use std::fs::read_to_string;

#[derive(Debug, PartialEq, PartialOrd)]
struct Cals {
    elv: usize,
    cal: Vec<usize>,
    sum: usize,
}

fn main() {
    let file = read_to_string("input.txt").expect("Unable to open input file.");

    let mut all_elves = Vec::new();
    let mut food = Vec::new();
    let mut cnt = 0;

    for line in file.lines() {
        if line.is_empty() {
            all_elves.push(Cals {
                elv: cnt,
                cal: food.clone(),
                sum: food.iter().sum(),
            });
            food = Vec::new();
            cnt += 1;
            continue;
        }

        food.push(line.parse::<usize>().expect("Unable to parse to usize."));
    }

    all_elves.sort_by_key(|item| item.sum);

    let top_three: usize = all_elves
        .iter()
        .rev()
        .take(3)
        .map(|x| x.sum)
        .into_iter()
        .sum();

    println!(
        "Part 1: {:?}",
        all_elves.last().expect("No `sum` found").sum
    );

    println!("Part 2: {:?}", top_three);
}
