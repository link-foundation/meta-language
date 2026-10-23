//! The items of a JavaScript module translated into Rust in the order of the
//! names they use, so an item may call and read the module's other items and
//! the items it imports.
//!
//! Mirrors `translateBound`, `scanItem` and `declaredSignatures` in
//! `js/src/self-translation.js`.

use std::collections::{BTreeMap, HashMap, HashSet};

mod deferred_scope;
mod dependency_rules;
mod owned_recursive_emission;
mod recursive_groups;

use super::{Context, Group, GroupKind, Translated, translate_group};
use crate::decorators::DecoratorSet;
use crate::translation::Language;
use crate::translation::ir::{Decl, Effect, Node, Program};
use crate::translation::javascript::ModuleContext;
use crate::translation::lexer::{Token, TokenKind, tokenize};
use crate::translation::surface::{External, ExternalParam};
use crate::translation::types::Type;

/// A named import an item makes: its specifier and the imported and bound names.
#[derive(Clone)]
struct ImportScan {
    specifier: String,
    names: Vec<(String, String)>,
}

/// What a JavaScript item declares and names, read from its tokens.
#[derive(Clone)]
pub(super) struct Scan {
    pub(super) declares: Option<String>,
    pub(super) exported: bool,
    is_async: bool,
    // Ordinary is the actual named declaration branch together with !is_async.
    function_declaration: bool,
    // Actual UTF-16 class and name ranges, present only for the exact empty form.
    empty_class_declaration: Option<[usize; 4]>,
    // Certified complete actual CST: 1=class definition, 2=const function value.
    deferred_binding: Option<u8>,
    inert_binding: Option<u8>,
    mentions: Vec<String>,
    imports: Option<ImportScan>,
}

/// What a JavaScript item declares and names, read from its tokens: the name
/// of the function or constant it declares, whether it is exported or async,
/// the identifiers it mentions, and the specifier and names of a named import.
/// `None` when the item does not tokenize.
fn scan_item(text: &str) -> Option<Scan> {
    let tokens = tokenize(text, Language::JavaScript).ok()?.tokens;
    let at = |index: usize| tokens.get(index).or_else(|| tokens.last());
    let kind = |index: usize, kind: TokenKind| at(index).is_some_and(|token| token.kind == kind);
    let value = |index: usize, value: &str| at(index).is_some_and(|token| token.value == value);
    let word = |index: usize, text: &str| kind(index, TokenKind::Identifier) && value(index, text);
    let name = |index: usize| at(index).map_or_else(String::new, |token| token.value.clone());
    let exported = word(0, "export");
    let mut index = usize::from(exported);
    let mut declares = None;
    let mut is_async = false;
    let mut function_declaration = false;
    let mut empty_class_declaration = None;
    let mut imports = None;
    let deferred_candidate = word(index, "class")
        || word(index, "const")
            && kind(index + 1, TokenKind::Identifier)
            && value(index + 2, "=")
            && (word(index + 3, "function")
                || value(index + 3, "(")
                || kind(index + 3, TokenKind::Identifier) && value(index + 4, "=>"));
    let deferred = if deferred_candidate {
        deferred_scope::read_binding(text)
    } else {
        None
    };
    let inert = if word(index, "const") {
        deferred_scope::read_inert_binding(text)
    } else {
        None
    };
    if word(index, "async") && word(index + 1, "function") {
        is_async = true;
        index += 1;
    }
    if word(index, "function") && kind(index + 1, TokenKind::Identifier) {
        declares = Some(name(index + 1));
        function_declaration = true;
    } else if word(index, "class") && kind(index + 1, TokenKind::Identifier) {
        // Reserve the actual named owner even for a class whose scope form refuses.
        declares = Some(name(index + 1));
        if kind(index + 2, TokenKind::Punct)
            && value(index + 2, "{")
            && kind(index + 3, TokenKind::Punct)
            && value(index + 3, "}")
            && kind(index + 4, TokenKind::Eof)
        {
            let class = at(index)?;
            let class_name = at(index + 1)?;
            let close = at(index + 3)?;
            declares = Some(class_name.value.clone());
            empty_class_declaration =
                Some([class.start, close.end, class_name.start, class_name.end]);
        }
    } else if word(index, "const")
        && kind(index + 1, TokenKind::Identifier)
        && value(index + 2, "=")
    {
        declares = Some(name(index + 1));
        is_async = word(index + 3, "async");
    } else if word(0, "import") && value(1, "{") {
        let mut names = Vec::new();
        let mut next = 2;
        while kind(next, TokenKind::Identifier) {
            let imported = name(next);
            let aliased = word(next + 1, "as") && kind(next + 2, TokenKind::Identifier);
            let local = if aliased {
                name(next + 2)
            } else {
                imported.clone()
            };
            names.push((imported, local));
            next += if aliased { 3 } else { 1 };
            if !value(next, ",") {
                break;
            }
            next += 1;
        }
        if value(next, "}") && word(next + 1, "from") && kind(next + 2, TokenKind::String) {
            imports = Some(ImportScan {
                specifier: name(next + 2),
                names,
            });
        }
    }
    let mut mentions: Vec<String> = Vec::new();
    for token in tokens
        .iter()
        .filter(|token: &&Token| token.kind == TokenKind::Identifier)
    {
        if !mentions.contains(&token.value) {
            mentions.push(token.value.clone());
        }
    }
    // Actual CST transport can retain a Unicode binder outside this lexer's
    // portable identifier subset; it does not supply a callable signature.
    if declares.is_none()
        && let Some((name, _)) = &inert
    {
        declares = Some(name.clone());
    }
    if let Some((_, name)) = &deferred
        && declares.as_ref() != Some(name)
    {
        return None;
    }
    if let Some((name, _)) = &inert
        && declares.as_ref() != Some(name)
    {
        return None;
    }
    Some(Scan {
        declares,
        exported,
        is_async,
        function_declaration,
        empty_class_declaration,
        deferred_binding: deferred.map(|(kind, _)| kind),
        inert_binding: inert.map(|(_, owner)| owner),
        mentions,
        imports,
    })
}

/// The signatures a translated item gives the other items of its module.
///
/// A constant gives its name and type when no data type is in it. Otherwise
/// they are its functions at the top level, other than the ones the
/// translator makes up, when no data type is among their types and no
/// parameter is a guarded natural number (mirrors `declaredSignatures`).
pub(super) fn declared_signatures(program: &Program, constant: bool) -> Vec<External> {
    if constant {
        let Some(Effect::Let { name, value, .. }) =
            program.main.as_ref().and_then(|main| main.effects.first())
        else {
            return Vec::new();
        };
        if !portable(&value.ty) {
            return Vec::new();
        }
        return vec![External::Constant {
            name: name.clone(),
            ty: value.ty.clone(),
            literal: matches!(value.node, Node::Lit { .. }),
        }];
    }
    program
        .declarations
        .iter()
        .filter_map(|decl| match decl {
            Decl::Fn(function)
                if function.module_path.is_empty()
                    && !function.name.starts_with("ml_")
                    && function
                        .params
                        .iter()
                        .all(|param| param.guard.is_none() && portable(&param.ty))
                    && portable(&function.ret) =>
            {
                Some(External::Function {
                    name: function.name.clone(),
                    params: function
                        .params
                        .iter()
                        .map(|param| ExternalParam {
                            name: param.name.clone(),
                            ty: param.ty.clone(),
                        })
                        .collect(),
                    ret: function.ret.clone(),
                })
            }
            _ => None,
        })
        .collect()
}

fn portable(ty: &Type) -> bool {
    match ty {
        Type::Data { .. } => false,
        _ => ty.children().into_iter().all(portable),
    }
}

/// Translates the groups of a JavaScript module into Rust, each item after
/// the items it names, with the signatures of the ones that translated bound.
///
/// A name that a cycle of items leaves untranslated stays unbound, so its
/// callers are carried with the checker's diagnostic.
pub(super) struct Binder<'g, 'a> {
    pub(super) groups: &'g [Group<'a>],
    pub(super) source: &'g str,
    pub(super) from: &'g str,
    pub(super) to: &'g str,
    pub(super) decorators: &'g DecoratorSet,
    pub(super) module_directory: &'g [String],
    pub(super) imports: &'g BTreeMap<String, Vec<External>>,
    pub(super) utf16_strings: bool,
}

pub(super) struct ModuleSignatureContext {
    pub(super) module_scope_known: bool,
    pub(super) owner_counts: HashMap<String, usize>,
    pub(super) module_binding_names: Vec<String>,
}

impl ModuleSignatureContext {
    pub(super) fn accept_signature_binding(&self, name: &str) -> bool {
        accept_signature_binding(
            self.module_scope_known,
            self.owner_counts.get(name).copied().unwrap_or(0),
        )
    }
}

fn accept_signature_binding(scope_known: bool, owner_count: usize) -> bool {
    recursive_groups::source_number(owner_count)
        .is_some_and(|count| dependency_rules::accept_signature_binding(scope_known, count))
}

impl Binder<'_, '_> {
    /// The translation of every group, with what each item scan found.
    pub(super) fn translate(&self) -> (Vec<Translated>, Vec<Option<Scan>>, ModuleSignatureContext) {
        let scans: Vec<Option<Scan>> = self
            .groups
            .iter()
            .map(|group| match &group.kind {
                GroupKind::Item { text, .. } if self.from == "TypeScript" => {
                    super::erase_type_script(text)
                        .ok()
                        .and_then(|erased| scan_item(&erased.code))
                }
                GroupKind::Item { text, .. } => scan_item(text),
                _ => None,
            })
            .collect();
        let mut owners: HashMap<String, usize> = HashMap::new();
        let mut owner_counts: HashMap<String, usize> = HashMap::new();
        let mut module_binding_names = Vec::new();
        for (index, scan) in scans.iter().enumerate() {
            let Some(scan) = scan else { continue };
            if let Some(imports) = &scan.imports {
                for (_, local) in &imports.names {
                    if !owners.contains_key(local) {
                        module_binding_names.push(local.clone());
                    }
                    *owner_counts.entry(local.clone()).or_default() += 1;
                    owners.entry(local.clone()).or_insert(index);
                }
            } else if let Some(name) = &scan.declares {
                if !owners.contains_key(name) {
                    module_binding_names.push(name.clone());
                }
                *owner_counts.entry(name.clone()).or_default() += 1;
                owners.entry(name.clone()).or_insert(index);
            }
        }
        let components = recursive_groups::read_components(&scans, &owners);
        let mut state = Visit {
            scans,
            owners,
            owner_counts,
            module_binding_names,
            components,
            attempted: HashSet::new(),
            module_scope_known: false,
            outs: (0..self.groups.len()).map(|_| None).collect(),
            visiting: HashSet::new(),
        };
        // Import leaves obtain only their ordinary actual translated signatures.
        for index in 0..self.groups.len() {
            if state.scans[index]
                .as_ref()
                .is_some_and(|scan| scan.imports.is_some())
            {
                self.visit(&mut state, index);
            }
        }
        state.module_scope_known = self.module_scope_known(&state);
        for index in 0..self.groups.len() {
            self.visit(&mut state, index);
        }
        let outs = state
            .outs
            .into_iter()
            .map(|out| out.expect("every group is visited"))
            .collect();
        (
            outs,
            state.scans,
            ModuleSignatureContext {
                module_scope_known: state.module_scope_known,
                owner_counts: state.owner_counts,
                module_binding_names: state.module_binding_names,
            },
        )
    }

    fn visit(&self, state: &mut Visit, index: usize) {
        if state.outs[index].is_some() || state.visiting.contains(&index) {
            return;
        }
        if self.try_recursive_group(state, index) {
            return;
        }
        let group = &self.groups[index];
        let Some(scan) = state.scans[index].clone() else {
            state.outs[index] = Some(translate_group(
                group,
                self.from,
                self.to,
                self.decorators,
                &Context {
                    utf16_strings: self.utf16_strings,
                    module_binding_scope: Some(Self::module_binding_scope(state)),
                    ..Context::default()
                },
            ));
            return;
        };
        state.visiting.insert(index);
        let mut externals = Vec::new();
        if let Some(imports) = &scan.imports {
            let provided = self
                .imports
                .get(&imports.specifier)
                .map_or(&[][..], Vec::as_slice);
            for (imported, local) in &imports.names {
                if let Some(signature) = provided
                    .iter()
                    .find(|candidate| candidate.name() == imported && accept_signature(candidate))
                {
                    externals.push(signature.renamed(local));
                }
            }
        } else {
            for name in &scan.mentions {
                let Some(&owner) = state.owners.get(name) else {
                    continue;
                };
                if owner == index
                    || !accept_signature_binding(
                        state.module_scope_known,
                        state.owner_counts.get(name).copied().unwrap_or(0),
                    )
                {
                    continue;
                }
                self.visit(state, owner);
                let Some(out) = &state.outs[owner] else {
                    continue;
                };
                if out.status != "translated"
                    || state.scans[owner]
                        .as_ref()
                        .is_some_and(|scan| scan.is_async)
                {
                    continue;
                }
                if let Some(signature) = out
                    .signatures
                    .iter()
                    .find(|candidate| candidate.name() == name && accept_signature(candidate))
                {
                    externals.push(signature.clone());
                }
            }
        }
        let context = Context {
            module: ModuleContext {
                externals,
                module_directory: Some(self.module_directory.to_vec()),
            },
            utf16_strings: self.utf16_strings,
            module_binding_scope: Some(Self::module_binding_scope(state)),
        };
        state.outs[index] = Some(translate_group(
            group,
            self.from,
            self.to,
            self.decorators,
            &context,
        ));
        state.visiting.remove(&index);
    }
}

/// The groups translated so far and the ones being translated.
struct Visit {
    scans: Vec<Option<Scan>>,
    owners: HashMap<String, usize>,
    owner_counts: HashMap<String, usize>,
    module_binding_names: Vec<String>,
    components: Option<recursive_groups::Components>,
    attempted: HashSet<usize>,
    module_scope_known: bool,
    outs: Vec<Option<Translated>>,
    visiting: HashSet<usize>,
}

const fn accept_signature(signature: &External) -> bool {
    let (constant, literal) = match signature {
        External::Constant { literal, .. } => (true, *literal),
        External::Function { .. } => (false, false),
    };
    crate::translation::frontend_rules::accept_declaration_signature(constant, literal)
}
