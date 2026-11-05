//! Structured host transport for the existing declared decorator runtime.
use crate::{Decorator, DecoratorAction, DecoratorLevel, DecoratorSet};
use serde_json::{Value, json};
fn fail(message: &str) -> String {
    format!("decorator data: {message}")
}

pub(crate) fn read(value: &Value) -> Result<DecoratorSet, String> {
    if let Some(text) = value.as_str() {
        return DecoratorSet::from_lino(text).map_err(|error| fail(&error.to_string()));
    }
    let mut result = Vec::new();
    for record in value
        .as_array()
        .ok_or_else(|| fail("decorators must be linked records or Links Notation"))?
    {
        let field = |name: &str| {
            record[name]
                .as_str()
                .ok_or_else(|| fail("invalid decorator record"))
        };
        let id = field("id")?;
        let level = DecoratorLevel::parse(field("level")?)
            .ok_or_else(|| fail("invalid decorator level"))?;
        let order = record
            .get("order")
            .map_or(Some(0.0), Value::as_f64)
            .filter(|value| {
                value.is_finite() && value.fract() == 0.0 && value.abs() <= 9_007_199_254_740_991.0
            })
            .ok_or_else(|| fail("invalid decorator order"))?;
        #[allow(clippy::cast_possible_truncation)]
        let order = order as i64;
        let mut conditions = Vec::new();
        for condition in record["when"]
            .as_array()
            .ok_or_else(|| fail("invalid decorator conditions"))?
        {
            let pair = condition
                .as_array()
                .filter(|values| values.len() == 2)
                .ok_or_else(|| fail("invalid decorator condition"))?;
            conditions.push((
                pair[0]
                    .as_str()
                    .ok_or_else(|| fail("invalid decorator condition"))?
                    .to_owned(),
                pair[1]
                    .as_str()
                    .ok_or_else(|| fail("invalid decorator condition"))?
                    .to_owned(),
            ));
        }
        let mut actions = Vec::new();
        for action in record["actions"]
            .as_array()
            .ok_or_else(|| fail("invalid decorator actions"))?
        {
            let field = |name: &str| {
                action[name]
                    .as_str()
                    .ok_or_else(|| fail("invalid decorator action"))
            };
            actions.push(match field("op")? {
                "drop" => DecoratorAction::Drop,
                "set" => DecoratorAction::Set {
                    field: field("field")?.to_owned(),
                    value: field("value")?.to_owned(),
                },
                "replace" => DecoratorAction::Replace {
                    field: field("field")?.to_owned(),
                    from: field("from")?.to_owned(),
                    to: field("to")?.to_owned(),
                },
                _ => return Err(fail("unknown decorator action")),
            });
        }
        result.push(
            Decorator::new(id, level, order, conditions, actions)
                .map_err(|error| fail(&error.to_string()))?,
        );
    }
    DecoratorSet::new(result).map_err(|error| fail(&error.to_string()))
}

pub(crate) fn write(set: &DecoratorSet) -> Value {
    Value::Array(set.decorators().iter().map(|value| {
        let actions: Vec<_> = value.actions().iter().map(|action| match action {
            DecoratorAction::Set { field, value } => json!({"op":"set","field":field,"value":value}),
            DecoratorAction::Replace { field, from, to } => json!({"op":"replace","field":field,"from":from,"to":to}),
            DecoratorAction::Drop => json!({"op":"drop"}),
        }).collect();
        json!({"id":value.id(),"level":value.level().as_str(),"order":value.order(),"when":value.when(),"actions":actions})
    }).collect())
}
