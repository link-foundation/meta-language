//! Structural tree adapter for the shared closed scalar loop rules.
use super::{SourceLoop, scalar_loop_policy, scalar_loop_snapshot_binding};
use crate::translation::diagnostics::{Result, unsupported};
use crate::translation::scalar_loop_rules::{
    accept_rust_scalar_loop_model, accept_scalar_loop_binary_operation,
    accept_scalar_loop_declaration_model, accept_scalar_loop_language,
    accept_scalar_loop_math_operation, accept_scalar_loop_primitive_owner,
    accept_scalar_loop_unary_operation, accept_scalar_loop_value_kind,
};
use crate::translation::source_signatures::{SourceSignature, validate_source_signature};
use crate::translation::types::Type;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;

fn fail(reason: &str) -> crate::translation::diagnostics::TranslationError {
    unsupported("Rust scalar loop", reason, None)
}
fn text<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .ok_or_else(|| fail("loop proof needs an actual text identity"))
}

fn simple_name(value: &Value) -> Option<&str> {
    match value["k"].as_str()? {
        "var" => value["name"].as_str(),
        "name" => {
            let path = value["path"].as_array()?;
            (path.len() == 1).then(|| path[0].as_str()).flatten()
        }
        _ => None,
    }
}

pub fn validate_source_loop(item: &impl Serialize, language: &str) -> Result<()> {
    validate_in_scope(item, language, None)
}

pub fn validate_source_loop_at(
    item: &impl Serialize,
    language: &str,
    full_name: &str,
) -> Result<()> {
    validate_in_scope(item, language, Some(full_name))
}

fn validate_in_scope(
    item: &impl Serialize,
    language: &str,
    declared_name: Option<&str>,
) -> Result<()> {
    let item = serde_json::to_value(item)
        .map_err(|_| fail("loop declaration cannot be represented as checked data"))?;
    if item.get("sourceLoop").is_none() {
        return Ok(());
    }
    let proof: SourceLoop = serde_json::from_value(item["sourceLoop"].clone())
        .map_err(|_| fail("the owned loop proof has an inconsistent layout"))?;
    let policy = scalar_loop_policy();
    if proof.model.is_empty()
        || !accept_rust_scalar_loop_model(&proof.model)
        || !accept_scalar_loop_language(language)
        || proof.language != language
        || proof.source_name != text(&item, "name")?
        || item["generated"].as_bool().unwrap_or(false)
        || proof.lowering != policy["lowering"].as_str().unwrap_or("")
    {
        return Err(fail(
            "the owned loop proof has an inconsistent model, name or lowering",
        ));
    }
    let params = item["params"]
        .as_array()
        .ok_or_else(|| fail("loop state needs actual parameters"))?;
    let ret: Type = serde_json::from_value(item["ret"].clone())
        .map_err(|_| fail("loop result needs its actual scalar type"))?;
    let mut types = Vec::new();
    let mut names = HashSet::new();
    if proof.state.len() != params.len() || params.is_empty() {
        return Err(fail("loop state must match every actual parameter"));
    }
    for (param, slot) in params.iter().zip(&proof.state) {
        if slot.name != text(param, "name")?
            || !names.insert(slot.name.clone())
            || param.get("guard").is_some_and(|value| !value.is_null())
            || param.get("default").is_some_and(|value| !value.is_null())
            || param["optional"].as_bool() == Some(true)
        {
            return Err(fail("loop state has inconsistent binding ownership"));
        }
        let ty: Type = serde_json::from_value(param["type"].clone())
            .map_err(|_| fail("loop parameters need actual scalar types"))?;
        if !accept_scalar_loop_value_kind(ty.kind()) {
            return Err(fail("loop parameters need direct scalar kinds"));
        }
        types.push(ty);
    }
    if !accept_scalar_loop_value_kind(ret.kind()) {
        return Err(fail("loop result needs a direct scalar kind"));
    }
    if language == "Rust" {
        if proof.source_interface.is_some() {
            return Err(fail("Rust loop headers use actual primitive owners"));
        }
        let signature: SourceSignature = serde_json::from_value(item["sourceSignature"].clone())
            .map_err(|_| fail("Rust loops need actual owned source headers"))?;
        validate_source_signature(&signature, &types, &ret, language)?;
        if !signature.lifetime_bindings.is_empty() {
            return Err(fail("loop lifetimes require a separate state model"));
        }
        for position in signature
            .parameters
            .iter()
            .chain(std::iter::once(&signature.result))
        {
            if !position.wrappers.is_empty()
                || !position.lifetimes.is_empty()
                || position.owner.kind != "primitive"
                || position.owner.path.len() != 1
                || !position.owner.arguments.is_empty()
                || !accept_scalar_loop_primitive_owner(position.ty.kind(), &position.owner.path[0])
            {
                return Err(fail(
                    "state and result require direct primitive f64 or Boolean owners",
                ));
            }
        }
    } else {
        let interface = proof
            .source_interface
            .as_ref()
            .ok_or_else(|| fail("JavaScript loops need their distinct Number/Boolean interface"))?;
        if item.get("sourceSignature").is_some()
            || interface.model != policy["javaScriptInterfaceModel"].as_str().unwrap_or("")
            || interface.parameters.len() != types.len()
            || interface
                .parameters
                .iter()
                .zip(&types)
                .any(|(position, ty)| position.kind != ty.kind())
            || interface.result.kind != ret.kind()
        {
            return Err(fail(
                "a JavaScript loop interface must match its actual header",
            ));
        }
    }
    let actual_name = item["fullName"]
        .as_str()
        .or(declared_name)
        .unwrap_or(&proof.source_name);
    if declared_name.is_some_and(|name| name != actual_name) {
        return Err(fail(
            "the loop identity disagrees with its actual declaration scope",
        ));
    }
    if let Some(path) = item.get("modulePath") {
        let path = path
            .as_array()
            .ok_or_else(|| fail("a loop module path needs actual owned segments"))?;
        let segments = path
            .iter()
            .map(Value::as_str)
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| fail("a loop module path has an invalid segment"))?;
        if segments
            .into_iter()
            .chain(std::iter::once(proof.source_name.as_str()))
            .collect::<Vec<_>>()
            .join(".")
            != actual_name
        {
            return Err(fail(
                "the loop identity disagrees with its actual module binding",
            ));
        }
    }
    if accept_scalar_loop_declaration_model(&proof.model) {
        let continuation = proof.continuation.as_ref().ok_or_else(|| {
            fail("the continuation target disagrees with its actual root-owned declaration scope")
        })?;
        if continuation.root != policy["continuationRoot"].as_str().unwrap_or("")
            || continuation.path.iter().any(String::is_empty)
            || continuation
                .path
                .iter()
                .map(String::as_str)
                .ne(actual_name.split('.'))
        {
            return Err(fail(
                "the continuation target disagrees with its actual root-owned declaration scope",
            ));
        }
    } else if proof.continuation.is_some() {
        return Err(fail("the owned loop proof has an inconsistent layout"));
    }
    let mut adapter = TreeAdapter {
        full_name: actual_name,
        proof: &proof,
        names,
        snapshots: HashSet::new(),
        used_snapshots: HashSet::new(),
        calls: 0,
        visits: 0,
    };
    let scope = proof
        .state
        .iter()
        .map(|slot| (slot.name.clone(), None))
        .collect::<Vec<_>>();
    adapter.expression(&item["body"], &scope, true)?;
    if adapter.calls == 0
        || adapter.calls != proof.transitions.len()
        || adapter
            .snapshots
            .iter()
            .any(|name| !adapter.used_snapshots.contains(name))
    {
        return Err(fail(
            "the loop proof must describe all actual snapshots and continuations",
        ));
    }
    Ok(())
}

type Scope<'a> = [(String, Option<&'a Value>)];
struct TreeAdapter<'a> {
    full_name: &'a str,
    proof: &'a SourceLoop,
    names: HashSet<String>,
    snapshots: HashSet<String>,
    used_snapshots: HashSet<String>,
    calls: usize,
    visits: usize,
}
impl<'a> TreeAdapter<'a> {
    fn visit(&mut self) -> Result<()> {
        self.visits += 1;
        if self.visits > 100_000 {
            Err(fail("loop proof exceeds its expression resource limit"))
        } else {
            Ok(())
        }
    }
    fn expression(&mut self, node: &'a Value, scope: &Scope<'a>, tail: bool) -> Result<()> {
        self.visit()?;
        let kind = text(node, "k")?;
        if let Some(ty) = node.get("type")
            && !accept_scalar_loop_value_kind(text(ty, "kind")?)
        {
            return Err(fail("loop expressions require scalar value kinds"));
        }
        match kind {
            "let" => {
                let name = text(node, "name")?;
                if scope.iter().any(|(binding, _)| binding == name) {
                    return Err(fail("loop locals need distinct binding identities"));
                }
                if self.proof.language == "JavaScript" && name.starts_with("ml_") {
                    if !scalar_loop_snapshot_binding(name) {
                        return Err(fail(
                            "a reserved local must be an exact checked write snapshot",
                        ));
                    }
                    self.snapshots.insert(name.into());
                }
                self.expression(&node["value"], scope, false)?;
                let mut next = scope.to_vec();
                next.push((name.into(), Some(&node["value"])));
                self.expression(&node["body"], &next, tail)?;
            }
            "if" => {
                self.expression(&node["cond"], scope, false)?;
                self.expression(&node["then"], scope, tail)?;
                self.expression(&node["else"], scope, tail)?;
            }
            "app" | "call" => self.continuation(node, scope, tail)?,
            "name" | "var" => {
                if !scope
                    .iter()
                    .any(|(name, _)| Some(name.as_str()) == simple_name(node))
                {
                    return Err(fail("loop expressions cannot capture undeclared values"));
                }
            }
            "num" | "bool" | "lit" => {}
            "binary" if accept_scalar_loop_binary_operation(text(node, "op")?) => {
                self.expression(&node["left"], scope, false)?;
                self.expression(&node["right"], scope, false)?;
            }
            "unary" if accept_scalar_loop_unary_operation(text(node, "op")?) => {
                self.expression(&node["arg"], scope, false)?;
            }
            "math" if accept_scalar_loop_math_operation(text(node, "op")?) => {
                for arg in node["args"]
                    .as_array()
                    .ok_or_else(|| fail("scalar math needs actual arguments"))?
                {
                    if arg.get("k").is_some() {
                        self.expression(arg, scope, false)?;
                    } else if arg["spread"].as_bool() != Some(true) {
                        self.expression(&arg["value"], scope, false)?;
                    } else {
                        return Err(fail("scalar math excludes spread arguments"));
                    }
                }
            }
            _ => {
                return Err(fail(
                    "the expression has no closed scalar loop interpretation",
                ));
            }
        }
        Ok(())
    }
    fn continuation(&mut self, node: &'a Value, scope: &Scope<'a>, tail: bool) -> Result<()> {
        let target = if node["k"] == "call" {
            node["fn"].as_str().map(str::to_owned)
        } else {
            node["fn"]["path"]
                .as_array()
                .and_then(|path| path.iter().map(Value::as_str).collect::<Option<Vec<_>>>())
                .map(|path| path.join("."))
        };
        let proof = self.proof;
        let transition = proof
            .transitions
            .get(self.calls)
            .ok_or_else(|| fail("every continuation needs its owned transition"))?;
        self.calls += 1;
        let args = node["args"]
            .as_array()
            .ok_or_else(|| fail("the continuation needs actual argument bindings"))?;
        let owned_target = if node["k"] == "call" {
            node["fn"].as_str() == Some(self.full_name)
        } else if let Some(continuation) = &proof.continuation {
            node["fn"]["k"] == "name"
                && node["fn"]["path"].as_array().is_some_and(|path| {
                    path.len() == continuation.path.len() + 1
                        && path[0].as_str() == Some(continuation.root.as_str())
                        && path[1..]
                            .iter()
                            .zip(&continuation.path)
                            .all(|(actual, owned)| actual.as_str() == Some(owned.as_str()))
                })
        } else {
            target.as_deref().is_some_and(|target| {
                target == self.full_name
                    || target == self.proof.source_name
                    || target.strip_prefix("crate.") == Some(self.full_name)
            })
        };
        if !tail
            || !owned_target
            || args.len() != self.proof.state.len()
            || transition.arguments.len() != args.len()
        {
            return Err(fail("only an owned tail continuation is admitted"));
        }
        let mut targets = HashSet::new();
        let mut bindings = HashSet::new();
        let mut order = None;
        for write in &transition.writes {
            let position = scope
                .iter()
                .position(|(name, _)| name == &write.binding)
                .ok_or_else(|| fail("a write snapshot needs its actual local binding"))?;
            if !self
                .proof
                .state
                .iter()
                .any(|slot| slot.name == write.target && slot.mutable)
                || !targets.insert(&write.target)
                || self.names.contains(&write.binding)
                || order.is_some_and(|prior| position <= prior)
            {
                return Err(fail(
                    "parallel writes must retain distinct mutable slots and actual evaluation order",
                ));
            }
            if self.proof.language == "JavaScript" && !self.snapshots.contains(&write.binding) {
                return Err(fail("a JavaScript write needs its exact snapshot binding"));
            }
            self.used_snapshots.insert(write.binding.clone());
            bindings.insert(write.binding.as_str());
            order = Some(position);
        }
        for binding in &bindings {
            let value = scope
                .iter()
                .find(|(name, _)| name.as_str() == *binding)
                .and_then(|(_, value)| *value)
                .ok_or_else(|| fail("snapshots cannot name parameter state"))?;
            self.prior_state(value, scope, &bindings, &mut HashSet::new())?;
        }
        for ((arg, slot), claimed) in args
            .iter()
            .zip(&self.proof.state)
            .zip(&transition.arguments)
        {
            let name = transition
                .writes
                .iter()
                .find(|write| write.target == slot.name)
                .map_or(slot.name.as_str(), |write| write.binding.as_str());
            let claimed_matches_slot = claimed == name;
            let argument_matches_slot = simple_name(arg) == Some(name);
            if !(claimed_matches_slot && argument_matches_slot) {
                return Err(fail(
                    "continuation arguments disagree with the complete ordered write proof",
                ));
            }
        }
        Ok(())
    }
    fn prior_state(
        &mut self,
        value: &'a Value,
        scope: &Scope<'a>,
        bindings: &HashSet<&str>,
        active: &mut HashSet<String>,
    ) -> Result<()> {
        self.visit()?;
        if let Some(name) = simple_name(value) {
            if bindings.contains(name) {
                return Err(fail(
                    "every parallel write reads the previous state snapshot",
                ));
            }
            if let Some(inner) = scope
                .iter()
                .find(|(binding, _)| binding == name)
                .and_then(|(_, value)| *value)
            {
                if !active.insert(name.into()) {
                    return Err(fail("snapshot binding dependencies cannot cycle"));
                }
                self.prior_state(inner, scope, bindings, active)?;
                active.remove(name);
            }
        }
        match value {
            Value::Array(values) => {
                for child in values {
                    self.prior_state(child, scope, bindings, active)?;
                }
            }
            Value::Object(values) => {
                for child in values.values() {
                    self.prior_state(child, scope, bindings, active)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
