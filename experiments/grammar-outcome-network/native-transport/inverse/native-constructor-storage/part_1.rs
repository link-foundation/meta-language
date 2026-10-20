// Generated typed host constructors from complete actual source declarations.
use super::*;
pub(super) fn construct(
    index: usize,
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    match index {
        24 => construct_24(choice, children, scalar),
        25 => construct_25(choice, children, scalar),
        26 => construct_26(choice, children, scalar),
        27 => construct_27(choice, children, scalar),
        28 => construct_28(choice, children, scalar),
        29 => construct_29(choice, children, scalar),
        30 => construct_30(choice, children, scalar.as_ref()),
        31 => construct_31(choice, children, scalar),
        32 => construct_32(choice, children, scalar),
        33 => construct_33(choice, children, scalar.as_ref()),
        34 => construct_34(choice, children, scalar),
        35 => construct_35(choice, children, scalar.as_ref()),
        36 => construct_36(choice, children, scalar),
        37 => construct_37(choice, children, scalar.as_ref()),
        38 => construct_38(choice, children, scalar),
        39 => construct_39(choice, children, scalar),
        40 => construct_40(choice, children, scalar),
        41 => construct_41(choice, children, scalar),
        42 => construct_42(choice, children, scalar.as_ref()),
        43 => construct_43(choice, children, scalar.as_ref()),
        44 => construct_44(choice, children, scalar),
        45 => construct_45(choice, children, scalar),
        46 => construct_46(choice, children, scalar),
        47 => construct_47(choice, children, scalar),
        _ => Err(fail("invalid constructor partition")),
    }
}
fn construct_24(
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

fn construct_25(
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
                "type:rust/src/grammar/feature_runtime/mod.rs@5978:2",
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

fn construct_26(
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
            "type:rust/src/grammar/feature_runtime/mod.rs@5978:4",
            StructuredValue
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfStructuredValue(values))
}

fn construct_27(
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

fn construct_28(
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
                "type:rust/src/grammar/feature_runtime/mod.rs@6120:2",
                Text
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_29(
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

fn construct_30(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<&NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let text = take_child!(
        children,
        "type:rust/src/grammar/feature_runtime/mod.rs@6251:3",
        Text
    );
    let Some(NativeConstructorValue::ExistingStaticReference(value)) = scalar else {
        return Err(fail("an actual existing static reference is required"));
    };
    let value = *value;
    if value != text {
        return Err(fail(
            "the existing static reference differs from its owned text",
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::ExistingStaticReference(value))
}

fn construct_31(
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

fn construct_32(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/mod.rs@6334:2",
                Size
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_33(
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

fn construct_34(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/mod.rs@6403:2",
                Size
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_35(
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

fn construct_36(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/mod.rs@6492:2",
                Size
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_37(
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

fn construct_38(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSequenceOfText(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/mod.rs@6585:2",
                SequenceOfText
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSequenceOfText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_39(
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
            "type:rust/src/grammar/feature_runtime/mod.rs@6585:4",
            Text
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfText(values))
}

fn construct_40(
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

fn construct_41(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(None))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/mod.rs@6669:2",
                Size
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_42(
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

fn construct_43(
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

fn construct_44(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSyntaxTree(Box::new(None)))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/mod.rs@9205:2",
                SyntaxTree,
                boxed
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSyntaxTree(Box::new(Some(
                value,
            ))))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_45(
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

fn construct_46(
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
            "type:rust/src/grammar/feature_runtime/mod.rs@9306:2",
            Ambiguity,
            boxed
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfAmbiguity(values))
}

fn construct_47(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/tree.rs#Ambiguity",
        Ambiguity,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::Ambiguity(Box::new(value)))
}
