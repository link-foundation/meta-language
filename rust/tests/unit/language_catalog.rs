use std::fs;
use std::path::PathBuf;

use meta_language::{
    canonical_language_name, grammar_provenance, language_candidates_for_path, language_catalog,
    language_entry, language_for_path, LinkNetwork, LinkType, ParseConfiguration,
};
use serde_json::Value;

fn repository_file(path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(path);
    fs::read_to_string(path).expect("repository file is readable")
}

fn inventory() -> Value {
    serde_json::from_str(&repository_file("parity/language-grammar-inventory.json"))
        .expect("inventory is valid JSON")
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("array")
        .iter()
        .map(|item| item.as_str().expect("string").to_string())
        .collect()
}

#[test]
fn rust_ships_the_language_catalog_the_javascript_package_ships() {
    assert_eq!(
        repository_file("rust/src/data/language-catalog.json"),
        repository_file("js/src/data/language-catalog.json"),
    );
    let inventory = inventory();
    let names: Vec<&str> = language_catalog()
        .iter()
        .map(|entry| entry.name.as_str())
        .collect();
    let expected: Vec<&str> = inventory["languages"]
        .as_array()
        .expect("languages")
        .iter()
        .map(|language| language["name"].as_str().expect("name"))
        .collect();
    assert_eq!(names, expected);
}

#[test]
fn every_inventory_name_and_alias_resolves_case_insensitively() {
    for language in inventory()["languages"].as_array().expect("languages") {
        let name = language["name"].as_str().expect("name");
        let mut aliases = strings(&language["aliases"]);
        aliases.push(name.to_string());
        for alias in aliases {
            assert_eq!(canonical_language_name(&alias), Some(name), "{alias}");
            assert_eq!(
                canonical_language_name(&alias.to_uppercase()),
                Some(name),
                "{alias}"
            );
            assert_eq!(
                language_entry(&alias).map(|entry| entry.name.as_str()),
                Some(name)
            );
        }
    }
    assert_eq!(canonical_language_name("klingon"), None);
}

#[test]
fn every_registered_extension_dispatches_to_its_language() {
    let inventory = inventory();
    let dispatch = &inventory["extensionDispatch"];
    let languages = inventory["languages"].as_array().expect("languages");
    for language in languages {
        let name = language["name"].as_str().expect("name");
        for extension in strings(&dispatch[name]) {
            let path = format!("src/Fixture{}", extension.to_uppercase());
            assert!(
                language_candidates_for_path(&path).contains(&name),
                "{name} {extension}"
            );
            // A shared suffix (every SQL dialect accepts .sql) selects the
            // first registered owner; a unique suffix selects its language.
            let owner = languages
                .iter()
                .map(|language| language["name"].as_str().expect("name"))
                .find(|owner| strings(&dispatch[*owner]).contains(&extension));
            assert_eq!(language_for_path(&path), owner, "{name} {extension}");
        }
    }
    assert_eq!(language_for_path("schema.sql"), Some("sql-ansi"));
    assert_eq!(
        language_for_path("schema.postgres.sql"),
        Some("sql-postgres")
    );
    assert_eq!(language_for_path("proc.TSQL"), Some("sql-server"));
    assert_eq!(
        language_candidates_for_path("q.sql"),
        [
            "sql-ansi",
            "sql-postgres",
            "sql-mysql",
            "sql-sqlite",
            "sql-server",
            "sql-oracle",
            "sql-bigquery",
            "sql-snowflake",
        ]
    );
    assert_eq!(language_candidates_for_path("q.mysql.sql")[0], "sql-mysql");
    assert_eq!(language_for_path("README"), None);
    assert_eq!(language_for_path("archive.tar.gz"), None);
}

#[test]
fn grammar_provenance_names_the_locked_grammar_versions_and_digests() {
    let lock: Value = serde_json::from_str(&repository_file("rust/src/data/grammar-lock.json"))
        .expect("grammar lock is valid JSON");
    for language in inventory()["languages"].as_array().expect("languages") {
        let name = language["name"].as_str().expect("name");
        let provenance = grammar_provenance(name);
        let ids: Vec<&str> = provenance
            .iter()
            .map(|grammar| grammar.id.as_str())
            .collect();
        let expected = language.get("grammars").map(strings).unwrap_or_default();
        assert_eq!(ids, expected, "{name}");
        for grammar in provenance {
            let locked = &lock["grammars"][&grammar.id];
            assert_eq!(
                grammar.version,
                locked["version"].as_str().expect("version")
            );
            assert_eq!(
                grammar.parser_sha256,
                locked["parserSha256"].as_str().expect("parser digest")
            );
        }
    }
}

#[test]
fn extension_dispatch_parses_through_the_ordinary_api_with_the_dispatched_grammar() {
    let source = "fn main() {}\n";
    let language = language_for_path("main.rs").expect("Rust extension");
    let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    assert_eq!(network.reconstruct_text(), source);
    assert!(network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Syntax)
            && link.metadata().term() == Some("function_item")
    }));
}

#[test]
fn parsed_networks_record_the_grammar_provenance_of_every_language_they_parse() {
    let inventory = inventory();
    let languages = inventory["languages"].as_array().expect("languages");
    for language in languages {
        let name = language["name"].as_str().expect("name");
        if grammar_provenance(name).is_empty() {
            continue;
        }
        let source = language["source"].as_str().expect("source");
        let network = LinkNetwork::parse(source, name, ParseConfiguration::default());
        let recorded: Vec<_> = network
            .parse_grammars()
            .into_iter()
            .filter(|(language, _)| language == name)
            .map(|(_, grammar)| grammar)
            .collect();
        assert_eq!(recorded, grammar_provenance(name), "{name}");
        assert_eq!(network.reconstruct_text(), source, "{name}");
    }
    let markdown = languages
        .iter()
        .find(|language| language["name"] == "Markdown")
        .and_then(|language| language["source"].as_str())
        .expect("Markdown source");
    let network = LinkNetwork::parse(markdown, "Markdown", ParseConfiguration::default());
    let expected: Vec<_> = ["Markdown", "JavaScript", "HTML"]
        .into_iter()
        .flat_map(|language| {
            grammar_provenance(language)
                .iter()
                .map(move |grammar| (language.to_string(), grammar.clone()))
        })
        .collect();
    assert_eq!(network.parse_grammars(), expected);
    let text = LinkNetwork::parse("plain text\n", "txt", ParseConfiguration::default());
    assert!(text.parse_grammars().is_empty());
}
