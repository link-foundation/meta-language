use meta_language::grammar::inference::active::{
    ActiveLearningConfig, Dfa, EquivalenceMethod, GrammarAcceptorOracle, Oracle,
    ParserMembershipOracle, SamplingEquivalenceOracle, Symbol, learn_active_grammar, learn_dfa,
    learn_grammar, sample_active_grammar_words,
};
use meta_language::{
    FromLinks, Grammar, GrammarFormat, GrammarParser, LinksDecoder, LinksEncoder, ParserRegistry,
    ToLinks, register_grammar,
};

#[test]
fn lstar_learns_minimal_even_a_dfa_from_exact_queries() {
    let oracle = ExactDfaOracle::new(even_a_target(), 6);
    let learned = learn_dfa(&oracle, &config(6, 64)).expect("learns even-a DFA");

    assert_eq!(learned.states, 2);
    assert!(learned.accepts_text(""));
    assert!(!learned.accepts_text("a"));
    assert!(learned.accepts_text("aa"));
    assert!(!learned.accepts_text("aaa"));
    assert!(learned.accepts_text("aaaa"));
}

#[test]
fn lstar_refines_hypothesis_with_counterexamples() {
    let oracle = ExactDfaOracle::new(ab_star_target(), 6);
    let learned = learn_dfa(&oracle, &config(6, 128)).expect("learns (ab)* DFA");

    assert_eq!(learned.states, 3);
    for accepted in ["", "ab", "abab", "ababab"] {
        assert!(learned.accepts_text(accepted), "{accepted:?}");
    }
    for rejected in ["a", "b", "aba", "abb", "ba"] {
        assert!(!learned.accepts_text(rejected), "{rejected:?}");
    }
}

#[test]
fn sampled_equivalence_oracle_is_seeded_and_deterministic() {
    let first = SamplingEquivalenceOracle::new(vec!['a'], even_a_membership);
    let second = SamplingEquivalenceOracle::new(vec!['a'], even_a_membership);

    assert_eq!(
        learn_dfa(&first, &config(6, 64)).expect("first learned DFA"),
        learn_dfa(&second, &config(6, 64)).expect("second learned DFA")
    );
}

#[test]
fn active_discovery_retains_ordered_unicode_observations() {
    let alphabet = vec!['λ', '🙂'];
    let membership = |word: &[Symbol]| {
        word.chunks_exact(2).remainder().is_empty()
            && word.chunks_exact(2).all(|pair| pair == ['λ', '🙂'])
    };
    let config = ActiveLearningConfig {
        max_word_len: 6,
        equivalence_samples: 64,
        seed: 31,
        ..ActiveLearningConfig::default()
    };
    let sampled = sample_active_grammar_words(&alphabet, &config).unwrap();
    assert_eq!(
        &sampled[..5],
        &[
            vec![],
            vec!['λ'],
            vec!['🙂'],
            vec!['λ', 'λ'],
            vec!['🙂', '🙂']
        ]
    );
    let oracle = SamplingEquivalenceOracle::new(alphabet, membership);
    let (grammar, learned) = learn_active_grammar(&oracle, &config).unwrap();
    assert_eq!(learned.equivalence, EquivalenceMethod::BoundedSampling);
    assert_eq!(learned.hypothesis.states, 3);
    assert!(learned.equivalence_queries > 1);
    assert!(learned.membership_queries > 0);
    let parser = GrammarParser::new(grammar);
    for word in sampled {
        let text: String = word.iter().collect();
        assert_eq!(parser.accepts(&text), membership(&word), "{text:?}");
    }
}

#[test]
fn learned_dfa_lowers_to_round_tripping_runtime_grammar() {
    let oracle = ExactDfaOracle::new(even_a_target(), 6);
    let grammar = learn_grammar(&oracle, &config(6, 64)).expect("learned grammar");
    let parser = GrammarParser::new(grammar.clone());

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Inferred));
    assert!(parser.accepts(""));
    assert!(parser.accepts("aa"));
    assert!(!parser.accepts("a"));
    assert!(!parser.accepts("aaa"));

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let mut decoder = LinksDecoder::new(&network);
    let round_tripped = Grammar::from_links(&mut decoder, root).expect("grammar decodes");

    assert_eq!(round_tripped, grammar);
}

#[test]
fn parser_membership_oracle_rejects_runtime_grammar_fallbacks() {
    let mut registry = ParserRegistry::new();
    register_grammar(&mut registry, "single-a", literal_grammar("a"));
    let oracle = ParserMembershipOracle::new(registry, "single-a", vec!['a', 'b']);

    assert!(oracle.membership(&['a']));
    assert!(!oracle.membership(&['b']));
    assert!(!oracle.membership(&['a', 'a']));
}

#[test]
fn grammar_acceptor_oracle_learns_runtime_grammar_language() {
    let grammar = a_star_grammar();
    let oracle = GrammarAcceptorOracle::new(grammar, vec!['a', 'b']);
    let learned = learn_dfa(&oracle, &config(4, 64)).expect("learns grammar acceptor");

    assert!(learned.accepts_text(""));
    assert!(learned.accepts_text("a"));
    assert!(learned.accepts_text("aaaa"));
    assert!(!learned.accepts_text("b"));
    assert!(!learned.accepts_text("aab"));
}

#[derive(Clone, Debug)]
struct ExactDfaOracle {
    target: Dfa,
    bound: usize,
}

impl ExactDfaOracle {
    const fn new(target: Dfa, bound: usize) -> Self {
        Self { target, bound }
    }
}

impl Oracle for ExactDfaOracle {
    fn alphabet(&self) -> &[Symbol] {
        &self.target.alphabet
    }

    fn membership(&self, word: &[Symbol]) -> bool {
        self.target.accepts(word)
    }

    fn equivalence(&self, hypothesis: &Dfa, _config: &ActiveLearningConfig) -> Option<Vec<Symbol>> {
        enumerate_words(&self.target.alphabet, self.bound)
            .into_iter()
            .find(|word| self.target.accepts(word) != hypothesis.accepts(word))
    }
}

fn even_a_target() -> Dfa {
    Dfa {
        alphabet: vec!['a'],
        states: 2,
        start: 0,
        accepting: vec![true, false],
        delta: vec![vec![1], vec![0]],
    }
}

fn ab_star_target() -> Dfa {
    Dfa {
        alphabet: vec!['a', 'b'],
        states: 3,
        start: 0,
        accepting: vec![true, false, false],
        delta: vec![vec![1, 2], vec![2, 0], vec![2, 2]],
    }
}

fn even_a_membership(word: &[Symbol]) -> bool {
    word.iter().filter(|symbol| **symbol == 'a').count() % 2 == 0
}

fn literal_grammar(literal: &str) -> Grammar {
    Grammar::builder()
        .start("start")
        .rule("start", Grammar::expr().term(literal))
        .build()
}

fn a_star_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("start")
        .rule("start", expr.rep0(expr.term("a")))
        .build()
}

const fn config(max_word_len: usize, equivalence_samples: usize) -> ActiveLearningConfig {
    ActiveLearningConfig {
        max_word_len,
        equivalence_samples,
        seed: 17,
        use_ttt: false,
        max_iterations: 64,
    }
}

fn enumerate_words(alphabet: &[Symbol], max_len: usize) -> Vec<Vec<Symbol>> {
    let mut words = vec![Vec::new()];
    let mut current = vec![Vec::new()];
    for _ in 0..max_len {
        let mut next = Vec::new();
        for prefix in &current {
            for symbol in alphabet {
                let mut word = prefix.clone();
                word.push(*symbol);
                words.push(word.clone());
                next.push(word);
            }
        }
        current = next;
    }
    words
}
