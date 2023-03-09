use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input.txt").expect("Unable to read input");

    // let mut range: Vec<char> = vec!['.'; 100];
    let mut fully_overlapping = 0;
    let mut partially_overlapping = 0;

    for line in file.lines() {
        // Split into paired ranges: xx-xx,xx-xx
        let pairs = line.split(",").collect::<Vec<_>>();
        let mut ranges = Vec::new();

        for p in pairs {
            let range: Vec<u32> = p
                .split("-")
                .map(|s| s.parse().expect("Malformed range"))
                .collect();

            ranges.push(range);
        }

        let (range1, range2) = (ranges[0].clone(), ranges[1].clone());
        if (range1[0] <= range2[0] && range1[1] >= range2[1])
            || (range2[0] <= range1[0] && range2[1] >= range1[1])
        {
            fully_overlapping += 1;
        }

        if (range1[0] <= range2[1] && range1[1] >= range2[0])
            || (range2[0] <= range1[1] && range2[1] >= range1[0])
        {
            partially_overlapping += 1;
        }
    }
    println!("Part 1: {fully_overlapping}");
    println!("Part 2: {partially_overlapping}");
}
