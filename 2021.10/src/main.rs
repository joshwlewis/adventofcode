use std::env;
use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;


fn main() {
    let filename = match env:: args().nth(1) {
        Some(f) => f,
        None => "input.txt".to_string(),
    };
    let input = fs::read_to_string(filename).expect("Error opening input file");
    let syntax = parse_syntax(&input);
    let error_score  = syntax_error_score(syntax.clone());
    println!("Syntax Error Score: {}", error_score);
    let completion_score  = syntax_completion_score(syntax);
    println!("Syntax Completion Score: {}", completion_score);
}

fn syntax_scores() -> HashMap<char, (usize, usize)> {
    let mut m = HashMap::new();
    m.insert(')', (1, 3));
    m.insert(']', (2, 57));
    m.insert('}', (3, 1197));
    m.insert('>', (4, 25137));
    m
}

fn syntax_error_score(lines: Vec<&str>) -> usize {
    lines.iter().map(|l| line_score(l).1).sum()
}

fn syntax_completion_score(lines: Vec<&str>) -> usize {
    let mut scores: Vec<usize> = lines.iter().map(|l| line_score(l).0).filter(|s| *s != 0).collect();
    scores.sort_unstable();
    scores[scores.len()/2]
}

fn line_score(line: &str) -> (usize, usize) {
    let mut close_q = VecDeque::new();
    let scores = syntax_scores();
    for chr in line.chars() {
        match chr {
            '(' => { close_q.push_front(')') },
            '[' => { close_q.push_front(']') },
            '{' => { close_q.push_front('}') },
            '<' => { close_q.push_front('>') },
            ')' | ']' | '}' | '>' => { 
              if close_q.pop_front().unwrap_or('#') != chr { 
                  let score = scores[&chr];
                  return (0, score.1); 
                } 
            },
            _ => { panic!("unknown char {}", chr); }
        }
    }
    let score = close_q.iter().fold(0, |acc, chr| {
        (acc*5) + scores[chr].0
    });
    (score, 0)
}

fn parse_syntax(input: &str) -> Vec<&str> {
    input.split('\n').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_sample_data() -> String {
        fs::read_to_string("sample.txt").expect("Error opening sample file")
    }

    #[test]
    fn test_parse_syntax() {
        let input = read_sample_data();
        let syntax = parse_syntax(&input);
        assert_eq!(syntax.len(), 10);
        assert_eq!(syntax[0].chars().next(), Some('['));
        assert_eq!(syntax[9].chars().next(), Some('<'));
    }

    #[test]
    fn test_syntax_error_score() {
        let input = read_sample_data();
        let syntax = parse_syntax(&input);
        let score = syntax_error_score(syntax);
        assert_eq!(score, 26397)
    }

    #[test]
    fn test_syntax_completion_score() {
        let input = read_sample_data();
        let syntax = parse_syntax(&input);
        let score = syntax_completion_score(syntax);
        assert_eq!(score, 288957);
    }

}
