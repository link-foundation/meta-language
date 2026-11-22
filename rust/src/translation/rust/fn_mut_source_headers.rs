//! Read selected type positions through the normal Rust source type checker.
use super::{
    Language, Result, RustParser, SourceOrigin, Token, TokenCursor,
    fn_mut_source_declarations::{FnMutSourceDeclaration, FnMutSourceDeclarations},
    unsupported,
};
use crate::translation::{
    source_fn_mut_specializations::SourceDeclaration, source_loop_folds::SourceFoldSpan,
};
use std::ops::Range;

pub(super) struct CheckedCallbackHeader {
    pub(super) owner: SourceDeclaration,
    pub(super) parameters: Vec<SourceOrigin>,
    pub(super) result: SourceOrigin,
}

impl RustParser {
    pub(super) fn resolve_fn_mut_source_type_parameter(
        &self,
        segments: &[String],
        token: &Token,
    ) -> Result<Option<SourceOrigin>> {
        let [name] = segments else {
            return Ok(None);
        };
        let Some((_, origin)) = self
            .fn_mut_source_type_bindings
            .iter()
            .find(|(source, _)| source == name)
        else {
            return Ok(None);
        };
        if self.cursor.is("<") {
            return Err(unsupported(
                "Rust source FnMut type parameter",
                "the actual source binder has one concrete type, without its own arguments",
                Some(token.span()),
            ));
        }
        let mut origin = origin.clone();
        origin
            .expanded_source_owner
            .clone_from(&origin.source_owner);
        Ok(Some(origin))
    }

    pub(super) fn read_fn_mut_source_callback_header(
        &mut self,
        directory: &FnMutSourceDeclarations,
        declaration: &FnMutSourceDeclaration,
        bindings: Vec<(String, SourceOrigin)>,
    ) -> Result<CheckedCallbackHeader> {
        let owner = directory.checked_owner(declaration);
        if owner.generics.len() != bindings.len()
            || owner
                .generics
                .iter()
                .zip(&bindings)
                .any(|(binder, (name, origin))| {
                    binder.name != *name
                        || origin.ty.is_none()
                        || origin.source_owner.is_none()
                        || !origin.wrappers.is_empty()
                })
        {
            return Err(unsupported(
                "Rust source FnMut type arguments",
                "every actual generic binder needs its direct concrete source type in declaration order",
                None,
            ));
        }
        let previous_bindings = std::mem::replace(&mut self.fn_mut_source_type_bindings, bindings);
        let module = self
            .inherent_owner
            .as_ref()
            .filter(|owner| owner.path == declaration.owner)
            .map_or_else(
                || declaration.owner.clone(),
                |owner| owner.module_path.clone(),
            );
        let previous_module = std::mem::replace(&mut self.module_path, module);
        let previous_origin =
            std::mem::replace(&mut self.origins.current_type, SourceOrigin::unknown());
        let previous_lifetime_header = std::mem::replace(&mut self.lifetime_header, true);
        let result = (|| {
            let parameters = declaration
                .callback
                .parameters
                .iter()
                .map(|range| self.read_fn_mut_source_type_range(directory, range))
                .collect::<Result<Vec<_>>>()?;
            let result =
                self.read_fn_mut_source_type_range(directory, &declaration.callback.result)?;
            for (origin, position) in parameters.iter().zip(&owner.callback.parameter_ranges) {
                if origin.ty.as_ref().is_some_and(|ty| ty.kind() == "function")
                    || origin.wrappers.len() > 1
                    || origin.wrappers.iter().any(|layer| layer != "reference")
                {
                    return Err(unsupported(
                        "Rust source FnMut callback",
                        "callback inputs need direct values or immutable whole references",
                        Some(super::Span::new(position.start, position.end)),
                    ));
                }
            }
            if !result.wrappers.is_empty() {
                return Err(unsupported(
                    "Rust source FnMut callback",
                    "the closed callback transfers an owned result",
                    Some(super::Span::new(
                        owner.callback.result_range.start,
                        owner.callback.result_range.end,
                    )),
                ));
            }
            Ok(CheckedCallbackHeader {
                owner,
                parameters,
                result,
            })
        })();
        self.lifetime_header = previous_lifetime_header;
        self.origins.current_type = previous_origin;
        self.module_path = previous_module;
        self.fn_mut_source_type_bindings = previous_bindings;
        result
    }

    fn read_fn_mut_source_type_range(
        &mut self,
        directory: &FnMutSourceDeclarations,
        range: &Range<usize>,
    ) -> Result<SourceOrigin> {
        let original = TokenCursor::new(directory.read_tokens(range), Language::Rust);
        let previous_cursor = std::mem::replace(&mut self.cursor, original);
        let result = self.parse_type().and_then(|_| {
            if !self.cursor.at_end() {
                let SourceFoldSpan { start, end } = directory.position(range);
                return Err(unsupported(
                    "Rust source FnMut type argument",
                    "the normal type parser must consume the complete original type position",
                    Some(super::Span::new(start, end)),
                ));
            }
            Ok(self.origins.current_type.clone())
        });
        self.cursor = previous_cursor;
        result
    }
}

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
        let source = "fn apply<T>(value: u8, mut map: impl FnMut(&T, bool) -> T) -> T { map(&value, false) }";
        let tokens = Arc::new(tokenize(source, Language::Rust).unwrap().tokens);
        let directory = FnMutSourceDeclarations::new(source, Arc::clone(&tokens)).unwrap();
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
        let admitted = parser
            .read_fn_mut_source_callback_header(
                &directory,
                &declaration,
                vec![("T".into(), argument)],
            )
            .unwrap();
        let expected = Type::Fixed {
            bits: 8,
            signed: false,
        };
        assert_eq!(admitted.owner.path, ["apply"]);
        assert_eq!(admitted.parameters[0].ty.as_ref(), Some(&expected));
        assert_eq!(admitted.parameters[0].wrappers, ["reference"]);
        assert_eq!(admitted.parameters[1].ty, Some(Type::Bool));
        assert_eq!(admitted.result.ty, Some(expected));
        assert_eq!(parser.cursor.index, 0);
        assert!(parser.fn_mut_source_type_bindings.is_empty());
    }
}
