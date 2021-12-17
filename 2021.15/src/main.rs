use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::cmp::Ordering;
use std::fmt;
use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let cave: Cave = input.parse().unwrap();
    let lowest_risk = cave.lowest_risk();
    println!("lowest risk: {}", lowest_risk);
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Loc {
    x: usize,
    y: usize,
}

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
struct Node {
    loc: Loc,
    risk: usize,
}

#[derive(Debug,Clone,PartialEq,Eq)]
struct Cave {
    height: usize,
    width: usize,
    grid: HashMap<Loc, usize>
}

impl Cave {
    fn lowest_risk(&self) -> usize {
        let mut pq: BinaryHeap<Node> = BinaryHeap::new();
        pq.push(Node {
            loc: Loc{x:0, y:1},
            risk: *self.map.get(&Point::new(0, 1)).unwrap(),
        });
        pq.push(Node {
            pos: Point::new(1, 0),
            cost: *self.map.get(&Point::new(1, 0)).unwrap(),
        });
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk).then_with(||
            match self.loc.x.cmp(&other.loc.x) {
                Ordering::Equal => self.loc.y.cmp(&other.loc.y),
                ordering => ordering,
            }
        )
    }
}

impl FromStr for Cave {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();
        let lines = s.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty());
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().chars().count();
        for (y, line) in s.split('\n').map(|n| n.trim()).enumerate() {
            if line.is_empty() { continue; }
            for (x, chr) in line.chars().enumerate() {
                let risk = chr.to_digit(10).unwrap();
                grid.insert(Loc{x,y}, risk as usize);
            }
        }
        Ok(Cave{ grid, width, height })
    }
}
