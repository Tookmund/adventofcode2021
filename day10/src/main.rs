use std::io;
use std::collections::HashMap;
use std::iter::{Iterator, IntoIterator};

// Map initialization macro borrowed from https://stackoverflow.com/a/27582993
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
}


fn syntax_error_score<B: io::BufRead>(bufread: B) -> io::Result<usize> {
    let close_to_open: HashMap<char, char> = collection! {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
    };

    let invalid_score: HashMap<char, usize> = collection! {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
    };
    let mut score = 0;
    for line in bufread.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line?.chars() {
            match c {
                ')' | ']' | '}' | '>' => {
                    let s = stack.pop().unwrap();
                    if s != close_to_open[&c] {
                        score += invalid_score[&c];
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
