//! The items of a JavaScript module translated into Rust in the order of the
//! names they use, so an item may call and read the module's other items and
//! the items it imports.
//!
//! Mirrors `translateBound`, `scanItem` and `declaredSignatures` in
//! `js/src/self-translation.js`.

use std::collections::{BTreeMap, HashMap, HashSet};

use super::{Group, GroupKind, Translated, translate_group};
use crate::decorators::DecoratorSet;
use crate::translation::Language;
use crate::translation::ir::{Decl, Program};
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
    declares: Option<String>,
    pub(super) exported: bool,
    is_async: bool,
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
    let mut imports = None;
    if word(index, "async") && word(index + 1, "function") {
        is_async = true;
        index += 1;
    }
    if word(index, "function") && kind(index + 1, TokenKind::Identifier) {
        declares = Some(name(index + 1));
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
    Some(Scan {
        declares,
        exported,
        is_async,
        mentions,
        imports,
    })
}

/// The signatures a translated item gives the other items of its module.
///
/// They are its functions at the top level, other than the ones the
/// translator makes up, when no data type is among their types and no
/// parameter is a guarded natural number.
pub(super) fn declared_signatures(program: &Program) -> Vec<External> {
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
        Type::Array { element } => portable(element),
        Type::Data { .. } => false,
        _ => true,
    }
}

/// Translates the groups of a JavaScript module into Rust, each item after
/// the items it names, with the signatures of the ones that translated bound.
///
/// A name that a cycle of items leaves untranslated stays unbound, so its
/// callers are carried with the checker's diagnostic.
pub(super) struct Binder<'g, 'a> {
    pub(super) groups: &'g [Group<'a>],
    pub(super) from: &'g str,
    pub(super) to: &'g str,
    pub(super) decorators: &'g DecoratorSet,
    pub(super) module_directory: &'g [String],
    pub(super) imports: &'g BTreeMap<String, Vec<External>>,
}

impl Binder<'_, '_> {
    /// The translation of every group, with what each item scan found.
    pub(super) fn translate(&self) -> (Vec<Translated>, Vec<Option<Scan>>) {
        let scans: Vec<Option<Scan>> = self
            .groups
            .iter()
            .map(|group| match &group.kind {
                GroupKind::Item { text, .. } => scan_item(text),
                _ => None,
            })
            .collect();
        let mut owners: HashMap<String, usize> = HashMap::new();
        for (index, scan) in scans.iter().enumerate() {
            let Some(scan) = scan else { continue };
            if let Some(imports) = &scan.imports {
                for (_, local) in &imports.names {
                    owners.entry(local.clone()).or_insert(index);
                }
            } else if let Some(name) = &scan.declares {
                owners.entry(name.clone()).or_insert(index);
            }
        }
        let mut state = Visit {
            scans,
            owners,
            outs: (0..self.groups.len()).map(|_| None).collect(),
            visiting: HashSet::new(),
        };
        for index in 0..self.groups.len() {
            self.visit(&mut state, index);
        }
        let outs = state
            .outs
            .into_iter()
            .map(|out| out.expect("every group is visited"))
            .collect();
        (outs, state.scans)
    }

    fn visit(&self, state: &mut Visit, index: usize) {
        if state.outs[index].is_some() || state.visiting.contains(&index) {
            return;
        }
        let group = &self.groups[index];
        let Some(scan) = state.scans[index].clone() else {
            state.outs[index] = Some(translate_group(
                group,
                self.from,
                self.to,
                self.decorators,
                &ModuleContext::default(),
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
                    .find(|candidate| candidate.name() == imported)
                {
                    externals.push(signature.renamed(local));
                }
            }
        } else {
            for name in &scan.mentions {
                let Some(&owner) = state.owners.get(name) else {
                    continue;
                };
                if owner == index {
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
                    .find(|candidate| candidate.name() == name)
                {
                    externals.push(signature.clone());
                }
            }
        }
        let context = ModuleContext {
            externals,
            module_directory: Some(self.module_directory.to_vec()),
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
    outs: Vec<Option<Translated>>,
    visiting: HashSet<usize>,
}
