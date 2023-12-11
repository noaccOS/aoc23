pub fn read_input() -> &'static str {
    include_str!("input")
}

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref NODE_RE: Regex =
        Regex::new(r"(?<key>\w+)\s*=\s*\((?<L>\w+),\s*(?<R>\w+)\)").unwrap();
}

pub struct Map {
    pub nodes: HashMap<String, (String, String)>,
    pub seq: String,
}

impl Map {
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let seq = lines.next().unwrap().to_string();
        lines.next().unwrap();
        let nodes: HashMap<_, _> = lines
            .map(|line| {
                let captures = NODE_RE.captures(line).unwrap();
                let key = captures.name("key").unwrap().as_str().to_string();
                let l = captures.name("L").unwrap().as_str().to_string();
                let r = captures.name("R").unwrap().as_str().to_string();
                (key, (l, r))
            })
            .collect();

        Self { nodes, seq }
    }

    pub fn steps(&self, from: &str, to: &str) -> usize {
        let mut seq = self.seq.as_bytes().iter().cycle();
        let mut key = from.to_string();
        let mut iters = 0;
        while key != to {
            iters += 1;
            let next_move = self.nodes[&key].clone();
            key = match seq.next().unwrap() {
                b'L' => next_move.0,
                b'R' => next_move.1,
                other => panic!("seq has unexpected character {:?}", other),
            };
        }
        iters
    }
}
