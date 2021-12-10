use std::io;

fn close_to_open(c: &char) -> Option<char> {
    match *c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

fn invalid_score(c: &char) -> Option<usize> {
    match *c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
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
                    if s != close_to_open(&c).unwrap() {
                        score += invalid_score(&c).unwrap();
                        break;
                    }
                },
                _ => stack.push(c),
            }
        }
    }
    Ok(score)
}

#[cfg(test)]
mod test {
    use crate::syntax_error_score;

    #[test]
    fn part1_example() {
        let ts: &[u8] = b"\
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
        assert_eq!(syntax_error_score(ts).unwrap(), 26397)
    }
}

fn main() -> io::Result<()>{
    println!("Syntax Error Score: {}", syntax_error_score(io::stdin().lock())?);
    Ok(())
}
