use std::io::{self, Read};

fn main() {
    let mut input_code = String::new();
    io::stdin().read_to_string(&mut input_code).unwrap();

    let incode: Vec<isize> = input_code.split(',').filter_map(|i|
        i.trim().parse::<isize>().ok()
    ).collect();

    let result = execute(incode);
    println!("Result: {:?}", result);
}

pub fn execute(code: Vec<isize>) -> Vec<isize> {
    execute_pos(0, &code).unwrap()
}

fn execute_pos(pos: usize, code: &Vec<isize>) -> Result<Vec<isize>, String> {
    let mut mcode = code.to_vec();
    let mut ins_chars = format!("{:05}", code[pos]).chars().collect::<Vec<char>>();
    let ins_opcode = ins_chars.split_off(3).iter().collect::<String>().parse::<usize>();
    let ins_params = ins_chars.iter().filter_map(|i| i.to_digit(10)).map(|i| i as usize).collect::<Vec<usize>>();

    match ins_opcode {
        Ok(1) => {
            mcode[code[pos+3] as usize] = modal_fetch(code, ins_params[2], pos+1) + modal_fetch(code, ins_params[1], pos+2);
            execute_pos(pos+4, &mcode)
        },
        Ok(2) => {
            mcode[code[pos+3] as usize] = modal_fetch(code, ins_params[2], pos+1) * modal_fetch(code, ins_params[1], pos+2);
            execute_pos(pos+4, &mcode)
        },
        Ok(3)=> {
            mcode[code[pos+1] as usize] = 5;
            execute_pos(pos+2, &mcode)
        },
        Ok(4)=> {
            println!("**DEBUG**: {}", modal_fetch(code, ins_params[2], pos+1));
            execute_pos(pos+2, &mcode)
        },
        Ok(5) => {
            match modal_fetch(code, ins_params[2], pos+1) {
                0 => execute_pos(pos+3, &mcode),
                _ => execute_pos(modal_fetch(code, ins_params[1], pos+2) as usize, &mcode),
            }
        },
        Ok(6) => {
            match modal_fetch(code, ins_params[2], pos+1) {
                0 => execute_pos(modal_fetch(code, ins_params[1], pos+2) as usize, &mcode),
                _ => execute_pos(pos+3, &mcode),
            }
        },
        Ok(7) => {
            let val = if modal_fetch(code, ins_params[2], pos+1) < modal_fetch(code, ins_params[1], pos+2) {
                1
            } else {
                0
            };
            mcode[code[pos+3] as usize] = val;
            execute_pos(pos +4, &mcode)
        }
        Ok(8) => {
            let val = if modal_fetch(code, ins_params[2], pos+1) == modal_fetch(code, ins_params[1], pos+2) {
                1
            } else {
                0
            };
            mcode[code[pos+3] as usize] = val;
            execute_pos(pos +4, &mcode)
        },
        Ok(99) => Ok(mcode),
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
