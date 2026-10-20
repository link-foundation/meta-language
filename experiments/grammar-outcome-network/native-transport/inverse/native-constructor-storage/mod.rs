// Generated typed host storage, not an owned-model validator or lifetime allocator.
/// Actual native storage values. Source identities remain on their separate slots.
#[derive(Debug)]
pub enum NativeConstructorValue {
    /// Native storage for `meta_language::GrammarSelection`.
    GrammarSelection(Box<meta_language::GrammarSelection>),
    /// Native storage for `meta_language::GrammarSelectionAlternative`.
    GrammarSelectionAlternative(Box<meta_language::GrammarSelectionAlternative>),
    /// Native storage for `meta_language::ParseOutcome`.
    ParseOutcome(Box<meta_language::ParseOutcome>),
    /// Native storage for `meta_language::ParseRejection`.
    ParseRejection(Box<meta_language::ParseRejection>),
    /// Native storage for `meta_language::OperationValue`.
    OperationValue(Box<meta_language::OperationValue>),
    /// Native storage for `meta_language::Ambiguity`.
    Ambiguity(Box<meta_language::Ambiguity>),
    /// Native storage for `meta_language::LeafText`.
    LeafText(Box<meta_language::LeafText>),
    /// Native storage for `meta_language::SyntaxAttributes`.
    SyntaxAttributes(meta_language::SyntaxAttributes),
    /// Native storage for `meta_language::SyntaxTree`.
    SyntaxTree(Box<meta_language::SyntaxTree>),
    /// Native storage for `String`.
    Text(String),
    /// Native storage for `Box<meta_language::SyntaxTree>`.
    BoxedSyntaxTree(Box<meta_language::SyntaxTree>),
    /// Native storage for `serde_json::Value`.
    StructuredValue(serde_json::Value),
    /// Native storage for `usize`.
    Size(usize),
    /// Native storage for `Vec<serde_json::Value>`.
    SequenceOfStructuredValue(Vec<serde_json::Value>),
    /// Native storage for `Vec<meta_language::GrammarSelectionAlternative>`.
    SequenceOfGrammarSelectionAlternative(Vec<meta_language::GrammarSelectionAlternative>),
    /// Native storage for `bool`.
    Boolean(bool),
    /// Native storage for `Option<serde_json::Value>`.
    OptionalStructuredValue(Option<serde_json::Value>),
    /// Native storage for `Option<Vec<serde_json::Value>>`.
    OptionalSequenceOfStructuredValue(Option<Vec<serde_json::Value>>),
    /// Native storage for `Option<String>`.
    OptionalText(Option<String>),
    /// Native storage for `&'static str`.
    ExistingStaticReference(&'static str),
    /// Native storage for `Option<usize>`.
    OptionalSize(Option<usize>),
    /// Native storage for `Option<Vec<String>>`.
    OptionalSequenceOfText(Option<Vec<String>>),
    /// Native storage for `Vec<String>`.
    SequenceOfText(Vec<String>),
    /// Native storage for `Option<meta_language::SyntaxTree>`.
    OptionalSyntaxTree(Box<Option<meta_language::SyntaxTree>>),
    /// Native storage for `Vec<meta_language::Ambiguity>`.
    SequenceOfAmbiguity(Vec<meta_language::Ambiguity>),
    /// Native storage for `Option<meta_language::ParseRejection>`.
    OptionalParseRejection(Box<Option<meta_language::ParseRejection>>),
    /// Native storage for `i64`.
    SignedInteger(i64),
    /// Native storage for `Vec<meta_language::SyntaxTree>`.
    SequenceOfSyntaxTree(Vec<meta_language::SyntaxTree>),
    /// Native storage for `Option<meta_language::SyntaxAttributes>`.
    OptionalSyntaxAttributes(Option<meta_language::SyntaxAttributes>),
    /// Native storage for `Option<Box<meta_language::GrammarSelection>>`.
    OptionalBoxedGrammarSelection(Option<Box<meta_language::GrammarSelection>>),
    /// Native storage for `Box<meta_language::GrammarSelection>`.
    BoxedGrammarSelection(Box<meta_language::GrammarSelection>),
    /// Native storage for `Option<Vec<meta_language::SyntaxTree>>`.
    OptionalSequenceOfSyntaxTree(Option<Vec<meta_language::SyntaxTree>>),
    /// Native storage for `indexmap::IndexMap<String, meta_language::OperationValue>`.
    OrderedMapOfTextAndOperationValue(indexmap::IndexMap<String, meta_language::OperationValue>),
}
impl NativeConstructorValue {
    /// The actual native storage type; this is not a concept equivalence proof.
    pub const fn storage_type(&self) -> &'static str {
        match self {
            Self::GrammarSelection(_) => "meta_language::GrammarSelection",
            Self::GrammarSelectionAlternative(_) => "meta_language::GrammarSelectionAlternative",
            Self::ParseOutcome(_) => "meta_language::ParseOutcome",
            Self::ParseRejection(_) => "meta_language::ParseRejection",
            Self::OperationValue(_) => "meta_language::OperationValue",
            Self::Ambiguity(_) => "meta_language::Ambiguity",
            Self::LeafText(_) => "meta_language::LeafText",
            Self::SyntaxAttributes(_) => "meta_language::SyntaxAttributes",
            Self::SyntaxTree(_) => "meta_language::SyntaxTree",
            Self::Text(_) => "String",
            Self::BoxedSyntaxTree(_) => "Box<meta_language::SyntaxTree>",
            Self::StructuredValue(_) => "serde_json::Value",
            Self::Size(_) => "usize",
            Self::SequenceOfStructuredValue(_) => "Vec<serde_json::Value>",
            Self::SequenceOfGrammarSelectionAlternative(_) => {
                "Vec<meta_language::GrammarSelectionAlternative>"
            }
            Self::Boolean(_) => "bool",
            Self::OptionalStructuredValue(_) => "Option<serde_json::Value>",
            Self::OptionalSequenceOfStructuredValue(_) => "Option<Vec<serde_json::Value>>",
            Self::OptionalText(_) => "Option<String>",
            Self::ExistingStaticReference(_) => "&'static str",
            Self::OptionalSize(_) => "Option<usize>",
            Self::OptionalSequenceOfText(_) => "Option<Vec<String>>",
            Self::SequenceOfText(_) => "Vec<String>",
            Self::OptionalSyntaxTree(_) => "Option<meta_language::SyntaxTree>",
            Self::SequenceOfAmbiguity(_) => "Vec<meta_language::Ambiguity>",
            Self::OptionalParseRejection(_) => "Option<meta_language::ParseRejection>",
            Self::SignedInteger(_) => "i64",
            Self::SequenceOfSyntaxTree(_) => "Vec<meta_language::SyntaxTree>",
            Self::OptionalSyntaxAttributes(_) => "Option<meta_language::SyntaxAttributes>",
            Self::OptionalBoxedGrammarSelection(_) => {
                "Option<Box<meta_language::GrammarSelection>>"
            }
            Self::BoxedGrammarSelection(_) => "Box<meta_language::GrammarSelection>",
            Self::OptionalSequenceOfSyntaxTree(_) => "Option<Vec<meta_language::SyntaxTree>>",
            Self::OrderedMapOfTextAndOperationValue(_) => {
                "indexmap::IndexMap<String, meta_language::OperationValue>"
            }
        }
    }
}
/// A moved typed value together with its exact source-qualified type occurrence.
#[derive(Debug)]
pub struct NativeConstructorSlot {
    /// Source identity, independently of the shared native storage carrier.
    pub type_reference: &'static str,
    /// The actual native value.
    pub value: NativeConstructorValue,
}
fn fail(message: &str) -> String {
    format!("native source storage: {message}")
}
fn require_empty_choice(choice: &str) -> Result<(), String> {
    if choice.is_empty() {
        Ok(())
    } else {
        Err(fail("unexpected constructor choice"))
    }
}
fn require_no_scalar(value: Option<NativeConstructorValue>) -> Result<(), String> {
    value.map_or(Ok(()), |value| {
        drop(value);
        Err(fail("unexpected scalar input"))
    })
}
fn require_no_children(
    children: &mut std::vec::IntoIter<NativeConstructorSlot>,
) -> Result<(), String> {
    if children.next().is_none() {
        Ok(())
    } else {
        Err(fail("extra source child"))
    }
}
macro_rules! take_child {
    ($children:expr, $source:expr, $variant:ident, boxed) => {
        *take_child!($children, $source, $variant)
    };
    ($children:expr, $source:expr, $variant:ident) => {{
        let slot = $children
            .next()
            .ok_or_else(|| fail("missing source child"))?;
        if slot.type_reference != $source {
            return Err(fail("foreign or reordered source child"));
        }
        match slot.value {
            NativeConstructorValue::$variant(value) => value,
            _ => return Err(fail("wrong source child storage type")),
        }
    }};
}
mod part_0;
mod part_1;
mod part_2;
mod part_3;
mod part_4;
/// Every captured source type occurrence, in byte-sorted order.
pub const SOURCE_REFERENCES: &[&str] = &[
    "declaration:rust/src/grammar/feature_runtime/grammar_selection.rs#GrammarSelection",
    "declaration:rust/src/grammar/feature_runtime/grammar_selection.rs#GrammarSelectionAlternative",
    "declaration:rust/src/grammar/feature_runtime/mod.rs#ParseOutcome",
    "declaration:rust/src/grammar/feature_runtime/mod.rs#ParseRejection",
    "declaration:rust/src/grammar/feature_runtime/operations.rs#OperationValue",
    "declaration:rust/src/grammar/feature_runtime/tree.rs#Ambiguity",
    "declaration:rust/src/grammar/feature_runtime/tree.rs#LeafText",
    "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxAttributes",
    "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxTree",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1716:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1810:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1810:2",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1923:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2163:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2257:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2316:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2383:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2475:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2475:2",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2573:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2656:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2656:2",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2781:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2901:0",
    "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2901:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@5978:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@5978:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@5978:4",
    "type:rust/src/grammar/feature_runtime/mod.rs@6120:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@6120:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@6251:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@6251:3",
    "type:rust/src/grammar/feature_runtime/mod.rs@6334:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@6334:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@6403:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@6403:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@6492:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@6492:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@6585:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@6585:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@6585:4",
    "type:rust/src/grammar/feature_runtime/mod.rs@6669:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@6669:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@9131:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@9205:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@9205:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@9306:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@9306:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@9376:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@9376:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@9496:0",
    "type:rust/src/grammar/feature_runtime/mod.rs@9496:2",
    "type:rust/src/grammar/feature_runtime/mod.rs@9496:4",
    "type:rust/src/grammar/feature_runtime/operations.rs@899:0",
    "type:rust/src/grammar/feature_runtime/operations.rs@930:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@1031:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@1636:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@1709:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@1709:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@1821:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18402:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18449:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18502:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18601:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18601:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@18601:4",
    "type:rust/src/grammar/feature_runtime/tree.rs@1870:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18711:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18711:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@18748:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18748:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@18783:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18783:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@18820:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18820:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@18820:4",
    "type:rust/src/grammar/feature_runtime/tree.rs@18878:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@18878:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@1927:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@1995:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@1995:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@2075:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2075:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@2265:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2265:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@2347:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2347:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@2450:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2499:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2556:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2632:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2632:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@2700:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2804:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@2804:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@2970:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3027:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3078:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3178:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3178:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@3345:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3345:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@3400:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3475:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3540:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3660:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3711:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3768:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3836:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3836:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@3948:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@3948:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@3948:4",
    "type:rust/src/grammar/feature_runtime/tree.rs@4107:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@4107:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@4107:4",
    "type:rust/src/grammar/feature_runtime/tree.rs@702:0",
    "type:rust/src/grammar/feature_runtime/tree.rs@702:2",
    "type:rust/src/grammar/feature_runtime/tree.rs@702:4",
    "type:rust/src/grammar/feature_runtime/tree.rs@956:0",
];
/// Pack actual typed children using the exact source constructor and field order.
/// Scalars are already native; this function does not recreate their provenance.
pub fn construct_source_slot(
    reference: &str,
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorSlot, String> {
    if children.len() > 100_000 {
        return Err(fail("source child storage budget"));
    }
    let index = SOURCE_REFERENCES
        .binary_search(&reference)
        .map_err(|_| fail("unknown source type occurrence"))?;
    let value = match index / 24 {
        0 => part_0::construct(index, choice, children, scalar)?,
        1 => part_1::construct(index, choice, children, scalar)?,
        2 => part_2::construct(index, choice, children, scalar)?,
        3 => part_3::construct(index, choice, children, scalar)?,
        4 => part_4::construct(index, choice, children, scalar)?,
        _ => return Err(fail("invalid constructor partition")),
    };
    Ok(NativeConstructorSlot {
        type_reference: SOURCE_REFERENCES[index],
        value,
    })
}
