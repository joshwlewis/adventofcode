use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
    let mut strcode = String::new();
    io::stdin().read_to_string(&mut strcode).unwrap();

    let intcode: Vec<isize> = strcode
        .split(',')
        .filter_map(|i| i.trim().parse::<isize>().ok())
        .collect();

    let blackhullpaint = painthull(&intcode, 0);
    printhull(&blackhullpaint);
    println!("Black Hull Paint Count: {:?}", blackhullpaint.keys().len());
    let whitehullpaint = painthull(&intcode, 1);
    printhull(&whitehullpaint);
}

pub fn printhull(hull: &HashMap<Panel, usize>) {
    let panels: Vec<Panel> = hull.iter().map(|(p, _c)| p.clone()).collect();
    let xmin = panels.iter().map(|p| p.x).min().unwrap();
    let xmax = panels.iter().map(|p| p.x).max().unwrap();
    let ymin = panels.iter().map(|p| p.y).min().unwrap();
    let ymax = panels.iter().map(|p| p.y).max().unwrap();

    for y in (ymin..=ymax).rev() {
        for x in xmin..=xmax {
            match hull.get(&Panel { x: x, y: y }) {
                Some(1) => print!("🎅"),
                _ => print!("👾"),
            }
        }
        print!("\n");
    }
}

pub fn painthull(code: &Vec<isize>, color: usize) -> HashMap<Panel, usize> {
    let mcode: Vec<isize> = code.to_vec();
    let (comsndr, comrcvr) = channel();
    let (botsndr, botrcvr) = channel();

    let comthread = thread::Builder::new()
        .name("Intcode Computer".to_string())
        .stack_size(1073741824)
        .spawn(move || execsr(&mcode, botsndr, comrcvr));

    let botthread = thread::Builder::new()
        .name("Paint Bot".to_string())
        .spawn(move || paintio(color, botrcvr, comsndr));

    comthread.unwrap().join().unwrap().unwrap();
    botthread.unwrap().join().unwrap()
}

pub fn paintio(c: usize, i: Receiver<isize>, o: Sender<isize>) -> HashMap<Panel, usize> {
    let mut hull: HashMap<Panel, usize> = HashMap::new();
    let mut cpanel = Panel { x: 0, y: 0 };
    let mut cdir = Dir('^');
    let mut ccolor = c;
    loop {
        match o.send(ccolor as isize) {
            Ok(_) => (),
            Err(_msg) => break,
        };

        match i.recv() {
            Ok(color) => {
                hull.insert(cpanel, color as usize);
            }
            Err(_msg) => break,
        };

        match i.recv() {
            Ok(dircode) => {
                cdir = cdir.next(dircode as usize);
            }
            Err(_msg) => break,
        }
        cpanel = cpanel.next(cdir);

        ccolor = match hull.get(&cpanel) {
            Some(c) => *c,
            None => 0usize,
        };
    }
    hull
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Dir(char);

impl Dir {
    fn next(&self, code: usize) -> Dir {
        match (self.0, code) {
            ('^', 0) => Dir('<'),
            ('^', 1) => Dir('>'),
            ('>', 0) => Dir('^'),
            ('>', 1) => Dir('v'),
            ('v', 0) => Dir('>'),
            ('v', 1) => Dir('<'),
            ('<', 0) => Dir('v'),
            ('<', 1) => Dir('^'),
            _ => panic!("wut direction next?"),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Panel {
    x: isize,
    y: isize,
}

impl Panel {
    fn next(&self, dir: Dir) -> Panel {
        match dir {
            Dir('^') => Panel {
                x: self.x,
                y: self.y + 1,
            },
            Dir('>') => Panel {
                x: self.x + 1,
                y: self.y,
            },
            Dir('v') => Panel {
                x: self.x,
                y: self.y - 1,
            },
            Dir('<') => Panel {
                x: self.x - 1,
                y: self.y,
            },
            _ => panic!("wut panel next!"),
        }
    }
}

fn execsr(code: &Vec<isize>, sndr: Sender<isize>, rcvr: Receiver<isize>) -> Result<isize, String> {
    execr(code, 0, 0, sndr, rcvr)
}

fn execr(
    code: &Vec<isize>,
    pos: usize,
    rel: isize,
    sndr: Sender<isize>,
    rcvr: Receiver<isize>,
) -> Result<isize, String> {
    let mut mcode = code.to_vec();
    let mut ins_chars = format!("{:05}", code[pos]).chars().collect::<Vec<char>>();
    let opcode = ins_chars
        .split_off(3)
        .iter()
        .collect::<String>()
        .parse::<usize>();
    let opmodes = ins_chars
        .iter()
        .filter_map(|i| i.to_digit(10))
        .map(|i| i as usize)
        .collect::<Vec<usize>>();

    match opcode {
        Ok(1) => {
            exset(
                &mut mcode,
                modgetw(code, opmodes[0], pos + 3, rel) as usize,
                modgetr(code, opmodes[2], pos + 1, rel) + modgetr(code, opmodes[1], pos + 2, rel),
            );
            execr(&mcode, pos + 4, rel, sndr, rcvr)
        }
        Ok(2) => {
            exset(
                &mut mcode,
                modgetw(code, opmodes[0], pos + 3, rel) as usize,
                modgetr(code, opmodes[2], pos + 1, rel) * modgetr(code, opmodes[1], pos + 2, rel),
            );
            execr(&mcode, pos + 4, rel, sndr, rcvr)
        }
        Ok(3) => {
            exset(
                &mut mcode,
                modgetw(code, opmodes[2], pos + 1, rel) as usize,
                rcvr.recv().unwrap(),
            );
            execr(&mcode, pos + 2, rel, sndr, rcvr)
        }
        Ok(4) => {
            let output = modgetr(code, opmodes[2], pos + 1, rel);
            match sndr.send(output) {
                Ok(_) => execr(&mcode, pos + 2, rel, sndr, rcvr),
                Err(_msg) => Ok(output),
            }
        }
        Ok(5) => match modgetr(code, opmodes[2], pos + 1, rel) {
            0 => execr(&mcode, pos + 3, rel, sndr, rcvr),
            _ => execr(
                &mcode,
                modgetr(code, opmodes[1], pos + 2, rel) as usize,
                rel,
                sndr,
                rcvr,
            ),
        },
        Ok(6) => match modgetr(code, opmodes[2], pos + 1, rel) {
            0 => execr(
                &mcode,
                modgetr(code, opmodes[1], pos + 2, rel) as usize,
                rel,
                sndr,
                rcvr,
            ),
            _ => execr(&mcode, pos + 3, rel, sndr, rcvr),
        },
        Ok(7) => {
            let val = if modgetr(code, opmodes[2], pos + 1, rel)
                < modgetr(code, opmodes[1], pos + 2, rel)
            {
                1
            } else {
                0
            };
            exset(
                &mut mcode,
                modgetw(code, opmodes[0], pos + 3, rel) as usize,
                val,
            );
            execr(&mcode, pos + 4, rel, sndr, rcvr)
        }
        Ok(8) => {
            let val = if modgetr(code, opmodes[2], pos + 1, rel)
                == modgetr(code, opmodes[1], pos + 2, rel)
            {
                1
            } else {
                0
            };
            exset(
                &mut mcode,
                modgetw(code, opmodes[0], pos + 3, rel) as usize,
                val,
            );
            execr(&mcode, pos + 4, rel, sndr, rcvr)
        }
        Ok(9) => {
            let newrel = rel + modgetr(code, opmodes[2], pos + 1, rel);
            execr(&mcode, pos + 2, newrel, sndr, rcvr)
        }
        Ok(99) => Ok(0),
        Ok(i) => Err(format!("Unknown instruction {} at position {}", i, pos)),
        Err(msg) => Err(format!("Error at position {}: {}", pos, msg)),
    }
}

fn modgetw(code: &Vec<isize>, mode: usize, pos: usize, rel: isize) -> isize {
    match mode {
        2 => rel + exget(code, pos), // 2
        _ => exget(code, pos),       // 0, 1
    }
}

fn modgetr(code: &Vec<isize>, mode: usize, pos: usize, rel: isize) -> isize {
    match mode {
        1 => exget(code, pos),                               // 1
        2 => exget(code, (rel + exget(code, pos)) as usize), // 2
        _ => exget(code, exget(code, pos) as usize),         // 0
    }
}

fn collectrcvr(rcvr: Receiver<isize>, coll: Vec<isize>) -> Vec<isize> {
    let mut mcoll = coll.to_vec();
    match rcvr.recv() {
        Ok(t) => {
            mcoll.push(t);
            collectrcvr(rcvr, mcoll)
        }
        Err(_msg) => mcoll,
    }
}

fn exget(code: &Vec<isize>, pos: usize) -> isize {
    if pos >= code.len() {
        0
    } else {
        code[pos]
    }
}

fn exset(code: &mut Vec<isize>, pos: usize, val: isize) {
    if pos >= code.len() {
        code.resize_with(pos + 1, Default::default)
    }
    code[pos] = val;
}

mod tests {
    use super::*;

    #[test]
    fn test_execphases_1() {
        let phases = vec![4, 3, 2, 1, 0];
        let code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let result = execphases(&code, &phases);
        assert_eq!(43210, result);
    }

    #[test]
    fn test_execphases_2() {
        let phases = vec![0, 1, 2, 3, 4];
        let code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let result = execphases(&code, &phases);
        assert_eq!(54321, result);
    }

    #[test]
    fn test_execphases_3() {
        let phases = vec![1, 0, 4, 3, 2];
        let code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let result = execphases(&code, &phases);
        assert_eq!(65210, result);
    }

    #[test]
    fn test_exec_1() {
        let code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let result = exec(&code, vec![]);
        assert_eq!(result, code)
    }

    #[test]
    fn test_exec_2() {
        let code = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let result = exec(&code, vec![]);
        assert_eq!(result[0], 1219070632396864);
    }

    #[test]
    fn test_exec_3() {
        let code = vec![104, 1125899906842624, 99];
        let result = exec(&code, vec![]);
        assert_eq!(result[0], 1125899906842624)
    }
}
