extern crate regex;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let claims: Vec<Claim> = stdin.lock().lines().filter_map(|l| l.ok()).map(|l| Claim::parse(&l)).collect();
    let count_map = map_claim_counts(claims.clone());
    let conflict_count = count_conflicts(count_map.clone());
    println!("{} coordinates have conflicting claims.", conflict_count);
    let nonconflicting_id = find_nonconflicting(claims.into_iter(), count_map).unwrap();
    println!("#{} does not conflict with any other claims.", nonconflicting_id);
}

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Coord(usize, usize);

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Claim {
    id:       usize,
    location: Coord,
    size:     Coord
}

impl Claim {
    fn parse<'a>(s: &'a str) -> Claim {
        let claimformat = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = claimformat.captures(s).unwrap();

        Claim{
          id: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
          location: Coord(
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
          ),
          size: Coord(
            caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
          ),
        }
    }

}
#[test]
fn test_parse() {
    let actual = Claim::parse("#123 @ 2,4: 3x5");
    let expected = Claim{id: 123, location: Coord(2,4), size: Coord(3,5)};
    assert_eq!(expected, actual);
}

fn map_claim_counts(claims: Vec<Claim>) -> HashMap<Coord,usize> {
    let mut map = HashMap::new();
    for claim in claims {
        let location = claim.location;
        let size = claim.size;
        for x in (location.0)..(location.0 + size.0) {
            for y in (location.1)..(location.1 + size.1) {
                let count = map.entry(Coord(x,y)).or_insert(0);
                *count += 1;
            }
        }
    }
    map
}
#[test]
fn test_map_claim_counts() {
    let one = Claim{id: 1,location: Coord(1,3), size: Coord(4,4)};
    let two = Claim{id: 2, location: Coord(3,1), size: Coord(4,4)};
    let three = Claim{id: 3, location: Coord(5,5), size: Coord(2,2)};
    let claims = vec!(one, two, three);
    let map = map_claim_counts(claims.into_iter());
    let onefour = map.get(&Coord(1,4)).unwrap();
    let threethree = map.get(&Coord(3,3)).unwrap();
    let fourfour = map.get(&Coord(4,4)).unwrap();
    let sixsix = map.get(&Coord(6,6)).unwrap();
    assert_eq!(1, *onefour);
    assert_eq!(2, *threethree);
    assert_eq!(2, *fourfour);
    assert_eq!(1, *sixsix);
}

fn count_conflicts(map: HashMap<Coord,usize>) -> usize {
    let mut total = 0usize;
    for count in map.values() {
        if count > &1 {
            total += 1
        }
    }
    total
}


fn find_nonconflicting(claims: impl Iterator<Item=Claim>, count_map: HashMap<Coord,usize>) -> Option<usize> {
    'claims: for claim in claims {
        for x in (claim.location.0)..(claim.location.0 + claim.size.0) {
            for y in (claim.location.1)..(claim.location.1 + claim.size.1) {
                match count_map.get(&Coord(x,y)) {
                    Some(1) => (),
                    _ => continue 'claims,
                }
            }
        }
        return Some(claim.id)
    }
    None
}
