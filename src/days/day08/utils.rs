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

    pub fn ghost_steps(&self) -> usize {
        let nodes: Vec<_> = self
            .nodes
            .keys()
            .filter(|k| k.ends_with("A"))
            .map(|k| k.clone())
            .collect();
        let endings = nodes.iter().map(|node| self.find_cycle(node));
        *all_least_common_multiples(endings.collect())
            .iter()
            .min()
            .unwrap()
    }

    fn find_cycle(&self, node: &str) -> Vec<usize> {
        use std::collections::HashSet;
        let mut ret = Vec::with_capacity(500);
        let mut current_node = node.to_string();
        let mut map = HashSet::<(String, &[u8])>::new();
        let mut cur_path = self.seq.as_bytes();
        let mut iter = 0;
        while map.insert((current_node.clone(), cur_path)) {
            let new_move = cur_path[0];
            iter += 1;
            cur_path = match cur_path {
                [_] => self.seq.as_bytes(),
                other => &other[1..],
            };
            let this_node = self.nodes[current_node.as_str()].clone();
            current_node = match new_move {
                b'L' => this_node.0,
                b'R' => this_node.1,
                other => panic!("seq has unexpected character {:?}", other),
            };
            if current_node.ends_with("Z") {
                ret.push(iter);
            }
        }
        ret
    }
}

use num::Integer;
use num::Unsigned;

fn lcm<T: Unsigned + Integer>(fst: &T, snd: &T) -> T {
    (*fst).lcm(snd)
}

fn merge_lcm(acc: &Vec<usize>, v: &Vec<usize>) -> Vec<usize> {
    if acc.is_empty() {
        return v.clone();
    }
    if v.is_empty() {
        return acc.clone();
    }
    acc.iter()
        .flat_map(|elem| v.iter().map(|other| lcm(elem, other)))
        .collect()
}

fn all_least_common_multiples(matrix: Vec<Vec<usize>>) -> Vec<usize> {
    matrix
        .iter()
        .fold(vec![], |acc, elem| merge_lcm(&acc, elem))
}
