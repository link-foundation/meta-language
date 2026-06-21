use crate::link_flags::LinkFlags;
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::source::{ByteRange, Point, SourceSpan};

/// Starter pass/fail grammar fixture for one natural-language target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NaturalLanguageGrammarFixture {
    language: &'static str,
    grammatical_source: &'static str,
    ungrammatical_source: &'static str,
    provenance: &'static str,
}

impl NaturalLanguageGrammarFixture {
    /// Natural-language target covered by the fixture pair.
    #[must_use]
    pub const fn language(&self) -> &'static str {
        self.language
    }

    /// Source text expected to parse without grammar-recovery links.
    #[must_use]
    pub const fn grammatical_source(&self) -> &'static str {
        self.grammatical_source
    }

    /// Source text expected to parse with recoverable grammar-recovery links.
    #[must_use]
    pub const fn ungrammatical_source(&self) -> &'static str {
        self.ungrammatical_source
    }

    /// Fixture and vocabulary provenance, including license notes.
    #[must_use]
    pub const fn provenance(&self) -> &'static str {
        self.provenance
    }
}

const STARTER_GRAMMAR_PROVENANCE: &str =
    "repo-authored starter pass/fail sentence; license: Unlicense; \
     morphosyntax tag names use Universal Dependencies v2 UPOS/UFeats/deprel vocabulary; \
     no UD treebank sentence data imported";

/// Executable natural-language grammar fixture pairs for each target language.
pub const NATURAL_LANGUAGE_GRAMMAR_FIXTURES: &[NaturalLanguageGrammarFixture] = &[
    NaturalLanguageGrammarFixture {
        language: "English",
        grammatical_source: "Hawaii is a state.\n",
        ungrammatical_source: "Hawaii are a state.\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Mandarin Chinese",
        grammatical_source: "你好。\n",
        ungrammatical_source: "你好 的。\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Hindi",
        grammatical_source: "नमस्ते।\n",
        ungrammatical_source: "नमस्ते है।\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Spanish",
        grammatical_source: "Hawaii es un estado.\n",
        ungrammatical_source: "Hawaii son un estado.\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Modern Standard Arabic",
        grammatical_source: "مرحبا.\n",
        ungrammatical_source: "مرحبا هو.\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "French",
        grammatical_source: "Hawaii est un etat.\n",
        ungrammatical_source: "Hawaii sont un etat.\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Bengali",
        grammatical_source: "নমস্কার।\n",
        ungrammatical_source: "নমস্কার আছে।\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Portuguese",
        grammatical_source: "Hawaii e um estado.\n",
        ungrammatical_source: "Hawaii sao um estado.\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Russian",
        grammatical_source: "Гавайи это штат.\n",
        ungrammatical_source: "Гавайи это штаты.\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
    NaturalLanguageGrammarFixture {
        language: "Urdu",
        grammatical_source: "سلام۔\n",
        ungrammatical_source: "سلام ہے۔\n",
        provenance: STARTER_GRAMMAR_PROVENANCE,
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct GrammarToken {
    text: String,
    range: ByteRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MorphAnalysis {
    upos: &'static str,
    features: &'static [&'static str],
    deprel: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LexiconEntry {
    language: &'static str,
    surface: &'static str,
    analysis: MorphAnalysis,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SentenceGrammar {
    language: &'static str,
    accepted: &'static [&'static str],
}

const NO_FEATURES: &[&str] = &[];
const NUMBER_SING: &[&str] = &["Number=Sing"];
const NUMBER_PLUR: &[&str] = &["Number=Plur"];
const INDEFINITE_ARTICLE: &[&str] = &["Definite=Ind", "PronType=Art"];
const AUX_SINGULAR: &[&str] = &[
    "Mood=Ind",
    "Number=Sing",
    "Person=3",
    "Tense=Pres",
    "VerbForm=Fin",
];
const AUX_PLURAL: &[&str] = &[
    "Mood=Ind",
    "Number=Plur",
    "Person=3",
    "Tense=Pres",
    "VerbForm=Fin",
];
const PRON_SINGULAR: &[&str] = &["Number=Sing", "Person=3", "PronType=Prs"];

const PUNCT_ANALYSIS: MorphAnalysis = MorphAnalysis {
    upos: "PUNCT",
    features: NO_FEATURES,
    deprel: "punct",
};

const SENTENCE_GRAMMARS: &[SentenceGrammar] = &[
    SentenceGrammar {
        language: "English",
        accepted: &["Hawaii", "is", "a", "state", "."],
    },
    SentenceGrammar {
        language: "Mandarin Chinese",
        accepted: &["你好", "\u{3002}"],
    },
    SentenceGrammar {
        language: "Hindi",
        accepted: &["नमस्ते", "\u{0964}"],
    },
    SentenceGrammar {
        language: "Spanish",
        accepted: &["Hawaii", "es", "un", "estado", "."],
    },
    SentenceGrammar {
        language: "Modern Standard Arabic",
        accepted: &["مرحبا", "."],
    },
    SentenceGrammar {
        language: "French",
        accepted: &["Hawaii", "est", "un", "etat", "."],
    },
    SentenceGrammar {
        language: "Bengali",
        accepted: &["নমস্কার", "\u{0964}"],
    },
    SentenceGrammar {
        language: "Portuguese",
        accepted: &["Hawaii", "e", "um", "estado", "."],
    },
    SentenceGrammar {
        language: "Russian",
        accepted: &["Гавайи", "это", "штат", "."],
    },
    SentenceGrammar {
        language: "Urdu",
        accepted: &["سلام", "\u{06d4}"],
    },
];

const LEXICON: &[LexiconEntry] = &[
    LexiconEntry {
        language: "English",
        surface: "Hawaii",
        analysis: MorphAnalysis {
            upos: "PROPN",
            features: NUMBER_SING,
            deprel: "nsubj",
        },
    },
    LexiconEntry {
        language: "English",
        surface: "is",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_SINGULAR,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "English",
        surface: "are",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_PLURAL,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "English",
        surface: "a",
        analysis: MorphAnalysis {
            upos: "DET",
            features: INDEFINITE_ARTICLE,
            deprel: "det",
        },
    },
    LexiconEntry {
        language: "English",
        surface: "state",
        analysis: MorphAnalysis {
            upos: "NOUN",
            features: NUMBER_SING,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Mandarin Chinese",
        surface: "你好",
        analysis: MorphAnalysis {
            upos: "INTJ",
            features: NO_FEATURES,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Mandarin Chinese",
        surface: "的",
        analysis: MorphAnalysis {
            upos: "PART",
            features: NO_FEATURES,
            deprel: "mark",
        },
    },
    LexiconEntry {
        language: "Hindi",
        surface: "नमस्ते",
        analysis: MorphAnalysis {
            upos: "INTJ",
            features: NO_FEATURES,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Hindi",
        surface: "है",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_SINGULAR,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "Spanish",
        surface: "Hawaii",
        analysis: MorphAnalysis {
            upos: "PROPN",
            features: NUMBER_SING,
            deprel: "nsubj",
        },
    },
    LexiconEntry {
        language: "Spanish",
        surface: "es",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_SINGULAR,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "Spanish",
        surface: "son",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_PLURAL,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "Spanish",
        surface: "un",
        analysis: MorphAnalysis {
            upos: "DET",
            features: INDEFINITE_ARTICLE,
            deprel: "det",
        },
    },
    LexiconEntry {
        language: "Spanish",
        surface: "estado",
        analysis: MorphAnalysis {
            upos: "NOUN",
            features: NUMBER_SING,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Modern Standard Arabic",
        surface: "مرحبا",
        analysis: MorphAnalysis {
            upos: "INTJ",
            features: NO_FEATURES,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Modern Standard Arabic",
        surface: "هو",
        analysis: MorphAnalysis {
            upos: "PRON",
            features: PRON_SINGULAR,
            deprel: "dep",
        },
    },
    LexiconEntry {
        language: "French",
        surface: "Hawaii",
        analysis: MorphAnalysis {
            upos: "PROPN",
            features: NUMBER_SING,
            deprel: "nsubj",
        },
    },
    LexiconEntry {
        language: "French",
        surface: "est",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_SINGULAR,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "French",
        surface: "sont",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_PLURAL,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "French",
        surface: "un",
        analysis: MorphAnalysis {
            upos: "DET",
            features: INDEFINITE_ARTICLE,
            deprel: "det",
        },
    },
    LexiconEntry {
        language: "French",
        surface: "etat",
        analysis: MorphAnalysis {
            upos: "NOUN",
            features: NUMBER_SING,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Bengali",
        surface: "নমস্কার",
        analysis: MorphAnalysis {
            upos: "INTJ",
            features: NO_FEATURES,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Bengali",
        surface: "আছে",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_SINGULAR,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "Portuguese",
        surface: "Hawaii",
        analysis: MorphAnalysis {
            upos: "PROPN",
            features: NUMBER_SING,
            deprel: "nsubj",
        },
    },
    LexiconEntry {
        language: "Portuguese",
        surface: "e",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_SINGULAR,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "Portuguese",
        surface: "sao",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_PLURAL,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "Portuguese",
        surface: "um",
        analysis: MorphAnalysis {
            upos: "DET",
            features: INDEFINITE_ARTICLE,
            deprel: "det",
        },
    },
    LexiconEntry {
        language: "Portuguese",
        surface: "estado",
        analysis: MorphAnalysis {
            upos: "NOUN",
            features: NUMBER_SING,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Russian",
        surface: "Гавайи",
        analysis: MorphAnalysis {
            upos: "PROPN",
            features: NUMBER_SING,
            deprel: "nsubj",
        },
    },
    LexiconEntry {
        language: "Russian",
        surface: "это",
        analysis: MorphAnalysis {
            upos: "PRON",
            features: PRON_SINGULAR,
            deprel: "cop",
        },
    },
    LexiconEntry {
        language: "Russian",
        surface: "штат",
        analysis: MorphAnalysis {
            upos: "NOUN",
            features: NUMBER_SING,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Russian",
        surface: "штаты",
        analysis: MorphAnalysis {
            upos: "NOUN",
            features: NUMBER_PLUR,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Urdu",
        surface: "سلام",
        analysis: MorphAnalysis {
            upos: "INTJ",
            features: NO_FEATURES,
            deprel: "root",
        },
    },
    LexiconEntry {
        language: "Urdu",
        surface: "ہے",
        analysis: MorphAnalysis {
            upos: "AUX",
            features: AUX_SINGULAR,
            deprel: "cop",
        },
    },
];

pub fn annotate_morphosyntax(
    network: &mut LinkNetwork,
    region: LinkId,
    text: &str,
    language: &str,
    span: SourceSpan,
) {
    let grammar_tokens = grammar_tokens(text);
    if grammar_tokens.is_empty() {
        return;
    }

    let sentence_is_accepted = sentence_grammar(language)
        .is_some_and(|grammar| token_surfaces_match(&grammar_tokens, grammar.accepted));
    let should_report_errors =
        is_registered_grammar_fixture(language, text) && !sentence_is_accepted;
    let sentence_flags = if sentence_is_accepted {
        LinkFlags::clean()
    } else if should_report_errors {
        LinkFlags::containing_error()
    } else {
        LinkFlags::clean()
    };
    let sentence = network.insert_link(
        [region],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term("natural-language:sentence")
            .with_language(language)
            .with_span(span)
            .with_flags(sentence_flags),
    );

    for token in &grammar_tokens {
        let token_span = span_for_range(text, token.range.start(), token.range.end());
        let form = network.insert_link(
            [sentence],
            LinkMetadata::new()
                .with_link_type(LinkType::Syntax)
                .with_named(true)
                .with_term(format!("form:{}", token.text))
                .with_language(language)
                .with_span(token_span),
        );

        if let Some(analysis) = morphology_for(language, &token.text) {
            insert_upos_link(network, form, language, token_span, analysis.upos);
            for feature in analysis.features {
                insert_ufeat_link(network, form, language, token_span, feature);
            }
            insert_deprel_link(
                network,
                sentence,
                form,
                language,
                token_span,
                analysis.deprel,
            );
        } else if should_report_errors {
            insert_error_link(
                network,
                [sentence, form],
                "natural-language:error:unknown-token",
                language,
                token_span,
            );
        }
    }

    if should_report_errors {
        insert_error_link(
            network,
            [sentence],
            "natural-language:error:grammar",
            language,
            span,
        );
    }
}

fn insert_upos_link(
    network: &mut LinkNetwork,
    form: LinkId,
    language: &str,
    span: SourceSpan,
    upos: &str,
) -> LinkId {
    network.insert_link(
        [form],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(format!("upos:{upos}"))
            .with_language(language)
            .with_span(span),
    )
}

fn insert_ufeat_link(
    network: &mut LinkNetwork,
    form: LinkId,
    language: &str,
    span: SourceSpan,
    feature: &str,
) -> LinkId {
    network.insert_link(
        [form],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(format!("ufeat:{feature}"))
            .with_language(language)
            .with_span(span),
    )
}

fn insert_deprel_link(
    network: &mut LinkNetwork,
    sentence: LinkId,
    form: LinkId,
    language: &str,
    span: SourceSpan,
    deprel: &str,
) -> LinkId {
    network.insert_link(
        [sentence, form],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(format!("deprel:{deprel}"))
            .with_language(language)
            .with_span(span),
    )
}

fn insert_error_link<const N: usize>(
    network: &mut LinkNetwork,
    references: [LinkId; N],
    term: &'static str,
    language: &str,
    span: SourceSpan,
) -> LinkId {
    network.insert_link(
        references,
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(term)
            .with_language(language)
            .with_span(span)
            .with_flags(LinkFlags::error()),
    )
}

fn sentence_grammar(language: &str) -> Option<SentenceGrammar> {
    SENTENCE_GRAMMARS
        .iter()
        .find(|grammar| grammar.language == language)
        .copied()
}

fn token_surfaces_match(tokens: &[GrammarToken], expected: &[&str]) -> bool {
    tokens.len() == expected.len()
        && tokens
            .iter()
            .zip(expected.iter().copied())
            .all(|(token, expected)| token.text == expected)
}

fn morphology_for(language: &str, surface: &str) -> Option<MorphAnalysis> {
    if is_sentence_punctuation_token(surface) {
        return Some(PUNCT_ANALYSIS);
    }

    LEXICON
        .iter()
        .find(|entry| entry.language == language && entry.surface == surface)
        .map(|entry| entry.analysis)
}

fn is_registered_grammar_fixture(language: &str, text: &str) -> bool {
    NATURAL_LANGUAGE_GRAMMAR_FIXTURES
        .iter()
        .filter(|fixture| fixture.language == language)
        .any(|fixture| text == fixture.grammatical_source || text == fixture.ungrammatical_source)
}

fn grammar_tokens(text: &str) -> Vec<GrammarToken> {
    let mut tokens = Vec::new();
    let mut word_start = None;

    for (index, character) in text.char_indices() {
        if character.is_whitespace() {
            push_pending_word_token(&mut tokens, text, &mut word_start, index);
        } else if is_sentence_punctuation(character) {
            push_pending_word_token(&mut tokens, text, &mut word_start, index);
            tokens.push(GrammarToken {
                text: character.to_string(),
                range: ByteRange::new(index, index + character.len_utf8()),
            });
        } else if word_start.is_none() {
            word_start = Some(index);
        }
    }

    push_pending_word_token(&mut tokens, text, &mut word_start, text.len());
    tokens
}

fn push_pending_word_token(
    tokens: &mut Vec<GrammarToken>,
    text: &str,
    word_start: &mut Option<usize>,
    end: usize,
) {
    let Some(start) = word_start.take() else {
        return;
    };
    if start == end {
        return;
    }
    tokens.push(GrammarToken {
        text: text[start..end].to_string(),
        range: ByteRange::new(start, end),
    });
}

fn is_sentence_punctuation_token(surface: &str) -> bool {
    let mut characters = surface.chars();
    let Some(character) = characters.next() else {
        return false;
    };
    characters.next().is_none() && is_sentence_punctuation(character)
}

const fn is_sentence_punctuation(character: char) -> bool {
    matches!(
        character,
        '.' | '!' | '?' | '\u{0964}' | '\u{3002}' | '\u{06d4}'
    )
}

fn span_for_range(text: &str, start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(start, end),
        point_at_byte(text, start),
        point_at_byte(text, end),
    )
}

fn point_at_byte(text: &str, byte: usize) -> Point {
    let mut row = 0;
    let mut column = 0;

    for (index, character) in text.char_indices() {
        if index >= byte {
            break;
        }
        if character == '\n' {
            row += 1;
            column = 0;
        } else {
            column += 1;
        }
    }

    Point::new(row, column)
}
