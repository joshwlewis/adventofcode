use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let ids = stdin.lock().lines().filter_map(|l| l.ok());
    let (twos, threes) = housesum(ids);
    println!("{} * {} = {}", twos, threes, twos * threes)
}

fn housesum(ids: impl Iterator<Item=String>) -> (i64, i64) {
    let mut twos = 0i64;
    let mut threes = 0i64;
    for id in ids {
        let (two, three) = idsum(id.as_str());
        if two { twos += 1 }
        if three { threes += 1 }
    }
    (twos, threes)
}

fn idsum(id: &str) -> (bool, bool) {
    let mut chrs = HashMap::new();
    for chr in id.chars() {
        let cnt = chrs.entry(chr).or_insert(0i64);
        *cnt += 1;
    }

    let mut two = false;
    let mut three = false;
    for (_chr, cnt) in chrs {
        if cnt == 2 && two == false{
            two = true;
        } else if cnt == 3 {
            three = true;
        }
    }
    (two, three)
}

#[test]
fn test_idsum_examples() {
    assert_eq!((false, false), idsum("abcdef"));
    assert_eq!((true, true), idsum("bababc"));
    assert_eq!((true, false), idsum("abbcde"));
    assert_eq!((false, true), idsum("abcccd"));
    assert_eq!((true, false), idsum("aabcdd"));
    assert_eq!((true, false), idsum("abcdee"));
    assert_eq!((false, true), idsum("ababab"));
}

#[test]
fn test_boxsum_example() {
    let ids = vec!("abcdef","bababc","abbcde","abcccd","aabcdd","abcdee","ababab");
    assert_eq!((4,3), housesum(ids.into_iter().map(|i| i.to_string())))
}

