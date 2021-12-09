use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let floor: Floor = input.parse().unwrap();
    println!("Floor Size: {}x{}", floor.width, floor.depths.len() / floor.width);
    let lows = floor.find_lows();
    let risk_sum: usize = risk_sum(lows.clone());
    println!("Lows: {:?}, Risk Sum: {}", lows, risk_sum);
}

fn risk_sum(depths: Vec<usize>) -> usize {
    depths.iter().sum::<usize>() + depths.iter().count()
}

#[derive(Debug,Clone)]
struct Floor {
    width: usize,
    depths: Vec<usize>
}

impl Floor {
    fn find_lows(&self) -> Vec<usize> {
        let mut lows = vec![];
        'd: for (i, depth) in self.depths.iter().enumerate() {
            for adj in self.get_adjacent_depths(i) {
                if adj <= *depth {
                    continue 'd;
                }
            }
            lows.push(*depth);
        }
        lows
    }

    fn get_adjacent_depths(&self, i: usize) -> Vec<usize> {
        let mut adj_depths = vec![];
        if i % self.width != 0 {
            adj_depths.push(self.depths[i-1]);
        }
        if i % self.width != self.width - 1 {
            adj_depths.push(self.depths[i+1]);
        }
        if i / self.width != 0 {
            adj_depths.push(self.depths[i-self.width])
        }
        if i / self.width < (self.depths.len() / self.width)-1 {
            adj_depths.push(self.depths[i+self.width])
        }
        adj_depths
    }
}
impl FromStr for Floor{
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut depths = vec![];
        let mut width = 0;
        for (i, l) in s.split('\n').enumerate() {
            for c in l.chars() {
                if i == 1 { width += 1 }
                let depth: usize = c.to_digit(10).unwrap() as usize;
                depths.push(depth);
            }
        }

        Ok(Floor{width: width, depths: depths})
    }
}
