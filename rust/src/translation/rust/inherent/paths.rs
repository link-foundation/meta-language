//! Method scopes retain the actual Rust module for source namespace lookup.
use super::super::RustParser;
use crate::translation::rust_inherent_rules::read_rust_inherent_parent_path;

impl RustParser {
    pub(in super::super) fn inherent_path(
        &self,
        segments: &[String],
        path: &[String],
    ) -> Option<Vec<String>> {
        let context = self.inherent_owner.as_ref()?;
        if segments.first().is_some_and(|head| head == "Self") {
            return Some(
                std::iter::once("crate".to_owned())
                    .chain(context.path.iter().cloned())
                    .chain(segments[1..].iter().cloned())
                    .collect(),
            );
        }
        let normalized = path == context.path;
        if !normalized && path != context.module_path {
            return None;
        }
        if normalized && segments.len() == 1 && self.origins.scope.contains_key(&segments[0]) {
            return Some(segments.to_vec());
        }
        let actual = &context.module_path;
        if segments.first().is_some_and(|head| head == "self") {
            return Some(
                std::iter::once("crate".to_owned())
                    .chain(actual.iter().cloned())
                    .chain(segments[1..].iter().cloned())
                    .collect(),
            );
        }
        if segments.first().is_some_and(|head| head == "super") {
            let count = segments
                .iter()
                .take_while(|name| name.as_str() == "super")
                .count();
            return Some(read_rust_inherent_parent_path(
                actual,
                f64::from(
                    u32::try_from(path.len()).expect("a source path fits the shared array limit"),
                ),
                f64::from(u32::try_from(count).expect("a source path fits the shared array limit")),
                &segments[count..],
            ));
        }
        let aliases = self.aliases.get(&actual.join("."));
        let resolved = aliases
            .and_then(|aliases| aliases.get(&segments[0]))
            .map_or_else(
                || segments.to_vec(),
                |target| target.iter().chain(&segments[1..]).cloned().collect(),
            );
        let found = self.origins.lookup(&resolved, actual);
        Some(found.map_or(resolved, |entry| {
            std::iter::once("crate".to_owned())
                .chain(entry.full_name.split('.').map(str::to_owned))
                .collect()
        }))
    }
}
