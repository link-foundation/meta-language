fn last_end(children: &[GrammarNode]) -> usize {
    children.last().map_or(0, |child| child.end)
}

// Appends `error` to `children`, extending a preceding ERROR node; with
// `keep_objects`, well-formed objects are appended as they are.
fn push_error(children: &mut Vec<GrammarNode>, error: GrammarNode, keep_objects: bool) {
    if keep_objects && !error.is_error && error.named && error.term != "keyword" {
        children.push(error);
        return;
    }
    let (start, end) = (error.start, error.end);
    let nodes = if error.is_error {
        error.children
    } else {
        vec![error]
    };
    match children.last_mut() {
        Some(last) if last.is_error => {
            last.children.extend(nodes);
            last.end = end;
        }
        _ => children.push(GrammarNode::error(start, end, nodes)),
    }
}

fn leaf_for(token: Token<'_>) -> GrammarNode {
    if VALUE_TOKENS.contains(&token.kind) {
        return GrammarNode::node(token.kind, token.start, token.end, Vec::new());
    }
    if token.kind == "keyword" {
        return COS_KEYWORDS
            .iter()
            .find(|keyword| **keyword == token.text)
            .map_or_else(
                || GrammarNode::node("keyword", token.start, token.end, Vec::new()),
                |keyword| GrammarNode::anonymous(keyword, token.start, token.end),
            );
    }
    GrammarNode::anonymous(token.kind, token.start, token.end)
}

// Object, generation, cross-reference and offset numbers are unsigned digits.
fn is_unsigned(token: Token<'_>) -> bool {
    token.kind == "integer" && token.text.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_keyword(token: Token<'_>, keyword: &str) -> bool {
    token.kind == "keyword" && token.text == keyword
}

fn is_header(bytes: &[u8]) -> bool {
    let Some(version) = bytes.strip_prefix(b"%PDF-") else {
        return false;
    };
    let mut parts = version.splitn(2, |byte| *byte == b'.');
    let is_digits = |part: &[u8]| !part.is_empty() && part.iter().all(u8::is_ascii_digit);
    matches!((parts.next(), parts.next()), (Some(major), Some(minor)) if is_digits(major) && is_digits(minor))
}

fn is_integer(run: &[u8]) -> bool {
    let digits = run
        .strip_prefix(b"+")
        .or_else(|| run.strip_prefix(b"-"))
        .unwrap_or(run);
    !digits.is_empty() && digits.iter().all(u8::is_ascii_digit)
}

fn is_real(run: &[u8]) -> bool {
    let unsigned = run
        .strip_prefix(b"+")
        .or_else(|| run.strip_prefix(b"-"))
        .unwrap_or(run);
    let Some(point) = unsigned.iter().position(|byte| *byte == b'.') else {
        return false;
    };
    let (whole, fraction) = (&unsigned[..point], &unsigned[point + 1..]);
    whole.iter().all(u8::is_ascii_digit)
        && fraction.iter().all(u8::is_ascii_digit)
        && !(whole.is_empty() && fraction.is_empty())
}

fn dictionary_entries(dictionary: &GrammarNode) -> impl Iterator<Item = &GrammarNode> {
    dictionary
        .children
        .iter()
        .filter(|child| child.term == "dictionary_entry")
}

fn entry_key<'a>(entry: &GrammarNode, text: &'a str) -> &'a str {
    &text[entry.children[0].start..entry.children[0].end]
}

// The value of a direct, non-negative integer `/Length` that JavaScript holds
// exactly, if any.
fn direct_length(dictionary: &GrammarNode, text: &str) -> Option<usize> {
    let entry =
        dictionary_entries(dictionary).find(|candidate| entry_key(candidate, text) == "/Length")?;
    let value = entry
        .children
        .get(1)
        .filter(|value| value.term == "integer")?;
    let run = &text.as_bytes()[value.start..value.end];
    let (negative, digits) = match run.split_first() {
        Some((b'-', digits)) => (true, digits),
        Some((b'+', digits)) => (false, digits),
        _ => (false, run),
    };
    let length = digits.iter().try_fold(0_u64, |length, digit| {
        length.checked_mul(10)?.checked_add(u64::from(digit - b'0'))
    })?;
    if length > MAX_SAFE_INTEGER || (negative && length != 0) {
        return None;
    }
    usize::try_from(length).ok()
}

fn is_content_stream_dictionary(dictionary: &GrammarNode, text: &str) -> bool {
    if dictionary
        .children
        .iter()
        .any(|child| child.is_error || child.is_missing)
    {
        return false;
    }
    dictionary_entries(dictionary).all(|entry| {
        let key = entry_key(entry, text);
        let value = &entry.children[1];
        let name = (value.term == "name").then(|| &text[value.start..value.end]);
        if !CONTENT_STREAM_KEYS.contains(&key) {
            return false;
        }
        match key {
            "/Type" => matches!(name, Some("/XObject" | "/Pattern")),
            "/Subtype" => name == Some("/Form"),
            _ => true,
        }
    })
}

// The first index of `needle` in `haystack` at or after `from`.
fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack
        .get(from..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|index| index + from)
}

const fn is_white(byte: Option<u8>) -> bool {
    matches!(byte, Some(b' ' | b'\n' | b'\r' | b'\t' | 0x0c | 0))
}

fn is_regular(byte: Option<u8>) -> bool {
    byte.is_some_and(|byte| !is_white(Some(byte)) && !b"()<>[]{}/%".contains(&byte))
}
