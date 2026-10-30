//! Actual token/source owners and AST coordinates. All class admission and
//! lowering decisions come from the automatically translated JavaScript source.
use super::{ModuleContext, Result, SProgram, Span, Token, TokenKind, unsupported};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::javascript_class_source_rules::{self as rules, ml_string::Text};

pub(super) struct ClassSource {
    pub(super) text: String,
    intervals: Vec<f64>,
    brands: Vec<String>,
}

#[allow(clippy::cast_precision_loss)] // actual token columns are bounded before transport
pub(super) fn read(
    source: &str,
    tokens: &[Token],
    context: &ModuleContext,
) -> Result<Option<ClassSource>> {
    let has_class = tokens.iter().enumerate().any(|(index, token)| {
        token.kind == TokenKind::Identifier
            && token.value == "class"
            && tokens
                .get(index.wrapping_sub(1))
                .is_none_or(|previous| previous.value != "." && previous.value != "?.")
            && tokens
                .get(index + 1)
                .is_some_and(|next| next.kind == TokenKind::Identifier || next.value == "{")
    });
    if !has_class {
        return Ok(None);
    }
    if !rules::accept_java_script_closed_class_source_size(
        source.encode_utf16().take(131_073).count() as f64,
        tokens.len() as f64,
    ) {
        return Err(unsupported(
            "JavaScript closed class source",
            &rules::read_java_script_closed_class_refusal(-8.0).to_string(),
            tokens.first().map(Token::span),
        ));
    }
    let values = tokens
        .iter()
        .map(|token| Text::from_text(&token.value))
        .collect::<Vec<_>>();
    let kinds = tokens
        .iter()
        .map(|token| Text::from_text(super::record_parameters::token_kind(token)))
        .collect::<Vec<_>>();
    // Token offsets are bounded by the actual <=128 Ki UTF-16 source profile.
    let starts = tokens
        .iter()
        .map(|token| token.start as f64)
        .collect::<Vec<_>>();
    let ends = tokens
        .iter()
        .map(|token| token.end as f64)
        .collect::<Vec<_>>();
    let prefix = crate::translation::source_identifiers::read_source_parser_identifier_supply(
        source, tokens,
    );
    let (at, brands, changed, error, intervals, text) =
        rules::lower_java_script_closed_class_source(
            &Text::from_text(source),
            &values,
            &kinds,
            &starts,
            &ends,
            &Text::from_text(&prefix),
        );
    if !error.0.is_empty() {
        let token = number_index(at)
            .and_then(|index| tokens.get(index))
            .or_else(|| tokens.first());
        return Err(unsupported(
            "JavaScript closed class source",
            &error.to_string(),
            token.map(Token::span),
        ));
    }
    if !changed {
        return Ok(None);
    }
    let context_error = rules::read_java_script_closed_class_context_error(
        true,
        context.externals.len() as f64,
        0.0,
        0.0,
        true,
        context.module_directory.is_some(),
    );
    if !context_error.0.is_empty() {
        return Err(unsupported(
            "JavaScript closed class context",
            &context_error.to_string(),
            Some(Span::new(0, source.encode_utf16().count())),
        ));
    }
    Ok(Some(ClassSource {
        text: text.to_string(),
        intervals,
        brands: brands.iter().map(ToString::to_string).collect(),
    }))
}

#[allow(clippy::cast_precision_loss)] // actual frontend-owned finite binding directory counts
pub(super) fn require_bindings(
    source: &str,
    constants: usize,
    scope: Option<&super::ModuleBindingScope>,
) -> Result<()> {
    let context_error = rules::read_java_script_closed_class_context_error(
        true,
        0.0,
        constants as f64,
        scope.map_or(0, |scope| scope.names.len()) as f64,
        scope.is_none_or(|scope| scope.known),
        false,
    );
    if context_error.0.is_empty() {
        return Ok(());
    }
    Err(unsupported(
        "JavaScript closed class context",
        &context_error.to_string(),
        Some(Span::new(0, source.encode_utf16().count())),
    ))
}

#[allow(clippy::cast_precision_loss)] // source intervals are bounded by the shared source profile
fn interval(plan: &ClassSource, place: Span) -> Option<Span> {
    let projected = rules::project_java_script_class_source_interval(
        &plan.intervals,
        place.start as f64,
        place.end as f64,
    );
    if projected.len() != 2 {
        return None;
    }
    Some(Span::new(
        number_index(projected[0])?,
        number_index(projected[1])?,
    ))
}

pub(super) fn project(result: Result<SProgram>, plan: &ClassSource) -> Result<SProgram> {
    let result = result.and_then(|surface| {
        validate_uses(&surface, plan)?;
        Ok(surface)
    });
    let program = match result {
        Ok(program) => program,
        Err(mut error) => {
            if let Some(place) = error.span.and_then(|place| interval(plan, place)) {
                error.span = Some(place);
            }
            return Err(error);
        }
    };
    // The existing surface interchange serializer transports typed syntax only.
    // This transient value never becomes a stored JSON artifact or an opaque
    // semantic payload; the result is the same explicit typed SProgram.
    let mut value = serde_json::to_value(program).map_err(|error| {
        unsupported(
            "JavaScript class source coordinates",
            &error.to_string(),
            None,
        )
    })?;
    let mut pending = vec![&mut value];
    while let Some(value) = pending.pop() {
        match value {
            serde_json::Value::Array(values) => pending.extend(values.iter_mut()),
            serde_json::Value::Object(fields) => {
                if let Some(place) = fields
                    .get_mut("span")
                    .and_then(serde_json::Value::as_object_mut)
                {
                    let start = place
                        .get("start")
                        .and_then(serde_json::Value::as_u64)
                        .and_then(|value| usize::try_from(value).ok());
                    let end = place
                        .get("end")
                        .and_then(serde_json::Value::as_u64)
                        .and_then(|value| usize::try_from(value).ok());
                    if let Some(projected) = start
                        .zip(end)
                        .and_then(|(start, end)| interval(plan, Span::new(start, end)))
                    {
                        place.insert("start".to_owned(), projected.start.into());
                        place.insert("end".to_owned(), projected.end.into());
                    }
                }
                pending.extend(
                    fields
                        .iter_mut()
                        .filter_map(|(name, value)| (name != "span").then_some(value)),
                );
            }
            _ => {}
        }
    }
    serde_json::from_value(value).map_err(|error| {
        unsupported(
            "JavaScript class source coordinates",
            &error.to_string(),
            None,
        )
    })
}

fn validate_uses(surface: &SProgram, plan: &ClassSource) -> Result<()> {
    let program = crate::translation::check::check_program(surface)?;
    let value = serde_json::to_value(program).map_err(|error| {
        unsupported(
            "JavaScript closed class observation",
            &error.to_string(),
            None,
        )
    })?;
    let mut pending = vec![(&value, false)];
    while let Some((node, direct)) = pending.pop() {
        let kind = node
            .get("k")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let operation = node
            .get("op")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let object = node.get("object");
        let instance = object
            .and_then(|object| object.get("type"))
            .is_some_and(|ty| {
                ty["kind"] == "record"
                    && ty["fields"].as_array().is_some_and(|fields| {
                        fields.iter().any(|field| {
                            field["name"]
                                .as_str()
                                .is_some_and(|name| plan.brands.iter().any(|brand| brand == name))
                        })
                    })
            });
        let private_field = node
            .get("field")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|name| plan.brands.iter().any(|brand| brand == name));
        let callable = node.get("type").is_some_and(|ty| ty["kind"] == "function");
        let error = rules::read_java_script_closed_class_use_error(
            &Text::from_text(kind),
            &Text::from_text(operation),
            instance,
            private_field,
            callable,
            direct,
        );
        if !error.0.is_empty() {
            return Err(unsupported(
                "JavaScript closed class observation",
                &error.to_string(),
                Some(Span::new(0, plan.text.encode_utf16().count())),
            ));
        }
        match node {
            serde_json::Value::Array(values) => {
                pending.extend(values.iter().map(|value| (value, false)))
            }
            serde_json::Value::Object(values) => pending.extend(
                values
                    .iter()
                    .filter(|(name, _)| {
                        !["type", "domain", "span", "modulePath"].contains(&name.as_str())
                    })
                    .map(|(name, value)| (value, kind == "apply" && name == "fn")),
            ),
            _ => {}
        }
    }
    Ok(())
}
