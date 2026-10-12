//! Filesystem boundary for structured project data and explicit external JSON.
//!
//! Logical `.json` identities resolve to their converted `.lino` files. The
//! decoder is the shared structured-data codec; these adapters neither parse
//! programming languages nor change fixture identities or digest inputs.

use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

/// The requested storage format at a project or external data boundary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProjectDataFormat {
    /// Infer storage from the resolved extension, accepting legacy JSON.
    #[default]
    Auto,
    /// Read exactly the requested path as external JSON, without alias lookup.
    Json,
    /// Read the corresponding Links Notation file.
    Lino,
}

/// Resolve a logical JSON identity to its canonical stored data file.
///
/// Other extensions retain their exact path, including source and grammar
/// files whose raw bytes form a checksum input.
///
/// # Errors
/// Returns an error if both JSON and Links Notation storage exist.
pub fn resolve_project_data_path(file: impl AsRef<Path>) -> Result<PathBuf, String> {
    resolve_project_data_path_with_format(file, ProjectDataFormat::Auto)
}

/// Resolve storage with an explicit external JSON or Links Notation boundary.
///
/// # Errors
/// Returns an error when automatic or Links Notation resolution finds both
/// formats. Explicit JSON always preserves the requested path.
pub fn resolve_project_data_path_with_format(
    file: impl AsRef<Path>,
    format: ProjectDataFormat,
) -> Result<PathBuf, String> {
    let logical = file.as_ref();
    if format == ProjectDataFormat::Json {
        return Ok(logical.to_path_buf());
    }
    let extension = logical.extension().and_then(|value| value.to_str());
    let structured = matches!(extension, Some("json" | "lino"));
    if format == ProjectDataFormat::Auto && !structured {
        return Ok(logical.to_path_buf());
    }
    let (json, lino) = if structured {
        (
            logical.with_extension("json"),
            logical.with_extension("lino"),
        )
    } else {
        let mut json = logical.as_os_str().to_os_string();
        json.push(".json");
        let mut lino = logical.as_os_str().to_os_string();
        lino.push(".lino");
        (PathBuf::from(json), PathBuf::from(lino))
    };
    if json.exists() && lino.exists() {
        return Err(format!(
            "project data has duplicate JSON and LiNo storage: {}",
            logical.display()
        ));
    }
    if format == ProjectDataFormat::Lino
        || extension == Some("lino")
        || lino.exists()
        || !json.exists()
    {
        Ok(lino)
    } else {
        Ok(json)
    }
}

/// Read the exact resolved storage bytes for checksums or source processing.
///
/// # Errors
/// Returns a storage conflict or filesystem read error with its physical path.
pub fn read_project_file_bytes(file: impl AsRef<Path>) -> Result<Vec<u8>, String> {
    let actual = resolve_project_data_path(file)?;
    fs::read(&actual).map_err(|error| format!("{}: {error}", actual.display()))
}

/// Read project data through its logical identity into a host data structure.
///
/// The Links decoder preserves structure; conversion to `serde_json::Value`
/// retains that type's existing map-order and Unicode interoperability limits.
/// Use the codec's ordered API when those distinctions must stay lossless.
///
/// # Errors
/// Returns a storage conflict, unsupported extension, read, decoding or host
/// deserialization error. Malformed LiNo never falls back to JSON.
pub fn read_project_data_file<T: DeserializeOwned>(file: impl AsRef<Path>) -> Result<T, String> {
    read_project_data_file_with_format(file, ProjectDataFormat::Auto)
}

/// Read data with an explicitly chosen external JSON or LiNo representation.
///
/// # Errors
/// Returns a storage conflict, unsupported format, read or decoding error.
pub fn read_project_data_file_with_format<T: DeserializeOwned>(
    file: impl AsRef<Path>,
    format: ProjectDataFormat,
) -> Result<T, String> {
    let actual = resolve_project_data_path_with_format(file, format)?;
    let encoding = match format {
        ProjectDataFormat::Auto => match actual.extension().and_then(|value| value.to_str()) {
            Some("json") => ProjectDataFormat::Json,
            Some("lino") => ProjectDataFormat::Lino,
            _ => {
                return Err(format!(
                    "project data has an unsupported storage extension: {}",
                    actual.display()
                ));
            }
        },
        encoding => encoding,
    };
    let text =
        fs::read_to_string(&actual).map_err(|error| format!("{}: {error}", actual.display()))?;
    match encoding {
        ProjectDataFormat::Lino => {
            let value = crate::structured_data_links::read_structured_data_lino(&text)
                .map_err(|error| format!("{}: {error}", actual.display()))?;
            serde_json::from_value(value).map_err(|error| format!("{}: {error}", actual.display()))
        }
        ProjectDataFormat::Json => {
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", actual.display()))
        }
        ProjectDataFormat::Auto => unreachable!("automatic storage is resolved above"),
    }
}
