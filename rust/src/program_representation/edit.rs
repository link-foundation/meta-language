use super::{
    scope_by_id, validate_identifier, ProgramRange, ProgramRepresentation,
    ProgramRepresentationError,
};

impl ProgramRepresentation {
    /// Returns exact ranges of CST nodes with the requested grammar term.
    #[must_use]
    pub fn query_syntax(&self, term: &str) -> Vec<ProgramRange> {
        self.source_mappings
            .iter()
            .filter(|mapping| mapping.term == term)
            .map(|mapping| mapping.range)
            .collect()
    }

    /// Replaces one exact source range and reparses the result.
    pub fn replace(
        &self,
        range: ProgramRange,
        replacement: &str,
    ) -> Result<Self, ProgramRepresentationError> {
        self.validate_range(range)?;
        let mut edited = self.source.clone();
        edited.replace_range(range.start..range.end, replacement);
        self.reparse_edit(&edited)
    }

    /// Inserts target-language source at an exact byte boundary.
    pub fn insert(
        &self,
        offset: usize,
        inserted: &str,
    ) -> Result<Self, ProgramRepresentationError> {
        self.replace(ProgramRange::new(offset, offset), inserted)
    }

    /// Deletes one exact source range.
    pub fn delete(&self, range: ProgramRange) -> Result<Self, ProgramRepresentationError> {
        self.replace(range, "")
    }

    /// Clones one exact source range at an exact byte boundary.
    pub fn clone_range(
        &self,
        range: ProgramRange,
        destination: usize,
    ) -> Result<Self, ProgramRepresentationError> {
        self.validate_range(range)?;
        self.insert(destination, &self.source[range.start..range.end])
    }

    /// Moves one exact range using byte offsets from the original source.
    pub fn move_range(
        &self,
        range: ProgramRange,
        destination: usize,
    ) -> Result<Self, ProgramRepresentationError> {
        self.validate_range(range)?;
        self.validate_range(ProgramRange::new(destination, destination))?;
        if destination > range.start && destination < range.end {
            return Err(ProgramRepresentationError::DestinationInsideRange);
        }
        if destination == range.start || destination == range.end {
            return Ok(self.clone());
        }
        let fragment = &self.source[range.start..range.end];
        let mut without = self.source.clone();
        without.replace_range(range.start..range.end, "");
        let adjusted = if destination > range.end {
            destination - (range.end - range.start)
        } else {
            destination
        };
        without.insert_str(adjusted, fragment);
        self.reparse_edit(&without)
    }

    /// Renames exactly one resolved binding and reparses the edited source.
    pub fn rename_binding(
        &self,
        binding_id: &str,
        replacement: &str,
    ) -> Result<Self, ProgramRepresentationError> {
        let binding = self
            .bindings
            .iter()
            .find(|binding| binding.id == binding_id)
            .ok_or_else(|| ProgramRepresentationError::UnknownBinding(binding_id.to_string()))?;
        validate_identifier(replacement, self.language)?;
        if binding.name == replacement {
            return Ok(self.clone());
        }
        let binding_scope = scope_by_id(&self.scopes, &binding.scope);
        if let Some(conflict) = self.bindings.iter().find(|candidate| {
            candidate.id != binding.id
                && candidate.name == replacement
                && (candidate.scope == binding.scope
                    || (range_inside_scope(
                        scope_by_id(&self.scopes, &candidate.scope).range,
                        binding_scope.range,
                    ) && binding.references.iter().any(|reference| {
                        range_inside_scope(
                            *reference,
                            scope_by_id(&self.scopes, &candidate.scope).range,
                        )
                    }))
                    || (range_inside_scope(
                        binding_scope.range,
                        scope_by_id(&self.scopes, &candidate.scope).range,
                    ) && candidate
                        .references
                        .iter()
                        .any(|reference| range_inside_scope(*reference, binding_scope.range))))
        }) {
            return Err(ProgramRepresentationError::CaptureConflict {
                identifier: replacement.to_string(),
                offset: conflict.declaration.start,
            });
        }
        if let Some(reference) = self.unresolved_references.iter().find(|reference| {
            reference.name == replacement
                && reference.range.start >= binding_scope.range.start
                && reference.range.end <= binding_scope.range.end
        }) {
            return Err(ProgramRepresentationError::CaptureConflict {
                identifier: replacement.to_string(),
                offset: reference.range.start,
            });
        }
        let mut ranges = binding.references.clone();
        ranges.push(binding.declaration);
        ranges.sort_by_key(|range| std::cmp::Reverse(range.start));
        let mut edited = self.source.clone();
        for range in ranges {
            let shorthand = self.language == "JavaScript"
                && self.source_mappings.iter().any(|mapping| {
                    mapping.range == range
                        && matches!(
                            mapping.term.as_str(),
                            "shorthand_property_identifier"
                                | "shorthand_property_identifier_pattern"
                        )
                });
            if shorthand {
                edited.replace_range(
                    range.start..range.end,
                    &format!("{}: {replacement}", binding.name),
                );
            } else {
                edited.replace_range(range.start..range.end, replacement);
            }
        }
        self.reparse_edit(&edited)
    }

    fn validate_range(&self, range: ProgramRange) -> Result<(), ProgramRepresentationError> {
        if range.start > range.end
            || range.end > self.source.len()
            || !self.source.is_char_boundary(range.start)
            || !self.source.is_char_boundary(range.end)
        {
            return Err(ProgramRepresentationError::InvalidRange {
                start: range.start,
                end: range.end,
            });
        }
        Ok(())
    }

    fn reparse_edit(&self, source: &str) -> Result<Self, ProgramRepresentationError> {
        let reparsed = Self::analyze(source, self.language, self.project.clone())?;
        if !reparsed.network.verify_full_match(None).is_clean() {
            return Err(ProgramRepresentationError::InvalidEdit);
        }
        Ok(reparsed)
    }
}

const fn range_inside_scope(range: ProgramRange, scope: ProgramRange) -> bool {
    range.start >= scope.start && range.end <= scope.end
}
