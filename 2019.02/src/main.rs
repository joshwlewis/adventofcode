use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let incode: Vec<usize> = input.split(',').filter_map(|i|
        i.trim().parse::<usize>().ok()
    ).collect();
    println!("Input:\n {:?}", incode);

    let alarmcode = set_input(&incode, 12, 2);
    println!("Input with 1202 alarm:\n {:?}", alarmcode);

    let resultcode = execute(alarmcode);
    println!("Result:\n {:?}", resultcode);

    let target: usize = 19690720;
    let (noun, verb) = find_inputs(incode, target).unwrap();
    println!("Inputs required for result {}: {},{}", target, noun, verb);
}

pub fn find_inputs(code: Vec<usize>, target: usize) -> Option<(usize,usize)> {
    let mut inputs = (0..100).flat_map(|n| (0..100).map(move |v| (n,v)));
    inputs.find(|x| {
        let icode = set_input(&code, x.0, x.1);
        let rcode = execute(icode);
        rcode[0] == target
    })
}

pub fn set_input(code: &Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
    let mut acode = code.to_vec();
    acode[1] = noun;
    acode[2] = verb;
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
