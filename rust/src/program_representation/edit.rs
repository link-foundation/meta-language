use super::{
    ProgramRange, ProgramRepresentation, ProgramRepresentationError, scope_by_id,
    validate_identifier,
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
        let mut edits = std::iter::once(binding.declaration)
            .chain(binding.references.iter().copied())
            .map(|range| {
                let shorthand = self.source_mappings.iter().any(|mapping| {
                    mapping.range == range
                        && shorthand_terms(self.language).contains(&mapping.term.as_str())
                });
                let text = if shorthand {
                    format!("{}: {replacement}", binding.name)
                } else {
                    replacement.to_string()
                };
                (range, text)
            })
            .collect::<Vec<_>>();
        edits.sort_by_key(|(range, _)| std::cmp::Reverse(range.start));
        let mut edited = self.source.clone();
        for (range, text) in &edits {
            edited.replace_range(range.start..range.end, text);
        }
        let renamed = self.reparse_edit(&edited)?;
        // Every other name must keep resolving exactly as before the rename.
        if let Some(offset) =
            first_resolution_change(self, &renamed, &binding.id, replacement, &edits)
        {
            return Err(ProgramRepresentationError::CaptureConflict {
                identifier: replacement.to_string(),
                offset,
            });
        }
        Ok(renamed)
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

fn shorthand_terms(language: &str) -> &'static [&'static str] {
    match language {
        "JavaScript" => &[
            "shorthand_property_identifier",
            "shorthand_property_identifier_pattern",
        ],
        "Rust" => &["shorthand_field_initializer"],
        _ => &[],
    }
}

#[derive(PartialEq, Eq)]
enum Resolved<'a> {
    Binding {
        name: &'a str,
        kind: &'a str,
        declaration: ProgramRange,
        references: Vec<ProgramRange>,
    },
    Unresolved {
        name: &'a str,
        range: ProgramRange,
    },
}

impl Resolved<'_> {
    const fn offset(&self) -> usize {
        match self {
            Self::Binding { declaration, .. } => declaration.start,
            Self::Unresolved { range, .. } => range.start,
        }
    }
}

// Compares the resolution of the renamed program with the original one moved
// through the rename edits; returns the first renamed-program offset whose
// binding structure differs, or `None` when the rename preserved it.
fn first_resolution_change(
    original: &ProgramRepresentation,
    renamed: &ProgramRepresentation,
    binding_id: &str,
    replacement: &str,
    edits: &[(ProgramRange, String)],
) -> Option<usize> {
    let shift = |offset: usize| {
        edits
            .iter()
            .filter(|(range, _)| range.end <= offset)
            .fold(offset, |total, (range, text)| {
                total + text.len() - (range.end - range.start)
            })
    };
    let moved = |range: ProgramRange| {
        let edit = edits.iter().find(|(candidate, _)| *candidate == range);
        let from = shift(range.start) + edit.map_or(0, |(_, text)| text.len() - replacement.len());
        let length = edit.map_or(range.end - range.start, |_| replacement.len());
        ProgramRange::new(from, from + length)
    };
    let expected = original
        .bindings
        .iter()
        .map(|candidate| Resolved::Binding {
            name: if candidate.id == binding_id {
                replacement
            } else {
                &candidate.name
            },
            kind: &candidate.kind,
            declaration: moved(candidate.declaration),
            references: candidate.references.iter().copied().map(moved).collect(),
        })
        .chain(
            original
                .unresolved_references
                .iter()
                .map(|reference| Resolved::Unresolved {
                    name: &reference.name,
                    range: moved(reference.range),
                }),
        )
        .collect::<Vec<_>>();
    let actual = renamed
        .bindings
        .iter()
        .map(|candidate| Resolved::Binding {
            name: &candidate.name,
            kind: &candidate.kind,
            declaration: candidate.declaration,
            references: candidate.references.clone(),
        })
        .chain(
            renamed
                .unresolved_references
                .iter()
                .map(|reference| Resolved::Unresolved {
                    name: &reference.name,
                    range: reference.range,
                }),
        )
        .collect::<Vec<_>>();
    (0..expected.len().max(actual.len())).find_map(|index| {
        let (left, right) = (expected.get(index), actual.get(index));
        (left != right).then(|| right.or(left).map_or(0, Resolved::offset))
    })
}

const fn range_inside_scope(range: ProgramRange, scope: ProgramRange) -> bool {
    range.start >= scope.start && range.end <= scope.end
}
