use std::io::{self, BufRead};
use std::env::args;
use std::collections::HashMap;

extern crate itertools;
use itertools::multipeek;

fn main() {
    let arg = args().nth(1).unwrap();
    if arg == "checksum" {
        let stdin = io::stdin();
        let ids = stdin.lock().lines().filter_map(|l| l.ok());
        let (twos, threes) = housesum(ids);
        println!("{} * {} = {}", twos, threes, twos * threes);
    } else if arg == "findpair" {
        let stdin = io::stdin();
        let ids = stdin.lock().lines().filter_map(|l| l.ok());
        let (one, two) = findpair(ids).unwrap();
        println!("{},{}", one, two)
    }
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

#[test]
fn test_housesum() {
    let ids = vec!("abcdef","bababc","abbcde","abcccd","aabcdd","abcdee","ababab");
    assert_eq!((4,3), housesum(ids.into_iter().map(|i| i.to_string())))
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
fn test_idsum() {
    assert_eq!((false, false), idsum("abcdef"));
    assert_eq!((true, true), idsum("bababc"));
    assert_eq!((true, false), idsum("abbcde"));
    assert_eq!((false, true), idsum("abcccd"));
    assert_eq!((true, false), idsum("aabcdd"));
    assert_eq!((true, false), idsum("abcdee"));
    assert_eq!((false, true), idsum("ababab"));
}

fn ispair(a: &str, b: &str) -> bool {
    if a == b {
        return false
    }
    let mut diff = false;
    for (chara, charb) in a.chars().zip(b.chars()) {
        if chara != charb {
            if diff {
                return false;
            } else {
                diff = true;
            }
        }
    }
    return diff
}

#[test]
fn test_ispair() {
    assert_eq!(false, ispair("abcde", "axcye"));
    assert_eq!(true, ispair("fghij", "fguij"));
}

fn findpair<'a>(ids: impl Iterator<Item=String>) -> Option<(String, String)> {
    let mut peeker = multipeek(ids);
    loop {
        match peeker.next() {
            Some(ida) => {
                loop {
                    match peeker.peek() {
                        Some(idb) => {
                            if ispair(&ida, &idb) {
                                return Some((ida.to_string(), idb.to_string()))
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
            None => {
                return None
            }
        }
    }
}

#[test]
fn test_findpair() {
    let ids = vec!("abcde","fghij","klmno","pqrst","fguij","axcye","wvxyz");
    let result = findpair(ids.iter().map(|i| i.to_string()));
    assert!(result.is_some());
    assert_eq!(result.unwrap(), ("fghij".to_string(), "fguij".to_string()));
}

