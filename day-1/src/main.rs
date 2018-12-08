use std::io::{self, Read};
use std::process::exit;
use std::env::args;

fn main() -> io::Result<()> {
    let arg = args().nth(1);
    match arg {
        Some(mode) => {
            if mode == "final" {
                let mut input = String::new();
                io::stdin().read_to_string(&mut input)?;
                print_exit(get_final(input.lines()));
            } else if mode == "repeated" {
                let mut input = String::new();
                io::stdin().read_to_string(&mut input)?;
                print_exit(get_repeated(input.lines()));
            } else {
              print_exit(Err("Use final or repeated as the first argument".to_string()));
            }
        }
        None => {
            print_exit(Err("Use final or repeated as the first argument".to_string()));
        }
    }
}


fn get_final<'a>(commands: impl Iterator<Item=&'a str>) -> Result<i64, String> {
    let mut freq = 0i64;
    for command in commands {
        match adjustment(command) {
            Ok(adj) => freq = freq + adj,
            Err(err) => return Err(err),
        }
    }
    Ok(freq)
}

fn get_repeated<'a>(commands: impl Iterator<Item=&'a str>) -> Result<i64, String> {
    let mut freqs = Vec::new();
    let mut freq = 0i64;
    freqs.push(freq);
    for command in commands {
        match adjustment(command) {
            Ok(adj) => freq = freq + adj,
            Err(err) => return Err(err),
        }
        if freqs.contains(&freq) {
            return Ok(freq);
        }
        freqs.push(freq);
    }
    Err("couldn't find a repeated frequency".to_string())
}

fn print_exit(res: Result<i64, String>) -> ! {
    match res {
        Ok(freq) => {
            println!("{}", freq);
            exit(0)
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(64)
        }
    }
}

fn adjustment<'a>(command: &'a str) -> Result<i64, String> {
    let mut chars = command.chars();
    let sign = chars.next();
    let freq = chars.collect::<String>().parse::<i64>();
    match (sign, freq) {
        (Some('+'), Ok(num)) => {
            Ok(num)
        }
        (Some('-'), Ok(num)) => {
            Ok(0 - num)
        }
        (None, _) => {
            Ok(0)
        }
        (_, Err(err)) => {
            let msg = format!("frequency could not be parsed in command '{}' because '{}'", command, err);
            Err(msg)
        }
        (Some(c), _) => {
            let msg = format!("sign ('{}') was not '+' or '-' in command '{}'", c, command);
            Err(msg)
        }
    }

}

#[test]
fn test_get_final_example_1() {
    let lines = vec!("+1","+1","+1");
    let res = get_final(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3)
}


#[test]
fn test_get_final_example_2() {
    let lines = vec!("+1","+1","-2");
    let res = get_final(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 0)
}

#[test]
fn test_get_final_example_3() {
    let lines = vec!("-1","-2","-3");
    let res = get_final(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), -6)
}

#[test]
fn test_get_final_bad_sign() {
    let lines = vec!("+1","*3");
    let res = get_final(lines.into_iter());
    assert!(res.is_err());
}

#[test]
fn test_get_final_not_number() {
    let lines = vec!("+1","-abc");
    let res = get_final(lines.into_iter());
    assert!(res.is_err());
}

#[test]
fn test_get_repeated_example_1() {
    let lines = vec!("+1","-1", "+1", "-1");
    let res = get_repeated(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 0)
}

#[test]
fn test_get_repeated_example_2() {
    let lines = vec!("+3","+3","+4","-2","-4","+3","+3","+4","-2","-4");
    let res = get_repeated(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 10)
}

#[test]
fn test_get_repeated_example_3() {
    let lines = vec!("+7","+7","-2","-7","-4","+7","+7","-2","-7","-4","+7","+7","-2","-7","-4");
    let res = get_repeated(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 14)
}

