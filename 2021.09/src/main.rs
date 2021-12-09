use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

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
    let basins = floor.find_basins(lows);
    let basin_sum = largest_basin_prod(basins.clone());
    println!("Basins: {:?}, Size Prod: {}", basins, basin_sum);
}

fn risk_sum(lows: Vec<(usize, usize)>) -> usize {
    let depths = lows.iter().map(|l| l.1);
    depths.clone().sum::<usize>() + depths.count()
}

fn largest_basin_prod(basins: Vec<usize>) -> usize {
    let mut bs = basins;
    bs.sort_unstable();
    bs[bs.len()-3..bs.len()].iter().product()
}

#[derive(Debug,Clone)]
struct Floor {
    width: usize,
    depths: Vec<usize>
}

impl Floor {
    fn find_lows(&self) -> Vec<(usize, usize)> {
        let mut lows = vec![];
        'd: for (i, depth) in self.depths.iter().enumerate() {
            for j in self.get_adjacent_indices(i) {
                if self.depths[j] <= *depth {
                    continue 'd;
                }
            }
            lows.push((i,*depth));
        }
        lows
    }

    fn find_basins(&self, lows: Vec<(usize, usize)>) -> Vec<usize> {
        let mut basins = vec![];
        for (i, _) in lows {
            let mut size = 1;
            let mut queue: VecDeque<usize> = VecDeque::new();
            let mut checked = HashSet::from([i]);
            for j in self.get_adjacent_indices(i) {
                queue.push_back(j);
            }
            loop {
                let j: usize;
                match queue.pop_front() {
                    Some(n) => { j = n; },
                    None => { break; },
                }
                if checked.get(&j).is_some() { continue; }
                if self.depths[j] == 9 {
                    continue;
                }
                size += 1;
                checked.insert(j);
                for k in self.get_adjacent_indices(j) {
                    queue.push_back(k);
                }
            }
            basins.push(size);
        }
        basins
    }

    fn get_adjacent_indices(&self, i: usize) -> Vec<usize> {
        let mut adj_idxs = vec![];
        if i % self.width != 0 {
            adj_idxs.push(i-1);
        }
        if i % self.width != self.width - 1 {
            adj_idxs.push(i+1);
        }
        if i / self.width != 0 {
            adj_idxs.push(i-self.width)
        }
        if i / self.width < (self.depths.len() / self.width)-1 {
            adj_idxs.push(i+self.width)
        }
        adj_idxs
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

        Ok(Floor{width, depths})
    }
}
