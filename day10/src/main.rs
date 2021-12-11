use std::io;
use std::io::prelude::*;

fn matching(c: &char) -> char {
    match *c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        c => panic!("Invalid matching character: {}", c),
    }
}

fn invalid_score(c: &char) -> usize {
    match *c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        c => panic!("Invalid invalid_score character: {}", c),
    }
}

fn incomplete_score(c: &char) -> usize {
    match *c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        c => panic!("Invalid incomplete_score character: {}", c),
    }
}

fn syntax_error_score<B: io::BufRead>(bufread: B) -> io::Result<usize> {
    let mut score = 0;
    for line in bufread.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line?.chars() {
            match c {
                ')' | ']' | '}' | '>' => {
                    let s = stack.pop().unwrap();
                    if s != matching(&c) {
                        score += invalid_score(&c);
                        break;
                    }
                },
                _ => stack.push(c),
            }
        }
    }
    Ok(score)
}

fn incomplete_lines_score<B: io::BufRead>(bufread: B) -> io::Result<usize> {
    let mut score: Vec<usize> = Vec::new();
    for line in bufread.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line?.chars() {
            match c {
                ')' | ']' | '}' | '>' => {
                    let s = stack.pop().unwrap();
                    if s != matching(&c) {
                        stack = Vec::new();
                        break;
                    }
                },
                _ => stack.push(c),
            }
        }
        if !stack.is_empty() {
            let mut line_score = 0;
            let mut missing: Vec<char> = Vec::new();
            // Reverse the stack because push/pop operate on the end of a vector
            for s in stack.iter().rev() {
                line_score *= 5;
                let missing_char = matching(&s);
                missing.push(missing_char);
                line_score += incomplete_score(&missing_char);
            }
            score.push(line_score);
        }
    }
    score.sort();
    if score.is_empty() {
        return Ok(0)
    } else {
        Ok(score[score.len()/2])
    }
}

#[cfg(test)]
mod test {
    use crate::syntax_error_score;
    use crate::incomplete_lines_score;

    const EXAMPLE: &[u8] = b"\
[({(<(())[]>[[{[]{<()<>>\n\
[(()[<>])]({[<{<<[]>>(\n\
{([(<{}[<>[]}>{[]{[(<()>\n\
(((({<>}<{<{<>}{[]{[]{}\n\
[[<[([]))<([[{}[[()]]]\n\
[{[{({}]{}}([{[{{{}}([]\n\
{<[[]]>}<{[{[{[]{()[[[]\n\
[<(<(<(<{}))><([]([]()\n\
<{([([[(<>()){}]>(<<{{\n\
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_example() {
        assert_eq!(syntax_error_score(EXAMPLE).unwrap(), 26397)
    }

    #[test]
    fn part2_example() {
        assert_eq!(incomplete_lines_score(EXAMPLE).unwrap(), 288957)
    }
}

fn main() -> io::Result<()>{
    let mut stdin = Vec::new();
    io::stdin().read_to_end(&mut stdin)?;
    println!("Syntax Error Score: {}", syntax_error_score(&stdin[..])?);
    println!("Incomplete Lines Score: {}", incomplete_lines_score(&stdin[..])?);
    Ok(())
}
