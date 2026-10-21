// Generated typed host constructors from complete actual source declarations.
use super::*;
pub(super) fn construct(
    index: usize,
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    match index {
        96 => construct_96(choice, children, scalar.as_ref()),
        97 => construct_97(choice, children, scalar),
        98 => construct_98(choice, children, scalar),
        99 => construct_99(choice, children, scalar),
        100 => construct_100(choice, children, scalar),
        101 => construct_101(choice, children, scalar),
        102 => construct_102(choice, children, scalar.as_ref()),
        103 => construct_103(choice, children, scalar.as_ref()),
        104 => construct_104(choice, children, scalar.as_ref()),
        105 => construct_105(choice, children, scalar),
        106 => construct_106(choice, children, scalar.as_ref()),
        107 => construct_107(choice, children, scalar.as_ref()),
        108 => construct_108(choice, children, scalar),
        109 => construct_109(choice, children, scalar),
        110 => construct_110(choice, children, scalar),
        111 => construct_111(choice, children, scalar),
        112 => construct_112(choice, children, scalar),
        113 => construct_113(choice, children, scalar),
        114 => construct_114(choice, children, scalar),
        115 => construct_115(choice, children, scalar),
        116 => construct_116(choice, children, scalar),
        117 => construct_117(choice, children, scalar),
        118 => construct_118(choice, children, scalar),
        119 => construct_119(choice, children, scalar),
        _ => Err(fail("invalid constructor partition")),
    }
}
fn construct_96(
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

fn construct_97(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/tree.rs#LeafText",
        LeafText,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::LeafText(Box::new(value)))
}

fn construct_98(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/tree.rs@3178:2",
                Text
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_99(
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

fn construct_100(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/tree.rs@3345:2",
                Text
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_101(
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

fn construct_102(
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

fn construct_103(
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

fn construct_104(
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

fn construct_105(
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

fn construct_106(
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

fn construct_107(
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

fn construct_108(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3836:2",
        SyntaxTree,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::BoxedSyntaxTree(Box::new(value)))
}

fn construct_109(
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

fn construct_110(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalBoxedGrammarSelection(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/tree.rs@3948:2",
                BoxedGrammarSelection
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalBoxedGrammarSelection(Some(
                value,
            )))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_111(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/tree.rs@3948:4",
        GrammarSelection,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::BoxedGrammarSelection(Box::new(
        value,
    )))
}

fn construct_112(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/grammar_selection.rs#GrammarSelection",
        GrammarSelection,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::GrammarSelection(Box::new(value)))
}

fn construct_113(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSequenceOfSyntaxTree(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/tree.rs@4107:2",
                SequenceOfSyntaxTree
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSequenceOfSyntaxTree(Some(
                value,
            )))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_114(
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
            "type:rust/src/grammar/feature_runtime/tree.rs@4107:4",
            SyntaxTree,
            boxed
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfSyntaxTree(values))
}

fn construct_115(
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

fn construct_116(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let mut values = indexmap::IndexMap::new();
    while !children.as_slice().is_empty() {
        let key = take_child!(
            children,
            "type:rust/src/grammar/feature_runtime/tree.rs@702:2",
            Text
        );
        let value = take_child!(
            children,
            "type:rust/src/grammar/feature_runtime/tree.rs@702:4",
            OperationValue,
            boxed
        );
        if values.insert(key, value).is_some() {
            return Err(fail("duplicate native map key"));
        }
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::OrderedMapOfTextAndOperationValue(
        values,
    ))
}

fn construct_117(
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

fn construct_118(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/operations.rs#OperationValue",
        OperationValue,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::OperationValue(Box::new(value)))
}

fn construct_119(
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
