// Generated typed host constructors from complete actual source declarations.
use super::*;
pub(super) fn construct(
    index: usize,
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    match index {
        48 => construct_48(choice, children, scalar),
        49 => construct_49(choice, children, scalar),
        50 => construct_50(choice, children, scalar),
        51 => construct_51(choice, children, scalar),
        52 => construct_52(choice, children, scalar),
        53 => construct_53(choice, children, scalar.as_ref()),
        54 => construct_54(choice, children, scalar),
        55 => construct_55(choice, children, scalar),
        56 => construct_56(choice, children, scalar),
        57 => construct_57(choice, children, scalar),
        58 => construct_58(choice, children, scalar),
        59 => construct_59(choice, children, scalar.as_ref()),
        60 => construct_60(choice, children, scalar),
        61 => construct_61(choice, children, scalar.as_ref()),
        62 => construct_62(choice, children, scalar.as_ref()),
        63 => construct_63(choice, children, scalar),
        64 => construct_64(choice, children, scalar),
        65 => construct_65(choice, children, scalar),
        66 => construct_66(choice, children, scalar.as_ref()),
        67 => construct_67(choice, children, scalar),
        68 => construct_68(choice, children, scalar.as_ref()),
        69 => construct_69(choice, children, scalar),
        70 => construct_70(choice, children, scalar.as_ref()),
        71 => construct_71(choice, children, scalar),
        _ => Err(fail("invalid constructor partition")),
    }
}
fn construct_48(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_no_scalar(scalar)?;
    match choice {
        "None" => {
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalParseRejection(Box::new(
                None,
            )))
        }
        "Some" => {
            let value = take_child!(
                children,
                "type:rust/src/grammar/feature_runtime/mod.rs@9376:2",
                ParseRejection,
                boxed
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalParseRejection(Box::new(
                Some(value),
            )))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_49(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    require_no_scalar(scalar)?;
    let value = take_child!(
        children,
        "declaration:rust/src/grammar/feature_runtime/mod.rs#ParseRejection",
        ParseRejection,
        boxed
    );
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::ParseRejection(Box::new(value)))
}

fn construct_50(
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
                "type:rust/src/grammar/feature_runtime/mod.rs@9496:2",
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

fn construct_51(
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
            "type:rust/src/grammar/feature_runtime/mod.rs@9496:4",
            StructuredValue
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfStructuredValue(values))
}

fn construct_52(
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

fn construct_53(
    choice: &str,
    children: Vec<NativeConstructorSlot>,
    scalar: Option<&NativeConstructorValue>,
) -> Result<NativeConstructorValue, String> {
    let mut children = children.into_iter();
    require_empty_choice(choice)?;
    let Some(NativeConstructorValue::SignedInteger(value)) = scalar else {
        return Err(fail("wrong native scalar storage type"));
    };
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SignedInteger(*value))
}

fn construct_54(
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

fn construct_55(
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

fn construct_56(
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

fn construct_57(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@1709:2",
                Text
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_58(
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

fn construct_59(
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

fn construct_60(
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

fn construct_61(
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

fn construct_62(
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

fn construct_63(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@18601:2",
                SequenceOfText
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSequenceOfText(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_64(
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
            "type:rust/src/grammar/feature_runtime/tree.rs@18601:4",
            Text
        ));
    }
    require_no_children(&mut children)?;
    Ok(NativeConstructorValue::SequenceOfText(values))
}

fn construct_65(
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

fn construct_66(
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

fn construct_67(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@18711:2",
                Size
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_68(
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

fn construct_69(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@18748:2",
                Size
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}

fn construct_70(
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

fn construct_71(
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
                "type:rust/src/grammar/feature_runtime/tree.rs@18783:2",
                Size
            );
            require_no_children(&mut children)?;
            Ok(NativeConstructorValue::OptionalSize(Some(value)))
        }
        _ => Err(fail("an explicit native Some or None choice is required")),
    }
}
