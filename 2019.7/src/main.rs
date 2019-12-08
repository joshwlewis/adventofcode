use std::io::{self, Read};

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
        (0..5).map(|p| n / 5isize.pow(p) % 5isize).rev().collect::<Vec<isize>>()
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

pub fn execute_chain(icode: &Vec<isize>, inputs: &Vec<isize>) -> isize {
    let mut output: isize = 0;
    for inp in inputs {
        output = *execute(icode.clone(), vec!(*inp, output)).first().unwrap();
    };
    output
}

pub fn execute(code: Vec<isize>, inputs: Vec<isize>) -> Vec<isize> {
    let outputs: Vec<isize> = Vec::new();
    execute_pos(&code, 0, inputs, outputs).unwrap()
}

fn execute_pos(code: &Vec<isize>, pos: usize, inputs: Vec<isize>, outputs: Vec<isize>) -> Result<Vec<isize>, String> {
    let mut mcode = code.to_vec();
    let mut ins_chars = format!("{:05}", code[pos]).chars().collect::<Vec<char>>();
    let ins_opcode = ins_chars.split_off(3).iter().collect::<String>().parse::<usize>();
    let ins_params = ins_chars.iter().filter_map(|i| i.to_digit(10)).map(|i| i as usize).collect::<Vec<usize>>();

    match ins_opcode {
        Ok(1) => {
            mcode[code[pos+3] as usize] = modal_fetch(code, ins_params[2], pos+1) + modal_fetch(code, ins_params[1], pos+2);
            execute_pos(&mcode, pos+4, inputs, outputs)
        },
        Ok(2) => {
            mcode[code[pos+3] as usize] = modal_fetch(code, ins_params[2], pos+1) * modal_fetch(code, ins_params[1], pos+2);
            execute_pos(&mcode, pos+4, inputs, outputs)
        },
        Ok(3)=> {
            let mut minputs = inputs.to_vec();
            let input = minputs.remove(0);
            mcode[code[pos+1] as usize] = input;
            execute_pos(&mcode, pos+2, minputs, outputs)
        },
        Ok(4)=> {
            let output = modal_fetch(code, ins_params[2], pos+1);
            let mut moutputs = outputs.to_vec();
            moutputs.push(output);
            execute_pos(&mcode, pos+2, inputs, moutputs)
        },
        Ok(5) => {
            match modal_fetch(code, ins_params[2], pos+1) {
                0 => execute_pos(&mcode, pos+3, inputs, outputs),
                _ => execute_pos(&mcode, modal_fetch(code, ins_params[1], pos+2) as usize, inputs, outputs),
            }
        },
        Ok(6) => {
            match modal_fetch(code, ins_params[2], pos+1) {
                0 => execute_pos(&mcode, modal_fetch(code, ins_params[1], pos+2) as usize, inputs, outputs),
                _ => execute_pos(&mcode, pos+3, inputs, outputs),
            }
        },
        Ok(7) => {
            let val = if modal_fetch(code, ins_params[2], pos+1) < modal_fetch(code, ins_params[1], pos+2) {
                1
            } else {
                0
            };
            mcode[code[pos+3] as usize] = val;
            execute_pos(&mcode, pos +4, inputs, outputs)
        }
        Ok(8) => {
            let val = if modal_fetch(code, ins_params[2], pos+1) == modal_fetch(code, ins_params[1], pos+2) {
                1
            } else {
                0
            };
            mcode[code[pos+3] as usize] = val;
            execute_pos(&mcode, pos +4, inputs, outputs)
        },
        Ok(99) => Ok(outputs),
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

    #[test]
    fn test_find_max_3() {
        let code = vec!(3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0);
        let result = find_max_output(&code);
        let phases = vec!(1,0,4,3,2);
        assert_eq!(65210, result);

    }
}
