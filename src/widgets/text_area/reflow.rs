pub fn reflow<C, P, F>(
    chars: &Vec<C>,
    line_width: usize, 
    is_separator: P, 
    is_newline: F,
) -> Vec<Vec<C>>
where 
    C: std::clone::Clone + std::cmp::PartialEq,
    P: Fn(&C) -> bool,
    F: Fn(&C) -> bool,
{
    if line_width == 0 {
        return Vec::new();
    }

    let mut ret = Vec::new();
    let mut itr = chars.iter();
    let mut line = Vec::new();
    let mut word = Vec::new();

    loop {
        match itr.next() {
            Some(c) => {
                word.push(c.clone());
                if is_separator(c) || is_newline(c) || word.len() == line_width {
                    // word end
                    append_word(&mut ret, &mut line, &mut word, line_width);
                } 
                if is_newline(c) || line.len() == line_width {
                    ret.push(std::mem::take(&mut line));
                }
            },
            None => {
                append_word(&mut ret, &mut line, &mut word, line_width);
                if line.len() != 0 {
                    ret.push(std::mem::take(&mut line));
                }
                break;
            }
        }
    }
    ret
}

fn append_word<C>(
    lines: &mut Vec<Vec<C>>,
    line: &mut Vec<C>,
    word: &mut Vec<C>,
    line_width: usize, 
) 
where C: std::clone::Clone + std::cmp::PartialEq {
    loop {
        if line.len() + word.len() <= line_width {
            // word fits on line
            line.append(word);
            break;
        } else {
            assert!(line.len() != 0);
            // start a new line
            lines.push(std::mem::take(line));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn char_vec(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn no_characters_no_lines() {
        assert_eq!(
            reflow(&char_vec(""), 2, |&c| c == ' ', |&c| c == '\n'), 
            Vec::<Vec<char>>::new()
        )
    }

    #[test]
    fn zero_width() {
        assert_eq!(
            reflow(&char_vec("reu"), 0, |&c| c == ' ', |&c| c == '\n'), 
            Vec::<Vec<char>>::new()
        )
    }

    #[test]
    fn word_within_line() {
        assert_eq!(
            reflow(&char_vec("reu"), 4, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("reu")]
        )
    }

    #[test]
    fn two_words_within_line() {
        assert_eq!(
            reflow(&char_vec("reu reu"), 8, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("reu reu")]
        )
    }

    #[test]
    fn two_words_over_two_lines() {
        assert_eq!(
            reflow(&char_vec("reu reu"), 4, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("reu "), char_vec("reu")]
        )
    }

    #[test]
    fn three_words_over_two_lines() {
        assert_eq!(
            reflow(&char_vec("reu reu reu"), 8, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("reu reu "), char_vec("reu")]
        )
    }

    #[test]
    fn three_words_over_three_lines() {
        assert_eq!(
            reflow(&char_vec("reu reu reu"), 5, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("reu "), char_vec("reu "), char_vec("reu")]
        )
    }

    #[test]
    fn line_break() {
        assert_eq!(
            reflow(&char_vec("reu\nreu"), 8, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("reu\n"), char_vec("reu")]
        )
    }

    #[test]
    fn clipping() {
        assert_eq!(
            reflow(&char_vec("reuben"), 5, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("reube"), char_vec("n")]
        )
    }

    #[test]
    fn multiple_clipping() {
        assert_eq!(
            reflow(&char_vec("reu"), 1, |&c| c == ' ', |&c| c == '\n'), 
            vec![char_vec("r"), char_vec("e"), char_vec("u")]
        )
    }
}