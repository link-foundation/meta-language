//! Binary/UTF-8 process boundaries over the public complete-graph operations.
use meta_language::{
    LinkId, LinkNetwork, MetaLanguageArchive, MetaLanguageArchiveEntry, MetaLanguageArchiveLimits,
    StructuredDataValue, create_meta_language_archive, extend_meta_language_archive,
    meta_language_archive_to_binary, meta_language_archive_to_text, read_meta_language_archive,
    read_meta_language_archive_binary, read_meta_language_archive_network_text,
    read_meta_language_archive_text, structured_data_to_lino_ordered,
};
use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;

const USAGE: &str = concat!(
    "usage: meta-language archive pack [--binary] --entry <identifier> <kind> <root> <links-file> ...\n",
    "       meta-language archive list|select|extend <archive-file> [--input-binary] [--binary]\n",
    "         select: --identifier <identifier> ...\n",
    "         extend: [--replace] [--remove <identifier>] [--entry <identifier> <kind> <root> <links-file>] ...\n",
);

pub(crate) struct Output {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: String,
}
struct Arguments {
    command: String,
    file: Option<String>,
    entries: Vec<MetaLanguageArchiveEntry>,
    identifiers: Vec<String>,
    remove: Vec<String>,
    binary: bool,
    input_binary: bool,
    replace: bool,
}
fn parse(args: &[String]) -> Result<Arguments, String> {
    let command = args.first().map_or("", String::as_str);
    if !["pack", "list", "select", "extend"].contains(&command) {
        return Err("unknown archive operation".to_owned());
    }
    let mut result = Arguments {
        command: command.to_owned(),
        file: None,
        entries: Vec::new(),
        identifiers: Vec::new(),
        remove: Vec::new(),
        binary: false,
        input_binary: false,
        replace: false,
    };
    let mut networks = BTreeMap::<String, Arc<LinkNetwork>>::new();
    let mut index = 1;
    while index < args.len() {
        let option = args[index].as_str();
        match option {
            "--binary" => result.binary = true,
            "--input-binary" => result.input_binary = true,
            "--replace" if command == "extend" => result.replace = true,
            "--identifier" if command == "select" && index + 1 < args.len() => {
                index += 1;
                result.identifiers.push(args[index].clone());
            }
            "--remove" if command == "extend" && index + 1 < args.len() => {
                index += 1;
                result.remove.push(args[index].clone());
            }
            "--entry" if ["pack", "extend"].contains(&command) && index + 4 < args.len() => {
                let identifier = args[index + 1].clone();
                let kind = args[index + 2].clone();
                let root = &args[index + 3];
                if root.is_empty()
                    || root.starts_with('0')
                    || !root.bytes().all(|value| value.is_ascii_digit())
                {
                    return Err("an entry root needs a positive decimal identity".to_owned());
                }
                let root = root
                    .parse::<u64>()
                    .map_err(|_| "an entry root exceeds the identity profile")?;
                let path = &args[index + 4];
                if !networks.contains_key(path) {
                    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
                    let network = read_meta_language_archive_network_text(
                        &text,
                        MetaLanguageArchiveLimits::default(),
                    )?;
                    networks.insert(path.clone(), Arc::new(network));
                }
                result.entries.push(MetaLanguageArchiveEntry {
                    identifier,
                    kind,
                    root: LinkId::from_u64(root),
                    network: Arc::clone(&networks[path]),
                });
                index += 4;
            }
            _ if !option.starts_with("--") && result.file.is_none() && command != "pack" => {
                result.file = Some(option.to_owned())
            }
            _ => return Err(format!("unexpected archive argument {option}")),
        }
        index += 1;
    }
    Ok(result)
}
fn string(value: &str) -> StructuredDataValue {
    StructuredDataValue::String(value.encode_utf16().collect())
}
fn entry_summary(entry: &MetaLanguageArchiveEntry) -> StructuredDataValue {
    #[allow(clippy::cast_precision_loss)]
    let fields = [
        ("identifier", string(&entry.identifier)),
        ("kind", string(&entry.kind)),
        (
            "root",
            StructuredDataValue::Number(entry.root.as_u64() as f64),
        ),
        (
            "nodes",
            StructuredDataValue::Number(entry.network.len() as f64),
        ),
    ];
    StructuredDataValue::Object(
        fields
            .into_iter()
            .map(|(name, value)| (name.encode_utf16().collect(), value))
            .collect(),
    )
}
fn execute(args: &Arguments) -> Result<Vec<u8>, String> {
    let limits = MetaLanguageArchiveLimits::default();
    let archive: MetaLanguageArchive;
    if args.command == "pack" {
        if args.input_binary {
            return Err("--input-binary applies to an archive input".to_owned());
        }
        archive = create_meta_language_archive(&args.entries, limits)?;
    } else {
        let file = args.file.as_ref().ok_or("an archive input is required")?;
        let input = if args.input_binary {
            read_meta_language_archive_binary(
                &fs::read(file).map_err(|error| error.to_string())?,
                limits,
            )?
        } else {
            read_meta_language_archive_text(
                &fs::read_to_string(file).map_err(|error| error.to_string())?,
                limits,
            )?
        };
        let restored = read_meta_language_archive(&input.network, input.root, limits)?;
        if args.command == "list" {
            if args.binary {
                return Err("list produces structured Links text".to_owned());
            }
            let rows =
                StructuredDataValue::Array(restored.entries.iter().map(entry_summary).collect());
            return structured_data_to_lino_ordered(&rows).map(String::into_bytes);
        }
        if args.command == "select" {
            if args.identifiers.is_empty() {
                return Err("select needs distinct identifiers".to_owned());
            }
            let mut selected = Vec::new();
            let mut seen = std::collections::BTreeSet::new();
            for identifier in &args.identifiers {
                if !seen.insert(identifier) {
                    return Err("select needs distinct identifiers".to_owned());
                }
                selected.push(
                    restored
                        .entries
                        .iter()
                        .find(|entry| &entry.identifier == identifier)
                        .ok_or_else(|| format!("unknown archive entry {identifier}"))?
                        .clone(),
                );
            }
            archive = create_meta_language_archive(&selected, limits)?;
        } else {
            archive = extend_meta_language_archive(
                &input,
                &args.entries,
                args.replace,
                &args.remove,
                limits,
            )?;
        }
    }
    if args.binary {
        meta_language_archive_to_binary(&archive, limits)
    } else {
        meta_language_archive_to_text(&archive, limits).map(String::into_bytes)
    }
}

pub(crate) fn run(args: &[String]) -> Output {
    if args
        .first()
        .is_some_and(|value| value == "help" || value == "--help")
    {
        return Output {
            exit_code: 0,
            stdout: USAGE.as_bytes().to_vec(),
            stderr: String::new(),
        };
    }
    match parse(args).and_then(|arguments| execute(&arguments)) {
        Ok(stdout) => Output {
            exit_code: 0,
            stdout,
            stderr: String::new(),
        },
        Err(error) => Output {
            exit_code: 2,
            stdout: Vec::new(),
            stderr: format!("error: {error}\n{USAGE}"),
        },
    }
}
