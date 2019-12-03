use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let intcode: Vec<usize> = input.split(',').filter_map(|i|
        i.trim().parse::<usize>().ok()
    ).collect();
    println!("Input:\n {:?}", intcode);

    let alarmcode = setup_1202_alarm(intcode);
    println!("With 1202 alarm:\n {:?}", alarmcode);

    let resultcode = execute(alarmcode);
    println!("Result:\n {:?}", resultcode);
}

pub fn setup_1202_alarm(code: Vec<usize>) -> Vec<usize> {
    let mut acode = code.to_vec();
    acode[1] = 12;
    acode[2] = 2;
    acode
}

pub fn execute(code: Vec<usize>) -> Vec<usize> {
    execute_pos(0, &code).unwrap()
}

fn execute_pos(pos: usize, code: &Vec<usize>) -> Result<Vec<usize>, String> {
    let mut mcode = code.to_vec();
    match code[pos] {
        1 => {
            mcode[code[pos+3]] = code[code[pos+1]] + code[code[pos+2]];
            execute_pos(pos+4, &mcode)
        },
        2 => {
            mcode[code[pos+3]] = code[code[pos+1]] * code[code[pos+2]];
            execute_pos(pos+4, &mcode)
        }
        99 => {
            Ok(mcode)
        },
        i => {
            Err(format!("Unknown instruction {} at position {}", i, pos))
        }
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        assert_eq!(execute(vec!(1,0,0,0,99)), vec!(2,0,0,0,99));
        assert_eq!(execute(vec!(2,3,0,3,99)), vec!(2,3,0,6,99));
        assert_eq!(execute(vec!(2,4,4,5,99,0)), vec!(2,4,4,5,99,9801));
        assert_eq!(execute(vec!(1,1,1,4,99,5,6,0,99)), vec!(30,1,1,4,2,5,6,0,99));
    }
}
