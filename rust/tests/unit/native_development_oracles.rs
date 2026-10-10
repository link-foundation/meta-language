use meta_language::{LinkNetwork, ParseConfiguration, grammar_names};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, oracle_rows, parse, parser, rebuilt};

#[test]
fn native_languages_retain_independent_development_oracles() {
    for (id, language, source, oracle, grammar, fixture) in [
        (
            "lean",
            "Lean",
            "def answer : Nat := 42\n",
            native_source_oracles::LEAN,
            include_str!("../../../parity/grammars/native/lean.lino"),
            include_str!("../../../parity/fixtures/native-grammars/lean.json"),
        ),
        (
            "rocq",
            "Rocq",
            "Definition answer : nat := 42.\n",
            native_source_oracles::ROCQ,
            include_str!("../../../parity/grammars/native/rocq.lino"),
            include_str!("../../../parity/fixtures/native-grammars/rocq.json"),
        ),
    ] {
        assert!(grammar_names(id).is_none());
        let fixture: Value = serde_json::from_str(fixture).expect("fixture reads");
        let tree = parse(&parser(grammar), source).expect("native parse succeeds");
        let (oracle, recovers) = oracle_rows(&oracle.into(), source);
        assert!(!recovers);
        assert_eq!(Rows::new(&fixture).rows(&tree, source), oracle);
        assert_eq!(rebuilt(&tree), source);
        let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
        assert_eq!(network.reconstruct_text(), source);
        assert!(
            network
                .parse_grammars()
                .iter()
                .any(|(_, grammar)| grammar.id == format!("native-{id}"))
        );
    }
}
