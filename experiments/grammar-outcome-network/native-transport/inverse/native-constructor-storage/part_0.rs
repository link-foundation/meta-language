// Generated typed host constructors from complete actual source declarations.
use super::*;
pub(super) fn construct(
    index: usize,
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    match index {
        0 => construct_0(choice, children, scalar),
        1 => construct_1(choice, children, scalar),
        2 => construct_2(choice, children, scalar),
        3 => construct_3(choice, children, scalar),
        4 => construct_4(choice, children, scalar),
        5 => construct_5(choice, children, scalar),
        6 => construct_6(choice, children, scalar),
        7 => construct_7(choice, children, scalar),
        8 => construct_8(choice, children, scalar),
        9 => construct_9(choice, children, scalar),
        10 => construct_10(choice, children, scalar),
        11 => construct_11(choice, children, scalar),
        12 => construct_12(choice, children, scalar),
        13 => construct_13(choice, children, scalar),
        14 => construct_14(choice, children, scalar),
        15 => construct_15(choice, children, scalar.as_ref()),
        16 => construct_16(choice, children, scalar.as_ref()),
        17 => construct_17(choice, children, scalar),
        18 => construct_18(choice, children, scalar),
        19 => construct_19(choice, children, scalar),
        20 => construct_20(choice, children, scalar),
        21 => construct_21(choice, children, scalar),
        22 => construct_22(choice, children, scalar.as_ref()),
        23 => construct_23(choice, children, scalar),
        _ => Err(fail("invalid constructor partition")),
    }
}
fn construct_0(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2163:0",
        Text
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2257:0",
        Text
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2316:0",
        Size
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2383:0",
        Size
    );
    let field_4 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2475:0",
        SequenceOfStructuredValue
    );
    let field_5 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2573:0",
        StructuredValue
    );
    let field_6 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2656:0",
        SequenceOfGrammarSelectionAlternative
    );
    let field_7 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2781:0",
        Boolean
    );
    let field_8 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2901:0",
        OptionalStructuredValue
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::GrammarSelection(Box::new(
        meta_language::GrammarSelection {
            mode: field_0,
            mapping: field_1,
            start: field_2,
            end: field_3,
            candidates: field_4,
            provenance: field_5,
            alternatives: field_6,
            ambiguous: field_7,
            source_mapping: field_8,
        },
    )))
}

fn construct_1(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1716:0",
        Text
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1810:0",
        BoxedSyntaxTree
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1923:0",
        StructuredValue
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::GrammarSelectionAlternative(
        Box::new(meta_language::GrammarSelectionAlternative {
            language: field_0,
            root: field_1,
            provenance: field_2,
        }),
    ))
}

fn construct_2(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@9131:0",
        Boolean
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@9205:0",
        OptionalSyntaxTree,
        boxed
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@9306:0",
        SequenceOfAmbiguity
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@9376:0",
        OptionalParseRejection,
        boxed
    );
    let field_4 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@9496:0",
        OptionalSequenceOfStructuredValue
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::ParseOutcome(Box::new(
        meta_language::ParseOutcome {
            ok: field_0,
            tree: field_1,
            ambiguities: field_2,
            rejection: field_3,
            source_planes: field_4,
        },
    )))
}

fn construct_3(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@5978:0",
        OptionalSequenceOfStructuredValue
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6120:0",
        OptionalText
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6251:0",
        ExistingStaticReference
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6334:0",
        OptionalSize
    );
    let field_4 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6403:0",
        OptionalSize
    );
    let field_5 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6492:0",
        OptionalSize
    );
    let field_6 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6585:0",
        OptionalSequenceOfText
    );
    let field_7 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6669:0",
        OptionalSize
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::ParseRejection(Box::new(
        meta_language::ParseRejection {
            diagnostics: field_0,
            offset_semantics: field_1,
            reason: field_2,
            offset: field_3,
            line: field_4,
            column: field_5,
            expected: field_6,
            limit: field_7,
        },
    )))
}

fn construct_4(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    require_no_scalar(scalar)?;
    match choice {
        "declaration:rust/src/grammar/feature_runtime/operations.rs#OperationValue@891:903#b8fc593af649bb6d541e47947a2ff0829791a1bcf84d2cb1d38c1a4e1b32ca92" => {
            construct_variant_4_0(children)
        }
        "declaration:rust/src/grammar/feature_runtime/operations.rs#OperationValue@925:937#63b5dbf4cca6bd4304398a5458aab505e46d0d26506ac9d760b6a51b75a39956" => {
            construct_variant_4_1(children)
        }
        _ => Err(fail("unknown source-qualified native variant")),
    }
}
fn construct_variant_4_0(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/operations.rs@899:0",
        SignedInteger
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::OperationValue(Box::new(
        meta_language::OperationValue::Integer(field_0),
    )))
}
fn construct_variant_4_1(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/operations.rs@930:0",
        Text
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::OperationValue(Box::new(
        meta_language::OperationValue::Text(field_0),
    )))
}
fn construct_5(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18402:0",
        Text
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18449:0",
        Size
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18502:0",
        Size
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18601:0",
        OptionalSequenceOfText
    );
    let field_4 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18711:0",
        OptionalSize
    );
    let field_5 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18748:0",
        OptionalSize
    );
    let field_6 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18783:0",
        OptionalSize
    );
    let field_7 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18820:0",
        OptionalSequenceOfStructuredValue
    );
    let field_8 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@18878:0",
        OptionalText
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Ambiguity(Box::new(
        meta_language::Ambiguity {
            rule: field_0,
            start: field_1,
            end: field_2,
            languages: field_3,
            source: field_4,
            source_start: field_5,
            source_end: field_6,
            source_route: field_7,
            offset_semantics: field_8,
        },
    )))
}

fn construct_6(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    require_no_scalar(scalar)?;
    match choice {
        "declaration:rust/src/grammar/feature_runtime/tree.rs#LeafText@951:963#63b5dbf4cca6bd4304398a5458aab505e46d0d26506ac9d760b6a51b75a39956" => {
            construct_variant_6_0(children)
        }
        "declaration:rust/src/grammar/feature_runtime/tree.rs#LeafText@1027:1038#f27840ee90b9f9bb200594457d5c45187b46f460d3d146196b7054817826b733" => {
            construct_variant_6_1(children)
        }
        _ => Err(fail("unknown source-qualified native variant")),
    }
}
fn construct_variant_6_0(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@956:0",
        Text
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::LeafText(Box::new(
        meta_language::LeafText::Text(field_0),
    )))
}
fn construct_variant_6_1(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@1031:0",
        Text
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::LeafText(Box::new(
        meta_language::LeafText::Hex(field_0),
    )))
}
fn construct_7(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@702:0",
        OrderedMapOfTextAndOperationValue
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxAttributes(value))
}

fn construct_8(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    require_no_scalar(scalar)?;
    match choice {
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxTree@1588:2106#ae97618edab3c47970c888f1ac25ffc94c06c6c34df8de63dfcaeba11839ea95" => {
            construct_variant_8_0(children)
        }
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxTree@2182:2836#89138e5fa320b49f88cd5725fdc314fff374325a4512caad838095221c3b499b" => {
            construct_variant_8_1(children)
        }
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxTree@2919:3199#8ad45d4b021d8aae5fd0f9318e0d39fa16b90354fad9411a1de1bd8a60fc22a1" => {
            construct_variant_8_2(children)
        }
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxTree@3275:3551#6c68ff528df2619b272282f1dddc1f04cc3b9437587f68460f340603d7e3eb64" => {
            construct_variant_8_3(children)
        }
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxTree@3599:4131#174f2054b76a43124ff7647313bf3545b4c45876bd85be213ab3765d0f55b330" => {
            construct_variant_8_4(children)
        }
        _ => Err(fail("unknown source-qualified native variant")),
    }
}
fn construct_variant_8_0(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@1636:0",
        Text
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@1709:0",
        OptionalText
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@1821:0",
        Boolean
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@1870:0",
        Size
    );
    let field_4 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@1927:0",
        Size
    );
    let field_5 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@1995:0",
        SequenceOfSyntaxTree
    );
    let field_6 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2075:0",
        OptionalSyntaxAttributes
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxTree(Box::new(
        meta_language::SyntaxTree::Node {
            kind: field_0,
            field: field_1,
            trivia: field_2,
            start: field_3,
            end: field_4,
            children: field_5,
            attributes: field_6,
        },
    )))
}
fn construct_variant_8_1(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2265:0",
        OptionalText
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2347:0",
        OptionalText
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2450:0",
        Boolean
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2499:0",
        Size
    );
    let field_4 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2556:0",
        Size
    );
    let field_5 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2632:0",
        OptionalSyntaxAttributes
    );
    let field_6 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2700:0",
        LeafText,
        boxed
    );
    let field_7 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2804:0",
        OptionalStructuredValue
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxTree(Box::new(
        meta_language::SyntaxTree::Token {
            kind: field_0,
            field: field_1,
            trivia: field_2,
            start: field_3,
            end: field_4,
            attributes: field_5,
            text: field_6,
            lexical: field_7,
        },
    )))
}
fn construct_variant_8_2(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@2970:0",
        Size
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3027:0",
        Size
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3078:0",
        LeafText,
        boxed
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3178:0",
        OptionalText
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxTree(Box::new(
        meta_language::SyntaxTree::Error {
            start: field_0,
            end: field_1,
            text: field_2,
            reason: field_3,
        },
    )))
}
fn construct_variant_8_3(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3345:0",
        OptionalText
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3400:0",
        Size
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3475:0",
        Size
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3540:0",
        Boolean
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxTree(Box::new(
        meta_language::SyntaxTree::Missing {
            kind: field_0,
            start: field_1,
            end: field_2,
            literal: field_3,
        },
    )))
}
fn construct_variant_8_4(
    children: Vec<NativeConstructorSlot>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    let field_0 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3660:0",
        Text
    );
    let field_1 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3711:0",
        Size
    );
    let field_2 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3768:0",
        Size
    );
    let field_3 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3836:0",
        BoxedSyntaxTree
    );
    let field_4 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3948:0",
        OptionalBoxedGrammarSelection
    );
    let field_5 = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@4107:0",
        OptionalSequenceOfSyntaxTree
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxTree(Box::new(
        meta_language::SyntaxTree::Embed {
            language: field_0,
            start: field_1,
            end: field_2,
            root: field_3,
            selection: field_4,
            source_children: field_5,
        },
    )))
}
fn construct_9(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::Text(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Text(value))
}

fn construct_10(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/grammar_selection.rs@1810:2",
        SyntaxTree,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::BoxedSyntaxTree(Box::new(value)))
}

fn construct_11(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxTree",
        SyntaxTree,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxTree(Box::new(value)))
}

fn construct_12(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::StructuredValue(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::StructuredValue(value))
}

fn construct_13(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::Text(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Text(value))
}

fn construct_14(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::Text(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Text(value))
}

fn construct_15(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<&NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::Size(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Size(*value))
}

fn construct_16(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<&NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::Size(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Size(*value))
}

fn construct_17(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let mut values = Vec::with_capacity(children.len());
    while !children.as_slice().is_empty() {
        values.push(take_child!(
            children,
            "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2475:2",
            StructuredValue
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfStructuredValue(values))
}

fn construct_18(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::StructuredValue(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::StructuredValue(value))
}

fn construct_19(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::StructuredValue(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::StructuredValue(value))
}

fn construct_20(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let mut values = Vec::with_capacity(children.len());
    while !children.as_slice().is_empty() {
        values.push(take_child!(
            children,
            "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2656:2",
            GrammarSelectionAlternative,
            boxed
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfGrammarSelectionAlternative(values))
}

fn construct_21(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/grammar_selection.rs#GrammarSelectionAlternative",
        GrammarSelectionAlternative,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::GrammarSelectionAlternative(
        Box::new(value),
    ))
}

fn construct_22(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<&NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::Boolean(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Boolean(*value))
}

fn construct_23(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalStructuredValue(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/grammar_selection.rs@2901:2",
                StructuredValue
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalStructuredValue(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}
