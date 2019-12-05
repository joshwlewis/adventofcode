use std::io::{self,BufRead};

fn main() {
    let sin = io::stdin();
    let instructions = sin.lock().lines().map(|line| {
        match line {
            Ok(ins) => Some(parse_instruction(ins)),
            Err(_) => None
        }
    });
}


pub fn trace_instruction(ins: Vec<(char,isize)>) -> Vec<(isize,isize)> {
    let mut trace: Vec<(isize, isize)> = Vec::new();
    let ppos = (0isize, 0isize);
    for (dir,num) in ins {
        match dir {
            'U' => {
                for y in ppos.1..ppos.1+num {
                    trace.push((ppos.0, y));
                };
            },
            'D' => {
                for y in ppos.1..ppos.1-num {
                    trace.push((ppos.0, y));
                }
            },
            'R' => {
                for x in ppos.0..ppos.0+num {
                    trace.push((x, ppos.1));
                }
            },
            'L' => {
                for x in ppos.0..ppos.0-num {
                    trace.push((x,ppos.1));
                }
            },
            d => println!("Unknown direction: {}", d)
        }
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
        let trace = trace_instruction(vec!(('R',2),('U',2),('L',1)));
    }
}
