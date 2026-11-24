//! Ordinary item translation retaining original compiler diagnostics.
use super::*;

pub(super) fn translate_group(
    group: &Group<'_>,
    from: &str,
    to: &str,
    decorators: &DecoratorSet,
    context: &Context,
) -> Translated {
    translate_group_bound(group, from, to, decorators, context, &HashMap::new())
}

pub(super) fn translate_group_bound(
    group: &Group<'_>,
    from: &str,
    to: &str,
    decorators: &DecoratorSet,
    context: &Context,
    literal_bindings: &HashMap<String, SExpr>,
) -> Translated {
    let done = |code: String, status: &'static str| Translated {
        code: Some(code),
        preludes: Vec::new(),
        status,
        reason: None,
        diagnostic: None,
        signatures: Vec::new(),
    };
    let (text, term) = match &group.kind {
        GroupKind::Provenance => {
            return Translated {
                code: None,
                preludes: Vec::new(),
                status: "provenance",
                reason: None,
                diagnostic: None,
                signatures: Vec::new(),
            };
        }
        GroupKind::Carried { language, lines } => {
            if family(language) == family(to) {
                return done(lines.join("\n"), "restored");
            }
            let mut code = vec![group.items[0].text.to_owned()];
            code.extend(lines.iter().map(|line| source_line(line)));
            return Translated {
                reason: Some("carried from another language".to_owned()),
                ..done(code.join("\n"), "carried")
            };
        }
        GroupKind::Comment { text, term } => {
            // A copied comment has no provenance, so it must also fit the source.
            if group
                .items
                .iter()
                .all(|item| comment_fits(item.text, from) && comment_fits(item.text, to))
            {
                return done(text.clone(), "comment");
            }
            return carry(text, term, from, "comment the target cannot hold");
        }
        GroupKind::Item { text, term } => (text, term),
    };
    if term == "ERROR" {
        return carry(text, term, from, "syntax");
    }
    if from == "TypeScript" && to == "JavaScript" {
        let erased = match erase_type_script(text) {
            Ok(erased) => erased,
            Err(error) => return carry(text, term, from, &error.reason),
        };
        let erased_code = erased.code.trim();
        let count = top_level_items(erased_code, "JavaScript")
            .iter()
            .filter(|item| !item.comment)
            .count();
        let code = if count == 0 { "" } else { erased_code };
        let mut lines = vec![format!(
            "{TRANSLATED}{from} {term} items={count} sha256={}",
            sha256(code)
        )];
        lines.extend(LINE_BREAK.split(text).map(source_line));
        if !code.is_empty() {
            lines.push(code.to_owned());
        }
        return done(lines.join("\n"), "translated");
    }
    let (emitted, signatures) = match emit_item(text, from, to, context, literal_bindings) {
        Ok(Some(emitted)) => emitted,
        Ok(None) => return carry(text, term, from, "top-level statement"),
        Err(error) => {
            let mut carried = carry(
                text,
                term,
                from,
                if from == "TypeScript"
                    || error.construct.as_deref() == Some("UTF16 Text representation")
                {
                    &error.reason
                } else {
                    error.kind.as_str()
                },
            );
            carried.diagnostic = Some(error);
            return carried;
        }
    };
    record_translated_definitions(
        (text, term, from, to, decorators),
        emitted,
        signatures,
        None,
    )
}
