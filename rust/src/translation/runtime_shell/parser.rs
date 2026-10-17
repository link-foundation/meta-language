//! Host cursor over the shared linked whole-runtime syntax, never a source slice.
use super::{fail, policy, position, source_interval};
use crate::translation::diagnostics::{Result, unsupported};
use crate::translation::ir::{Decl, Program};
use crate::translation::runtime_shell_rules::{
    accept_rust_runtime_shell_reserved_name, accept_rust_runtime_shell_stack_shift,
};
use crate::translation::rust::pure_unit_call_rules::{
    accept_rust_pure_unit_name, accept_rust_pure_unit_name_character,
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

fn source_fail(
    detail: &str,
    span: Option<&Value>,
) -> crate::translation::diagnostics::TranslationError {
    unsupported(
        "closed Rust runtime shell",
        detail,
        span.and_then(|span| serde_json::from_value(span.clone()).ok()),
    )
}

#[derive(Clone)]
pub(super) struct Helper {
    pub name: usize,
    pub node: Value,
    pub value: Value,
}
pub struct Parsed {
    pub(super) operations: Value,
    pub(super) helpers: Vec<Helper>,
    pub(super) helper_uses: Vec<(usize, usize)>,
    pub(super) local_uses: Vec<(usize, usize)>,
    pub(super) surface: Value,
}
struct Cursor<'a> {
    tokens: &'a [Value],
    program: Option<&'a Program>,
    index: usize,
    captures: BTreeMap<String, usize>,
    functions: BTreeMap<String, usize>,
    helpers: Vec<Helper>,
    helper_uses: Vec<(usize, usize)>,
    local_uses: Vec<(usize, usize)>,
    statements: Vec<(usize, Value, Value)>,
    locals: BTreeSet<String>,
    entry_locals: BTreeSet<String>,
}
fn nodes<'a>(node: &'a Value, label: &str, found: &mut Vec<&'a Value>) {
    if node["kind"] == "source operation" {
        if node["op"] == label {
            found.push(node);
        }
        if let Some(parts) = node["parts"].as_array() {
            for child in parts {
                nodes(child, label, found);
            }
        }
    }
}
pub(super) fn single<'a>(root: &'a Value, label: &str) -> Result<&'a Value> {
    let mut found = Vec::new();
    nodes(root, label, &mut found);
    if found.len() != 1 {
        return Err(fail(&format!(
            "the complete runtime requires one owned {label}"
        )));
    }
    Ok(found[0])
}
pub(super) fn token_name(
    tokens: &[Value],
    index: usize,
    program: Option<&Program>,
) -> Result<String> {
    let token = tokens
        .get(index)
        .ok_or_else(|| fail("an owned token position is invalid"))?;
    if let Some(target) = token.get("declaration") {
        let index = position(target, "declarationPosition")?;
        let Some(Decl::Fn(function)) = program.and_then(|program| program.declarations.get(index))
        else {
            return Err(fail(
                "a source occurrence references an unowned declaration",
            ));
        };
        return Ok(function.name.clone());
    }
    if let Some(target) = token.get("binding") {
        let index = position(target, "bindingPosition")?;
        return tokens
            .get(index)
            .and_then(|token| token["value"].as_str())
            .map(str::to_owned)
            .ok_or_else(|| fail("a source occurrence references an unowned lexical binder"));
    }
    token["value"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| fail("the actual token scalar has a different lexical domain"))
}
fn ordinary(name: &str, local: bool) -> bool {
    let prefix = policy()["generatedLocalPrefix"].as_str().unwrap();
    let generated = local
        && name.strip_prefix(prefix).is_some_and(|suffix| {
            !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
        });
    (generated || accept_rust_pure_unit_name(name))
        && accept_rust_runtime_shell_reserved_name(name)
        && name.chars().enumerate().all(|(index, letter)| {
            accept_rust_pure_unit_name_character(f64::from(u32::from(letter)), index == 0)
        })
}
impl Cursor<'_> {
    fn take(&mut self) -> Result<usize> {
        let index = self.index;
        if index >= self.tokens.len() {
            return Err(fail("the complete source token sequence is missing"));
        }
        self.index += 1;
        Ok(index)
    }
    fn token(&self, node: &Value) -> Result<&Value> {
        let index = position(node, "tokenPosition")?;
        self.tokens
            .get(index)
            .ok_or_else(|| fail("an owned token position is invalid"))
    }
    fn match_pattern(&mut self, descriptor: &Value) -> Result<Value> {
        if let Some(expected) = descriptor.as_str() {
            let index = self.take()?;
            let token = &self.tokens[index];
            let kind = if expected.bytes().all(|byte| byte.is_ascii_digit()) {
                "number"
            } else if expected.bytes().enumerate().all(|(index, byte)| {
                byte.is_ascii_alphabetic() || byte == b'_' || index > 0 && byte.is_ascii_digit()
            }) {
                "identifier"
            } else {
                "punct"
            };
            if token["kind"] != kind
                || token["value"] != expected
                || token.get("declaration").is_some()
                || token.get("binding").is_some()
                || kind == "number" && (token["suffix"] != "" || token["floating"] != false)
            {
                return Err(source_fail(
                    &format!("the owned runtime grammar expected {expected}"),
                    Some(&token["span"]),
                ));
            }
            return Ok(json!({"tokenPosition":index}));
        }
        if let Some(label) = descriptor["operation"].as_str() {
            let begin = self.index;
            let mut parts = Vec::new();
            for child in descriptor["pattern"]
                .as_array()
                .ok_or_else(|| fail("the linked runtime grammar requires an ordered pattern"))?
            {
                parts.push(self.match_pattern(child)?);
            }
            let node = json!({"kind":"source operation","op":label,"parts":parts,"span":self.range(begin,self.index)?});
            if label == "owned public Unit helper" {
                let name = *self
                    .captures
                    .get("helper")
                    .ok_or_else(|| fail("a helper has no owned header"))?;
                let row = Helper {
                    name,
                    node: node.clone(),
                    value: single(&node, "pure Unit helper value")?.clone(),
                };
                self.functions.insert(
                    token_name(self.tokens, name, self.program)?,
                    self.helpers.len(),
                );
                self.helpers.push(row);
            }
            if label == "owned Unit call binding" {
                self.statements.push((
                    *self
                        .captures
                        .get("unitBinding")
                        .ok_or_else(|| fail("a Unit call has no owned binding"))?,
                    node.clone(),
                    single(&node, "owned Unit invocation")?.clone(),
                ));
            }
            return Ok(node);
        }
        if let Some(slot) = descriptor["bind"].as_str() {
            let index = self.take()?;
            let token = &self.tokens[index];
            let name = token_name(self.tokens, index, self.program)?;
            if token["kind"] != "identifier"
                || descriptor
                    .get("fixed")
                    .is_some_and(|fixed| fixed.as_str() != Some(name.as_str()))
                || descriptor.get("fixed").is_none() && !ordinary(&name, slot == "unitBinding")
            {
                return Err(source_fail(
                    &format!(
                        "an actual owned {slot} identifier is outside its closed source interpretation"
                    ),
                    Some(&token["span"]),
                ));
            }
            if slot == "helper" {
                if self.program.is_some() != token.get("declaration").is_some()
                    || self.program.is_none()
                        && (token.get("value").is_none() || token.get("binding").is_some())
                {
                    return Err(source_fail(
                        "a helper header must reference its actual owned declaration",
                        Some(&token["span"]),
                    ));
                }
                if self.functions.contains_key(&name) {
                    return Err(source_fail(
                        "each source helper declaration must have one distinct identity",
                        Some(&token["span"]),
                    ));
                }
            } else {
                if token.get("value").is_none()
                    || token.get("binding").is_some()
                    || token.get("declaration").is_some()
                {
                    return Err(source_fail(
                        "a local or worker binder must retain its actual declaration token",
                        Some(&token["span"]),
                    ));
                }
                let scope = if slot == "unitBinding" {
                    &mut self.locals
                } else {
                    &mut self.entry_locals
                };
                if !scope.insert(name) {
                    return Err(source_fail(
                        "the owned runtime binding is duplicated or shadowed",
                        Some(&token["span"]),
                    ));
                }
            }
            self.captures.insert(slot.into(), index);
            return Ok(json!({"tokenPosition":index}));
        }
        if let Some(slot) = descriptor["reference"].as_str() {
            let target = *self
                .captures
                .get(slot)
                .ok_or_else(|| fail("the runtime reference has no lexical binding"))?;
            let index = self.take()?;
            let token = &self.tokens[index];
            if token["kind"] != "identifier"
                || token_name(self.tokens, index, self.program)?
                    != token_name(self.tokens, target, self.program)?
                || if self.program.is_some() {
                    position(&token["binding"], "bindingPosition")? != target
                } else {
                    token.get("value").is_none()
                        || token.get("binding").is_some()
                        || token.get("declaration").is_some()
                }
            {
                return Err(source_fail(
                    "the runtime reference does not target its actual lexical binding",
                    Some(&token["span"]),
                ));
            }
            self.local_uses.push((index, target));
            return Ok(json!({"tokenPosition":index}));
        }
        if descriptor["helperReference"] == true {
            let index = self.take()?;
            let token = &self.tokens[index];
            let name = token_name(self.tokens, index, self.program)?;
            let row = self
                .functions
                .get(&name)
                .and_then(|index| self.helpers.get(*index))
                .ok_or_else(|| {
                    source_fail(
                        "the worker call must target an actual owned pure Unit helper",
                        Some(&token["span"]),
                    )
                })?;
            if token["kind"] != "identifier"
                || if self.program.is_some() {
                    token["declaration"] != self.tokens[row.name]["declaration"]
                } else {
                    token.get("value").is_none()
                        || token.get("binding").is_some()
                        || token.get("declaration").is_some()
                }
            {
                return Err(source_fail(
                    "the worker call must target an actual owned pure Unit helper",
                    Some(&token["span"]),
                ));
            }
            self.helper_uses.push((index, row.name));
            return Ok(json!({"tokenPosition":index}));
        }
        if let Some(expected) = descriptor.get("string") {
            let index = self.take()?;
            if self.tokens[index]["kind"] != "string" || self.tokens[index]["value"] != *expected {
                return Err(source_fail(
                    "the actual runtime string literal differs from its closed operation",
                    Some(&self.tokens[index]["span"]),
                ));
            }
            return Ok(json!({"tokenPosition":index}));
        }
        if let Some(slot) = descriptor["number"].as_str() {
            let index = self.take()?;
            let token = &self.tokens[index];
            let text = token["value"].as_str().unwrap_or_default();
            let number = text.parse::<u32>().ok();
            if token["kind"] != "number"
                || token["suffix"] != ""
                || token["floating"] != false
                || !number.is_some_and(|number| {
                    number.to_string() == text
                        && accept_rust_runtime_shell_stack_shift(f64::from(number))
                })
            {
                return Err(source_fail(
                    "the stack shift requires its actual bounded unsigned integer argument",
                    Some(&token["span"]),
                ));
            }
            self.captures.insert(slot.into(), index);
            return Ok(json!({"tokenPosition":index}));
        }
        if let Some(label) = descriptor["repeat"].as_str() {
            let begin = self.index;
            let mut parts = Vec::new();
            loop {
                let next = token_name(self.tokens, self.index, self.program)?;
                let again = descriptor.get("while").map_or_else(
                    || descriptor["until"].as_str() != Some(next.as_str()),
                    |value| value.as_str() == Some(next.as_str()),
                );
                if !again {
                    break;
                }
                let before = self.index;
                let rule = descriptor["rule"]
                    .as_str()
                    .ok_or_else(|| fail("a repeated runtime operation has no linked rule"))?;
                parts.push(self.match_pattern(&policy()["syntax"][rule])?);
                if self.index <= before || self.index >= self.tokens.len() {
                    return Err(source_fail(
                        "the complete runtime sequence is not closed",
                        self.tokens.get(self.index).map(|token| &token["span"]),
                    ));
                }
            }
            return Ok(
                json!({"kind":"source operation","op":label,"parts":parts,"span":self.range(begin,self.index)?}),
            );
        }
        if descriptor["eof"] == true {
            let index = self.take()?;
            if self.tokens[index]["kind"] != "eof"
                || self.tokens[index]["value"] != ""
                || self.index != self.tokens.len()
            {
                return Err(source_fail(
                    "the runtime interpretation must consume the whole artifact",
                    Some(&self.tokens[index]["span"]),
                ));
            }
            return Ok(json!({"tokenPosition":index}));
        }
        Err(fail(
            "the linked runtime grammar contains an unrecognized operation",
        ))
    }
    fn range(&self, begin: usize, end: usize) -> Result<Value> {
        let first = self
            .tokens
            .get(begin)
            .ok_or_else(|| fail("a source operation has no lexical boundary"))?;
        let (start, _) = source_interval(&first["span"], true)?;
        let finish = if end > begin {
            source_interval(&self.tokens[end - 1]["span"], true)?.1
        } else {
            start
        };
        Ok(json!({"start":start,"end":finish}))
    }
}
pub(super) fn read_syntax(tokens: &[Value], program: Option<&Program>) -> Result<Parsed> {
    let mut cursor = Cursor {
        tokens,
        program,
        index: 0,
        captures: BTreeMap::new(),
        functions: BTreeMap::new(),
        helpers: Vec::new(),
        helper_uses: Vec::new(),
        local_uses: Vec::new(),
        statements: Vec::new(),
        locals: BTreeSet::new(),
        entry_locals: BTreeSet::new(),
    };
    let operations = cursor.match_pattern(&policy()["syntax"]["crate"])?;
    if cursor.index != tokens.len() {
        return Err(fail(
            "the selected runtime left unconsumed source operations",
        ));
    }
    let shift = single(&operations, "unsigned stack size shift")?;
    if cursor.token(&shift["parts"][1])?["span"]["end"]
        != cursor.token(&shift["parts"][2])?["span"]["start"]
    {
        return Err(source_fail(
            "the stack request must own the actual contiguous left-shift operator",
            Some(&shift["span"]),
        ));
    }
    let items=cursor.helpers.iter().map(|row|Ok(json!({"k":"fn","name":token_name(tokens,row.name,program)?,"params":[],"ret":{"kind":"unit"},"body":{"k":"unit","span":row.value["span"]},"span":row.node["span"]}))).collect::<Result<Vec<_>>>()?;
    let effects=cursor.statements.iter().map(|(name,node,call)|{
        let target=position(&call["parts"][2],"tokenPosition")?;let first=cursor.token(&call["parts"][0])?;
        Ok(json!({"k":"let","name":token_name(tokens,*name,program)?,"type":{"kind":"unit"},"value":{"k":"app","fn":{"k":"name","path":["crate",token_name(tokens,target,program)?],"span":{"start":first["span"]["start"],"end":tokens[target]["span"]["end"]}},"args":[],"span":call["span"]},"span":node["span"]}))
    }).collect::<Result<Vec<_>>>()?;
    let worker = single(&operations, "owned Unit worker definition")?;
    let surface =
        json!({"language":"Rust","items":items,"main":{"effects":effects,"span":worker["span"]}});
    Ok(Parsed {
        operations,
        helpers: cursor.helpers,
        helper_uses: cursor.helper_uses,
        local_uses: cursor.local_uses,
        surface,
    })
}
