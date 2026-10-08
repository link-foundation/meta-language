//! The layout of emitted Rust (issue #217), the twin of
//! `js/src/translation/rust-layout.js`.
//!
//! The emitters write each expression on one line, so a long array, call or
//! macro argument list stays one line however long it gets. [`wrap_rust`]
//! breaks every line longer than [`RUST_WIDTH`] the way rustfmt lays out a
//! list that does not fit: the bracket's items go one per line, indented four
//! spaces past the line, and the closing bracket returns to the line's
//! indentation. Both roots lay out alike, so a consumer can hold its committed
//! Rust byte-equal to the translator.
//!
//! Only parentheses and square brackets are broken: a brace opens a block,
//! whose statements are not comma lists. Among the brackets of a line that no
//! other parenthesis or square bracket encloses, the one with the longest
//! content is broken first (the leftmost on a tie), and every line that
//! results is laid out again, so nested lists break from the outside in. Two
//! or more items each end with a comma; a single item keeps none, so a
//! parenthesized expression never becomes a tuple. A line with nothing to
//! break (one long string literal) stays as it is.

/// The width rustfmt lays Rust out to by default.
pub const RUST_WIDTH: usize = 100;

const INDENT: &str = "    ";

/// A parenthesis or square bracket closed on its line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustBracket {
    /// The position of the opening bracket.
    pub open: usize,
    /// The position of the closing bracket.
    pub close: usize,
    /// Whether another parenthesis or square bracket encloses it.
    pub enclosed: bool,
    /// The positions of its own commas.
    pub commas: Vec<usize>,
}

struct Open {
    character: char,
    open: usize,
    enclosed: bool,
    commas: Vec<usize>,
}

fn is_identifier_character(character: Option<&char>) -> bool {
    character.is_some_and(|character| character.is_alphanumeric() || *character == '_')
}

const fn closer(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        _ => '>',
    }
}

/// The length of the string, raw string or character literal that starts at
/// `index` of `characters`, or 0 when none starts there.
fn literal_length(characters: &[char], index: usize) -> usize {
    let at = |position: usize| characters.get(position).copied();
    let before = index
        .checked_sub(1)
        .and_then(|position| characters.get(position));
    match characters[index] {
        '"' => {
            let mut end = index + 1;
            while end < characters.len() && characters[end] != '"' {
                end += if characters[end] == '\\' { 2 } else { 1 };
            }
            (end + 1).min(characters.len()) - index
        }
        'r' if !is_identifier_character(before) => {
            let mut hashes = 0;
            while at(index + 1 + hashes) == Some('#') {
                hashes += 1;
            }
            if at(index + 1 + hashes) != Some('"') {
                return 0;
            }
            let closing: Vec<char> = std::iter::once('"')
                .chain(std::iter::repeat_n('#', hashes))
                .collect();
            let mut end = index + 2 + hashes;
            while end < characters.len() {
                if characters[end..].starts_with(&closing) {
                    return end + closing.len() - index;
                }
                end += 1;
            }
            characters.len() - index
        }
        '\'' => {
            if at(index + 1) == Some('\\') {
                let mut end = index + 2;
                while end < characters.len() && characters[end] != '\'' {
                    end += 1;
                }
                if end < characters.len() {
                    end + 1 - index
                } else {
                    0
                }
            } else if at(index + 2) == Some('\'') {
                3
            } else {
                0
            }
        }
        _ => 0,
    }
}

/// The parentheses and square brackets closed on a line, with their commas.
#[must_use]
pub fn rust_brackets(characters: &[char]) -> Vec<RustBracket> {
    let mut stack: Vec<Open> = Vec::new();
    let mut brackets = Vec::new();
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        let literal = literal_length(characters, index);
        if literal > 0 {
            index += literal;
            continue;
        }
        if character == '/' && characters.get(index + 1) == Some(&'/') {
            break;
        }
        let before = index
            .checked_sub(1)
            .and_then(|position| characters.get(position));
        let generic = character == '<' && (is_identifier_character(before) || before == Some(&':'));
        if matches!(character, '(' | '[' | '{') || generic {
            let enclosed = stack.iter().any(|open| matches!(open.character, '(' | '['));
            stack.push(Open {
                character,
                open: index,
                enclosed,
                commas: Vec::new(),
            });
        } else if character == ','
            && let Some(top) = stack.last_mut()
        {
            top.commas.push(index);
        } else if stack
            .last()
            .is_some_and(|top| character == closer(top.character))
            && !(character == '>' && before == Some(&'-'))
        {
            if let Some(open) = stack.pop()
                && matches!(open.character, '(' | '[')
            {
                brackets.push(RustBracket {
                    open: open.open,
                    close: index,
                    enclosed: open.enclosed,
                    commas: open.commas,
                });
            }
        }
        index += 1;
    }
    brackets
}

/// The bracket to break in a line that is too long, if any.
fn bracket_to_break(characters: &[char]) -> Option<RustBracket> {
    let mut chosen: Option<RustBracket> = None;
    for bracket in rust_brackets(characters) {
        let length = bracket.close - bracket.open - 1;
        let content: String = characters[bracket.open + 1..bracket.close].iter().collect();
        if bracket.enclosed || content.trim().is_empty() {
            continue;
        }
        let better = chosen.as_ref().is_none_or(|current| {
            let current_length = current.close - current.open - 1;
            length > current_length || (length == current_length && bracket.open < current.open)
        });
        if better {
            chosen = Some(bracket);
        }
    }
    chosen
}

/// The lines one long line is laid out as.
fn wrap_line(line: &str, width: usize, lines: &mut Vec<String>) {
    let characters: Vec<char> = line.chars().collect();
    if characters.len() <= width {
        lines.push(line.to_owned());
        return;
    }
    let Some(bracket) = bracket_to_break(&characters) else {
        lines.push(line.to_owned());
        return;
    };
    let indent = &line[..line.len() - line.trim_start().len()];
    let mut cuts = vec![bracket.open];
    cuts.extend(&bracket.commas);
    cuts.push(bracket.close);
    let items: Vec<String> = cuts
        .windows(2)
        .map(|pair| {
            characters[pair[0] + 1..pair[1]]
                .iter()
                .collect::<String>()
                .trim()
                .to_owned()
        })
        .filter(|item| !item.is_empty())
        .collect();
    let comma = if items.len() > 1 { "," } else { "" };
    let head: String = characters[..=bracket.open].iter().collect();
    let tail: String = characters[bracket.close..].iter().collect();
    wrap_line(head.trim_end(), width, lines);
    for item in &items {
        wrap_line(&format!("{indent}{INDENT}{item}{comma}"), width, lines);
    }
    wrap_line(&format!("{indent}{tail}"), width, lines);
}

/// `code` with every line longer than `width` characters broken at its
/// longest bracketed list, recursively.
#[must_use]
pub fn wrap_rust(code: &str, width: usize) -> String {
    let mut lines = Vec::new();
    for line in code.split('\n') {
        wrap_line(line, width, &mut lines);
    }
    lines.join("\n")
}
