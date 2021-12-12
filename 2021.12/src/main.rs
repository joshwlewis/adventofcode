use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let c: Caves = input.parse().unwrap();
    println!("{:?}", c);
    let no_small_revisit_paths = c.paths_with_no_small_revisits().len();
    println!("no small revisit paths: {}", no_small_revisit_paths);
    let one_small_revisit_paths = c.paths_with_one_small_revisit().len();
    println!("one small revisit paths: {}", one_small_revisit_paths);
}

#[derive(Debug,Clone)]
struct Caves {
    map: HashMap<String, HashSet<String>>,
}

impl Caves {
    fn paths_with_no_small_revisits(&self) -> HashSet<Vec<&str>> {
        let mut complete_paths = HashSet::new();
        let mut partial_paths = VecDeque::from([vec!["start"]]);
        while let Some(path) = partial_paths.pop_front() {
            let cave = path.last().unwrap();
            'n: for next in self.map.get(*cave).unwrap() {
                if next == &next.to_lowercase() {
                    for c in path.iter() {
                        if c == next { continue 'n; }
                    }
                }
                let mut new_path = path.clone();
                new_path.push(next);
                if next == "end" {
                    complete_paths.insert(new_path);
                } else {
                    partial_paths.push_back(new_path)
                }
            }
        }
        complete_paths
    }

    fn paths_with_one_small_revisit(&self) -> HashSet<Vec<&str>> {
        let mut complete_paths = HashSet::new();
        let mut partial_paths = VecDeque::from([(vec!["start"], "")]);
        while let Some((path, revisit)) = partial_paths.pop_front() {
            let cave = path.last().unwrap();
            'n: for next in self.map.get(*cave).unwrap() {
                let next_is_small = next == &next.to_lowercase() && next.chars().count() == 2;
                if next_is_small {
                    let max_visits = if next == revisit { 1 } else { 0 };
                    let mut visited = 0;
                    for p in path.iter() {
                        if p == next { visited += 1 }
                        if visited > max_visits { continue 'n; }
                    }
                }
                let mut new_path = path.clone();
                new_path.push(next);
                match (&next[..], next_is_small, revisit) {
                    ("start", _, _) => {
                        continue 'n;
                    }
                    ("end", _, _) => { 
                        complete_paths.insert(new_path); 
                    },
                    (_, true, "") => {
                        partial_paths.push_back((new_path.clone(), ""));
                        partial_paths.push_back((new_path, next));
                    },
                    (_, true, _) => {
                        partial_paths.push_back((new_path, revisit));
                    },
                    (_, false, _) => {
                        partial_paths.push_back((new_path, revisit));
                    },
                }
            }
        }
        complete_paths
    }
}

impl FromStr for Caves {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        for line in s.trim().split('\n').map(|s| s.trim()) {
            let mut halves = line.split('-');
            let left = String::from(halves.next().unwrap());
            let right = String::from(halves.next().unwrap());
            match map.get_mut(&left) {
                Some(set) => { set.insert(right.clone()); },
                None => { map.insert(left.clone(), HashSet::from([right.clone()])); },
            }
            match map.get_mut(&right) {
                Some(set) => { set.insert(left); },
                None => { map.insert(right, HashSet::from([left])); },
            }

        }
        Ok(Caves{map})
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn read_caves(f: &str) -> Caves {
        let c = fs::read_to_string(f).expect("Error opening sample file");
        c.parse().unwrap()
    }

    #[test]
    fn test_small_path_no_revisit_count() {
        let c = read_caves("small_sample.txt");
        let ps = c.paths_with_no_small_revisits();
        assert_eq!(ps.len(), 10);
    }

    #[test]
    fn test_medium_path_no_revisit_count() {
        let c = read_caves("medium_sample.txt");
        let ps = c.paths_with_no_small_revisits();
        assert_eq!(ps.len(), 19);
    }

    #[test]
    fn test_large_path_no_revisit_count() {
        let c = read_caves("large_sample.txt");
        let ps = c.paths_with_no_small_revisits();
        assert_eq!(ps.len(), 226);
    }

    #[test]
    fn test_small_path_one_revisit_count() {
        let c = read_caves("small_sample.txt");
        let ps = c.paths_with_one_small_revisit();
        assert_eq!(ps.len(), 36);
    }

}
