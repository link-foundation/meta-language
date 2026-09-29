//! `console.log` with several arguments, which prints `util.format` of them.
//!
//! Mirrors `consoleFormat` in `js/src/translation/javascript.js`.

use super::{unsupported, BinaryOp, Result, SExpr, SNode, ShowStyle};

/// The style of the value a directive takes, when it is portable.
const fn directive_style(directive: char) -> Option<ShowStyle> {
    match directive {
        's' => Some(ShowStyle::JsConsole),
        'd' => Some(ShowStyle::JsFormatNumber),
        'i' => Some(ShowStyle::JsFormatInteger),
        _ => None,
    }
}

const fn text(value: String, place: Option<crate::translation::Span>) -> SExpr {
    SExpr::new(SNode::Str { value }, place)
}

fn show(arg: SExpr, style: ShowStyle) -> SExpr {
    let place = arg.span;
    SExpr::new(
        SNode::Show {
            arg: Box::new(arg),
            style,
        },
        place,
    )
}

/// `console.log(a, b, …)` prints `util.format(a, b, …)`: a literal first
/// string reads its `%s`, `%d`, `%i`, `%c` and `%%` directives, each taking
/// the next argument, and the arguments left over follow, each after a
/// space, strings as they are and other values as the console shows them. A
/// first argument that is not a literal string is shown like the rest, and
/// the checker refuses it when it is a string, whose directives only the run
/// reads.
pub(super) fn console_format(args: Vec<SExpr>) -> Result<SExpr> {
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Ok(text(String::new(), None));
    };
    let whole = first.span;
    let mut pieces = Vec::new();
    let mut values = args.peekable();
    if let SNode::Str { value: source } = &first.node {
        let chars: Vec<char> = source.chars().collect();
        let mut written = String::new();
        let mut index = 0;
        while index < chars.len() {
            let char = chars[index];
            index += 1;
            let Some(&directive) = chars.get(index).filter(|_| char == '%') else {
                written.push(char);
                continue;
            };
            if directive == '%' {
                written.push('%');
                index += 1;
                continue;
            }
            // A directive with no argument left, or an unknown one, stays as it is written.
            if values.peek().is_none() || !"sdifjoOc".contains(directive) {
                written.push(char);
                continue;
            }
            let value = values.next().unwrap_or_else(|| unreachable!());
            index += 1;
            if directive == 'c' {
                if !matches!(value.node, SNode::Str { .. }) {
                    return Err(unsupported(
                        "%c with a computed style",
                        "console.log discards the CSS a %c directive takes; pass it as a string literal",
                        value.span,
                    ));
                }
                continue;
            }
            let Some(style) = directive_style(directive) else {
                return Err(unsupported(
                    &format!("console.log %{directive} directive"),
                    "the portable directives are %s, %d, %i, %c and %%",
                    whole,
                ));
            };
            if !written.is_empty() {
                pieces.push(text(std::mem::take(&mut written), whole));
            }
            pieces.push(show(value, style));
        }
        if !written.is_empty() || pieces.is_empty() {
            pieces.push(text(written, whole));
        }
    } else {
        pieces.push(show(first, ShowStyle::JsFormatFirst));
    }
    for value in values {
        pieces.push(text(" ".to_owned(), value.span));
        pieces.push(show(value, ShowStyle::JsConsole));
    }
    let mut pieces = pieces.into_iter();
    let first = pieces.next().unwrap_or_else(|| unreachable!());
    Ok(pieces.fold(first, |left, right| {
        SExpr::new(
            SNode::Binary {
                op: BinaryOp::Concat,
                left: Box::new(left),
                right: Box::new(right),
                rounding: None,
            },
            whole,
        )
    }))
}
