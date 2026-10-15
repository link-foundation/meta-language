//! Bind checked expressions to the shared lexical ownership decisions.
use super::records::{child, key};
use super::{Result, Value, allowed, array, contains, fail, pair_type, policy, value_type};
use crate::translation::rust::owned_local_state_rules::{
    accept_rust_owned_local_binary_operation, accept_rust_owned_local_expression,
    accept_rust_owned_local_text_units,
};
use serde_json::json;
use std::collections::BTreeMap;

#[derive(Clone)]
pub(super) struct Slot {
    ty: Value,
    available: bool,
    fields: Option<Vec<Self>>,
}

impl Slot {
    pub(super) fn new(ty: Value) -> Self {
        let fields = if pair_type(&ty) {
            Some(vec![
                Self::new(json!({"kind":"string"})),
                Self::new(ty["elements"][1].clone()),
            ])
        } else {
            None
        };
        Self {
            ty,
            available: true,
            fields,
        }
    }
}

pub(super) type Scope = BTreeMap<String, Slot>;
type State = BTreeMap<String, String>;

fn merge(scope: &mut Scope, inner: &Scope) -> Result<()> {
    for (name, slot) in scope {
        let other = inner
            .get(name)
            .ok_or_else(|| fail("a value has no actual lexical owner"))?;
        slot.available &= other.available;
        if let Some(fields) = &mut slot.fields {
            let others = other
                .fields
                .as_ref()
                .ok_or_else(|| fail("tuple field layout or lexical ownership changed"))?;
            for (field, other) in fields.iter_mut().zip(others) {
                field.available &= other.available;
            }
        }
    }
    Ok(())
}

pub(super) struct Validator<'a> {
    proof: &'a Value,
    bindings: BTreeMap<String, Value>,
    reads: BTreeMap<String, Value>,
    visits: usize,
    mutable: Option<String>,
}

impl<'a> Validator<'a> {
    pub(super) const fn new(
        proof: &'a Value,
        bindings: BTreeMap<String, Value>,
        reads: BTreeMap<String, Value>,
    ) -> Self {
        Self {
            proof,
            bindings,
            reads,
            visits: 0,
            mutable: None,
        }
    }

    fn read(&self, node: &Value, path: &[Value], scope: &mut Scope) -> Result<()> {
        let variable = node["k"] == "var";
        let name = if variable {
            &node["name"]
        } else {
            &node["object"]["name"]
        };
        let name = name
            .as_str()
            .ok_or_else(|| fail("a value has no actual lexical owner"))?;
        let entry = scope
            .get_mut(name)
            .ok_or_else(|| fail("a moved String or tuple field is used again"))?;
        let value = if variable {
            entry
        } else {
            let index = node["index"]
                .as_u64()
                .and_then(|index| usize::try_from(index).ok())
                .ok_or_else(|| fail("tuple field layout or lexical ownership changed"))?;
            entry
                .fields
                .as_mut()
                .and_then(|fields| fields.get_mut(index))
                .ok_or_else(|| fail("tuple field layout or lexical ownership changed"))?
        };
        if !value.available {
            return Err(fail("a moved String or tuple field is used again"));
        }
        let fact = self
            .reads
            .get(&key(&json!(path))?)
            .ok_or_else(|| fail("a String use lacks its actual owned source occurrence"))?;
        let binding = if variable {
            name.to_owned()
        } else {
            format!("{name}.{}", node["index"])
        };
        if fact["binding"] != binding {
            return Err(fail(
                "a String use lacks its actual owned source occurrence",
            ));
        }
        if fact["mode"] == "move" {
            value.available = false;
        }
        Ok(())
    }

    pub(super) fn expression(
        &mut self,
        node: &Value,
        path: &[Value],
        scope: &mut Scope,
        state: &State,
        expected_join: Option<&str>,
    ) -> Result<()> {
        self.visits += 1;
        if self.visits > 100_000 || path.len() > 256 {
            return Err(fail("state proof exceeds its semantic traversal limit"));
        }
        let kind = node["k"].as_str().unwrap_or_default();
        if !accept_rust_owned_local_expression(kind)
            || !allowed(node, &policy()["expressions"][kind])
            || (!value_type(&node["type"]) && !pair_type(&node["type"]))
        {
            return Err(fail(
                "an expression has an unsupported value shape or domain",
            ));
        }
        match kind {
            "lit" => Self::literal(node),
            "var" => {
                let name = node["name"]
                    .as_str()
                    .ok_or_else(|| fail("a value has no actual lexical owner"))?;
                if scope.get(name).is_none_or(|value| value.ty != node["type"]) {
                    return Err(fail("a value has no actual lexical owner"));
                }
                if node["type"]["kind"] == "tuple" {
                    return Err(fail("whole tuple reads need their own partial-move proof"));
                }
                if expected_join.is_some_and(|name| {
                    state
                        .get(name)
                        .is_none_or(|current| node["name"] != *current)
                }) {
                    return Err(fail(
                        "a conditional state result must be its current owned String slot",
                    ));
                }
                if node["type"]["kind"] == "string" {
                    self.read(node, path, scope)?;
                }
                Ok(())
            }
            "tupleIndex" => self.tuple_index(node, path, scope),
            "tuple" => {
                let items = array(
                    &node["items"],
                    "an initializer has the wrong owned tuple layout",
                )?;
                if !pair_type(&node["type"]) || items.len() != 2 {
                    return Err(fail("an initializer has the wrong owned tuple layout"));
                }
                for (index, item) in items.iter().enumerate() {
                    let mut next = child(path, "items");
                    next.push(Value::from(index));
                    self.expression(item, &next, scope, state, None)?;
                }
                Ok(())
            }
            "binary" => {
                if !accept_rust_owned_local_binary_operation(
                    node["op"].as_str().unwrap_or_default(),
                ) || node["type"]["kind"] != "bool"
                    || node["left"]["type"]["kind"] == "string"
                    || node["left"]["type"] != node["right"]["type"]
                    || node.get("domain").is_some_and(|domain| {
                        !domain.is_null()
                            && (!value_type(domain) || *domain != node["left"]["type"])
                    })
                {
                    return Err(fail(
                        "only exact equality in the actual Boolean/i32 domain is admitted",
                    ));
                }
                self.expression(&node["left"], &child(path, "left"), scope, state, None)?;
                self.expression(&node["right"], &child(path, "right"), scope, state, None)
            }
            "toString" => {
                if node["arg"]["k"] != "lit" || node["arg"]["type"]["kind"] != "string" {
                    return Err(fail(
                        "the constructor must be an actual standard literal String value",
                    ));
                }
                self.expression(&node["arg"], &child(path, "arg"), scope, state, None)
            }
            "if" => {
                if node["cond"]["type"]["kind"] != "bool" {
                    return Err(fail("a condition must retain its actual Boolean type"));
                }
                self.expression(&node["cond"], &child(path, "cond"), scope, state, None)?;
                let mut then_scope = scope.clone();
                let mut else_scope = scope.clone();
                self.expression(
                    &node["then"],
                    &child(path, "then"),
                    &mut then_scope,
                    state,
                    expected_join,
                )?;
                self.expression(
                    &node["else"],
                    &child(path, "else"),
                    &mut else_scope,
                    state,
                    expected_join,
                )?;
                merge(scope, &then_scope)?;
                merge(scope, &else_scope)
            }
            _ => self.binding(node, path, scope, state, expected_join),
        }
    }

    fn literal(node: &Value) -> Result<()> {
        let kind = node["type"]["kind"].as_str().unwrap_or_default();
        if kind == "string" && !node["value"].is_string()
            || kind == "bool" && !node["value"].is_boolean()
        {
            return Err(fail(
                "a primitive literal differs from its actual value domain",
            ));
        }
        if kind == "fixed" {
            let text = node["value"]
                .as_str()
                .ok_or_else(|| fail("an integer literal exceeds its actual signed i32 domain"))?;
            let digits = text.strip_prefix('-').unwrap_or(text);
            if digits.is_empty()
                || !digits.bytes().all(|byte| byte.is_ascii_digit())
                || text.parse::<i32>().is_err()
            {
                return Err(fail(
                    "an integer literal exceeds its actual signed i32 domain",
                ));
            }
        }
        if kind == "string" {
            let units = node["value"]
                .as_str()
                .unwrap_or_default()
                .encode_utf16()
                .map(f64::from)
                .collect::<Vec<_>>();
            if !accept_rust_owned_local_text_units(&units) {
                return Err(fail(
                    "text literals must remain valid Unicode scalar String values",
                ));
            }
        }
        Ok(())
    }

    fn tuple_index(&self, node: &Value, path: &[Value], scope: &mut Scope) -> Result<()> {
        let object = &node["object"];
        let index = node["index"]
            .as_u64()
            .and_then(|index| usize::try_from(index).ok());
        let name = object["name"].as_str().unwrap_or_default();
        if !allowed(object, &policy()["expressions"]["var"])
            || object["k"] != "var"
            || index.is_none_or(|index| index > 1)
            || !pair_type(&object["type"])
            || scope
                .get(name)
                .is_none_or(|value| value.ty != object["type"])
            || index.is_none_or(|index| node["type"] != object["type"]["elements"][index])
        {
            return Err(fail("tuple field layout or lexical ownership changed"));
        }
        if node["type"]["kind"] == "string" {
            self.read(node, path, scope)?;
        }
        Ok(())
    }

    fn binding(
        &mut self,
        node: &Value,
        path: &[Value],
        scope: &mut Scope,
        state: &State,
        expected_join: Option<&str>,
    ) -> Result<()> {
        let fact = self
            .bindings
            .get(&key(&json!(path))?)
            .cloned()
            .ok_or_else(|| fail("each actual version requires its distinct lexical owner"))?;
        let name = node["name"]
            .as_str()
            .ok_or_else(|| fail("each actual version requires its distinct lexical owner"))?;
        if scope.contains_key(name) {
            return Err(fail(
                "each actual version requires its distinct lexical owner",
            ));
        }
        let role = fact["role"].as_str().unwrap_or_default();
        let source_name = fact["sourceName"].as_str().unwrap_or_default();
        self.expression(
            &node["value"],
            &child(path, "value"),
            scope,
            state,
            if role == "join" {
                Some(source_name)
            } else {
                None
            },
        )?;
        let mut inner = scope.clone();
        let mut next = state.clone();
        if role == "tuple field" {
            let index = fact["index"]
                .as_u64()
                .and_then(|index| usize::try_from(index).ok());
            let initializer = array(
                &self.proof["initializers"],
                "state facts must retain their ordered roles",
            )?
            .iter()
            .any(|initializer| {
                initializer["binding"] == fact["initializer"]
                    && index.is_some_and(|index| {
                        initializer["fields"]
                            .as_array()
                            .and_then(|fields| fields.get(index))
                            .is_some_and(|field| *field == fact["sourceName"])
                    })
            });
            if !initializer
                || node["value"]["k"] != "tupleIndex"
                || node["value"]["object"]["name"] != fact["initializer"]
                || node["value"]["index"] != fact["index"]
                || fact["mutable"] != Value::Bool(index == Some(0))
            {
                return Err(fail(
                    "the once-evaluated tuple field and source mutability were changed",
                ));
            }
            if fact["mutable"] == true {
                if self
                    .mutable
                    .as_deref()
                    .is_some_and(|mutable| mutable != source_name)
                {
                    return Err(fail("only one tuple-owned mutable String slot is admitted"));
                }
                self.mutable = Some(source_name.to_owned());
                next.insert(source_name.to_owned(), name.to_owned());
            }
        } else if contains(&json!(["replacement", "join"]), &fact["role"]) {
            let previous = self.mutable.as_ref().and_then(|mutable| state.get(mutable));
            if self.mutable.as_deref() != Some(source_name)
                || previous.is_none_or(|previous| fact["previous"] != *previous)
                || fact["type"]["kind"] != "string"
                || fact["mutable"] != false
                || role == "join" && node["value"]["k"] != "if"
            {
                return Err(fail("replacement order or state ownership changed"));
            }
            if let Some(value) = previous.and_then(|previous| inner.get_mut(previous)) {
                value.available = false;
            }
            next.insert(source_name.to_owned(), name.to_owned());
        } else if fact["mutable"] != false
            || !fact["previous"].is_null()
            || !fact["initializer"].is_null()
            || !fact["index"].is_null()
        {
            return Err(fail(
                "an immutable binding has contradictory source state fields",
            ));
        }
        inner.insert(name.to_owned(), Slot::new(node["value"]["type"].clone()));
        self.expression(
            &node["body"],
            &child(path, "body"),
            &mut inner,
            &next,
            expected_join,
        )?;
        merge(scope, &inner)
    }
}
