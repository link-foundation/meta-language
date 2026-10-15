//! Rendering and observation contracts over the actual complete owned graph.
use super::parser::{single, token_name};
use super::{Parsed, fail, policy, position, source_interval};
use crate::translation::Language;
use crate::translation::diagnostics::Result;
use crate::translation::emit_common::{Assumption, Emitted, Encoding, Mapping};
use crate::translation::ir::{Decl, Program};
use serde_json::Value;

fn assumptions(parsed: &Parsed) -> Result<Vec<Assumption>> {
    policy()["assumptions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let label = row["operation"].as_str().unwrap();
            let operation = single(&parsed.operations, label)?;
            let (start, end) = source_interval(&operation["span"], true)?;
            Ok(Assumption {
                id: row["id"].as_str().unwrap().into(),
                statement: row["meaning"].as_str().unwrap().into(),
                details: vec![format!("source operation {label} at {start}..{end}")],
            })
        })
        .collect()
}
pub fn record_assumptions(emitted: &mut Emitted, parsed: &Parsed) -> Result<()> {
    emitted.assumptions.extend(assumptions(parsed)?);
    emitted.encodings.insert(0,Encoding{id:"closed-pure-rust-runtime-shell".into(),statement:format!("the complete represented source runtime is interpreted under {}; only ordered pure Unit helper calls, empty output and normal completion are projected, with all source runtime operations retained as links",policy()["observationModel"].as_str().unwrap())});
    Ok(())
}
fn render(node: &Value, tokens: &[Value], program: &Program) -> Result<String> {
    fn collect(node: &Value, found: &mut Vec<usize>) -> Result<()> {
        if node["kind"] == "source operation" {
            for child in node["parts"]
                .as_array()
                .ok_or_else(|| fail("an owned operation has no ordered parts"))?
            {
                collect(child, found)?;
            }
        } else {
            found.push(position(node, "tokenPosition")?);
        }
        Ok(())
    }
    let mut ordered = Vec::new();
    collect(node, &mut ordered)?;
    let mut result = String::new();
    let mut previous: Option<&Value> = None;
    for index in ordered {
        let token = tokens
            .get(index)
            .ok_or_else(|| fail("an owned operation references a missing source token"))?;
        if token["kind"] == "eof" {
            continue;
        }
        if let Some(previous) = previous {
            let word =
                |token: &Value| matches!(token["kind"].as_str(), Some("identifier" | "number"));
            if source_interval(&previous["span"], true)?.1
                != source_interval(&token["span"], true)?.0
                || word(previous) && word(token)
            {
                result.push(' ');
            }
        }
        if token["kind"] == "string" {
            result.push_str(
                &serde_json::to_string(token["value"].as_str().unwrap())
                    .map_err(|failure| fail(&failure.to_string()))?,
            );
        } else {
            result.push_str(&token_name(tokens, index, Some(program))?);
        }
        previous = Some(token);
    }
    Ok(result)
}
pub fn emit_rust(program: &Program, parsed: &Parsed) -> Result<Emitted> {
    let tokens = program
        .source_runtime_shell
        .as_ref()
        .and_then(|proof| proof["tokens"].as_array())
        .ok_or_else(|| fail("the complete runtime has no owned source tokens"))?;
    let attribute = render(
        single(&parsed.operations, "inert runtime lint attribute")?,
        tokens,
        program,
    )?;
    let helpers = parsed
        .helpers
        .iter()
        .map(|row| render(&row.node, tokens, program))
        .collect::<Result<Vec<_>>>()?;
    let worker = render(
        single(&parsed.operations, "owned Unit worker definition")?,
        tokens,
        program,
    )?;
    let entry = render(
        single(&parsed.operations, "complete standard runtime entry")?,
        tokens,
        program,
    )?;
    let mut sections = vec![attribute.clone()];
    sections.extend(helpers.iter().cloned());
    sections.push(worker);
    sections.push(entry);
    let mappings = program
        .declarations
        .iter()
        .filter_map(|entry| {
            if let Decl::Fn(function) = entry {
                Some(Mapping {
                    kind: "function".into(),
                    source: function.full_name.clone(),
                    target: function.name.clone(),
                    source_span: function.span,
                })
            } else {
                None
            }
        })
        .collect();
    Ok(Emitted{language:Language::Rust,text:sections.join("\n\n")+"\n",entry:Some("main".into()),preludes:vec![attribute],definitions:helpers,mappings,assumptions:assumptions(parsed)?,encodings:vec![Encoding{id:"closed-pure-rust-runtime-shell".into(),statement:"the actual complete owned operation graph is rendered; failure branches and resource arguments remain represented".into()}],theorems:Vec::new()})
}
