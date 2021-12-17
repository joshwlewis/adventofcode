use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::cmp::Ordering;
use std::fmt;
use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let hex = fs::read_to_string(filename).expect("Error opening input file");
    let bin = hex_to_binary(hex.clone());
    println!("hex:{}, bin:{}", hex, bin);
}

fn hex_to_binary(hex: String) ->  String {
    let mut binary = String::new();
    for chr in hex.trim().chars() {
        let bin = chr.to_digit(16).unwrap();
        binary += &format!("{:b}", bin).to_string();
    }
    binary
}

struct Hdr {
    ver: usize,
    typ: usize,
}

struct Pkt {
    hdr: Hdr,
    cnt: Cnt,
}

enum Cnt {
    Opr{cnts: Vec<Pkt>},
    Lit{val: usize},
}

impl FromStr for Pkt {
    type Err = ParseIntError;
    fn from_str(bin: &str) -> Result<Self, Self::Err> {
        let ver = usize::from_str_radix(&bin[0..3],2).unwrap();
        let typ = usize::from_str_radix(&bin[3..6],2).unwrap();
        let hdr = Hdr{ver,typ};
        let cnt: Cnt;
        if typ == 4 {
            let mut bail = false;
            let mut literals = String::new();
            for (i, chr) in bin.chars().enumerate() {
                if i % 5 == 0 {
                    if chr == '0' {
                        bail = true
                    }
                } else {
                    literals.push(chr)
                }
                if bail && i % 5 == 4 {
                    let val = usize::from_str_radix(&literals, 2).unwrap();
                    cnt = Cnt::Lit{val};
                    break;
                }
            }
        } else {
            let ltype = usize::from_str_radix(&bin[6..7],2).unwrap();
            if ltype == 0 {
                let bits = usize::from_str_radix(&bin[6..22], 2).unwrap();
                let subPkt: Pkt = bin[23..24+bits].parse();
            } else {
            }
        }
        Ok(Pkt{hdr, cnt})
    }
}
