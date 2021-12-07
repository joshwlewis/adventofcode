use std::env;
use std::cmp;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "./input.txt".to_string(),
    };
    println!("Filename: {}", filename);
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers: Vec<isize> = vec![];
    for line in reader.lines() {
        numbers.extend(
            line.unwrap().split(',').map(|n| {
                n.parse::<isize>().unwrap()
            })
        );
    }
    let (min_loc, max_loc) = min_max_location(numbers.clone());
    println!("Min: {}, Max: {}", min_loc, max_loc);

    let (best_move_count, best_move_location) = min_moves(numbers.clone(), min_loc, max_loc);
    println!("Best Move Count: {}, Best Move Location: {}", best_move_count, best_move_location);

    let (best_fuel_count, best_fuel_location) = min_fuel(numbers, min_loc, max_loc);
    println!("Best Fuel Count: {}, Best Fuel Location: {}", best_fuel_count, best_fuel_location);
}

fn min_max_location(locations: Vec<isize>) -> (isize, isize) {
    let first = locations[0];
    let (mut min, mut max) = (first, first);
    for loc in locations {
        min = cmp::min(min, loc);
        max = cmp::max(max, loc);
    }
    return (min, max);
}

fn min_moves(locations: Vec<isize>, min_loc: isize, max_loc: isize) -> (isize, isize) {
    return min_fuel_calc(locations, min_loc, max_loc, |d: isize| d.abs());
}

fn min_fuel(locations: Vec<isize>, min_loc: isize, max_loc: isize) -> (isize, isize) {
    return min_fuel_calc(locations, min_loc, max_loc, |d: isize| {
        let n = d.abs();
        return n*(n+1)/2;
    })
}

fn min_fuel_calc<F>(locations: Vec<isize>, min_loc: isize, max_loc: isize, fuel_calc: F) -> (isize, isize)
where F: Fn(isize) -> isize {
    let mut best_fuel = fuel_calc(max_loc) * locations.len() as isize;
    let mut best_loc = 0;
    let mut dists = vec![];
    for loc in locations {
        dists.push(min_loc - loc);
    }
    for target in min_loc..max_loc {
        for (i, _d) in dists.clone().iter().enumerate() {
            dists[i] = dists[i]+1
        }
        let fuel = dists.iter().fold(0, |sum, d| sum + fuel_calc(d.clone()));
        if fuel < best_fuel {
            best_fuel = fuel;
            best_loc = target;
        }
    }
    return (best_fuel, best_loc)
}
