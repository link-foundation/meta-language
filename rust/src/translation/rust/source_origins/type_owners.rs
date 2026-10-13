//! Lexical catalog lookup with shared type-namespace owner admission.
use super::{SourceDeclaration, SourceLookup, SourceOrigins};

impl SourceOrigins {
    pub(in super::super) fn lookup(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_filtered(segments, path, false)
    }

    pub(in super::super) fn lookup_type_owner(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_filtered(segments, path, true)
    }

    fn lookup_filtered(
        &self,
        segments: &[String],
        path: &[String],
        type_owner: bool,
    ) -> Option<SourceLookup> {
        let mut names = segments.to_vec();
        let mut start = path.to_vec();
        let mut absolute = false;
        match names.first().map(String::as_str) {
            Some("crate") => {
                names.remove(0);
                start.clear();
                absolute = true;
            }
            Some("self") => {
                names.remove(0);
                absolute = true;
            }
            Some("super") => {
                while names.first().is_some_and(|name| name == "super") {
                    names.remove(0);
                    start.pop();
                }
                absolute = true;
            }
            _ => {}
        }
        for depth in (0..=start.len()).rev() {
            let full_name = start[..depth]
                .iter()
                .chain(names.iter())
                .cloned()
                .collect::<Vec<_>>()
                .join(".");
            if let Some(declaration) = self.catalog.get(&full_name) {
                let kind = match declaration {
                    SourceDeclaration::ExternalCrate => "externalCrate",
                    SourceDeclaration::Module => "module",
                    SourceDeclaration::Data => "data",
                    SourceDeclaration::Constructor { .. } => "ctor",
                    SourceDeclaration::Function { .. } => "fn",
                };
                if !type_owner || crate::translation::source_signature_rules::accept_rust_source_signature_catalog_owner(kind) {
                    return Some(SourceLookup { declaration: declaration.clone(), full_name });
                }
            }
            if absolute {
                break;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceDeclaration, SourceOrigins};
    use crate::translation::rust::source_origins::SourceOrigin;
    use std::collections::HashMap;

    #[test]
    fn anchored_type_owner_lookup_never_falls_through_a_value_namespace_hit() {
        let catalog = HashMap::from([
            ("f64".to_owned(), SourceDeclaration::Data),
            (
                "nested.f64".to_owned(),
                SourceDeclaration::Function {
                    parameters: Vec::new(),
                    result: SourceOrigin::unknown(),
                },
            ),
        ]);
        let origins = SourceOrigins::new(Some(catalog), "");
        let path = vec!["nested".to_owned()];
        assert_eq!(
            origins
                .lookup_type_owner(&["f64".to_owned()], &path)
                .unwrap()
                .full_name,
            "f64"
        );
        assert!(
            origins
                .lookup_type_owner(&["self".to_owned(), "f64".to_owned()], &path)
                .is_none()
        );
        assert!(
            origins
                .lookup_type_owner(
                    &["super".to_owned(), "f64".to_owned()],
                    &["nested".to_owned(), "deep".to_owned()]
                )
                .is_none()
        );
        assert_eq!(
            origins
                .lookup_type_owner(&["crate".to_owned(), "f64".to_owned()], &path)
                .unwrap()
                .full_name,
            "f64"
        );
        assert_eq!(
            origins
                .lookup(&["f64".to_owned()], &path)
                .unwrap()
                .full_name,
            "nested.f64"
        );
    }
}
