//! Structured links encoding and source-free proof revalidation.

use std::collections::{BTreeMap, BTreeSet};

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};
use serde_json::{Value, json};

use super::{
    GLOBAL_CONCEPT_PROOF_METHOD, GlobalConceptOptions, array, canonical,
    create_global_concept_space_in, declarations, production, representative, string,
};
use crate::formal_concept_definition::read_formal_concept_definition;
use crate::global_concept_rules::verify_global_concept_correspondence;
use crate::grammar::Grammar;
use crate::grammar::interchange::{
    deserialize_grammar, parse_grammar_links, percent_decode_links_text, percent_encode_links_text,
    render_declaration_links, render_links_expression, render_rule_fields, serialize_grammar,
};
use crate::link_network::LinkNetwork;

fn definition_grammar(definition: &Value) -> Result<Grammar, String> {
    let mut rule = definition.clone();
    rule["name"] = json!("global_definition");
    deserialize_grammar(
        &json!({"schemaVersion":1,"start":"global_definition","sourceFormat":null,"rules":[rule]})
            .to_string(),
    )
    .map_err(|error| error.to_string())
}

/// Serializes every copied definition, qualified origin and proof as links.
///
/// # Errors
/// Returns a validation error for malformed concept space data.
pub fn global_concept_space_to_links(space: &Value) -> Result<String, String> {
    let word = |value: &Value| string(value).map(percent_encode_links_text);
    let mut lines = vec!["(global-concept-space (schema-version 1))".to_owned()];
    for source in array(&space["sources"])? {
        let id = word(&source["id"])?;
        let start = if source["start"].is_null() {
            "(start unspecified)".to_owned()
        } else {
            format!("(start defined {})", word(&source["start"])?)
        };
        lines.push(format!(
            "(grammar-origin {id} {} {} {start} (rule-order {}))",
            word(&source["language"])?,
            word(&source["edition"])?,
            array(&source["ruleOrder"])?
                .iter()
                .map(word)
                .collect::<Result<Vec<_>, _>>()?
                .join(" ")
        ));
        if !source["sourceFormat"].is_null() {
            lines.push(format!(
                "(grammar-source-format {id} {})",
                word(&source["sourceFormat"])?
            ));
        }
        let context = &source["context"];
        if !context["matching"].is_null() {
            lines.push(format!(
                "(grammar-context {id} (matching {}))",
                word(&context["matching"])?
            ));
        }
        if !context["settling"].is_null() {
            lines.push(format!(
                "(grammar-context {id} (settling {}))",
                array(&context["settling"])?
                    .iter()
                    .map(word)
                    .collect::<Result<Vec<_>, _>>()?
                    .join(" ")
            ));
        }
        let grammar = deserialize_grammar(&json!({"schemaVersion":1,"start":"global_context","rules":[{"name":"global_context","kind":"normal","expression":{"kind":"empty"}}],"declarations":context}).to_string()).map_err(|error| error.to_string())?;
        for declaration in render_declaration_links(grammar.declarations()) {
            lines.push(format!("(grammar-context {id} {declaration})"));
        }
    }
    if let Some(definitions) = space["formalDefinitions"].as_object() {
        for (alias, definition) in definitions {
            let root = definition["root"]
                .as_u64()
                .ok_or("a formal origin needs a root")?;
            let definition_text = string(&definition["links"])?;
            lines.push(format!(
                "(formal-origin-definition {} {root} (links {}))",
                percent_encode_links_text(alias),
                definition_text.trim().replace('\n', " ")
            ));
        }
    }
    for concept in array(&space["concepts"])? {
        let id = word(&concept["id"])?;
        lines.push(format!("(global-concept {id})"));
        for origin in array(&concept["origins"])? {
            let alias = word(&origin["alias"])?;
            let meaning = if origin["concept"].is_null() {
                String::new()
            } else {
                format!(" (meaning {})", word(&origin["concept"])?)
            };
            lines.push(format!("(concept-origin {id} {alias} (source {}) (language {}) (edition {}) (rule {}){meaning})", word(&origin["source"])?, word(&origin["language"])?, word(&origin["edition"])?, word(&origin["rule"])?));
            for source_alias in array(&origin["sourceNames"])? {
                lines.push(format!(
                    "(source-alias {alias} {} {})",
                    word(&source_alias["source"])?,
                    word(&source_alias["name"])?
                ));
            }
        }
        for definition in array(&concept["definitions"])? {
            let grammar = definition_grammar(&definition["definition"])?;
            let rule = grammar
                .rules()
                .first()
                .ok_or("missing concept definition")?;
            lines.push(format!(
                "(concept-definition {id} {} {} {} {})",
                word(&definition["origin"])?,
                rule.kind.as_str(),
                render_links_expression(&rule.expr),
                render_rule_fields(&rule.attributes).join(" ")
            ));
        }
    }
    for proof in array(&space["proofs"])? {
        let pairs = array(&proof["correspondence"])?
            .iter()
            .map(|pair| Ok(format!("({} {})", word(&pair[0])?, word(&pair[1])?)))
            .collect::<Result<Vec<_>, String>>()?;
        lines.push(format!(
            "(concept-proof {} {} {} (correspondence {}))",
            word(&proof["method"])?,
            word(&proof["first"])?,
            word(&proof["second"])?,
            pairs.join(" ")
        ));
    }
    for failure in array(&space["failures"])? {
        lines.push(format!(
            "(unproved-equivalence {} {})",
            array(&failure["members"])?
                .iter()
                .map(word)
                .collect::<Result<Vec<_>, _>>()?
                .join(" "),
            word(&failure["reason"])?
        ));
    }
    for ambiguity in array(&space["ambiguities"])? {
        lines.push(format!(
            "(ambiguous-origin {} {})",
            word(&ambiguity["alias"])?,
            array(&ambiguity["meanings"])?
                .iter()
                .map(word)
                .collect::<Result<Vec<_>, _>>()?
                .join(" ")
        ));
    }
    for (old, current) in space["redirects"]
        .as_object()
        .ok_or("invalid concept redirects")?
    {
        lines.push(format!(
            "(concept-redirect {} {})",
            percent_encode_links_text(old),
            word(current)?
        ));
    }
    Ok(format!("{}\n", lines.join("\n")))
}

fn link_atom(node: &LiNo<String>) -> Result<&str, String> {
    match node {
        LiNo::Ref(word) => Ok(word),
        LiNo::Link {
            id: Some(word),
            values,
        } if values.is_empty() => Ok(word),
        LiNo::Link { .. } => Err("expected a global concept word".to_owned()),
    }
}

fn link_word(node: &LiNo<String>) -> Result<String, String> {
    percent_decode_links_text(link_atom(node)?).map_err(|error| error.to_string())
}

fn link_parts(node: &LiNo<String>) -> Result<(&str, &[LiNo<String>]), String> {
    match node {
        LiNo::Link { id: None, values } if !values.is_empty() => {
            Ok((link_atom(&values[0])?, &values[1..]))
        }
        _ => Err("expected a structured global concept link".to_owned()),
    }
}

fn print_link(node: &LiNo<String>) -> Result<String, String> {
    match node {
        LiNo::Link { id: None, values } if !values.is_empty() => Ok(format!(
            "({})",
            values
                .iter()
                .map(print_link)
                .collect::<Result<Vec<_>, _>>()?
                .join(" ")
        )),
        _ => link_atom(node).map(str::to_owned),
    }
}

fn print_network_link(node: &LiNo<String>) -> String {
    match node {
        LiNo::Link { id, values } if !values.is_empty() => {
            let prefix = id.as_ref().map_or(String::new(), |id| format!("{id}: "));
            format!(
                "({prefix}{})",
                values
                    .iter()
                    .map(print_network_link)
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        }
        LiNo::Link { id, .. } => id.clone().unwrap_or_default(),
        LiNo::Ref(value) => value.clone(),
    }
}

/// Reloads structured links and rechecks all recorded shared concept proofs.
/// Original imported grammars are reconstructed from their linked definitions.
///
/// # Errors
/// Returns a validation error for malformed links or an unproved shared identity.
pub fn read_global_concept_space_links(
    source: &str,
    options: &GlobalConceptOptions,
) -> Result<Value, String> {
    read_validated_global_concept_archive(source, options).map(|archive| archive.space)
}

pub(super) struct ValidatedGlobalConceptArchive {
    pub(super) space: Value,
    pub(super) imported: Vec<Value>,
}

pub(super) fn read_validated_global_concept_archive(
    source: &str,
    options: &GlobalConceptOptions,
) -> Result<ValidatedGlobalConceptArchive, String> {
    let statements = parse_lino_to_links_with_config(source, &ParserConfig::without_comments())
        .map_err(|error| error.to_string())?;
    if statements.is_empty() {
        return Err("missing global concept space header".to_owned());
    }
    let mut space = json!({"schemaVersion":1,"sources":[],"concepts":[],"mappings":{},"proofs":[],"failures":[],"ambiguities":[],"redirects":{},"formalDefinitions":{}});
    let mut sources = BTreeMap::new();
    let mut concepts = BTreeMap::new();
    let mut origins: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut contexts: BTreeMap<String, Vec<LiNo<String>>> = BTreeMap::new();
    for (at, statement) in statements.iter().enumerate() {
        let (head, args) = link_parts(statement)?;
        if at == 0 {
            if head != "global-concept-space"
                || args.len() != 1
                || print_link(&args[0])? != "(schema-version 1)"
            {
                return Err("unsupported global concept space header".to_owned());
            }
            continue;
        }
        match head {
            "formal-origin-definition" => {
                if args.len() != 3 {
                    return Err("invalid or duplicated formal origin definition".to_owned());
                }
                let alias = link_word(&args[0])?;
                if space["formalDefinitions"].get(&alias).is_some() {
                    return Err("invalid or duplicated formal origin definition".to_owned());
                }
                let root = link_word(&args[1])?
                    .parse::<u64>()
                    .map_err(|_| "a formal origin needs a root".to_owned())?;
                let (tag, links) = link_parts(&args[2])?;
                if tag != "links" {
                    return Err("a formal origin needs actual linked definitions".to_owned());
                }
                let text = format!(
                    "{}\n",
                    links
                        .iter()
                        .map(print_network_link)
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                let network = LinkNetwork::from_lino(&text).map_err(|error| error.to_string())?;
                let definition = json!({"root":root,"links":network.to_lino()});
                read_formal_concept_definition(&definition)?;
                space["formalDefinitions"][alias] = definition;
            }
            "grammar-origin" => {
                if args.len() != 5 {
                    return Err(
                        "a grammar origin has source, language, edition, start and rule order"
                            .to_owned(),
                    );
                }
                let (start_tag, start_values) = link_parts(&args[3])?;
                if start_tag != "start" {
                    return Err("missing grammar start".to_owned());
                }
                let start = match start_values {
                    [policy] if link_atom(policy)? == "unspecified" => Value::Null,
                    [policy, rule] if link_atom(policy)? == "defined" => json!(link_word(rule)?),
                    _ => return Err("invalid grammar start".to_owned()),
                };
                let (order_tag, order_values) = link_parts(&args[4])?;
                if order_tag != "rule-order" {
                    return Err("missing grammar rule order".to_owned());
                }
                let rule_order = order_values
                    .iter()
                    .map(link_word)
                    .collect::<Result<Vec<_>, _>>()?;
                if rule_order.iter().collect::<BTreeSet<_>>().len() != rule_order.len() {
                    return Err("duplicated grammar rule order".to_owned());
                }
                let id = link_word(&args[0])?;
                if sources.contains_key(&id) {
                    return Err(format!("duplicate grammar origin {id}"));
                }
                let entries = space["sources"]
                    .as_array_mut()
                    .ok_or("invalid grammar sources")?;
                sources.insert(id.clone(), entries.len());
                contexts.insert(id.clone(), Vec::new());
                entries.push(json!({"id":id,"language":link_word(&args[1])?,"edition":link_word(&args[2])?,"start":start,"ruleOrder":rule_order,"context":null}));
            }
            "grammar-source-format" => {
                if args.len() != 2 {
                    return Err("a grammar source format has an origin and one format".to_owned());
                }
                let id = link_word(&args[0])?;
                let index = *sources
                    .get(&id)
                    .ok_or("unknown or duplicate grammar source format origin")?;
                if space["sources"][index].get("sourceFormat").is_some() {
                    return Err("unknown or duplicate grammar source format origin".to_owned());
                }
                space["sources"][index]["sourceFormat"] = json!(link_word(&args[1])?);
            }
            "grammar-context" => {
                if args.len() != 2 {
                    return Err("unknown grammar context origin".to_owned());
                }
                contexts
                    .get_mut(&link_word(&args[0])?)
                    .ok_or("unknown grammar context origin")?
                    .push(args[1].clone());
            }
            "global-concept" => {
                if args.len() != 1 {
                    return Err("a global concept has one identity".to_owned());
                }
                let id = link_word(&args[0])?;
                if concepts.contains_key(&id) {
                    return Err(format!("duplicate global concept {id}"));
                }
                let entries = space["concepts"]
                    .as_array_mut()
                    .ok_or("invalid global concepts")?;
                concepts.insert(id.clone(), entries.len());
                entries.push(
                    json!({"id":id,"origins":[],"definition":null,"definitions":[],"proofs":[]}),
                );
            }
            "concept-origin" => {
                if !(6..=7).contains(&args.len()) {
                    return Err(
                        "a concept origin has identity, alias and qualification fields".to_owned(),
                    );
                }
                let id = link_word(&args[0])?;
                let alias = link_word(&args[1])?;
                let index = *concepts
                    .get(&id)
                    .ok_or("unknown concept or duplicate origin")?;
                if origins.contains_key(&alias) {
                    return Err("unknown concept or duplicate origin".to_owned());
                }
                let mut fields = BTreeMap::new();
                for field in &args[2..] {
                    let (key, values) = link_parts(field)?;
                    if !["source", "language", "edition", "rule", "meaning"].contains(&key)
                        || values.len() != 1
                        || fields.contains_key(key)
                    {
                        return Err(format!("invalid concept origin field {key}"));
                    }
                    fields.insert(key.to_owned(), link_word(&values[0])?);
                }
                for key in ["source", "language", "edition", "rule"] {
                    if !fields.contains_key(key) {
                        return Err("incomplete concept origin qualification".to_owned());
                    }
                }
                let grammar = &space["sources"][*sources
                    .get(&fields["source"])
                    .ok_or("inconsistent concept origin qualification")?];
                if grammar["language"] != fields["language"]
                    || grammar["edition"] != fields["edition"]
                    || alias != format!("{}:{}", fields["source"], fields["rule"])
                {
                    return Err("inconsistent concept origin qualification".to_owned());
                }
                let entries = space["concepts"][index]["origins"]
                    .as_array_mut()
                    .ok_or("invalid concept origins")?;
                origins.insert(alias.clone(), (index, entries.len()));
                entries.push(json!({"alias":alias,"source":fields["source"],"language":fields["language"],"edition":fields["edition"],"rule":fields["rule"],"concept":fields.get("meaning"),"sourceNames":[]}));
                space["mappings"][alias] = json!(id);
            }
            "source-alias" => {
                if args.len() != 3 {
                    return Err("unknown source alias origin".to_owned());
                }
                let (concept, origin) = *origins
                    .get(&link_word(&args[0])?)
                    .ok_or("unknown source alias origin")?;
                space["concepts"][concept]["origins"][origin]["sourceNames"]
                    .as_array_mut()
                    .ok_or("invalid source aliases")?
                    .push(json!({"source":link_word(&args[1])?,"name":link_word(&args[2])?}));
            }
            "concept-definition" => {
                if args.len() < 4 {
                    return Err("incomplete concept definition".to_owned());
                }
                let id = link_word(&args[0])?;
                let alias = link_word(&args[1])?;
                let index = *concepts
                    .get(&id)
                    .ok_or("unknown or duplicated concept definition origin")?;
                if space["mappings"][&alias] != id
                    || array(&space["concepts"][index]["definitions"])?
                        .iter()
                        .any(|entry| entry["origin"] == alias)
                {
                    return Err("unknown or duplicated concept definition origin".to_owned());
                }
                let word = percent_encode_links_text(&alias);
                let grammar = parse_grammar_links(&format!(
                    "(grammar (start {word}))\n(rule {word} {} {})\n",
                    print_link(&args[2])?,
                    args[3..]
                        .iter()
                        .map(print_link)
                        .collect::<Result<Vec<_>, _>>()?
                        .join(" ")
                ))
                .map_err(|error| error.to_string())?;
                let serialized: Value = serde_json::from_str(&serialize_grammar(&grammar))
                    .map_err(|error| error.to_string())?;
                let definition = production(&serialized["rules"][0]);
                space["concepts"][index]["definitions"]
                    .as_array_mut()
                    .ok_or("invalid concept definitions")?
                    .push(json!({"origin":alias,"definition":definition}));
                if space["concepts"][index]["definition"].is_null() {
                    space["concepts"][index]["definition"] = definition;
                }
            }
            "concept-proof" => {
                if args.len() != 4 {
                    return Err("incomplete concept proof".to_owned());
                }
                let (tag, pairs) = link_parts(&args[3])?;
                if tag != "correspondence" {
                    return Err("missing proof correspondence".to_owned());
                }
                let pairs = pairs
                    .iter()
                    .map(|pair| match pair {
                        LiNo::Link { id: None, values } if values.len() == 2 => {
                            Ok(json!([link_word(&values[0])?, link_word(&values[1])?]))
                        }
                        _ => Err("invalid proof correspondence pair".to_owned()),
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                space["proofs"].as_array_mut().ok_or("invalid concept proofs")?.push(json!({"method":link_word(&args[0])?,"first":link_word(&args[1])?,"second":link_word(&args[2])?,"correspondence":pairs}));
            }
            "unproved-equivalence" => {
                if args.len() != 3 {
                    return Err("invalid unproved equivalence".to_owned());
                }
                space["failures"].as_array_mut().ok_or("invalid equivalence failures")?.push(json!({"members":[link_word(&args[0])?,link_word(&args[1])?],"reason":link_word(&args[2])?}));
            }
            "ambiguous-origin" => {
                if args.len() < 3 {
                    return Err("invalid ambiguous origin".to_owned());
                }
                space["ambiguities"].as_array_mut().ok_or("invalid concept ambiguities")?.push(json!({"alias":link_word(&args[0])?,"meanings":args[1..].iter().map(link_word).collect::<Result<Vec<_>,_>>()?}));
            }
            "concept-redirect" => {
                if args.len() != 2 {
                    return Err("invalid concept redirect".to_owned());
                }
                let old = link_word(&args[0])?;
                if space["redirects"].get(&old).is_some() {
                    return Err("duplicate concept redirect".to_owned());
                }
                space["redirects"][old] = json!(link_word(&args[1])?);
            }
            _ => return Err(format!("unknown global concept link {head}")),
        }
    }
    let mut imported = Vec::new();
    for index in 0..array(&space["sources"])?.len() {
        let source = space["sources"][index].clone();
        let id = string(&source["id"])?;
        let context = contexts.get(id).ok_or("missing grammar context")?;
        let mut header = Vec::new();
        let mut declared = Vec::new();
        for field in context {
            if ["matching", "settling"].contains(&link_parts(field)?.0) {
                header.push(print_link(field)?);
            } else {
                declared.push(print_link(field)?);
            }
        }
        let format = source
            .get("sourceFormat")
            .map(|format| {
                string(format)
                    .map(|format| format!("(format {})", percent_encode_links_text(format)))
            })
            .transpose()?
            .unwrap_or_default();
        let placeholder = parse_grammar_links(&format!(
            "(grammar (start placeholder) {format} {})\n{}\n(rule placeholder normal empty)\n",
            header.join(" "),
            declared.join("\n")
        ))
        .map_err(|error| error.to_string())?;
        let serialized: Value = serde_json::from_str(&serialize_grammar(&placeholder))
            .map_err(|error| error.to_string())?;
        let context = declarations(serialized.get("declarations"));
        space["sources"][index]["context"] = context.clone();
        let mut rules = Vec::new();
        for concept in array(&space["concepts"])? {
            if array(&concept["origins"])?.is_empty()
                || array(&concept["definitions"])?.len() != array(&concept["origins"])?.len()
            {
                return Err("a copied global origin needs exactly one production".to_owned());
            }
            for definition in array(&concept["definitions"])? {
                let alias = string(&definition["origin"])?;
                let (concept_index, origin_index) =
                    *origins.get(alias).ok_or("missing production origin")?;
                let origin = &space["concepts"][concept_index]["origins"][origin_index];
                if origin["source"] != id {
                    continue;
                }
                let mut rule = definition["definition"].clone();
                rule["name"] = origin["rule"].clone();
                rule["sourceNames"] = origin["sourceNames"].clone();
                if !origin["concept"].is_null() {
                    rule["concept"] = origin["concept"].clone();
                }
                rules.push(rule);
            }
        }
        let order = array(&source["ruleOrder"])?;
        if order.len() != rules.len()
            || order
                .iter()
                .any(|name| !rules.iter().any(|rule| rule["name"] == *name))
        {
            return Err("grammar rule order disagrees with copied productions".to_owned());
        }
        if !source["start"].is_null() && !order.contains(&source["start"]) {
            return Err("grammar start names no copied production".to_owned());
        }
        rules.sort_by_key(|rule| {
            order
                .iter()
                .position(|name| *name == rule["name"])
                .unwrap_or(order.len())
        });
        imported.push(json!({"id":source["id"],"language":source["language"],"edition":source["edition"],"grammar":{"schemaVersion":serialized["schemaVersion"],"start":source["start"],"sourceFormat":source["sourceFormat"],"rules":rules,"declarations":context}}));
    }
    let mut required = Vec::new();
    for failure in array(&space["failures"])? {
        required.push([
            string(&failure["members"][0])?.to_owned(),
            string(&failure["members"][1])?.to_owned(),
        ]);
    }
    let settings = GlobalConceptOptions {
        records: options.records.clone(),
        formal_definitions: Some(space["formalDefinitions"].clone()),
        previous: Some(space.clone()),
        equivalences: required,
    };
    let (checked, graph, aliases) = create_global_concept_space_in(&imported, &settings)?;
    if space["formalDefinitions"]
        .as_object()
        .ok_or("invalid formal definitions")?
        .keys()
        .any(|alias| !aliases.contains_key(alias))
    {
        return Err("unknown formal definition origin".to_owned());
    }
    let mut witnessed: Vec<_> = (0..aliases.len()).collect();
    for proof in array(&space["proofs"])? {
        let pairs = array(&proof["correspondence"])?;
        if proof["method"] != GLOBAL_CONCEPT_PROOF_METHOD
            || pairs.is_empty()
            || pairs[0][0] != proof["first"]
            || pairs[0][1] != proof["second"]
        {
            return Err("invalid global concept proof witness".to_owned());
        }
        let mut first = Vec::new();
        let mut second = Vec::new();
        for pair in pairs {
            let one = string(&pair[0])?;
            let other = string(&pair[1])?;
            first.push(
                *aliases
                    .get(one)
                    .ok_or("invalid global concept proof witness")?,
            );
            second.push(
                *aliases
                    .get(other)
                    .ok_or("invalid global concept proof witness")?,
            );
        }
        let number = |index: usize| serde_json::Number::from(index).as_f64().unwrap();
        let to_numbers = |values: &[usize]| values.iter().map(|index| number(*index)).collect();
        if !verify_global_concept_correspondence(
            graph.shapes.clone(),
            graph.meanings.clone(),
            to_numbers(&graph.offsets),
            to_numbers(&graph.edges),
            to_numbers(&first),
            to_numbers(&second),
        ) {
            return Err("unproved global concept correspondence".to_owned());
        }
        for (index, pair) in pairs.iter().enumerate() {
            if space["mappings"][string(&pair[0])?] != space["mappings"][string(&pair[1])?] {
                return Err("a proof correspondence has different global identities".to_owned());
            }
            let one = representative(&witnessed, first[index]);
            let other = representative(&witnessed, second[index]);
            witnessed[one.max(other)] = one.min(other);
        }
    }
    if canonical(&checked["mappings"]) != canonical(&space["mappings"]) {
        return Err("shared global identity lacks a current one-to-one proof".to_owned());
    }
    if canonical(&checked["failures"]) != canonical(&space["failures"])
        || canonical(&checked["ambiguities"]) != canonical(&space["ambiguities"])
    {
        return Err("global concept uncertainty disagrees with current definitions".to_owned());
    }
    for index in 0..array(&space["concepts"])?.len() {
        let concept = &space["concepts"][index];
        let members = array(&concept["origins"])?;
        if members.is_empty() {
            return Err("a copied global origin needs exactly one production".to_owned());
        }
        let root = representative(
            &witnessed,
            *aliases
                .get(string(&members[0]["alias"])?)
                .ok_or("unknown concept origin")?,
        );
        let present: BTreeSet<_> = members
            .iter()
            .map(|origin| string(&origin["alias"]))
            .collect::<Result<_, _>>()?;
        for alias in &present {
            if representative(
                &witnessed,
                *aliases.get(*alias).ok_or("unknown concept origin")?,
            ) != root
            {
                return Err("a shared global concept lacks its proof witnesses".to_owned());
            }
        }
        let proofs: Vec<_> = array(&space["proofs"])?
            .iter()
            .enumerate()
            .filter_map(|(index, proof)| {
                proof["correspondence"]
                    .as_array()
                    .is_some_and(|pairs| {
                        pairs.iter().any(|pair| {
                            present.contains(pair[0].as_str().unwrap_or_default())
                                && present.contains(pair[1].as_str().unwrap_or_default())
                        })
                    })
                    .then_some(index)
            })
            .collect();
        space["concepts"][index]["proofs"] = json!(proofs);
    }
    for (old, current) in space["redirects"]
        .as_object()
        .ok_or("invalid concept redirects")?
    {
        let current = string(current)?;
        if concepts.contains_key(old) || !concepts.contains_key(current) || old == current {
            return Err("invalid global concept identity redirect".to_owned());
        }
    }
    Ok(ValidatedGlobalConceptArchive { space, imported })
}
