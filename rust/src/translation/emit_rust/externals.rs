//! The other items of the module: imports of other modules' items, and calls
//! and reads of items the program does not declare.

use super::{Expr, External, Result, RustEmitter, SImport, Type, is_copy, snake};

/// The Rust name of another item of the module, `name` when it is bound under
/// another name: a function's in snake case, as its own translation names it,
/// and a constant's as it is written.
pub(super) fn external_name(external: &External, name: &str) -> String {
    match external {
        External::Function { .. } => snake(name),
        External::Constant { .. } => name.to_owned(),
    }
}

impl RustEmitter<'_> {
    /// An import of items of another module of the crate, `use crate::m::{a, b as c};`.
    ///
    /// A function is named in snake case, as its own translation names it, and
    /// any other item as it is written.
    pub(super) fn use_declaration(&self, import: &SImport) -> String {
        let items: Vec<String> = import
            .names
            .iter()
            .map(|name| {
                let external = self
                    .program
                    .externals
                    .iter()
                    .find(|external| external.name() == name.local);
                let (from, to) = external.map_or_else(
                    || (name.imported.clone(), name.local.clone()),
                    |external| {
                        (
                            external_name(external, &name.imported),
                            external_name(external, &name.local),
                        )
                    },
                );
                if from == to {
                    from
                } else {
                    format!("{from} as {to}")
                }
            })
            .collect();
        let mut path = vec!["crate".to_owned()];
        path.extend(import.module.iter().map(String::as_str).map(snake));
        let path = path.join("::");
        match items.as_slice() {
            [item] => format!("use {path}::{item};"),
            _ => format!("use {path}::{{{}}};", items.join(", ")),
        }
    }

    /// A call of another item's function of the module, or a read of one of its constants.
    pub(super) fn external_call(&mut self, func: &str, args: &[Expr], ty: &Type) -> Result<String> {
        let external = self.program.external(func).cloned();
        match external {
            Some(External::Function { .. }) => {
                let mut texts = Vec::with_capacity(args.len());
                for arg in args {
                    texts.push(self.expr(arg)?);
                }
                Ok(format!("{}({})", snake(func), texts.join(", ")))
            }
            // A literal Number, boolean or machine integer is a const, a
            // literal string a `&str` const, and any other value a static.
            Some(External::Constant { literal: true, .. }) if is_copy(ty) => Ok(func.to_owned()),
            Some(External::Constant { literal: true, .. }) if *ty == Type::String => {
                Ok(format!("String::from({func})"))
            }
            _ if is_copy(ty) => Ok(format!("*{func}")),
            _ => Ok(format!("{func}.clone()")),
        }
    }
}
