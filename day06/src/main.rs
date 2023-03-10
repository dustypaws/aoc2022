use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug)]
struct Code {
    buf: VecDeque<char>,
    msg: VecDeque<char>,
}

impl Code {
    fn new() -> Self {
        Self {
            buf: VecDeque::with_capacity(4),
            msg: VecDeque::with_capacity(14),
        }
    }

    fn rot_and_append(&mut self, c: char, t: &str) {
        let v = if t == "start" {
            &mut self.buf
        } else {
            &mut self.msg
        };
        let l = if t == "start" { 4 } else { 14 };

        if v.len() < l {
            v.push_back(c);
        } else {
            v.pop_front();
            v.push_back(c);
        }
    }

    fn is_unique(&self, t: &str) -> bool {
        let v = if t == "start" { &self.buf } else { &self.msg };
        let l = if t == "start" { 4 } else { 14 };

        if v.len() < l {
            return false;
        }
        for (x, a) in v.iter().enumerate() {
            for (y, b) in v.iter().enumerate() {
                if x == y {
                    continue;
                }
                if a == b {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let file = read_to_string("input.txt").expect("Unable to read input");

    let mut cod = Code::new();
    let mut start_token_pos = 0;

    for (i, c) in file.chars().enumerate() {
        // Looking for "Start-Token"
        cod.rot_and_append(c, "start");
        if cod.is_unique("start") {
            println!("Part 1: {:?} at position: {}", cod.buf, i + 1);
            start_token_pos = i + 1;
            break;
        }
    }
    for (i, c) in file.chars().enumerate().skip(start_token_pos) {
        // Looking for "Message-Start-Token"
        cod.rot_and_append(c, "msg");
        if cod.is_unique("msg") {
            println!("Part 2: {:?} at position: {}", cod.msg, i + 1);
            break;
        }
    }
}
