#[cfg(test)]
mod tests {
    use super::*;
    use crate::translation::{
        lexer::tokenize,
        rust::{SourceOrigins, SourceProfiles},
        types::Type,
    };
    use std::sync::Arc;

    #[test]
    fn the_actual_callback_type_positions_resolve_concrete_binders() {
        let source = "fn apply<T>(value: &mut u8, mut map: impl FnMut(&T, bool) -> T) -> T { map(&*value, false) }";
        let tokens = Arc::new(tokenize(source, Language::Rust).unwrap().tokens);
        let directory = Rc::new(
            FnMutSourceDeclarations::new(source, Arc::clone(&tokens), "1.90.0".to_owned()).unwrap(),
        );
        let declaration = directory.capture(&[], &tokens[0]).unwrap().unwrap();
        let value = tokens.iter().position(|token| token.value == "u8").unwrap();
        let mut parser = RustParser::new(
            (*tokens).clone(),
            SourceOrigins::new(None, ""),
            SourceProfiles {
                exclusive_source: "Rust exclusive source loans 1",
                fn_mut_source: "Rust closed source FnMut specializations 1",
                ..SourceProfiles::default()
            },
            false,
            None,
        );
        let argument = parser
            .read_fn_mut_source_type_range(&directory, &(value..value + 1))
            .unwrap();
        parser.fn_mut_source_declarations = Some(Rc::clone(&directory));
        let ReplayedFnMutSourceHeader {
            function,
            callback: admitted,
            resolved: header,
        } = parser
            .preview_fn_mut_source_header(&declaration, vec![("T".into(), argument)])
            .unwrap();
        let expected = Type::Fixed {
            bits: 8,
            signed: false,
        };
        assert_eq!(admitted.owner.path, ["apply"]);
        assert_eq!(function.params[1].name, "map");
        assert_eq!(function.span.unwrap().end, source.len());
        assert_eq!(admitted.parameters[0].ty.as_ref(), Some(&expected));
        assert_eq!(admitted.parameters[0].wrappers, ["reference"]);
        assert_eq!(admitted.parameters[1].ty, Some(Type::Bool));
        assert_eq!(admitted.result.ty.as_ref(), Some(&expected));
        assert_eq!(header.parameters[0].ty.as_ref(), Some(&expected));
        assert_eq!(header.exclusive_parameters, [0]);
        assert_eq!(header.parameters[0].wrappers, ["reference"]);
        assert!(matches!(
            header.parameters[1].ty,
            Some(Type::Function { .. })
        ));
        assert_eq!(header.result.ty, Some(expected));
        assert_eq!(parser.cursor.index, 0);
        assert!(parser.fn_mut_source_type_bindings.is_empty());
    }

    #[test]
    fn a_template_is_selected_by_its_source_owner_and_explicit_type_arguments() {
        let source = "fn apply<T>(value: u8, mut map: impl FnMut(&T) -> T) -> T { map(&value) } fn main() { apply::<u8>(0, |x| *x); }";
        let tokens = Arc::new(tokenize(source, Language::Rust).unwrap().tokens);
        let directory = Rc::new(
            FnMutSourceDeclarations::new(source, Arc::clone(&tokens), "1.90.0".to_owned()).unwrap(),
        );
        let mut parser = RustParser::new(
            (*tokens).clone(),
            SourceOrigins::new(None, ""),
            SourceProfiles {
                exclusive_source: "Rust exclusive source loans 1",
                fn_mut_source: "Rust closed source FnMut specializations 1",
                ..SourceProfiles::default()
            },
            true,
            None,
        );
        parser.fn_mut_source_declarations = Some(directory);
        assert!(parser.capture_fn_mut_source_item(&[]).unwrap());
        assert_eq!(parser.cursor.peek().value, "fn");
        let segments = vec!["apply".to_owned()];
        let template = parser
            .select_fn_mut_source_template(&segments, &[])
            .unwrap();
        parser.cursor.index = tokens.iter().position(|token| token.value == "::").unwrap();
        let bindings = parser.read_fn_mut_source_type_arguments(&template).unwrap();
        assert_eq!(parser.cursor.peek().value, "(");
        let ReplayedFnMutSourceHeader {
            callback,
            resolved: concrete,
            ..
        } = parser
            .preview_fn_mut_source_header(&template, bindings)
            .unwrap();
        assert_eq!(callback.owner.callback.name, "map");
        assert_eq!(
            concrete.result.ty,
            Some(Type::Fixed {
                bits: 8,
                signed: false
            })
        );
        assert_eq!(parser.cursor.peek().value, "(");
        parser
            .origins
            .scope
            .insert("apply".to_owned(), SourceOrigin::value());
        assert!(
            parser
                .select_fn_mut_source_template(&segments, &[])
                .is_none()
        );
        assert!(
            parser
                .select_fn_mut_source_template(&["crate".into(), "apply".into()], &[])
                .is_some()
        );
    }
}
