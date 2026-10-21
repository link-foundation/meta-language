// Generated typed host constructors from complete actual source declarations.
use super::*;
pub(super) fn construct(
    index: usize,
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    match index {
        72 => construct_72(choice, children, scalar.as_ref()),
        73 => construct_73(choice, children, scalar),
        74 => construct_74(choice, children, scalar),
        75 => construct_75(choice, children, scalar),
        76 => construct_76(choice, children, scalar),
        77 => construct_77(choice, children, scalar),
        78 => construct_78(choice, children, scalar.as_ref()),
        79 => construct_79(choice, children, scalar),
        80 => construct_80(choice, children, scalar),
        81 => construct_81(choice, children, scalar),
        82 => construct_82(choice, children, scalar),
        83 => construct_83(choice, children, scalar),
        84 => construct_84(choice, children, scalar),
        85 => construct_85(choice, children, scalar),
        86 => construct_86(choice, children, scalar),
        87 => construct_87(choice, children, scalar.as_ref()),
        88 => construct_88(choice, children, scalar.as_ref()),
        89 => construct_89(choice, children, scalar.as_ref()),
        90 => construct_90(choice, children, scalar),
        91 => construct_91(choice, children, scalar),
        92 => construct_92(choice, children, scalar),
        93 => construct_93(choice, children, scalar),
        94 => construct_94(choice, children, scalar),
        95 => construct_95(choice, children, scalar.as_ref()),
        _ => Err(fail("invalid constructor partition")),
    }
}
fn construct_72(
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

fn construct_73(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSequenceOfStructuredValue(
                None,
            ))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/tree.rs@18820:2",
                SequenceOfStructuredValue
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSequenceOfStructuredValue(
                Some(value),
            ))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_74(
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
            "type:rust/src/grammar/feature_runtime/tree.rs@18820:4",
            StructuredValue
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfStructuredValue(values))
}

fn construct_75(
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

fn construct_76(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@18878:2",
                Text
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_77(
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

fn construct_78(
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

fn construct_79(
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
            "type:rust/src/grammar/feature_runtime/tree.rs@1995:2",
            SyntaxTree,
            boxed
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfSyntaxTree(values))
}

fn construct_80(
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

fn construct_81(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSyntaxAttributes(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/tree.rs@2075:2",
                SyntaxAttributes
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSyntaxAttributes(Some(
                value,
            )))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_82(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxAttributes",
        SyntaxAttributes
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxAttributes(value))
}

fn construct_83(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@2265:2",
                Text
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_84(
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

fn construct_85(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@2347:2",
                Text
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_86(
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

fn construct_87(
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

fn construct_88(
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

fn construct_89(
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

fn construct_90(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSyntaxAttributes(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/tree.rs@2632:2",
                SyntaxAttributes
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSyntaxAttributes(Some(
                value,
            )))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_91(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/tree.rs#SyntaxAttributes",
        SyntaxAttributes
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SyntaxAttributes(value))
}

fn construct_92(
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

fn construct_93(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@2804:2",
                StructuredValue
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalStructuredValue(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_94(
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

fn construct_95(
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
