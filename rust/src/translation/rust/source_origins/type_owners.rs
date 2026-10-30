//! Lexical catalog lookup with shared type-namespace owner admission.
use super::{SourceDeclaration, SourceLookup, SourceOrigins};

impl SourceOrigins {
    pub(in super::super) fn lookup(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_filtered(segments, path, false, None)
    }

    pub(in super::super) fn lookup_type_owner(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_filtered(segments, path, true, None)
    }

    pub(in super::super) fn lookup_literal_match_owner(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<SourceLookup> {
        self.lookup_filtered(segments, path, false, Some(crate::translation::rust_literal_match_rules::accept_rust_literal_match_catalog_owner))
    }

    fn lookup_filtered(
        &self,
        segments: &[String],
        path: &[String],
        type_owner: bool,
        catalog_owner: Option<fn(&str) -> bool>,
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
                    SourceDeclaration::Constant { .. } => "const",
                };
                if (!type_owner || crate::translation::source_signature_rules::accept_rust_source_signature_catalog_owner(kind)) && catalog_owner.is_none_or(|accept| accept(kind)) {
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
