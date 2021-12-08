use std::str::FromStr;
use std::num::ParseIntError;
use std::env;
use std::fs;
use std::fmt;

fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let segment_data = parse_segment_data(input).unwrap();
    let count_1478 = count_1478_output(segment_data);
    println!("1478 Output Count: {}", count_1478);
}

fn parse_segment_data(input: String) -> Result<SegmentData, ParseIntError> {
    let mut seg_data: SegmentData = vec![];
    for segment_str in input.split("\n") {
        if segment_str == "" {
            continue
        }
        let seg = segment_str.parse::<Segment>()?;
        seg_data.push(seg);
    }
    return Ok(seg_data);
}

fn count_1478_output(segments: SegmentData) -> usize {
    segments.iter().map(|seg| seg.outputs.iter().filter(|d| {
            let c = d.count();
            return c == 2 || c == 3 || c == 4 || c == 7;
    }).count()).sum()
}

type SegmentData = Vec<Segment>;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Segment {
    signals: [Digit; 10],
    outputs: [Digit; 4],
}

#[derive(PartialEq, Eq, Hash, Default, Copy, Clone)]
struct Digit {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl FromStr for Segment {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut halves = s.split(" | ");
        let signals_str = match halves.next() {
            Some(s) => s,
            None => "",
        };
        let mut signals: [Digit; 10] = [Default::default(); 10];
        for (i, s) in signals_str.split(' ').enumerate() {
            let digit = s.parse::<Digit>()?;
            signals[i] = digit;
        }
        let digits_str = match halves.next() {
            Some(s) => s,
            None => "",
        };
        let mut outputs: [Digit; 4] = [Default::default(); 4];
        for (i,s) in digits_str.split(' ').enumerate() {
            let digit = s.parse::<Digit>()?;
            outputs[i] = digit;
        }
        return Ok(Segment { signals, outputs })
    }

}
impl Digit {
    fn count(&self) -> usize {
        let mut count = 0;
        if self.a { count+=1 };
        if self.b { count+=1 };
        if self.c { count+=1 };
        if self.d { count+=1 };
        if self.e { count+=1 };
        if self.f { count+=1 };
        if self.g { count+=1 };
        return count;
    }
}

impl FromStr for Digit {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Examples: "cb", "fdcagb"
        let mut d = Digit{a:false,b:false,c:false,d:false,e:false,f:false,g:false};
        for c in s.chars() {
            match c {
                'a' => d.a = true,
                'b' => d.b = true,
                'c' => d.c = true,
                'd' => d.d = true,
                'e' => d.e = true,
                'f' => d.f = true,
                'g' => d.g = true,
                _ => (),
            }
        }
        return Ok(d);
    }
}

impl fmt::Debug for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("");
        if self.a {
            s.push('a');
        }
        if self.b {
            s.push('b');
        }
        if self.c {
            s.push('c');
        }
        if self.d {
            s.push('d');
        }
        if self.e {
            s.push('e');
        }
        if self.f {
            s.push('f')
        }
        if self.g {
            s.push('g');
        }
        write!(f, "Digit({})", s)
    }
}
