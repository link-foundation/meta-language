use super::{DynamicMetaLanguage, DynamicMetaLanguageOptions, create_dynamic_meta_language, fail};
use crate::dynamic_meta_language_rules::read_dynamic_meta_language_identifier;
use crate::{GlobalConceptOptions, GlobalConceptSource};
use serde_json::{Value, json};
use std::collections::BTreeSet;

impl DynamicMetaLanguage {
    /// Extend the same algorithms with complete new or explicitly replaced
    /// grammars. Previous concrete snapshots retain their pinned contexts.
    ///
    /// # Errors
    /// Refuses duplicate sources, implicit replacements and invalid premises.
    pub fn extend(
        &self,
        additions: &[GlobalConceptSource],
        options: &DynamicMetaLanguageOptions,
    ) -> Result<Self, String> {
        let mut sources = self.sources.clone();
        let mut added = BTreeSet::new();
        for source in additions {
            if !added.insert(&source.id) {
                return Err(fail("duplicate added source"));
            }
            if let Some(position) = sources.iter().position(|old| old.id == source.id) {
                if !options.replace {
                    return Err(fail("grammar replacement requires an explicit selection"));
                }
                sources[position] = source.clone();
            } else {
                sources.push(source.clone());
            }
        }
        let mut records = self.configuration["records"]
            .as_array()
            .cloned()
            .ok_or_else(|| fail("records must be an array"))?;
        if let Some(additions) = &options.concepts.records {
            for record in additions
                .as_array()
                .ok_or_else(|| fail("records must be an array"))?
            {
                let identifier = record["id"]
                    .as_str()
                    .ok_or_else(|| fail("a record needs an identity"))?;
                if let Some(position) = records
                    .iter()
                    .position(|old| old["id"].as_str() == Some(identifier))
                {
                    records[position] = record.clone();
                } else {
                    records.push(record.clone());
                }
            }
        }
        let mut definitions = serde_json::Map::new();
        let definition = self
            .entries
            .iter()
            .find(|entry| entry.identifier == read_dynamic_meta_language_identifier("definitions"))
            .ok_or_else(|| fail("missing language definitions"))?;
        let list = definition
            .network
            .link(definition.root)
            .and_then(|root| root.references().get(3))
            .copied()
            .and_then(|root| definition.network.link(root))
            .ok_or_else(|| fail("missing formal definition inventory"))?;
        for reference in list.references() {
            let alias = definition
                .network
                .link(*reference)
                .and_then(|link| link.metadata().definition())
                .ok_or_else(|| fail("missing formal definition alias"))?;
            let archive = self
                .formal_definitions
                .get(alias)
                .ok_or_else(|| fail("missing formal definition"))?;
            definitions.insert(
                alias.to_owned(),
                json!({"links":archive.network.to_lino(),"root":archive.root.as_u64()}),
            );
        }
        if let Some(additions) = &options.concepts.records {
            for record in additions
                .as_array()
                .ok_or_else(|| fail("records must be an array"))?
            {
                let identifier = record["id"]
                    .as_str()
                    .ok_or_else(|| fail("a record needs an identity"))?;
                if !options
                    .concepts
                    .formal_definitions
                    .as_ref()
                    .and_then(Value::as_object)
                    .is_some_and(|values| values.contains_key(identifier))
                {
                    if let Some(definition) = record
                        .get("formalDefinition")
                        .filter(|value| !value.is_null())
                    {
                        definitions.insert(identifier.to_owned(), definition.clone());
                    }
                }
            }
        }
        if let Some(values) = &options.concepts.formal_definitions {
            definitions.extend(
                values
                    .as_object()
                    .ok_or_else(|| fail("formal definitions must be an object"))?
                    .clone(),
            );
        }
        let mut equivalences: Vec<[String; 2]> =
            serde_json::from_value(self.configuration["equivalences"].clone())
                .map_err(|error| fail(&error.to_string()))?;
        equivalences.extend(options.concepts.equivalences.clone());
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .filter(|entry| {
                entry.identifier != read_dynamic_meta_language_identifier("definitions")
                    && entry.identifier != read_dynamic_meta_language_identifier("configuration")
            })
            .cloned()
            .collect();
        entries.extend(options.entries.clone());
        create_dynamic_meta_language(
            &sources,
            &DynamicMetaLanguageOptions {
                concepts: GlobalConceptOptions {
                    records: Some(Value::Array(records)),
                    formal_definitions: Some(Value::Object(definitions)),
                    previous: Some(self.space.clone()),
                    equivalences,
                },
                entries,
                limits: Some(options.limits.unwrap_or(self.limits)),
                depth_limit: Some(options.depth_limit.unwrap_or(self.depth_limit)),
                replace: options.replace,
            },
        )
    }
}
