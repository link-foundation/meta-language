use meta_language::grammar::inference::state_merging::Apta;
use meta_language::{
    evaluate, infer_dfa, GrammarFormat, GrammarOracle, MergeStrategy, Sample, SampleConfig,
};

#[test]
fn apta_construction_shares_prefixes_and_records_labels() {
    let sample = labelled_sample(&["ab", "ac"], &["a"]);
    let apta = Apta::from_sample(&sample);

    assert_eq!(apta.states.len(), 4);
    assert_eq!(apta.transitions[0].get("a"), Some(&1));
    assert_eq!(apta.transitions[1].get("b"), Some(&2));
    assert_eq!(apta.transitions[1].get("c"), Some(&3));
    assert!(apta.states[2].accepting);
    assert!(apta.states[3].accepting);
    assert!(apta.states[1].rejecting);
    assert_eq!(apta.states[0].arrival_count, 2);
    assert_eq!(apta.states[1].arrival_count, 2);
}

#[test]
fn rpni_learns_ab_star_from_labelled_examples() {
    let sample = labelled_sample(&["", "ab", "abab"], &["a", "b", "aba"]);
    let automaton = infer_dfa(&sample, MergeStrategy::Rpni);
    let grammar = automaton.to_grammar();
    let oracle = GrammarOracle::new(&grammar);

    assert!(automaton.accepts(&symbols("")));
    assert!(automaton.accepts(&symbols("ab")));
    assert!(automaton.accepts(&symbols("abab")));
    assert!(automaton.accepts(&symbols("ababab")));
    assert!(!automaton.accepts(&symbols("a")));
    assert!(!automaton.accepts(&symbols("b")));
    assert!(!automaton.accepts(&symbols("aba")));

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Inferred));
    assert!(oracle.accepts(""));
    assert!(oracle.accepts("ab"));
    assert!(oracle.accepts("ababab"));
    assert!(!oracle.accepts("aba"));
}

#[test]
fn edsm_is_at_least_as_accurate_and_records_evidence_merges() {
    let sample = labelled_sample(&["", "ab", "abab", "cd", "cdcd"], &["a", "aba", "c", "cdc"]);
    let rpni = infer_dfa(&sample, MergeStrategy::Rpni);
    let edsm = infer_dfa(&sample, MergeStrategy::Edsm);
    let rpni_grammar = rpni.to_grammar();
    let edsm_grammar = edsm.to_grammar();
    let positives = ["", "ab", "abab", "cd", "cdcd"];
    let golden = GrammarOracle::new(&edsm_grammar);
    let config = SampleConfig {
        seed: 17,
        count: 32,
        max_depth: 6,
        repeat_cap: 3,
    };

    let rpni_scores =
        evaluate(&rpni_grammar, &golden, None, &positives, &config).expect("rpni scores");
    let edsm_scores =
        evaluate(&edsm_grammar, &golden, None, &positives, &config).expect("edsm scores");

    assert!(edsm_scores.f1 >= rpni_scores.f1);
    assert!(
        edsm.merge_history.iter().any(|event| event.evidence > 0),
        "{:#?}",
        edsm.merge_history
    );
    let first_evidence = edsm
        .merge_history
        .first()
        .map(|event| event.evidence)
        .expect("EDSM records at least one merge");
    let highest_evidence = edsm
        .merge_history
        .iter()
        .map(|event| event.evidence)
        .max()
        .expect("EDSM records at least one merge");
    assert_eq!(first_evidence, highest_evidence);
}

#[test]
fn alergia_adds_probabilities_and_lower_alpha_is_stricter() {
    let sample = positive_sample(&["ax", "ax", "ax", "ay", "az", "bx", "bx", "by", "by", "bz"]);
    let loose = infer_dfa(&sample, MergeStrategy::Alergia { alpha: 0.8 });
    let strict = infer_dfa(&sample, MergeStrategy::Alergia { alpha: 0.2 });

    assert!(loose.states.len() < strict.states.len());
    assert!(loose
        .transition_probabilities
        .iter()
        .any(|weights| !weights.is_empty()));
    assert!(loose
        .transition_probabilities
        .iter()
        .flat_map(std::collections::BTreeMap::values)
        .all(|weight| weight.true_probability().basis_points() <= 10_000));
    assert!(loose
        .final_probabilities
        .iter()
        .flatten()
        .all(|weight| weight.true_probability().basis_points() <= 10_000));
}

#[test]
fn extracted_grammar_matches_automaton_acceptance() {
    let sample = labelled_sample(&["", "aa", "aaaa"], &["a", "aaa", "b"]);
    let automaton = infer_dfa(&sample, MergeStrategy::Rpni);
    let grammar = automaton.to_grammar();
    let oracle = GrammarOracle::new(&grammar);

    for text in ["", "a", "aa", "aaa", "aaaa", "aaaaaa", "b", "aab"] {
        assert_eq!(
            oracle.accepts(text),
            automaton.accepts(&symbols(text)),
            "mismatch on {text:?} with {grammar:#?}"
        );
    }
}

#[test]
fn inference_is_deterministic_for_all_strategies() {
    let sample = labelled_sample(&["", "ab", "abab", "ac", "acac"], &["a", "aba", "aca"]);

    for strategy in [
        MergeStrategy::Rpni,
        MergeStrategy::Edsm,
        MergeStrategy::Alergia { alpha: 0.5 },
    ] {
        let first = infer_dfa(&sample, strategy);
        let second = infer_dfa(&sample, strategy);

        assert_eq!(first, second);
        assert_eq!(first.to_grammar(), second.to_grammar());
    }
}

fn labelled_sample(positives: &[&str], negatives: &[&str]) -> Sample {
    Sample {
        positives: positives.iter().map(|text| symbols(text)).collect(),
        negatives: negatives.iter().map(|text| symbols(text)).collect(),
    }
}

fn positive_sample(positives: &[&str]) -> Sample {
    Sample {
        positives: positives.iter().map(|text| symbols(text)).collect(),
        negatives: Vec::new(),
    }
}

fn symbols(text: &str) -> Vec<String> {
    text.chars().map(|value| value.to_string()).collect()
}
