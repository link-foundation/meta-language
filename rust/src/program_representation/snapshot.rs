use serde_json::{json, Value};

use super::{
    construct_program, LinkType, ProgramProjectContext, ProgramRepresentation,
    ProgramRepresentationError,
};

/// Schema revision for source-buffer-independent program snapshots.
pub const PROGRAM_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

impl ProgramRepresentation {
    /// Serializes exact retained token fragments without a duplicate source field.
    #[must_use]
    pub fn serialize_snapshot(&self) -> String {
        let fragments = retained_fragments(self);
        json!({
            "schemaVersion": PROGRAM_SNAPSHOT_SCHEMA_VERSION,
            "language": self.language,
            "project": {
                "root": self.project.root(),
                "files": self.project.files(),
                "dependencies": self.project.dependencies(),
                "extensions": self.project.extensions(),
            },
            "fragments": fragments,
        })
        .to_string()
    }

    /// Reloads a validated snapshot without access to its original source buffer.
    pub fn from_snapshot(snapshot: &str) -> Result<Self, ProgramRepresentationError> {
        let value: Value = serde_json::from_str(snapshot).map_err(|error| {
            ProgramRepresentationError::InvalidSnapshot(format!("invalid JSON: {error}"))
        })?;
        let object = value.as_object().ok_or_else(|| {
            ProgramRepresentationError::InvalidSnapshot("expected an object".to_string())
        })?;
        let version = object.get("schemaVersion").and_then(Value::as_u64);
        if version != Some(u64::from(PROGRAM_SNAPSHOT_SCHEMA_VERSION)) {
            return Err(ProgramRepresentationError::InvalidSnapshot(format!(
                "unsupported schema version {version:?}"
            )));
        }
        let language = required_string(object.get("language"), "language")?;
        let project = read_project(object.get("project"))?;
        let fragments = object
            .get("fragments")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                ProgramRepresentationError::InvalidSnapshot(
                    "fragments must be an array".to_string(),
                )
            })?;
        let mut source = String::new();
        let mut expected_start = 0_u64;
        for fragment in fragments {
            let fragment = fragment.as_object().ok_or_else(|| {
                ProgramRepresentationError::InvalidSnapshot(
                    "fragment must be an object".to_string(),
                )
            })?;
            let byte_start = fragment.get("byteStart").and_then(Value::as_u64);
            let byte_end = fragment.get("byteEnd").and_then(Value::as_u64);
            let text = fragment.get("text").and_then(Value::as_str);
            let (Some(byte_start), Some(byte_end), Some(text)) = (byte_start, byte_end, text)
            else {
                return Err(ProgramRepresentationError::InvalidSnapshot(
                    "fragment fields have invalid types".to_string(),
                ));
            };
            let text_length = u64::try_from(text.len()).map_err(|_| {
                ProgramRepresentationError::InvalidSnapshot(
                    "fragment text length does not fit u64".to_string(),
                )
            })?;
            if byte_start != expected_start
                || byte_end < byte_start
                || text_length != byte_end - byte_start
            {
                return Err(ProgramRepresentationError::InvalidSnapshot(format!(
                    "invalid fragment span at byte {expected_start}"
                )));
            }
            source.push_str(text);
            expected_start = byte_end;
        }
        construct_program(&source, language, project)
    }
}

/// Constructs a clean program from ordered structured source fragments.
pub fn construct_program_from_fragments<S: AsRef<str>>(
    fragments: &[S],
    language: &str,
    project: ProgramProjectContext,
) -> Result<ProgramRepresentation, ProgramRepresentationError> {
    let source = fragments
        .iter()
        .fold(String::new(), |mut source, fragment| {
            source.push_str(fragment.as_ref());
            source
        });
    construct_program(&source, language, project)
}

fn retained_fragments(program: &ProgramRepresentation) -> Vec<Value> {
    let mut tokens = program
        .network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Token))
        .filter(|link| !link.metadata().flags().is_missing())
        .filter_map(|link| {
            let range = link.metadata().span()?.byte_range();
            Some((
                range.start(),
                link.id().as_u64(),
                range.end(),
                link.metadata().term()?.to_string(),
            ))
        })
        .collect::<Vec<_>>();
    tokens.sort_by_key(|(start, id, _end, _text)| (*start, *id));
    let mut covered_until = 0;
    let mut fragments = Vec::new();
    for (start, _id, end, text) in tokens {
        if start < covered_until {
            continue;
        }
        debug_assert_eq!(
            start, covered_until,
            "source-token spans must be contiguous"
        );
        debug_assert_eq!(text.len(), end - start, "source-token span must match text");
        fragments.push(json!({ "byteStart": start, "byteEnd": end, "text": text }));
        covered_until = end;
    }
    debug_assert_eq!(
        fragments
            .iter()
            .filter_map(|fragment| fragment["text"].as_str())
            .collect::<String>(),
        program.source
    );
    fragments
}

fn read_project(
    value: Option<&Value>,
) -> Result<ProgramProjectContext, ProgramRepresentationError> {
    let project = value.and_then(Value::as_object).ok_or_else(|| {
        ProgramRepresentationError::InvalidSnapshot("project must be an object".to_string())
    })?;
    Ok(ProgramProjectContext::new(
        required_string(project.get("root"), "project.root")?,
        string_array(project.get("files"), "project.files")?,
        string_array(project.get("dependencies"), "project.dependencies")?,
    )
    .with_extensions(string_array(
        project.get("extensions"),
        "project.extensions",
    )?))
}

fn required_string<'a>(
    value: Option<&'a Value>,
    name: &str,
) -> Result<&'a str, ProgramRepresentationError> {
    value.and_then(Value::as_str).ok_or_else(|| {
        ProgramRepresentationError::InvalidSnapshot(format!("{name} must be a string"))
    })
}

fn string_array(
    value: Option<&Value>,
    name: &str,
) -> Result<Vec<String>, ProgramRepresentationError> {
    value
        .and_then(Value::as_array)
        .ok_or_else(|| {
            ProgramRepresentationError::InvalidSnapshot(format!("{name} must be an array"))
        })?
        .iter()
        .map(|entry| {
            entry.as_str().map(ToString::to_string).ok_or_else(|| {
                ProgramRepresentationError::InvalidSnapshot(format!(
                    "{name} entries must be strings"
                ))
            })
        })
        .collect()
}
