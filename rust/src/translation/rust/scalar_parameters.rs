//! Token-region premises; source bodies still pass the ordinary parser and origin checks.

use super::scalar_parameter_rules::detect_rust_source_assignment_token;
use super::{TokenCursor, TokenKind};

pub(super) fn read_rust_scalar_parameter_body(cursor: &TokenCursor) -> (bool, bool) {
    if !cursor.is("{") {
        return (false, false);
    }
    let mut depth = 0;
    let mut possible_write = false;
    let mut offset = 0;
    loop {
        let token = cursor.peek_at(offset);
        if token.kind == TokenKind::Eof {
            return (false, possible_write);
        }
        let kind = if token.kind == TokenKind::Punct {
            "punct"
        } else {
            ""
        };
        possible_write |= detect_rust_source_assignment_token(kind, &token.value);
        if offset > 0 && token.kind == TokenKind::Punct {
            let previous = cursor.peek_at(offset - 1);
            if previous.kind == TokenKind::Punct && previous.end == token.start {
                let mut spelling = previous.value.clone();
                spelling.push_str(&token.value);
                possible_write |= detect_rust_source_assignment_token("punct", &spelling);
            }
        }
        if token.kind == TokenKind::Punct && token.value == "{" {
            depth += 1;
        }
        if token.kind == TokenKind::Punct && token.value == "}" {
            depth -= 1;
            if depth == 0 {
                return (true, possible_write);
            }
        }
        offset += 1;
    }
}
