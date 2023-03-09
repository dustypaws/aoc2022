use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input.txt").expect("Unable to read input");

    /*
     * Part 1
     */
    let mut chars: Vec<char> = vec![];
    for line in file.lines() {
        let (one, two) = line.split_at(line.len() / 2);

        'inner: for c in one.chars() {
            if two.contains(c) {
                chars.push(c);
                break 'inner;
            } else {
                continue;
            };
        }
    }

    println!("Part 1: {}", calc(chars));

    /*
     * Part 2
     */
    let mut tmp_str: Vec<_> = vec![];
    let mut intersections = vec![];
    for line in file.lines() {
        // "Chunk" into a vector of three
        if tmp_str.len() == 3 {
            tmp_str = vec![];
        }
        tmp_str.push(line);

        if tmp_str.len() == 3 {
            let intersect = tmp_str[0]
                .chars()
                .filter(|&c| tmp_str[1].contains(c))
                .filter(|&c| tmp_str[2].contains(c))
                .collect::<Vec<char>>();
            intersections.push(intersect[0]);
        }
    }

    println!("Part 2: {}", calc(intersections));
}

fn calc(chars: Vec<char>) -> u32 {
    let mut sum = 0;
    for c in chars {
        if (c as u32) >= 97 {
            sum += (c as u32) - 96;
        } else {
            sum += (c as u32) - 64 + 26;
        }
    }
    sum
}
