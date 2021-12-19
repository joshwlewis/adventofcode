use std::env;
use std::fs;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");

    let (x0,x1,y0,y1) = get_range(input.clone());
    println!("range: {}:{} {}:{}", x0,x1,y0,y1);
    let highest_y = shoot_highest(y1);
    println!("lowest_y_target: {}, highest_y_shot: {}", y1, highest_y);
    let hits = shoot_all(x0,x1,y0,y1);
    println!("hits:{}, all_hits: {:?}", hits.len(), hits)
}

fn get_range(input: String) -> (isize, isize, isize, isize) {
    let mut rs = input.split(&['=', '.', ',', ' ', '\n'][..]).filter_map(|x| x.parse::<isize>().ok());
    (rs.next().unwrap(),rs.next().unwrap(),rs.next().unwrap(),rs.next().unwrap())
}

fn shoot_highest(y0: isize) -> isize {
    let vy = -y0-1;
    (vy * vy + vy) / 2
}

fn shoot_all(x0: isize,x1: isize,y0: isize,y1: isize) -> Vec<(isize,isize)> {
    let first_vy = y0;
    let last_vy = -y0 - 1;
    let first_vx = (-0.5 + (0.25 + 2.0 * x0 as f64).sqrt()).ceil() as isize;
    let mut hits: Vec<(isize,isize)> = vec![];
    for vy in first_vy..=last_vy {
        let y0t = if vy <= 0 { 0 } else { 2 * vy + 1 };
        'x: for vx in first_vx.. {
            let mut vxi = std::cmp::max(0, vx - y0t);
            let mut vyi = vy - y0t;
            let mut x = (vx.pow(2) + vx - vxi.pow(2) - vxi) / 2;
            let mut y = 0;
            for t in y0t.. {
                if x > x1 {
                    if y > y1 || t == 1 {
                        break 'x;
                    } else {
                        continue 'x;
                    }
                } else if y < y0 {
                    continue 'x;
                } else if x >= x0 && y <= y1 {
                    hits.push((x,y));
                    continue 'x;
                }
                x += vxi;
                y += vyi;
                vyi -= 1;
                if vxi > 0 {
                    vxi -= 1;
                }
            }
        }
    }
    hits
}
