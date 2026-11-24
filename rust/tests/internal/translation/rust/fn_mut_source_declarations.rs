#[cfg(test)]
mod tests {
    use super::{Arc, FnMutSourceDeclarations};
    use crate::translation::{Language, lexer::tokenize};

    #[test]
    fn shared_header_plan_retains_real_generic_and_callback_source_ranges() {
        let source = "fn apply<T, E>(value: T, action: &mut impl FnMut(T, (u8, u8)) -> Result<T, Option<E>>) -> Result<T, Option<E>> { /* 🧪 */ action(value, (1, 2)) }";
        let tokens = Arc::new(tokenize(source, Language::Rust).unwrap().tokens);
        let directory =
            FnMutSourceDeclarations::new(source, Arc::clone(&tokens), "1.90.0".to_owned()).unwrap();
        let declaration = directory
            .capture(&["sample".to_owned()], &tokens[0])
            .unwrap()
            .unwrap();
        let checked = directory.checked_owner(&declaration);
        let spelling = crate::translation::lexer::Source::new(source);
        assert_eq!(checked.path, ["sample", "apply"]);
        assert_eq!(
            spelling.slice(checked.callback.binder.start, checked.callback.binder.end),
            "action"
        );
        assert_eq!(
            spelling.slice(
                checked.callback.result_range.start,
                checked.callback.result_range.end
            ),
            "Result<T, Option<E>>"
        );
        assert_eq!(checked.range.end, source.encode_utf16().count());
        let spellings = |position: std::ops::Range<usize>| {
            directory
                .source
                .slice(tokens[position.start].start, tokens[position.end - 1].end)
        };
        assert_eq!(declaration.owner, ["sample"]);
        assert_eq!(declaration.name, "apply");
        assert_eq!(spellings(declaration.declaration), source);
        assert!(spellings(declaration.body).contains("/* 🧪 */"));
        assert_eq!(
            spellings(declaration.result.unwrap()),
            "Result<T, Option<E>>"
        );
        assert_eq!(declaration.formal_count, 2);
        assert_eq!(
            declaration
                .generics
                .iter()
                .map(|index| tokens[*index].value.as_str())
                .collect::<Vec<_>>(),
            ["T", "E"]
        );
        let callback = declaration.callback;
        assert_eq!(callback.parameter, 1);
        assert_eq!(tokens[callback.binder].value, "action");
        assert!(callback.exclusive);
        assert!(!callback.mutable);
        assert_eq!(
            spellings(callback.callback_type),
            "&mut impl FnMut(T, (u8, u8)) -> Result<T, Option<E>>"
        );
        assert_eq!(
            callback
                .parameters
                .into_iter()
                .map(spellings)
                .collect::<Vec<_>>(),
            ["T", "(u8, u8)"]
        );
        assert_eq!(spellings(callback.result), "Result<T, Option<E>>");
    }
}
