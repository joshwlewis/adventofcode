use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let pass_range: Vec<usize> = input.split("-")
        .filter_map(|n| n.trim().parse::<usize>().ok())
        .collect();
    let pass_matches: Vec<usize> = (pass_range[0]..pass_range[1]+1)
        .filter(|pass| meets_criteria(pass))
        .collect();

    println!("Password matches: {:?}", pass_matches.len())

}

fn meets_criteria(pass_i: &usize) -> bool {
    let pass_s = pass_i.to_string();
    let formatted = format!("{:06}", &pass_s);
    has_six_digits(pass_i) && has_adjacent_duplicates(&formatted) && has_increasing_digits(&formatted)
}

fn has_six_digits(pass: &usize) -> bool {
    let min: usize = 100_000;
    let max: usize = 999_999;
    &min < pass && pass < &max
}

fn has_adjacent_duplicates(pass: &String) -> bool {
    let chrs = pass.chars().collect::<Vec<char>>();
    let min_i = 0isize;
    let max_i = (chrs.len() - 1) as isize;
    (min_i..max_i).any(|i| {
        match (iget(&chrs, i-1), iget(&chrs, i), iget(&chrs,i+1), iget(&chrs, i+2)) {
            (Some(a), Some(b), Some(c), Some(d)) => b == c && a != b && c != d,
            (Some(a), Some(b), Some(c), None)    => b == c && a != b,
            (None,    Some(b), Some(c), Some(d)) => b == c && c != d,
            (None,    Some(b), Some(c), None)    => b == c,
            (_,       _,       _,       _)       => false,
        }
    })
}

fn has_increasing_digits(pass: &String) -> bool {
    let mut pass_chars = pass.chars();
    let mut last_char = pass_chars.next().unwrap();
    let mut result = true;
    for pass_char in pass_chars {
        if last_char > pass_char {
            result = false;
            break;
        }
        last_char = pass_char;
    };
    result
}


fn iget<'a,T>(v: &'a Vec<T>, i: isize) -> Option<&'a T> {
    if i >= 0 {
        v.get(i as usize)
    } else {
        None
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_meets_criteria() {
        assert!(!meets_criteria(&111111));
        assert!(!meets_criteria(&223450));
        assert!(!meets_criteria(&123789));
        assert!(meets_criteria(&112233));
        assert!(!meets_criteria(&123444));
        assert!(meets_criteria(&111122));
        assert!(meets_criteria(&334444));
    }
}

