use std::io::{self,BufRead};
use std::collections::HashSet;

fn main() {
    let sin = io::stdin();
    let instructions = sin.lock().lines().filter_map(|line| {
        match line {
            Ok(ins) => Some(parse_instruction(ins)),
            Err(_) => None
        }
    });

    let points = instructions.map(|i| trace_instruction(i))
        .map(|p| p.into_iter().collect::<HashSet<(isize,isize)>>())
        .collect::<Vec<HashSet<(isize,isize)>>>();
    let intersections: Vec<&(isize,isize)> = points[0].union(&points[1]).collect();

    let mut distances = intersections.iter().map(|(x,y)| x.abs() + y.abs()).collect::<Vec<isize>>();
    distances.sort();
    let close: Vec<&isize> = distances.iter().take(12).collect();
    println!("Distances: {:?}", close);
}

pub fn trace_instruction(ins: Vec<(char,isize)>) -> Vec<(isize,isize)> {
    let mut ppos = (0isize, 0isize);
    let mut trace = vec!((ppos.0, ppos.1));
    for (dir,num) in ins {
        match dir {
            'U' => {
                for y in (ppos.1+1)..=(ppos.1+num) {
                    ppos.1 = y;
                    trace.push((ppos.0, ppos.1));
                };
            },
            'D' => {
                for y in ((ppos.1-num)..=(ppos.1-1)).rev() {
                    ppos.1 = y;
                    trace.push((ppos.0,ppos.1));
                }
            },
            'R' => {
                for x in ppos.0+1..=ppos.0+num {
                    ppos.0 = x;
                    trace.push((ppos.0,ppos.1));
                }
            },
            'L' => {
                for x in ((ppos.0-num)..=(ppos.0-1)).rev() {
                    ppos.0 = x;
                    trace.push((ppos.0, ppos.1));
                }
            },
            d => println!("Unknown direction: {}", d),
        }
        println!("pos: {:?}", ppos);
    };
    trace
}

pub fn parse_instruction(ins: String) -> Vec<(char,isize)> {
    ins.split(',')
        .map(|i| i.trim())
        .filter_map(|i| {
            let mut ichrs = i.chars();
            match ichrs.next() {
                Some(d) => Some((d, ichrs.collect::<String>())),
                None    => None,
            }
        })
        .filter_map(|(d,ns)| {
            match ns.parse::<isize>() {
                Ok(i) => Some((d, i)),
                Err(_) => None
           }
        })
        .collect()
}


mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        let ins = parse_instruction("R75,D30,R83,U83,L12,D49,R71".to_string());
        assert_eq!(ins[0], ('R',75));
        assert_eq!(ins[6], ('R',71));
    }

    #[test]
    fn test_trace_instruction() {
        let trace = trace_instruction(vec!(('R',2),('U',2),('L',1),('D',5)));
        assert_eq!(trace[0], (0,0));
        assert_eq!(trace[2], (2,0));
        assert_eq!(trace[4], (2,2));
        assert_eq!(trace[6], (1,1));
        assert_eq!(trace[10], (1,-3));
    }
}
