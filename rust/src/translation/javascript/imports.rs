//! Imports: the assertion module, and the items of other modules of a crate.

use super::{
    Assertion, JavaScriptParser, Result, SImport, SImportName, Token, TokenKind, span, unsupported,
};

const ASSERTION_MODULES: [&str; 4] = [
    "node:assert",
    "node:assert/strict",
    "assert",
    "assert/strict",
];
const NAMESPACE_IMPORT: &str = "import the items of a module by name, as in import { f } from './m.mjs', or the assertion module as a whole, as in import assert from 'node:assert/strict'";
const ASSERTION_WHOLE: &str =
    "import the assertion module as a whole, e.g. import assert from 'node:assert/strict'";
/// The file extensions a module specifier may end with, longest first.
const EXTENSIONS: [&str; 6] = [".mjs", ".cjs", ".mts", ".cts", ".js", ".ts"];

/// A named import: the item's name, the name it binds, and the token after
/// the item's name, where a refused named import of node:assert ends.
struct Named {
    imported: String,
    local: String,
    aliased: bool,
    after: Token,
}

impl JavaScriptParser {
    /// `import assert from 'node:assert/strict'` binds the assertion module.
    ///
    /// In a module of a crate, which self-translation reads, `import { a, b as
    /// c } from './m.mjs'` imports items of another module of the crate.
    /// Every other import is outside the portable core.
    pub(super) fn import_declaration(&mut self) -> Result<()> {
        let start = self.cursor.advance();
        if self.cursor.is("*") {
            return Err(unsupported(
                "namespace import",
                NAMESPACE_IMPORT,
                Some(self.to_here(&start)),
            ));
        }
        let mut local = None;
        let mut braced = false;
        let mut names = Vec::new();
        if self.cursor.eat("{").is_some() {
            braced = true;
            while !self.cursor.is("}") {
                let imported = self.cursor.identifier(Some("import"))?;
                let after = self.peek();
                let alias = if self.cursor.eat("as").is_some() {
                    Some(self.cursor.identifier(Some("import"))?)
                } else {
                    None
                };
                names.push(Named {
                    local: alias
                        .as_ref()
                        .map_or_else(|| imported.value.clone(), |alias| alias.value.clone()),
                    aliased: alias.is_some(),
                    imported: imported.value,
                    after,
                });
                if self.cursor.eat(",").is_none() {
                    break;
                }
            }
            self.cursor.expect("}", Some("import"))?;
        } else {
            local = Some(self.cursor.identifier(Some("import"))?);
        }
        self.cursor.expect("from", Some("import"))?;
        let module = self.cursor.advance();
        if module.kind != TokenKind::String {
            return Err(Self::fail("expected a module name", &module));
        }
        self.cursor.eat(";");
        let specifier = module.value.clone();
        let place = Some(span(&start, &module));
        if braced && names.is_empty() {
            return Err(unsupported(
                &format!("import {{}} from '{specifier}'"),
                "an import names the items it imports",
                place,
            ));
        }
        if ASSERTION_MODULES.contains(&specifier.as_str()) {
            let refused = names
                .iter()
                .enumerate()
                .find(|(index, name)| *index > 0 || name.imported != "strict" || !name.aliased);
            if let Some((_, refused)) = refused {
                return Err(unsupported(
                    &format!("import {{ {} }}", refused.imported),
                    ASSERTION_WHOLE,
                    Some(span(&start, &refused.after)),
                ));
            }
            if self.assertion.is_some() {
                return Err(unsupported(
                    "second assertion import",
                    "import node:assert once",
                    place,
                ));
            }
            let name = names.first().map_or_else(
                || local.map_or_else(String::new, |token| token.value),
                |name| name.local.clone(),
            );
            self.scope.tdz.remove(&name);
            self.assertion = Some(Assertion {
                name,
                strict: !names.is_empty() || specifier.ends_with("/strict"),
            });
            return Ok(());
        }
        if !(specifier.starts_with("./") || specifier.starts_with("../")) {
            let reason = if specifier.starts_with("node:") {
                "Node.js built-in modules are outside the portable core"
            } else {
                "packages are outside the portable core; a module imports the items of the other modules of its crate by relative path, as in import { f } from './m.mjs'"
            };
            return Err(unsupported(
                &format!("import from '{specifier}'"),
                reason,
                place,
            ));
        }
        if names.is_empty() {
            return Err(unsupported(
                &format!("default import from '{specifier}'"),
                "a translated module has no default export; import its items by name, as in import { f } from './m.mjs'",
                place,
            ));
        }
        let Some(directory) = &self.module_directory else {
            return Err(unsupported(
                &format!("import from '{specifier}'"),
                "a relative import names another module of a crate, and self-translation translates a crate module by module",
                place,
            ));
        };
        let Some(module) = crate_path(&specifier, directory) else {
            return Err(unsupported(
                &format!("import from '{specifier}'"),
                "the specifier names no module file inside the crate",
                place,
            ));
        };
        self.imports.push(SImport {
            module,
            names: names
                .into_iter()
                .map(|name| SImportName {
                    imported: name.imported,
                    local: name.local,
                })
                .collect(),
            span: place,
        });
        Ok(())
    }
}

/// The path inside the crate of the module a relative `specifier` names from
/// the module directory `directory`, or `None` when it climbs above the crate
/// root or names no file.
fn crate_path(specifier: &str, directory: &[String]) -> Option<Vec<String>> {
    let mut parts: Vec<&str> = specifier.split('/').collect();
    let file = parts.pop()?;
    let file = EXTENSIONS
        .iter()
        .find_map(|extension| file.strip_suffix(extension))
        .unwrap_or(file);
    let mut path = directory.to_vec();
    for part in parts {
        match part {
            ".." => {
                path.pop()?;
            }
            "." => {}
            "" => return None,
            other => path.push(other.to_owned()),
        }
    }
    if file.is_empty() || file == "." || file == ".." {
        return None;
    }
    path.push(file.to_owned());
    Some(path)
}
