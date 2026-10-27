//! Preserve the distinct source struct value and associated-item namespaces.
use super::super::{Result, RustParser, SourceDeclaration, Span, unsupported};
use crate::translation::rust_source_render_interfaces::read_rust_record_constructor_name;

impl RustParser {
    pub(in super::super) fn prepare_record_method_namespace(
        &mut self,
        owner: &[String],
        method: &str,
        range: Option<Span>,
    ) -> Result<()> {
        let key = owner
            .iter()
            .cloned()
            .chain(std::iter::once(method.to_owned()))
            .collect::<Vec<_>>()
            .join(".");
        let Some(existing @ SourceDeclaration::Constructor { .. }) =
            self.origins.catalog.get(&key).cloned()
        else {
            return Ok(());
        };
        let full_name = owner.join(".");
        if !self
            .named_records
            .get(&full_name)
            .is_some_and(|record| record.update)
        {
            return Err(unsupported(
                "Rust associated item namespace",
                "an enum constructor homograph needs its complete associated-item resolution",
                range,
            ));
        }
        let name = owner.last().map_or("", String::as_str);
        let reserved = self
            .reserved_binding_names
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        let constructor = read_rust_record_constructor_name(name, true, &reserved);
        let next = owner
            .iter()
            .cloned()
            .chain(std::iter::once(constructor))
            .collect::<Vec<_>>()
            .join(".");
        if self.origins.collecting {
            self.origins.catalog.remove(&key);
            self.origins.catalog.insert(next.clone(), existing);
        }
        self.named_records
            .get_mut(&full_name)
            .expect("the source struct record was just selected")
            .constructor = next;
        Ok(())
    }
}
