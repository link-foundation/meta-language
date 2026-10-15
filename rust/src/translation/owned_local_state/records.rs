//! Actual expression traversal in the JavaScript host's source-role order.
use super::{Result, Value, array, fail, read_owned_local_position};
use std::collections::BTreeSet;

#[derive(Clone)]
pub(super) struct Record {
    pub node: Value,
    pub path: Vec<Value>,
    pub binding: String,
}

#[derive(Default)]
pub(super) struct Records {
    pub bindings: Vec<Record>,
    pub reads: Vec<Record>,
    pub constructors: Vec<Record>,
    visits: usize,
}

pub(super) fn child(path: &[Value], field: &str) -> Vec<Value> {
    let mut next = path.to_vec();
    next.push(Value::String(field.to_owned()));
    next
}

pub(super) fn key(path: &Value) -> Result<String> {
    let parts = array(path, "a source fact needs its ordered occurrence path")?
        .iter()
        .map(|part| {
            if part.is_string() {
                Ok(part.clone())
            } else {
                read_owned_local_position(part, f64::INFINITY).map(Value::from)
            }
        })
        .collect::<Result<Vec<_>>>()?;
    serde_json::to_string(&parts)
        .map_err(|_| fail("a source fact needs its ordered occurrence path"))
}

impl Records {
    pub(super) fn collect(body: &Value) -> Result<Self> {
        let mut records = Self::default();
        records.visit(body, &[])?;
        Ok(records)
    }

    fn visit(&mut self, node: &Value, path: &[Value]) -> Result<()> {
        self.visits += 1;
        if self.visits > 100_000 || path.len() > 256 {
            return Err(fail("body exceeds its traversal limit"));
        }
        let record = |binding| Record {
            node: node.clone(),
            path: path.to_vec(),
            binding,
        };
        match node["k"].as_str() {
            Some("let") => {
                self.visit(&node["value"], &child(path, "value"))?;
                self.bindings.push(record(String::new()));
                self.visit(&node["body"], &child(path, "body"))?;
                return Ok(());
            }
            Some("var") if node["type"]["kind"] == "string" => {
                self.reads
                    .push(record(node["name"].as_str().unwrap_or_default().to_owned()));
                return Ok(());
            }
            Some("lit") if node["type"]["kind"] == "string" => {
                self.constructors.push(record(String::new()));
                return Ok(());
            }
            Some("tupleIndex") => {
                if node["type"]["kind"] == "string" && node["object"]["k"] == "var" {
                    self.reads.push(record(format!(
                        "{}.{}",
                        node["object"]["name"].as_str().unwrap_or_default(),
                        node["index"]
                    )));
                }
                return Ok(());
            }
            _ => {}
        }
        for field in ["arg", "left", "right", "cond", "then", "else"] {
            if let Some(value) = node.get(field) {
                self.visit(value, &child(path, field))?;
            }
        }
        for field in ["items", "args"] {
            if let Some(items) = node.get(field) {
                for (index, item) in array(items, "an expression needs its ordered item roles")?
                    .iter()
                    .enumerate()
                {
                    let mut next = child(path, field);
                    next.push(Value::from(index));
                    self.visit(item, &next)?;
                }
            }
        }
        Ok(())
    }
}

pub(super) fn prepare(proof: &Value, body: &Value) -> Result<Value> {
    let mut value = proof.clone();
    let records = Records::collect(body)?;
    let mut used = BTreeSet::new();
    for binding in value["bindings"]
        .as_array_mut()
        .ok_or_else(|| fail("state facts must retain their ordered roles"))?
    {
        if !binding["path"].is_null() {
            continue;
        }
        let found = records
            .bindings
            .iter()
            .filter(|record| record.node["name"] == binding["name"])
            .collect::<Vec<_>>();
        if found.len() != 1 {
            return Err(fail(
                "each local version needs its actual unique checked binder",
            ));
        }
        binding["path"] = Value::Array(found[0].path.clone());
    }
    for read in value["reads"]
        .as_array_mut()
        .ok_or_else(|| fail("state facts must retain their ordered roles"))?
    {
        if !read["path"].is_null() {
            continue;
        }
        let found = records
            .reads
            .iter()
            .find(|record| {
                !used.contains(&Value::Array(record.path.clone()).to_string())
                    && read["binding"] == record.binding
                    && record.node["span"] == read["span"]
            })
            .ok_or_else(|| {
                fail("every source move or copy needs its actual checked value occurrence")
            })?;
        let path = Value::Array(found.path.clone());
        used.insert(path.to_string());
        read["path"] = path;
    }
    let bindings = value["bindings"].clone();
    for initializer in value["initializers"]
        .as_array_mut()
        .ok_or_else(|| fail("state facts must retain their ordered roles"))?
    {
        if initializer["path"].is_null() {
            initializer["path"] = bindings
                .as_array()
                .and_then(|bindings| {
                    bindings
                        .iter()
                        .find(|binding| binding["name"] == initializer["binding"])
                })
                .map_or(Value::Null, |binding| binding["path"].clone());
        }
    }
    for constructor in value["literalConstructors"]
        .as_array_mut()
        .ok_or_else(|| fail("state facts must retain their ordered roles"))?
    {
        if !constructor["path"].is_null() {
            continue;
        }
        let found = records
            .constructors
            .iter()
            .find(|record| record.node["span"] == constructor["span"])
            .ok_or_else(|| {
                fail("a standard constructor needs its actual checked String literal occurrence")
            })?;
        constructor["path"] = Value::Array(found.path.clone());
    }
    Ok(value)
}
