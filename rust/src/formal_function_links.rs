//! Closed function selection from an already supported linked formal model.
//! Original declaration, binding and recursive call identities are reused.
use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::formal_concept_definition::{
    FormalConceptArchive, FormalConceptDefinitionInput, FormalConceptDefinitionOptions,
    encode_formal_concept_definition,
};
use crate::formal_function_rules::{
    accept_formal_function_declaration, accept_formal_function_dependency,
    include_formal_function_relation,
};
use crate::translation::ir::{Decl, Expr, Item, Main, Node};
use crate::typed_program_links::{
    decode_typed_program_links, encode_typed_program_links, typed_program_links_schema,
};
use crate::{LinkId, LinkNetwork, LinkType};

const ENTRY_NAME: &str = "selected_function_definition";

/// Select a declaration by its checked qualified name or actual owned link.
#[derive(Clone, Copy, Debug)]
pub enum FormalFunctionSelector<'a> {
    /// Qualified declaration map key, used only to locate an owned identity.
    Name(&'a str),
    /// An actual declaration identity in the selected program.
    Identity(LinkId),
}

fn fail(message: &str) -> String {
    format!("formal function: {message}")
}

#[derive(Clone)]
struct Relation {
    label: String,
    child: LinkId,
}
type Fields = BTreeMap<LinkId, Vec<Relation>>;

fn relations(network: &LinkNetwork) -> Fields {
    let mut result: Fields = BTreeMap::new();
    for link in network.links() {
        if link.metadata().link_type() != Some(LinkType::Field) {
            continue;
        }
        let [parent, label, child] = link.references() else {
            continue;
        };
        if let Some(label) = network.link(*label).and_then(|link| link.metadata().term()) {
            result.entry(*parent).or_default().push(Relation {
                label: label.to_owned(),
                child: *child,
            });
        }
    }
    result
}

fn directory(
    network: &LinkNetwork,
    root: LinkId,
    fields: &Fields,
) -> Result<(LinkId, BTreeMap<String, LinkId>), String> {
    let body = *network
        .link(root)
        .and_then(|link| link.references().first())
        .ok_or_else(|| fail("missing checked program root"))?;
    let label = typed_program_links_schema()["fields"]["declarations"]
        .as_str()
        .ok_or_else(|| fail("missing declaration mapping schema"))?;
    let declarations = fields
        .get(&body)
        .and_then(|fields| fields.iter().find(|relation| relation.label == label))
        .ok_or_else(|| fail("missing declaration mapping"))?
        .child;
    let mut result = BTreeMap::new();
    for id in network
        .link(declarations)
        .ok_or_else(|| fail("missing declaration mapping"))?
        .references()
    {
        let [name, declaration] = network
            .link(*id)
            .ok_or_else(|| fail("missing declaration entry"))?
            .references()
        else {
            return Err(fail("invalid declaration entry"));
        };
        let name = network
            .link(*name)
            .and_then(|link| link.metadata().definition())
            .ok_or_else(|| fail("missing declaration key"))?;
        result.insert(name.to_owned(), *declaration);
    }
    Ok((body, result))
}

fn reachable(
    network: &LinkNetwork,
    root: LinkId,
    fields: &Fields,
    redirects: &BTreeMap<LinkId, LinkId>,
    semantic: bool,
) -> Result<BTreeSet<LinkId>, String> {
    let mut seen = BTreeSet::new();
    let mut pending = vec![root];
    let mut at = 0;
    while at < pending.len() {
        let id = pending[at];
        at += 1;
        if seen.contains(&id) || redirects.contains_key(&id) {
            continue;
        }
        if seen.len() >= 100_000 || pending.len() > 100_000 {
            return Err(fail("reference resource limit exceeded"));
        }
        seen.insert(id);
        pending.extend_from_slice(
            network
                .link(id)
                .ok_or_else(|| fail("dangling selected reference"))?
                .references(),
        );
        for relation in fields.get(&id).into_iter().flatten() {
            if !semantic || include_formal_function_relation(&relation.label) {
                pending.push(relation.child);
            }
        }
    }
    Ok(seen)
}

fn import_selection(
    original: &LinkNetwork,
    generated: &LinkNetwork,
    root: LinkId,
    redirects: &BTreeMap<LinkId, LinkId>,
) -> Result<(LinkNetwork, LinkId), String> {
    let fields = relations(generated);
    let selected = reachable(generated, root, &fields, redirects, false)?;
    let mut ids = redirects.clone();
    let mut network = original.clone();
    for id in &selected {
        ids.insert(
            *id,
            network.insert_link(
                [],
                generated
                    .link(*id)
                    .ok_or_else(|| fail("missing generated selection node"))?
                    .metadata()
                    .clone(),
            ),
        );
    }
    for id in &selected {
        let references = generated
            .link(*id)
            .unwrap()
            .references()
            .iter()
            .map(|id| ids[id])
            .collect::<Vec<_>>();
        network.set_references(ids[id], &references);
        for relation in fields.get(id).into_iter().flatten() {
            network.insert_field(ids[id], &relation.label, ids[&relation.child]);
        }
    }
    Ok((network, ids[&root]))
}

fn signature_field(fields: &Fields, id: LinkId, name: &str) -> Result<LinkId, String> {
    let label = typed_program_links_schema()["fields"]
        .get(name)
        .and_then(Value::as_str)
        .unwrap_or(name);
    fields
        .get(&id)
        .and_then(|fields| fields.iter().find(|relation| relation.label == label))
        .map(|relation| relation.child)
        .ok_or_else(|| fail("missing selected signature field"))
}

fn redirect_signature(
    original: &LinkNetwork,
    chosen: LinkId,
    generated: &LinkNetwork,
    wrapper: LinkId,
    redirects: &mut BTreeMap<LinkId, LinkId>,
) -> Result<(), String> {
    fn redirect(
        redirects: &mut BTreeMap<LinkId, LinkId>,
        from: LinkId,
        to: LinkId,
    ) -> Result<(), String> {
        if redirects.get(&from).is_some_and(|previous| *previous != to) {
            return Err(fail("the generated signature changes shared type identity"));
        }
        redirects.insert(from, to);
        Ok(())
    }
    let first = relations(original);
    let second = relations(generated);
    let ret = signature_field(&first, chosen, "ret")?;
    redirect(redirects, signature_field(&second, wrapper, "ret")?, ret)?;
    let body = signature_field(&second, wrapper, "body")?;
    redirect(redirects, signature_field(&second, body, "type")?, ret)?;
    let original_params = original
        .link(signature_field(&first, chosen, "params")?)
        .ok_or_else(|| fail("missing selected parameters"))?
        .references();
    let generated_params = generated
        .link(signature_field(&second, wrapper, "params")?)
        .ok_or_else(|| fail("missing generated parameters"))?
        .references();
    let arguments = generated
        .link(signature_field(&second, body, "args")?)
        .ok_or_else(|| fail("missing generated arguments"))?
        .references();
    if original_params.len() != generated_params.len() || original_params.len() != arguments.len() {
        return Err(fail("the generated signature changes parameter arity"));
    }
    for (index, parameter) in original_params.iter().enumerate() {
        for name in ["type", "guard", "default"] {
            if let (Ok(target), Ok(origin)) = (
                signature_field(&first, *parameter, name),
                signature_field(&second, generated_params[index], name),
            ) {
                redirect(redirects, origin, target)?;
            }
        }
        redirect(
            redirects,
            signature_field(&second, arguments[index], "type")?,
            signature_field(&first, *parameter, "type")?,
        )?;
    }
    Ok(())
}

/// Select a flat owned function and preserve its exact closed recursive context.
///
/// The generated typed call wrapper names the entry under the existing checked
/// program model. Unrelated declarations and main effects stay outside meaning.
///
/// # Errors
/// Refuses unsupported formal bases, foreign selectors, captured externals,
/// non-flat contexts, reserved name collisions and resource exhaustion.
pub fn encode_formal_function_definition(
    network: &LinkNetwork,
    root: LinkId,
    selector: FormalFunctionSelector<'_>,
    provenance: Option<&Value>,
) -> Result<FormalConceptArchive, String> {
    encode_formal_concept_definition(
        FormalConceptDefinitionInput::LinkedProgram { network, root },
        FormalConceptDefinitionOptions::default(),
    )?;
    let mut program =
        decode_typed_program_links(network, root).map_err(|error| error.to_string())?;
    if !program.imports.is_empty() {
        return Err(fail("imported declaration contexts are unsupported"));
    }
    let fields = relations(network);
    let (body, input) = directory(network, root, &fields)?;
    let name = match selector {
        FormalFunctionSelector::Name(name) => name.to_owned(),
        FormalFunctionSelector::Identity(id) => input
            .iter()
            .find(|(_, value)| **value == id)
            .map(|(name, _)| name.clone())
            .ok_or_else(|| fail("the selected identity is not an owned function declaration"))?,
    };
    let chosen = program
        .declarations
        .iter()
        .find(|definition| definition.full_name() == name)
        .ok_or_else(|| fail("the selected identity is not an owned function declaration"))?;
    let chosen = match chosen {
        Decl::Fn(definition) if accept_formal_function_declaration("fn") => definition.clone(),
        _ => {
            return Err(fail(
                "the selected identity is not an owned function declaration",
            ));
        }
    };
    if chosen
        .params
        .iter()
        .any(|parameter| parameter.guard.is_some() || parameter.default_value.is_some())
    {
        return Err(fail(
            "parameter guards and defaults need a separate wrapper context proof",
        ));
    }
    let selected = reachable(
        network,
        *input
            .get(&name)
            .ok_or_else(|| fail("missing selected declaration identity"))?,
        &fields,
        &BTreeMap::new(),
        true,
    )?;
    if program
        .declarations
        .iter()
        .any(|definition| definition.full_name() == ENTRY_NAME)
    {
        return Err(fail(
            "the reserved function definition entry name is already declared",
        ));
    }
    if let Some(external) = fields
        .get(&body)
        .and_then(|fields| fields.iter().find(|relation| relation.label == "externals"))
        && network
            .link(external.child)
            .ok_or_else(|| fail("missing external declarations"))?
            .references()
            .iter()
            .any(|id| selected.contains(id))
    {
        return Err(fail("captured external definitions are unsupported"));
    }
    program.declarations.retain(|definition| {
        input
            .get(definition.full_name())
            .is_some_and(|id| selected.contains(id))
    });
    for definition in &program.declarations {
        let kind = match definition {
            Decl::Fn(_) => "fn",
            Decl::Data(_) => "data",
            Decl::Theorem(_) => "theorem",
        };
        if !accept_formal_function_dependency(kind) || !definition.module_path().is_empty() {
            return Err(fail(
                "the function closure has an unsupported declaration context",
            ));
        }
    }
    let mut wrapper = chosen.clone();
    ENTRY_NAME.clone_into(&mut wrapper.name);
    ENTRY_NAME.clone_into(&mut wrapper.full_name);
    wrapper.module_path.clear();
    wrapper.body = Expr::new(
        Node::Call {
            func: chosen.full_name,
            args: chosen
                .params
                .iter()
                .map(|parameter| {
                    Expr::new(
                        Node::Var {
                            name: parameter.name.clone(),
                        },
                        parameter.ty.clone(),
                    )
                })
                .collect(),
        },
        chosen.ret,
    );
    wrapper.recursive = false;
    wrapper.decreasing = None;
    wrapper.generated = true;
    wrapper.mutual = false;
    wrapper.span = None;
    wrapper.source_signature = None;
    program.declarations.push(Decl::Fn(wrapper));
    program.items = program
        .declarations
        .iter()
        .map(|definition| Item::Decl {
            full_name: definition.full_name().to_owned(),
        })
        .collect();
    program.externals.clear();
    program.main = Some(Main {
        effects: vec![],
        span: None,
        sequential_async: false,
    });
    let generated = encode_typed_program_links(&program).map_err(|error| error.to_string())?;
    let (_, prepared) = directory(
        &generated.network,
        generated.root,
        &relations(&generated.network),
    )?;
    let mut redirects = program
        .declarations
        .iter()
        .filter(|definition| definition.full_name() != ENTRY_NAME)
        .map(|definition| {
            Ok((
                *prepared
                    .get(definition.full_name())
                    .ok_or_else(|| fail("missing generated declaration identity"))?,
                *input
                    .get(definition.full_name())
                    .ok_or_else(|| fail("missing original declaration identity"))?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    redirect_signature(
        network,
        input[&name],
        &generated.network,
        prepared[ENTRY_NAME],
        &mut redirects,
    )?;
    let (network, root) =
        import_selection(network, &generated.network, generated.root, &redirects)?;
    encode_formal_concept_definition(
        FormalConceptDefinitionInput::LinkedProgram {
            network: &network,
            root,
        },
        FormalConceptDefinitionOptions {
            provenance: provenance.cloned(),
            ..FormalConceptDefinitionOptions::default()
        },
    )
}
