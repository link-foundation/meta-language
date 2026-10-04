#!/usr/bin/env node
// Generates the export parity map of requirement
// I195-PARITY-FEATURE-COMPLETENESS: the `javascriptExports` object of
// parity/language-features.json, one entry per public export of
// js/src/index.js naming the public Rust item that carries the same feature,
// and rust/tests/unit/javascript_export_parity.rs, which references every one
// of those items through `meta_language::` so the Rust compiler proves each
// exists and is public.
//
//   node js/scripts/generate-export-parity.mjs          # write
//   node js/scripts/generate-export-parity.mjs --check  # verify
//
// An export resolves by convention (camelCase becomes snake_case; PascalCase
// and UPPER_CASE names are kept) to the shortest public path of a Rust item of
// that name, or through OVERRIDES for exports whose Rust counterpart is a
// method or carries another name. An export that resolves to nothing fails the
// generator, so a new JavaScript export cannot land without its Rust feature.
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const featuresPath = path.join(root, 'parity/language-features.json');
const rustTestPath = path.join(root, 'rust/tests/unit/javascript_export_parity.rs');

/**
 * Exports whose Rust counterpart the convention does not find: a method, an
 * item of another name, or one of several items of the same name. `reference`
 * spells the item for the compiler where a generic needs its parameters.
 */
export const OVERRIDES = Object.freeze({
  // The enum whose variants are the fixture kinds.
  ApiStyleFixtureKind: { rust: 'ApiStyleFixture' },
  // One error enum carries every rename and transformation failure.
  BindingRenameError: { rust: 'ProgramRepresentationError' },
  ProgramTransformationError: { rust: 'ProgramRepresentationError' },
  // Methods of the access mode and of the network.
  accessModeIsMutable: { rust: 'AccessMode::is_mutable' },
  accessModeIsReadOnly: { rust: 'AccessMode::is_read_only' },
  accessModeLabel: { rust: 'AccessMode::label' },
  asReadOnly: { rust: 'LinkNetwork::as_read_only' },
  as_read_only: { rust: 'LinkNetwork::as_read_only' },
  freeze: { rust: 'LinkNetwork::freeze' },
  insertConceptRecord: { rust: 'LinkNetwork::insert_concept_record' },
  parseEngine: { rust: 'LinkNetwork::parse_engine' },
  parse_engine: { rust: 'LinkNetwork::parse_engine' },
  idKey: { rust: 'LinkId::as_u64' },
  // The program snapshot is a method pair of the program.
  createProgramSnapshot: { rust: 'ProgramRepresentation::serialize_snapshot' },
  readProgramSnapshot: { rust: 'ProgramRepresentation::from_snapshot' },
  // One compile entry point builds the native executor's parser.
  compileGrammar: { rust: 'compile_feature_grammar' },
  createGrammarParser: { rust: 'compile_feature_grammar' },
  renderSyntaxTree: { rust: 'SyntaxTree::render' },
  validateGrammar: { rust: 'validate' },
  renderRuleFieldLinks: { rust: 'render_rule_fields' },
  lowerGraphQL: { rust: 'lower_graphql' },
  lowerGraphQl: { rust: 'lower_graphql' },
  sourceMeanings: { rust: 'source_meanings_in' },
  // Predicate hosts are unit types implementing `QueryPredicateHost`.
  rejectPredicateHost: { rust: 'RejectPredicateHost' },
  sourceTextPredicateHost: { rust: 'SourceTextPredicateHost' },
  // The decorator factories construct the Rust decorator types.
  decorator: { rust: 'Decorator' },
  decoratorSet: { rust: 'DecoratorSet' },
});

const snake = (name) => name
  .replace(/([a-z0-9])([A-Z])/gu, '$1_$2')
  .replace(/([A-Z]+)([A-Z][a-z])/gu, '$1_$2')
  .toLowerCase();

// The kind of a Rust item from the keyword that declares it.
const KEYWORD_KINDS = Object.freeze({
  fn: 'function', struct: 'type', enum: 'type', trait: 'type', type: 'type', union: 'type',
  const: 'constant', static: 'constant',
});

function kindByCase(name) {
  if (/^[A-Z][A-Z0-9_]*$/u.test(name) && name.length > 1) return 'constant';
  if (/^[A-Z]/u.test(name)) return 'type';
  return 'function';
}

/** The file of module `name` declared in `file`, or null for an inline module. */
function moduleFile(file, name) {
  const base = path.basename(file);
  const dir = base === 'lib.rs' || base === 'mod.rs'
    ? path.dirname(file)
    : path.join(path.dirname(file), path.basename(file, '.rs'));
  for (const candidate of [path.join(dir, `${name}.rs`), path.join(dir, name, 'mod.rs')]) {
    if (existsSync(candidate)) return candidate;
  }
  return null;
}

// The leaf names a `use` tree brings into scope: `[source path, local name]`.
function useLeaves(tree, prefix = []) {
  const leaves = [];
  let depth = 0;
  let start = 0;
  const parts = [];
  for (let index = 0; index <= tree.length; index += 1) {
    const character = tree[index];
    if (character === '{') depth += 1;
    else if (character === '}') depth -= 1;
    else if ((character === ',' && depth === 0) || index === tree.length) {
      parts.push(tree.slice(start, index).trim());
      start = index + 1;
    }
  }
  for (const part of parts.filter(Boolean)) {
    const brace = part.indexOf('{');
    if (brace >= 0) {
      const head = part.slice(0, brace).replace(/::$/u, '').split('::').filter(Boolean);
      leaves.push(...useLeaves(part.slice(brace + 1, part.lastIndexOf('}')), [...prefix, ...head]));
      continue;
    }
    const [target, alias] = part.split(/\s+as\s+/u);
    const segments = [...prefix, ...target.split('::').filter(Boolean)];
    const last = segments.at(-1);
    if (last === 'self') leaves.push([segments.slice(0, -1), alias ?? segments.at(-2)]);
    else leaves.push([segments, alias ?? last]);
  }
  return leaves;
}

/**
 * The public surface of the crate: every item reachable from the crate root
 * through `pub` modules, `pub` items, inherent `pub` methods of those items,
 * and `pub use` re-exports, as `{ path, name, kind }` with `path` relative to
 * the crate root.
 */
export function rustPublicSurface(crateRoot = path.join(root, 'rust/src/lib.rs')) {
  const modules = new Map();
  function scan(file, modulePath, isPublic) {
    const text = readFileSync(file, 'utf8')
      .replace(/^#\[cfg\(test\)\]\s*\nmod tests[\s\S]*$/mu, '');
    const module = { file, path: modulePath, isPublic, items: new Map(), methods: [], uses: [], globs: [] };
    modules.set(modulePath.join('::'), module);
    for (const match of text.matchAll(/^(pub(?:\([^)]*\))? )?mod (\w+);/gmu)) {
      const child = moduleFile(file, match[2]);
      const childPublic = isPublic && match[1] === 'pub ';
      if (child) scan(child, [...modulePath, match[2]], childPublic);
    }
    const declaration = /^pub (?:const |async |unsafe |extern "C" )*(fn|struct|enum|trait|type|union|const|static) (\w+)/gmu;
    for (const match of text.matchAll(declaration)) {
      if (!module.items.has(match[2])) module.items.set(match[2], KEYWORD_KINDS[match[1]]);
    }
    for (const match of text.matchAll(/^impl(?:<[^{]*?>)? ([\w:]+)(?:<[^{]*?>)?(?: where[^{]*)? \{\n([\s\S]*?)^\}/gmu)) {
      const owner = match[1].split('::').at(-1);
      for (const method of match[2].matchAll(/^ {4}pub (?:const |async |unsafe )*fn (\w+)/gmu)) {
        module.methods.push([owner, method[1]]);
      }
    }
    for (const match of text.matchAll(/^pub use ([^;]+);/gmu)) {
      for (const [segments, name] of useLeaves(match[1].replace(/\s+/gu, ' '))) {
        if (name === '*') module.globs.push(segments.slice(0, -1));
        else module.uses.push([segments, name]);
      }
    }
  }
  scan(crateRoot, [], true);

  // The kind of the item a `use` path names, following re-exports.
  function resolve(fromModule, segments, seen = new Set()) {
    let base = fromModule.path;
    let rest = segments;
    if (rest[0] === 'crate') [base, rest] = [[], rest.slice(1)];
    else if (rest[0] === 'self') rest = rest.slice(1);
    while (rest[0] === 'super') [base, rest] = [base.slice(0, -1), rest.slice(1)];
    if (!modules.has([...base, rest[0]].join('::')) && modules.has(rest[0]) && rest.length > 1) base = [];
    const owner = modules.get([...base, ...rest.slice(0, -1)].join('::'));
    const name = rest.at(-1);
    if (!owner) return null;
    const key = `${owner.path.join('::')}::${name}`;
    if (seen.has(key)) return null;
    seen.add(key);
    if (owner.items.has(name)) return owner.items.get(name);
    const reexport = owner.uses.find(([, local]) => local === name);
    return reexport ? resolve(owner, reexport[0], seen) : null;
  }

  const surface = [];
  const typePaths = new Map();
  const add = (itemPath, name, kind) => {
    surface.push({ path: itemPath, name, kind });
    if (kind === 'type') typePaths.set(name, [...(typePaths.get(name) ?? []), itemPath]);
  };
  for (const module of modules.values()) {
    if (!module.isPublic) continue;
    const prefix = module.path.length > 0 ? `${module.path.join('::')}::` : '';
    for (const [name, kind] of module.items) add(`${prefix}${name}`, name, kind);
    for (const [segments, name] of module.uses) {
      add(`${prefix}${name}`, name, resolve(module, segments) ?? kindByCase(name));
    }
    for (const glob of module.globs) {
      const target = modules.get([...module.path, ...glob.filter((s) => s !== 'self')].join('::'));
      for (const [name, kind] of target?.items ?? []) add(`${prefix}${name}`, name, kind);
    }
  }
  // Inherent methods of public types, under every public path of the type.
  for (const module of modules.values()) {
    for (const [owner, method] of module.methods) {
      for (const ownerPath of typePaths.get(owner) ?? []) add(`${ownerPath}::${method}`, method, 'method');
    }
  }
  return surface;
}

const depth = (itemPath) => itemPath.split('::').length;

/** Resolves every JavaScript export to `{ rust, kind }`, or reports the ones that resolve to nothing. */
export function resolveExports(exportNames, surface) {
  const byName = new Map();
  for (const item of surface) {
    if (item.kind === 'method') continue;
    byName.set(item.name, [...(byName.get(item.name) ?? []), item]);
  }
  const byPath = new Map(surface.map((item) => [item.path, item]));
  const entries = {};
  const missing = [];
  for (const name of [...exportNames].sort()) {
    const override = OVERRIDES[name];
    if (override) {
      const item = byPath.get(override.rust);
      if (!item) {
        missing.push(`${name}: the override ${override.rust} is not a public Rust item`);
        continue;
      }
      entries[name] = { rust: override.rust, kind: item.kind };
      continue;
    }
    const candidates = [...new Set([name, snake(name), name.toUpperCase(), snake(name).toUpperCase()])];
    const hits = candidates.flatMap((candidate) => byName.get(candidate) ?? [])
      .sort((left, right) => depth(left.path) - depth(right.path) || left.path.localeCompare(right.path));
    if (hits.length === 0) missing.push(`${name}: no public Rust item named ${candidates.join(', ')}`);
    else entries[name] = { rust: hits[0].path, kind: hits[0].kind };
  }
  return { entries, missing };
}

/**
 * Orders `use` paths the way rustfmt (style edition 2024) orders consecutive
 * `use` items: segment by segment, `_` before every other character, digit
 * runs by value, and other characters by code point.
 */
export function rustfmtPathOrder(left, right) {
  const leftSegments = left.split('::');
  const rightSegments = right.split('::');
  for (let index = 0; index < Math.min(leftSegments.length, rightSegments.length); index += 1) {
    const order = versionOrder(leftSegments[index], rightSegments[index]);
    if (order !== 0) return order;
  }
  return leftSegments.length - rightSegments.length;
}

function versionOrder(left, right) {
  const tokens = (text) => text.match(/\d+|./gu) ?? [];
  const leftTokens = tokens(left);
  const rightTokens = tokens(right);
  for (let index = 0; index < Math.min(leftTokens.length, rightTokens.length); index += 1) {
    const [a, b] = [leftTokens[index], rightTokens[index]];
    if (a === b) continue;
    const aDigit = /^\d/u.test(a);
    const bDigit = /^\d/u.test(b);
    if (aDigit && bDigit) {
      const difference = Number(a) - Number(b);
      if (difference !== 0) return difference;
      return a.length - b.length;
    }
    if (a === '_') return -1;
    if (b === '_') return 1;
    const aCode = aDigit ? 0x30 : a.codePointAt(0);
    const bCode = bDigit ? 0x30 : b.codePointAt(0);
    if (aCode !== bCode) return aCode - bCode;
  }
  return leftTokens.length - rightTokens.length;
}

/** The `javascriptExports` block of parity/language-features.json, as text. */
export function renderExportsBlock(entries) {
  const lines = Object.entries(entries).map(
    ([name, entry]) => `    ${JSON.stringify(name)}: { "rust": ${JSON.stringify(entry.rust)}, "kind": ${JSON.stringify(entry.kind)} }`,
  );
  return `  "javascriptExportsDoc": "Generated by js/scripts/generate-export-parity.mjs: every public export of js/src/index.js with the public Rust item that carries the same feature. rust/tests/unit/javascript_export_parity.rs references every item so the Rust build fails when one disappears, and parity/fixtures/export-parity/cases.lino compares the observable output of both runtimes.",\n  "javascriptExports": {\n${lines.join(',\n')}\n  },\n`;
}

/** parity/language-features.json with its `javascriptExports` block replaced or inserted. */
export function withExportsBlock(text, entries) {
  const block = renderExportsBlock(entries);
  const existing = /^ {2}"javascriptExportsDoc": [^\n]*\n {2}"javascriptExports": \{\n[\s\S]*?^ {2}\},\n/mu;
  if (existing.test(text)) return text.replace(existing, () => block);
  const anchor = text.indexOf('  "features": [');
  if (anchor < 0) throw new Error('parity/language-features.json has no "features" array');
  return `${text.slice(0, anchor)}${block}${text.slice(anchor)}`;
}

const rustString = (value) => JSON.stringify(value);

/** rust/tests/unit/javascript_export_parity.rs, formatted as rustfmt formats it. */
export function renderRustTest(entries) {
  const items = [...new Set(Object.values(entries).filter((entry) => entry.kind !== 'method').map((entry) => entry.rust))]
    .sort(rustfmtPathOrder);
  const methods = [...new Set(Object.values(entries).filter((entry) => entry.kind === 'method').map((entry) => entry.rust))]
    .sort();
  const rows = Object.entries(entries).map(([name, entry]) => {
    // rustfmt keeps a tuple on one line while its items fit in 60 columns.
    const inner = `${rustString(name)}, ${rustString(entry.rust)}, ${rustString(entry.kind)}`;
    if (inner.length <= 60) return `    (${inner}),`;
    return `    (\n        ${rustString(name)},\n        ${rustString(entry.rust)},\n        ${rustString(entry.kind)},\n    ),`;
  });
  return `// @generated by js/scripts/generate-export-parity.mjs. Do not edit by hand:
// run \`node js/scripts/generate-export-parity.mjs\` after changing the public
// JavaScript or Rust surface.
//
// Requirement I195-PARITY-FEATURE-COMPLETENESS: every public export of
// js/src/index.js names the public Rust item that carries the same feature.
// The \`use\` items and method references below make the Rust build fail when
// one of those items disappears or stops being public, and the test checks
// that parity/language-features.json lists exactly the exports below.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use super::issue_195_observations::{Observation, record};

/// Every public item a JavaScript export maps to, imported to prove it exists.
#[allow(unused_imports)]
mod items {
${items.map((item) => `    use meta_language::${item} as _;`).join('\n')}
}

/// Every JavaScript export as \`(export, Rust path, kind)\`.
const EXPORTS: &[(&str, &str, &str)] = &[
${rows.join('\n')}
];

const FIXTURE_ID: &str = "planned:repository-directive:i195-parity-feature-completeness";

fn language_features() -> Value {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !dir.join("parity/language-features.json").is_file() {
        assert!(dir.pop(), "parity/language-features.json should exist");
    }
    let text = fs::read_to_string(dir.join("parity/language-features.json"))
        .expect("parity/language-features.json should be readable");
    serde_json::from_str(&text).expect("parity/language-features.json should be JSON")
}

#[test]
fn export_parity_rust_has_every_listed_feature() {
${methods.map((method) => `    let _ = meta_language::${method};`).join('\n')}
    let features = language_features();
    let listed = features["javascriptExports"]
        .as_object()
        .expect("javascriptExports should be an object");
    assert_eq!(listed.len(), EXPORTS.len(), "each export is listed");
    for (name, rust, kind) in EXPORTS {
        let entry = &listed[*name];
        assert_eq!(entry["rust"], *rust, "{name} maps to {rust}");
        assert_eq!(entry["kind"], *kind, "{name} is a {kind}");
    }
    record(&Observation {
        requirement_id: "I195-PARITY-FEATURE-COMPLETENESS",
        suffix: "behavior",
        fixture_id: FIXTURE_ID,
        fixture_file: "docs/vision.md",
        assertions: &["everyPublicExportListed", "rustHasEveryFeature"],
        test_name: "javascript_export_parity::export_parity_rust_has_every_listed_feature",
    });
}
`;
}

/** The current exports, their resolution and both generated files. */
export async function exportParity() {
  const javascript = await import(path.join(root, 'js/src/index.js'));
  const exportNames = Object.keys(javascript);
  const { entries, missing } = resolveExports(exportNames, rustPublicSurface());
  const features = withExportsBlock(readFileSync(featuresPath, 'utf8'), entries);
  return { exportNames, entries, missing, features, rustTest: renderRustTest(entries) };
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const { entries, missing, features, rustTest } = await exportParity();
  if (process.argv.includes('--report')) {
    for (const [name, entry] of Object.entries(entries)) console.log(`${name} -> ${entry.rust} [${entry.kind}]`);
  }
  let failed = false;
  if (missing.length > 0) {
    console.error(`JavaScript exports without a public Rust item:\n- ${missing.join('\n- ')}`);
    failed = true;
  }
  const outputs = [[featuresPath, features], [rustTestPath, rustTest]];
  if (process.argv.includes('--check')) {
    for (const [file, expected] of outputs) {
      const actual = existsSync(file) ? readFileSync(file, 'utf8') : '';
      if (actual !== expected) {
        console.error(`${path.relative(root, file)} is stale: run node js/scripts/generate-export-parity.mjs`);
        failed = true;
      }
    }
    if (!failed) console.log(`export parity is current: ${Object.keys(entries).length} JavaScript exports map to Rust`);
  } else {
    for (const [file, expected] of outputs) writeFileSync(file, expected);
    console.log(`wrote ${Object.keys(entries).length} export mappings`);
  }
  if (failed) process.exit(1);
}
