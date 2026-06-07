use std::sync::OnceLock;

use lindera::dictionary::load_dictionary;
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use lingua::Language::{
    Arabic, Bengali, Chinese, English, French, Hindi, Portuguese, Russian, Spanish, Urdu,
};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use unicode_bidi::BidiInfo;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

use crate::configuration::{LanguageIdentificationDetector, ParseConfiguration};
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::source::{ByteRange, Point, SourceSpan};

const LINGUA_LANGUAGES: [Language; 10] = [
    English, Chinese, Hindi, Spanish, Arabic, French, Bengali, Portuguese, Russian, Urdu,
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct WordSegment {
    text: String,
    range: ByteRange,
}

pub fn annotate_natural_language(
    network: &mut LinkNetwork,
    document: LinkId,
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) {
    let Some(declared_language) = canonical_natural_language(language) else {
        return;
    };

    let document_span = span_for_range(text, 0, text.len());
    let detected_language =
        identify_language(text, configuration.language_identification_detector())
            .unwrap_or(declared_language);

    let region = network.insert_link(
        [document],
        LinkMetadata::new()
            .with_link_type(LinkType::Region)
            .with_named(true)
            .with_term("natural-language region")
            .with_language(declared_language)
            .with_span(document_span),
    );
    let language_link = network.insert_link(
        [region],
        LinkMetadata::new()
            .with_link_type(LinkType::Language)
            .with_named(true)
            .with_term(detected_language)
            .with_language(detected_language)
            .with_span(document_span),
    );

    insert_semantic_annotation(
        network,
        region,
        language_link,
        detector_term(configuration.language_identification_detector()),
        detected_language,
        document_span,
    );

    let (segmentation_engine, segments) = if declared_language == "Mandarin Chinese" {
        ("lindera-jieba", lindera_segments(text))
    } else {
        ("unicode-segmentation", unicode_segments(text))
    };

    insert_semantic_annotation(
        network,
        region,
        language_link,
        format!("segmentation:{segmentation_engine}"),
        detected_language,
        document_span,
    );

    for segment in segments {
        network.insert_link(
            [region],
            LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_named(true)
                .with_term(segment.text)
                .with_language(detected_language)
                .with_span(span_for_range(
                    text,
                    segment.range.start(),
                    segment.range.end(),
                )),
        );
    }

    insert_unicode_annotations(
        network,
        region,
        language_link,
        text,
        detected_language,
        document_span,
    );
}

fn insert_unicode_annotations(
    network: &mut LinkNetwork,
    region: LinkId,
    language_link: LinkId,
    text: &str,
    language: &str,
    span: SourceSpan,
) {
    let nfc = text.nfc().collect::<String>();
    let nfd = text.nfd().collect::<String>();

    insert_semantic_annotation(
        network,
        region,
        language_link,
        format!("normalization:nfc:{}", normalization_status(text, &nfc)),
        language,
        span,
    );
    insert_semantic_annotation(
        network,
        region,
        language_link,
        format!("normalization:nfd:{}", normalization_status(text, &nfd)),
        language,
        span,
    );
    insert_semantic_annotation(
        network,
        region,
        language_link,
        format!("bidi:{}", bidi_direction(text)),
        language,
        span,
    );
}

fn insert_semantic_annotation(
    network: &mut LinkNetwork,
    region: LinkId,
    language_link: LinkId,
    term: impl Into<String>,
    language: &str,
    span: SourceSpan,
) -> LinkId {
    network.insert_link(
        [region, language_link],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term(term)
            .with_language(language)
            .with_span(span),
    )
}

const fn detector_term(detector: LanguageIdentificationDetector) -> &'static str {
    match detector {
        LanguageIdentificationDetector::Lingua => "identifier:lingua",
        LanguageIdentificationDetector::Whatlang => "identifier:whatlang",
    }
}

fn identify_language(text: &str, detector: LanguageIdentificationDetector) -> Option<&'static str> {
    match detector {
        LanguageIdentificationDetector::Lingua => identify_with_lingua(text),
        LanguageIdentificationDetector::Whatlang => identify_with_whatlang(text),
    }
}

fn identify_with_lingua(text: &str) -> Option<&'static str> {
    static DETECTOR: OnceLock<LanguageDetector> = OnceLock::new();
    let detector =
        DETECTOR.get_or_init(|| LanguageDetectorBuilder::from_languages(&LINGUA_LANGUAGES).build());
    detector
        .detect_language_of(text)
        .map(canonical_lingua_language)
}

fn identify_with_whatlang(text: &str) -> Option<&'static str> {
    canonical_whatlang_language(whatlang::detect(text)?.lang())
}

const fn canonical_lingua_language(language: Language) -> &'static str {
    match language {
        English => "English",
        Chinese => "Mandarin Chinese",
        Hindi => "Hindi",
        Spanish => "Spanish",
        Arabic => "Modern Standard Arabic",
        French => "French",
        Bengali => "Bengali",
        Portuguese => "Portuguese",
        Russian => "Russian",
        Urdu => "Urdu",
    }
}

const fn canonical_whatlang_language(language: whatlang::Lang) -> Option<&'static str> {
    match language {
        whatlang::Lang::Eng => Some("English"),
        whatlang::Lang::Cmn => Some("Mandarin Chinese"),
        whatlang::Lang::Hin => Some("Hindi"),
        whatlang::Lang::Spa => Some("Spanish"),
        whatlang::Lang::Ara => Some("Modern Standard Arabic"),
        whatlang::Lang::Fra => Some("French"),
        whatlang::Lang::Ben => Some("Bengali"),
        whatlang::Lang::Por => Some("Portuguese"),
        whatlang::Lang::Rus => Some("Russian"),
        whatlang::Lang::Urd => Some("Urdu"),
        _ => None,
    }
}

fn canonical_natural_language(language: &str) -> Option<&'static str> {
    match language.to_ascii_lowercase().as_str() {
        "english" => Some("English"),
        "mandarin" | "mandarin chinese" | "chinese" => Some("Mandarin Chinese"),
        "hindi" => Some("Hindi"),
        "spanish" => Some("Spanish"),
        "arabic" | "modern standard arabic" => Some("Modern Standard Arabic"),
        "french" => Some("French"),
        "bengali" => Some("Bengali"),
        "portuguese" => Some("Portuguese"),
        "russian" => Some("Russian"),
        "urdu" => Some("Urdu"),
        _ => None,
    }
}

fn lindera_segments(text: &str) -> Vec<WordSegment> {
    let Ok(dictionary) = load_dictionary("embedded://jieba") else {
        return Vec::new();
    };
    let segmenter = Segmenter::new(Mode::Normal, dictionary, None);
    let tokenizer = Tokenizer::new(segmenter);
    let Ok(tokens) = tokenizer.tokenize(text) else {
        return Vec::new();
    };

    tokens
        .into_iter()
        .filter(|token| contains_word_character(token.surface.as_ref()))
        .map(|token| WordSegment {
            text: token.surface.into_owned(),
            range: ByteRange::new(token.byte_start, token.byte_end),
        })
        .collect()
}

fn unicode_segments(text: &str) -> Vec<WordSegment> {
    text.split_word_bound_indices()
        .filter(|(_start, segment)| contains_word_character(segment))
        .map(|(start, segment)| WordSegment {
            text: segment.to_string(),
            range: ByteRange::new(start, start + segment.len()),
        })
        .collect()
}

fn contains_word_character(text: &str) -> bool {
    text.chars().any(char::is_alphanumeric)
}

fn normalization_status(original: &str, normalized: &str) -> &'static str {
    if original == normalized {
        "stable"
    } else {
        "changes"
    }
}

fn bidi_direction(text: &str) -> &'static str {
    let bidi = BidiInfo::new(text, None);
    if bidi
        .paragraphs
        .iter()
        .any(|paragraph| paragraph.level.is_rtl())
    {
        "rtl"
    } else {
        "ltr"
    }
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
