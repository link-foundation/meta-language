//! Whole closed JavaScript class scopes use the ordinary shared frontend and
//! emit every checked declaration together. No instance interface is published
//! to external consumers before its complete prototype model is represented.
use super::SelfTranslationSignature;
use crate::translation::Language;
use crate::translation::diagnostics::{Result, unsupported};
use crate::translation::emit_common::Emitted;
use crate::translation::lexer::{TokenKind, tokenize};

pub(super) fn read(source: &str) -> Result<Option<(Emitted, Vec<SelfTranslationSignature>)>> {
    let tokens = tokenize(source, Language::JavaScript)?.tokens;
    if !tokens.iter().enumerate().any(|(index, token)| {
        token.kind == TokenKind::Identifier
            && token.value == "class"
            && tokens
                .get(index.wrapping_sub(1))
                .is_none_or(|previous| previous.value != "." && previous.value != "?.")
            && tokens
                .get(index + 1)
                .is_some_and(|next| next.kind == TokenKind::Identifier || next.value == "{")
    }) {
        return Ok(None);
    }
    let surface = crate::translation::javascript::parse_javascript(source)?;
    let program = crate::translation::check::check_program(&surface)?;
    if program
        .main
        .as_ref()
        .is_some_and(|main| !main.effects.is_empty())
    {
        return Err(unsupported(
            "JavaScript closed class publication",
            "a whole executable needs its independent entry-point composition",
            None,
        ));
    }
    let emitted = crate::translation::emit_rust::emit_rust_with_array_consumption(
        &program,
        Default::default(),
    )?;
    Ok(Some((emitted, Vec::new())))
}
