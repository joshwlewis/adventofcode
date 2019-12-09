use std::io::{self, Read};
use std::sync::mpsc::{channel,Sender,Receiver};
use std::{thread, time};

fn main() {
    let mut input_code = String::new();
    io::stdin().read_to_string(&mut input_code).unwrap();

    let incode: Vec<isize> = input_code.split(',').filter_map(|i|
        i.trim().parse::<isize>().ok()
    ).collect();


    let max = find_max_output(&incode);
    println!("Max Output: {:?}", max);
}

pub fn find_max_output(icode: &Vec<isize>) -> isize {
    let phase_settings: Vec<Vec<isize>> = (0..3125).map(|n| {
        (0..5).map(|p| n / 5isize.pow(p) % 5isize)
              .map(|n| n + 5).rev()
              .collect::<Vec<isize>>()
    }).filter(|phases| {
        let mut mphases = phases.to_vec();
        mphases.sort();
        mphases.dedup();
        mphases.len() == 5
    }).collect();

    let mut max = 0;
    for phases in phase_settings {
        let output = execute_chain(&icode, &phases);
        if output > max {
            max = output;
        }
    }
    max
}

pub fn execute_chain(icode: &Vec<isize>, phases: &Vec<isize>) -> isize {
    let mut sndrs: Vec<Sender<isize>> = Vec::new();
    let mut rcvrs: Vec<Receiver<isize>> = Vec::new();
    for _i in 0..phases.len() {
        let (s,r) = channel();
        sndrs.push(s);
        rcvrs.push(r);
    };
    let lastrcvr = rcvrs.remove(rcvrs.len()-1);
    rcvrs.insert(0, lastrcvr);

    let mut threads = Vec::new();
    for i in 0..phases.len() {
        let previ = if i == 0 { phases.len() - 1} else { i - 1 };
        let phssndr = sndrs[previ].clone();
        let phs = phases[i];

        phssndr.send(phs).unwrap();
        if i == 0 { phssndr.send(0).unwrap(); }

        let code = icode.to_vec();
        let sndr = sndrs[i].clone();
        let rcvr = rcvrs.remove(0);
        let t = thread::spawn(move || {
            execute(&code, 0, sndr, rcvr)
        });
        threads.push(t);
    };

    let mut results = Vec::new();
    for t in threads {
        let r = t.join().unwrap().unwrap();
        results.push(r);
    }
    *results.last().unwrap()
}

fn execute(code: &Vec<isize>, pos: usize, sndr: Sender<isize>, rcvr: Receiver<isize>) -> Result<isize, String> {
    let mut mcode = code.to_vec();
    let mut ins_chars = format!("{:05}", code[pos]).chars().collect::<Vec<char>>();
    let ins_opcode = ins_chars.split_off(3).iter().collect::<String>().parse::<usize>();
    let ins_params = ins_chars.iter().filter_map(|i| i.to_digit(10)).map(|i| i as usize).collect::<Vec<usize>>();

    match ins_opcode {
        Ok(1) => {
            mcode[code[pos+3] as usize] = modal_fetch(code, ins_params[2], pos+1) + modal_fetch(code, ins_params[1], pos+2);
            execute(&mcode, pos+4, sndr, rcvr)
        },
        Ok(2) => {
            mcode[code[pos+3] as usize] = modal_fetch(code, ins_params[2], pos+1) * modal_fetch(code, ins_params[1], pos+2);
            execute(&mcode, pos+4, sndr, rcvr)
        },
        Ok(3)=> {
            let input = rcvr.recv().unwrap();
            mcode[code[pos+1] as usize] = input;
            execute(&mcode, pos+2, sndr, rcvr)
        },
        Ok(4)=> {
            let output = modal_fetch(code, ins_params[2], pos+1);
            match sndr.send(output) {
                Ok(_) => execute(&mcode, pos+2, sndr, rcvr),
                Err(msg) => {
                    Ok(output)
                },
            }
        },
        Ok(5) => {
            match modal_fetch(code, ins_params[2], pos+1) {
                0 => execute(&mcode, pos+3, sndr, rcvr),
                _ => execute(&mcode, modal_fetch(code, ins_params[1], pos+2) as usize, sndr, rcvr),
            }
        },
        Ok(6) => {
            match modal_fetch(code, ins_params[2], pos+1) {
                0 => execute(&mcode, modal_fetch(code, ins_params[1], pos+2) as usize, sndr, rcvr),
                _ => execute(&mcode, pos+3, sndr, rcvr),
            }
        },
        Ok(7) => {
            let val = if modal_fetch(code, ins_params[2], pos+1) < modal_fetch(code, ins_params[1], pos+2) {
                1
            } else {
                0
            };
            mcode[code[pos+3] as usize] = val;
            execute(&mcode, pos +4, sndr, rcvr)
        }
        Ok(8) => {
            let val = if modal_fetch(code, ins_params[2], pos+1) == modal_fetch(code, ins_params[1], pos+2) {
                1
            } else {
                0
            };
            mcode[code[pos+3] as usize] = val;
            execute(&mcode, pos +4, sndr, rcvr)
        },
        Ok(99) => Ok(0),
        Ok(i) => Err(format!("Unknown instruction {} at position {}", i, pos)),
        Err(msg) => Err(format!("Error at position {}: {}", pos, msg)),
    }
}


fn modal_fetch(code: &Vec<isize>, mode: usize, pos: usize) -> isize {
    match mode {
        0 => code[code[pos] as usize],
        1 => code[pos],
        _ => 0,
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_execute_chain_1() {
        let phases = vec!(4,3,2,1,0);
        let code = vec!(3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0);
        let result = execute_chain(&code, &phases);
        assert_eq!(43210, result);
    }

    #[test]
    fn test_execute_chain_2() {
        let phases = vec!(0,1,2,3,4);
        let code = vec!(3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0);
        let result = execute_chain(&code, &phases);
        assert_eq!(54321, result);

    }

    #[test]
    fn test_execute_chain_3() {
        let phases = vec!(1,0,4,3,2);
        let code = vec!(3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0);
        let result = execute_chain(&code, &phases);
        assert_eq!(65210, result);

    }
}
