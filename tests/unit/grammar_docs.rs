use std::path::{Component, Path, PathBuf};

const GRAMMAR_DOCS: &[&str] = &[
    "docs/grammar/README.md",
    "docs/grammar/architecture.md",
    "docs/grammar/authoring.md",
    "docs/grammar/import-export.md",
    "docs/grammar/fidelity.md",
    "docs/grammar/codegen.md",
    "docs/grammar/inference.md",
    "docs/grammar/translation.md",
    "docs/grammar/cli-and-runtime.md",
];

#[test]
fn grammar_documentation_pages_exist() {
    let root = repository_root();

    for doc in GRAMMAR_DOCS {
        assert!(
            root.join(doc).is_file(),
            "expected grammar documentation page {doc} to exist"
        );
    }
}

#[test]
fn grammar_documentation_relative_links_resolve() {
    let root = repository_root();

    for doc in GRAMMAR_DOCS {
        let path = root.join(doc);
        let markdown =
            std::fs::read_to_string(&path).expect("grammar documentation page should be readable");

        for target in markdown_links(&markdown) {
            let Some(relative_target) = relative_target(&target) else {
                continue;
            };
            let resolved = normalize(
                &path
                    .parent()
                    .expect("documentation page should have a parent")
                    .join(relative_target),
            );

            assert!(
                resolved.exists(),
                "relative link {target:?} in {doc} should resolve to {}",
                resolved.display()
            );
        }
    }
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn markdown_links(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut offset = 0;

    while let Some(open) = markdown[offset..].find("](") {
        let target_start = offset + open + 2;
        let Some(close) = markdown[target_start..].find(')') else {
            break;
        };
        let target = markdown[target_start..target_start + close].trim();
        if !target.is_empty() && !target.contains('\n') {
            links.push(target.to_string());
        }
        offset = target_start + close + 1;
    }

    links
}

fn relative_target(target: &str) -> Option<&str> {
    if target.starts_with('#')
        || target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
    {
        return None;
    }

    let target = target
        .split_once('#')
        .map_or(target, |(path, _anchor)| path);
    let target = target.split_once('?').map_or(target, |(path, _query)| path);
    let target = target
        .strip_prefix('<')
        .and_then(|path| path.strip_suffix('>'))
        .unwrap_or(target);

    if target.is_empty() {
        None
    } else {
        Some(target)
    }
}

fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
            Component::RootDir => normalized.push(Path::new("/")),
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
        }
    }

    normalized
}
