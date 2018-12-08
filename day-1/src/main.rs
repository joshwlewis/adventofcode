use std::io::{self, Read};
use std::process::exit;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    match summarize(input.lines()) {
        Ok(freq) => {
            println!("{}", freq);
            exit(0);
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(64);
        }
    }
}

fn summarize<'a>(lines: impl Iterator<Item=&'a str>) -> Result<i64, String> {
    let mut sum = 0i64;
    for line in lines {
        let mut chars = line.chars();
        let sign = chars.next();
        let freq = chars.collect::<String>().parse::<i64>();
        match (sign, freq) {
            (Some('+'), Ok(num)) => {
                sum = sum + num;
            }
            (Some('-'), Ok(num)) => {
                sum = sum - num;
            }
            (None, _) => {
                (); // Empty line; ignore
            }
            (_, Err(err)) => {
                let msg = format!("frequency could not be parsed on line '{}' because '{}'", line, err);
                return Err(msg);
            }
            (Some(c), _) => {
                let msg = format!("sign ('{}') was not '+' or '-' on line '{}'", c, line);
                return Err(msg);
            }
        }
    }
    Ok(sum)
}

#[test]
fn test_summarize_example_1() {
    let lines = vec!("+1","+1","+1");
    let res = summarize(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3)
}


#[test]
fn test_summarize_example_2() {
    let lines = vec!("+1","+1","-2");
    let res = summarize(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 0)
}

#[test]
fn test_summarize_example_3() {
    let lines = vec!("-1","-2","-3");
    let res = summarize(lines.into_iter());
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), -6)
}

#[test]
fn test_summarize_bad_sign() {
    let lines = vec!("+1","*3");
    let res = summarize(lines.into_iter());
    assert!(res.is_err());
}

#[test]
fn test_summarize_not_number() {
    let lines = vec!("+1","-abc");
    let res = summarize(lines.into_iter());
    assert!(res.is_err());
}
