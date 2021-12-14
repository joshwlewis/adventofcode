use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::cmp;
use std::fmt;
use std::collections::HashSet;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let grid: Grid = input.parse().unwrap();
    let grid1 = grid.fold_once();
    println!("first fold dots: {}", grid1.dots.len());
    let final_grid = grid.fold();
    println!("last fold result:\n{}", final_grid)
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Dot {
    x: usize,
    y: usize,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Fold {
    dim: char,
    loc: usize,
}

#[derive(Debug,Clone)]
struct Grid {
    dots: HashSet<Dot>,
    folds: Vec<Fold>,
    width: usize,
    height: usize,
}

impl Grid {
    fn fold(self) -> Grid {
        (0..self.folds.len()).fold(self, |g, _| g.fold_once())
    }

    fn fold_once(&self) -> Grid {
        let mut folds = self.folds.clone();
        let fold = folds.pop().unwrap();
        let mut dots = HashSet::new();
        for dot in self.dots.iter() {
            match (fold.dim, fold.loc) {
                ('x', n) if dot.x > n => {
                    dots.insert(Dot{x: n-(dot.x-n), y: dot.y});
                },
                ('y', n) if dot.y > n => {
                    dots.insert(Dot{x: dot.x, y: n-(dot.y-n)});
                }
                _ => { dots.insert(*dot); }
            }
        }
        let (width, height) = match fold.dim {
            'x' => (fold.loc, self.height),
            'y' => (self.width, fold.loc),
            _ => (self.width, self.height),
        };
        Grid{folds, dots, width, height}
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                match self.dots.get(&Dot{x,y}) {
                    Some(_) => out.push('▦'),
                    None => out.push('·'),
                }
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}

impl FromStr for Grid {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dots = HashSet::new();
        let mut folds = vec![];
        let mut xmax = 0;
        let mut ymax = 0;
        for line in s.split('\n').map(|n| n.trim()) {
            if line.is_empty() { continue; }
            if line.contains("fold") {
                let mut pair = line.split(' ').nth(2).unwrap().split('=');
                let dim: char = pair.next().unwrap().parse().unwrap();
                let loc: usize = pair.next().unwrap().parse().unwrap();
                folds.push(Fold{dim,loc});
            } else {
                let mut pair = line.split(',');
                let x: usize = pair.next().unwrap().parse().unwrap();
                let y: usize = pair.next().unwrap().parse().unwrap();
                xmax = cmp::max(x,xmax);
                ymax = cmp::max(y,ymax);

                dots.insert(Dot{x,y});
            }
        }
        folds.reverse();
        Ok(Grid{ dots, folds, width: xmax+1,height: ymax+1 })
    }
}
