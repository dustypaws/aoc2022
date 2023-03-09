use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read input");

    let mut score_1 = 0;
    let mut score_2 = 0;
    for line in input.lines() {
        let (opponent_move, player_move) = line.split_once(" ").expect("Malformed instruction");

        let right_col = match player_move {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => unreachable!("Bad player move"),
        };

        let left_col = match opponent_move {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => unreachable!("Bad opponent move"),
        };

        score_1 += right_col
            // Tie
            + if left_col == right_col {
                3
            // Win
            } else if right_col == 1 && left_col == 3
                || right_col == 2 && left_col == 1
                || right_col == 3 && left_col == 2
            {
                6
            // Loose
            } else {
                0
            };

        score_2 += if right_col == 1 {
            // Loose
            if left_col == 3 {
                2
            } else if left_col == 2 {
                1
            } else {
                3
            }
        } else if right_col == 2 {
            // Tie
            left_col + 3
        } else {
            // Win
            if left_col == 1 {
                8
            } else if left_col == 2 {
                9
            } else {
                7
            }
        };
    }

    /*
     * Part 1
     */

    println!("Part 1: {score_1}");

    /*
     * Part 2
     */

    println!("Part 2: {score_2}");
}
