use std::collections::HashSet;
use std::f32::consts::PI;
use std::io::{self, Read};

fn main() {
    let mut smap = String::new();
    io::stdin().read_to_string(&mut smap).unwrap();

    let amap = parsemap(smap);
    let (besta, bestv) = highestviz(&amap);
    println!("The best asteroid is {:?} with {} visibility", besta, bestv);

    let boomorder = besta.boomorder(&amap);
    println!("The 200th asteroid will be {:?}", boomorder[199]);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Astrd {
    x: usize,
    y: usize,
}

fn astrd(x: usize, y: usize) -> Astrd {
    Astrd { x: x, y: y }
}

fn parsemap(map: String) -> Vec<Astrd> {
    map.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, letter)| match letter {
                    '#' => Some(astrd(x, y)),
                    _ => None,
                })
        })
        .collect()
}

fn highestviz(map: &Vec<Astrd>) -> (&Astrd, usize) {
    map.iter()
        .map(|a| (a, a.visibility(map)))
        .max_by(|(_aa, va), (_ba, vb)| va.cmp(vb))
        .unwrap()
}

impl Astrd {
    // Returns  positive angle between two asteroids relative to up (-y axis)
    fn angleto(&self, other: &Self) -> f32 {
        let dx = (other.x as isize - self.x as isize) as f32;
        let dy = (other.y as isize - self.y as isize) as f32;
        let ang = f32::atan2(dy, dx) + (PI / 2.0);
        if ang < 0f32 {
            ang + 2.0 * PI
        } else {
            ang
        }
    }

    fn distanceto(&self, other: &Self) -> f32 {
        let dx = (other.x as isize - self.x as isize) as f32;
        let dy = (other.y as isize - self.y as isize) as f32;
        f32::hypot(dx, dy)
    }

    fn hashangleto(&self, other: &Self) -> usize {
        (self.angleto(other) * 10e9f32) as usize
    }

    fn hashdistanceto(&self, other: &Self) -> usize {
        (self.distanceto(other) * 10e9f32) as usize
    }

    fn visibility(&self, map: &Vec<Astrd>) -> usize {
        map.iter()
            .filter_map(|a| match self == a {
                true => None,
                false => Some(a),
            })
            .map(|a| self.hashangleto(a))
            .collect::<HashSet<usize>>()
            .len()
    }

    fn boomorder<'a>(&self, map: &'a Vec<Astrd>) -> Vec<Astrd> {
        let mut distorder: Vec<Astrd> = map
            .to_vec()
            .iter()
            .filter_map(|a| match self == a {
                true => None,
                false => Some(a.clone()),
            })
            .collect();

        distorder.sort_by(|a, b| self.hashdistanceto(a).cmp(&self.hashdistanceto(b)));

        let angleset: HashSet<usize> = map.iter().map(|a| self.hashangleto(a)).collect();
        let mut angles: Vec<usize> = angleset.iter().map(|a| *a).collect();
        angles.sort();

        let mut boomorder = Vec::new();
        while boomorder.len() <= map.len() {
            for ang in &angles {
                match distorder.iter().find(|a| self.hashangleto(a) == *ang) {
                    Some(a) => boomorder.push(a.clone()),
                    None => (),
                }
            }
        }

        boomorder
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_parsemap() {
        let r = parsemap("...#...\n.#...#".to_string());
        assert_eq!(r[0], Astrd { x: 3, y: 0 });
        assert_eq!(r[2], Astrd { x: 5, y: 1 });
    }

    #[test]
    fn test_astrd_angleto() {
        let result = Astrd { x: 0, y: 0 }.angleto(&Astrd { x: 1, y: 1 });
        assert_eq!(result, 2.3561945);
    }

    #[test]
    fn test_astrd_visibility() {
        let map = vec![
            astrd(0, 0),
            astrd(2, 2),
            astrd(4, 4),
            astrd(5, 5),
            astrd(0, 1),
            astrd(0, 5),
            astrd(1, 0),
            astrd(5, 0),
        ];
        let result = astrd(3, 3).visibility(&map);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_highestviz() {
        let smap = "......#.#.\n\
                    #..#.#....\n\
                    ..#######.\n\
                    .#.#.###..\n\
                    .#..#.....\n\
                    ..#....#.#\n\
                    #..#....#.\n\
                    .##.#..###\n\
                    ##...#..#.\n\
                    .#....####";
        let amap = parsemap(smap.to_string());
        let result = highestviz(&amap);
        assert_eq!(result, (&astrd(5, 8), 33));
    }
}
