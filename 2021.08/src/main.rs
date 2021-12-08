use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let segment_data = parse_segment_data(input).unwrap();
    let count_1478 = count_1478_output(segment_data.clone());
    println!("1478 Output Count: {}", count_1478);
    let sum = sum_outputs(segment_data);
    println!("Output Sum: {}", sum);
}

fn parse_segment_data(input: String) -> Result<SegmentData, ParseIntError> {
    let mut seg_data: SegmentData = vec![];
    for segment_str in input.split('\n') {
        if segment_str.is_empty() {
            continue
        }
        let seg = segment_str.parse::<Segment>()?;
        seg_data.push(seg);
    }
    Ok(seg_data)
}

fn count_1478_output(segments: SegmentData) -> usize {
    segments.iter().map(|seg| seg.outputs.iter().filter(|d| {
        let c = d.count();
        c == 2 || c == 3 || c == 4 || c == 7
    }).count()).sum()
}

fn sum_outputs(segments: SegmentData) -> usize {
    let ten: usize = 10;
    segments.iter().map(|seg| {
        let mut sum: usize = 0;
        let digit_map = get_digit_map(&seg.signals);
        let mut outputs = seg.outputs.clone();
        outputs.reverse();
        for (i, d) in outputs.iter().enumerate() {
            match digit_map.get(d) {
                Some(val) => { sum += ten.pow(i as u32) * val; },
                None => { panic!("didn't find {:?} in key {:?} for signal {:?}", d, digit_map, seg.signals) },
            }
        }
        sum
    }).sum()
}

fn get_digit_map(signals: &[Digit; 10]) -> HashMap<Digit, usize> {
    let mut digit_map: HashMap<Digit, usize> = HashMap::new();

    let mut four = DEFAULT_DIGIT;
    let mut seven = DEFAULT_DIGIT;
    for d in signals {
        match d.count() {
            2 => { digit_map.insert(d.clone(), 1); },
            4 => {
                four = d.clone();
                digit_map.insert(d.clone(), 4);
            }
            3 => {
                seven = d.clone();
                digit_map.insert(d.clone(), 7);
            },
            7 => { digit_map.insert(d.clone(), 8); }
            _ => (),
        };
    }
    for d in signals {
        let val = match (d.count(), d.missing_from(&four), d.missing_from(&seven)) {
            (6, 1, 0) => 0,
            (2, _, _) => 1,
            (5, 2, 1) => 2,
            (5, 1, 0) => 3,
            (4, _, _) => 4,
            (5, 1, 1) => 5,
            (6, 1, 1) => 6,
            (3, _, _) => 7,
            (7, _, _) => 8,
            (6, 0, 0) => 9,
            t => panic!("don't know what to do with {:?} in {:?} -- {:?}", d, signals, t),
        };
        digit_map.insert(d.clone(), val);
    }
    digit_map
}


type SegmentData = Vec<Segment>;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Segment {
    signals: [Digit; 10],
    outputs: [Digit; 4],
}

#[derive(PartialEq, Eq, Hash, Default, Debug, Clone)]
struct Digit { s: String }
const DEFAULT_DIGIT: Digit = Digit{s: String::new()};

impl FromStr for Segment {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(" | ");
        let signals_str = halves.next().unwrap_or("");
        let mut signals: [Digit; 10] = [DEFAULT_DIGIT; 10];
        for (i, s) in signals_str.split(' ').enumerate() {
            let digit = s.parse::<Digit>()?;
            signals[i] = digit;
        }
        let digits_str = halves.next().unwrap_or("");
        let mut outputs: [Digit; 4] = [DEFAULT_DIGIT; 4];
        for (i,s) in digits_str.split(' ').enumerate() {
            let digit = s.parse::<Digit>()?;
            outputs[i] = digit;
        }
        Ok(Segment { signals, outputs })
    }
}
impl Digit {
    fn count(&self) -> usize {
        self.s.chars().count()
    }

    fn missing_from(&self, other: &Self) -> usize {
        let mut missing: usize = 0;
        for oc in other.s.chars() {
            if !self.s.contains(oc) {
                missing += 1
            }
        }
        missing
    }
}

impl FromStr for Digit {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Examples: "cb", "fdcagb"
        let mut chars = s.chars()
            .filter(|c| c.is_alphabetic() && c.is_lowercase())
            .collect::<Vec<char>>();
        chars.sort_unstable();
        let ds: String = chars.iter().collect();
        Ok(Digit{ s: ds })
    }
}
