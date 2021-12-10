use std::{collections::LinkedList, io::BufRead};

enum SyntaxCheckingResult {
    Ok(u64),
    Error(u64),
}

#[derive(PartialEq, Eq)]
enum ChunkDelimiter {
    Parenthesis,
    SquareBracket,
    CurlyBracket,
    LessGreaterSign,
}

impl ChunkDelimiter {
    fn from(c: char) -> Option<ChunkDelimiter> {
        match c {
            '(' => Some(ChunkDelimiter::Parenthesis),
            ')' => Some(ChunkDelimiter::Parenthesis),
            '[' => Some(ChunkDelimiter::SquareBracket),
            ']' => Some(ChunkDelimiter::SquareBracket),
            '{' => Some(ChunkDelimiter::CurlyBracket),
            '}' => Some(ChunkDelimiter::CurlyBracket),
            '<' => Some(ChunkDelimiter::LessGreaterSign),
            '>' => Some(ChunkDelimiter::LessGreaterSign),
            _ => None,
        }
    }

    fn is_closing(c: char) -> bool {
        c == ')' || c == ']' || c == '}' || c == '>'
    }
}

fn main() {
    let (syntax_error_score, mut autocomplete_scores) = std::io::stdin().lock().lines().fold(
        (0, Vec::new()),
        |(syntax_error_score, mut autocomplete_scores), line| {
            let line = line.expect("failed to read stdin line");

            match check_syntax_errors(&line) {
                SyntaxCheckingResult::Ok(points) => {
                    autocomplete_scores.push(points);
                    (syntax_error_score, autocomplete_scores)
                }
                SyntaxCheckingResult::Error(points) => {
                    (syntax_error_score + points, autocomplete_scores)
                }
            }
        },
    );

    autocomplete_scores.sort();

    println!("Part 1: {}", syntax_error_score);
    println!(
        "Part 2: {}",
        autocomplete_scores
            .get(autocomplete_scores.len() / 2)
            .unwrap()
    );
}

fn check_syntax_errors(line: &str) -> SyntaxCheckingResult {
    let mut stack: LinkedList<ChunkDelimiter> = LinkedList::new();

    for c in line.chars() {
        let delimiter = ChunkDelimiter::from(c).expect("invalid input");

        if ChunkDelimiter::is_closing(c) {
            let expected = stack.pop_front();
            if let Some(expected) = expected {
                if expected == delimiter {
                    continue;
                }
            }

            return match delimiter {
                ChunkDelimiter::Parenthesis => SyntaxCheckingResult::Error(3),
                ChunkDelimiter::SquareBracket => SyntaxCheckingResult::Error(57),
                ChunkDelimiter::CurlyBracket => SyntaxCheckingResult::Error(1197),
                ChunkDelimiter::LessGreaterSign => SyntaxCheckingResult::Error(25137),
            };
        } else {
            stack.push_front(delimiter);
        }
    }

    let points = stack.iter().fold(0, |acc, delimiter| {
        acc * 5
            + match delimiter {
                ChunkDelimiter::Parenthesis => 1,
                ChunkDelimiter::SquareBracket => 2,
                ChunkDelimiter::CurlyBracket => 3,
                ChunkDelimiter::LessGreaterSign => 4,
            }
    });

    SyntaxCheckingResult::Ok(points)
}
