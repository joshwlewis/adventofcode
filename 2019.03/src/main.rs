use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn main() {
    let sin = io::stdin();
    let instructions = sin.lock().lines().filter_map(|line| match line {
        Ok(ins) => Some(parse_instruction(ins)),
        Err(_) => None,
    });

    let traces: Vec<Vec<(isize, isize)>> = instructions.map(|i| trace_instruction(i)).collect();
    let pointsets = traces
        .iter()
        .map(|p| {
            p.into_iter()
                .map(|p| *p)
                .collect::<HashSet<(isize, isize)>>()
        })
        .collect::<Vec<HashSet<(isize, isize)>>>();
    let intersections: Vec<&(isize, isize)> = pointsets[0]
        .intersection(&pointsets[1])
        .filter(|(x, y)| *x != 0 && *y != 0)
        .collect();

    let mut mdists = intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .collect::<Vec<isize>>();
    mdists.sort();
    let leastmdist: &isize = mdists.first().unwrap();
    println!("Closest manhattan distance: {:?}", leastmdist);

    let stepdista = stepdist(&traces[0]);
    let stepdistb = stepdist(&traces[1]);

    let leastsdist = intersections
        .iter()
        .map(|(x, y)| stepdista.get(&(*x, *y)).unwrap() + stepdistb.get(&(*x, *y)).unwrap())
        .min();

    println!("Closest step distance: {:?}", leastsdist);
}

pub fn stepdist(trace: &Vec<(isize, isize)>) -> HashMap<(isize, isize), usize> {
    let mut stephash: HashMap<(isize, isize), usize> = HashMap::new();
    for (i, (x, y)) in trace.iter().enumerate() {
        match stephash.get(&(*x, *y)) {
            Some(_) => (),
            None => {
                stephash.insert((*x, *y), i);
                ()
            }
        }
    }
    stephash
}
pub fn trace_instruction(ins: Vec<(char, isize)>) -> Vec<(isize, isize)> {
    let mut ppos = (0isize, 0isize);
    let mut trace = vec![(ppos.0, ppos.1)];
    for (dir, num) in ins {
        match dir {
            'U' => {
                for y in (ppos.1 + 1)..=(ppos.1 + num) {
                    ppos.1 = y;
                    trace.push((ppos.0, ppos.1));
                }
            }
            'D' => {
                for y in ((ppos.1 - num)..=(ppos.1 - 1)).rev() {
                    ppos.1 = y;
                    trace.push((ppos.0, ppos.1));
                }
            }
            'R' => {
                for x in ppos.0 + 1..=ppos.0 + num {
                    ppos.0 = x;
                    trace.push((ppos.0, ppos.1));
                }
            }
            'L' => {
                for x in ((ppos.0 - num)..=(ppos.0 - 1)).rev() {
                    ppos.0 = x;
                    trace.push((ppos.0, ppos.1));
                }
            }
            d => println!("Unknown direction: {}", d),
        }
    }
    trace
}

pub fn parse_instruction(ins: String) -> Vec<(char, isize)> {
    ins.split(',')
        .map(|i| i.trim())
        .filter_map(|i| {
            let mut ichrs = i.chars();
            match ichrs.next() {
                Some(d) => Some((d, ichrs.collect::<String>())),
                None => None,
            }
        })
        .filter_map(|(d, ns)| match ns.parse::<isize>() {
            Ok(i) => Some((d, i)),
            Err(_) => None,
        })
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let ins = parse_instruction("R75,D30,R83,U83,L12,D49,R71".to_string());
        assert_eq!(ins[0], ('R', 75));
        assert_eq!(ins[6], ('R', 71));
    }

    #[test]
    fn test_trace_instruction() {
        let trace = trace_instruction(vec![('R', 2), ('U', 2), ('L', 1), ('D', 5)]);
        assert_eq!(trace[0], (0, 0));
        assert_eq!(trace[2], (2, 0));
        assert_eq!(trace[4], (2, 2));
        assert_eq!(trace[6], (1, 1));
        assert_eq!(trace[10], (1, -3));
    }
}
