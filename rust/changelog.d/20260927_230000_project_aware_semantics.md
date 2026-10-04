---
bump: minor
---

### Added
- `ProgramProjectContext::with_entry` names the analyzed program's path within a project and supplies the project's files (`ProgramProjectSource`), enabling project-aware semantics for JavaScript (`package.json`, relative module resolution, named/default/namespace imports, JSON import attributes, tagged templates), Rust (`Cargo.toml`, `mod` trees, `use` paths, `pub` visibility, `macro_rules!` expansion, derives and attributes), Lean (`lakefile.toml` libraries, imports, namespaces, notations, attributes, tactics), and Rocq (`_CoqProject` load paths, `Require`, notations, lemmas and tactics).
- `ProgramRepresentation::project_modules`, `project_facts`, `project_references`, and `expansions` report the resolved modules, the project files and manifests read, the links from the entry program to declarations in other project files, and the macro, template and notation expansions; every semantic construct's evidence includes these project links (`ProgramFact::file`).
- Missing or broken project context is diagnosed (`missing-project-context`, `missing-project-symbol`, `inaccessible-project-symbol`, `missing-project-library`, `missing-import-attribute`, `import-attribute-mismatch`, `project-parse-error`) without fabricating links, and program snapshots keep the project entry and sources.

### Fixed
- JavaScript module requests take the statement's last string outside its import attributes, Rust `use` requests take only the leading identifier, and `node:assert`/`node:assert/strict` are recognized toolchain modules, matching the JavaScript runtime.
