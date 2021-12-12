use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::fmt;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let mut octo: Octopi = input.parse().unwrap();
    let flashes_after_100 = octo.clone().flashes_after_n_steps(100);
    println!("Flashes after 100: {}", flashes_after_100);
    let steps = octo.steps_to_sync();
    println!("Steps to synchronize: {}", steps);
}

#[derive(Debug,Clone)]
struct Octopi {
    height: usize,
    width: usize,
    energy: HashMap<Loc, usize>,
}

impl Octopi {
    fn flashes_after_n_steps(&mut self, steps: usize) -> usize {
        (0..steps).map(|_| self.step()).sum()
    }

    fn steps_to_sync(&mut self) -> usize {
        let mut steps = 1;
        let target = self.width * self.height;
        while self.step() != target {
            steps +=1;
        }
        steps
    }

    fn step(&mut self) -> usize {
        let mut queue: VecDeque<Loc> = VecDeque::new();
        let mut primed: HashSet<Loc> = HashSet::new();
        for (loc, e) in self.energy.iter_mut() {
            if *e < 10 { *e += 1 }
            if *e == 10 {
                queue.push_back(*loc);
                primed.insert(*loc);
            }
        }
        while let Some(loc) = queue.pop_front() {
            for near in self.nearby(loc) {
                let e = self.energy.get_mut(&near).unwrap();
                if *e < 10 { *e += 1 }
                if *e == 10 && !primed.contains(&near) {
                    queue.push_back(near);
                    primed.insert(near);
                }
            }
        }
        for loc in primed.iter() {
            self.energy.insert(*loc, 0);
        }
        primed.len()
    }

    fn nearby(&self, loc: Loc) -> Vec<Loc> {
        let mut nearby = Vec::new();
        let ds = [(1,1),(1,0),(1,-1),(0,1),(0,-1),(-1,1),(-1,0),(-1,-1)];
        for (dx,dy) in ds {
            let x = loc.x as isize + dx;
            let y = loc.y as isize + dy;
            if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
                continue;
            }
            nearby.push(Loc{x: x as usize, y: y as usize});
        }
        nearby
    }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
struct Loc {
    x: usize,
    y: usize,
}

impl FromStr for Octopi {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut energy = HashMap::new();
        let lines = s.trim().split('\n');
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().chars().count();
        for (y, line) in lines.enumerate() {
            for (x, chr) in line.chars().enumerate() {
                let e = chr.to_digit(10).unwrap() as usize;
                energy.insert(Loc{x,y}, e);
            }
        }
        Ok(Octopi{ energy, width, height })
    }
}
impl fmt::Display for Octopi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for y in 0..self.height {
            let mut line = String::new();

            for x in 0..self.width {
                let num = *self.energy.get(&Loc{x, y}).unwrap();
                line.push(char::from_digit(num as u32, 10).unwrap());
            }

            out.push_str(&line);
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}
