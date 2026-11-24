// Package complete checked source output. The recorded styles change only
// declaration ownership and immutable host parameter loans; algorithms stay
// in the original complete generated declarations and retained artifact.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { tokenize } from '../src/translation/lexer.js';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { partitionPortableRustLibrary, readPortableSourceLintStyles, readPortableSourceParameterStyles } from './portable-source-library-style.mjs';
import {readNativeCompilerSourceStyles,readNativeNumericPatternSourceStyles} from './native-compiler-source-styles.mjs';
import {readNumericModuleBindingSourceStyles} from './numeric-module-binding-source-styles.mjs';
import {readDeclarationDependencyBranchStyles} from './declaration-dependency-branch-styles.mjs';

/** Relocate actual module paths while retaining every literal and comment byte. */
export function readPortableLibraryRelocation(source, completeItems) {
  const declarations = new Set([...source.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
  const edits = [], relocatedItems = [];
  let offset = 0;
  for (const item of completeItems ?? [source]) {
    const tokens = tokenize(item, 'Rust').tokens;
    const segments = [];
    let cursor = 0;
    for (let index = 0; index < tokens.length; index++) {
      const token = tokens[index];
      if (token.kind === 'identifier' && token.value === 'crate' && tokens[index + 1]?.value === '::'
        && tokens[index + 2]?.kind === 'identifier' && declarations.has(tokens[index + 2].value)) {
        edits.push({start: offset + token.start, end: offset + token.end, from: token.raw, to: 'self'});
        segments.push(item.slice(cursor, token.start), 'self');
        cursor = token.end;
      }
    }
    segments.push(item.slice(cursor));
    relocatedItems.push(segments.join(''));
    offset += item.length + 2;
  }
  const relocated = relocatedItems.join('\n\n');
  const legacyTargets = new Set();
  const global = source.replace(/crate::(?=([A-Za-z_][A-Za-z_0-9]*))/gu, (reference, identifier) => {
    let matched = false;
    for (let end = 1; end <= identifier.length; end += 1) {
      const prefix = identifier.slice(0, end);
      if (!declarations.has(prefix)) continue;
      legacyTargets.add(prefix);
      matched = true;
    }
    // Any matching legacy action changes only the initial crate component.
    // A declaration that is a prefix of another therefore gives the same result.
    return matched ? 'self::' : reference;
  });
  const legacy = [...declarations].filter(declaration => legacyTargets.has(declaration))
    .map(declaration => ({op: 'replace', field: 'source', from: 'crate::' + declaration, to: 'self::' + declaration}));
  // Retain every existing receipt when its lexical meaning was already correct.
  const actions = global === relocated ? legacy : [{op: 'replace', field: 'source', from: source, to: relocated}];
  return {source: relocated, actions, edits, offsetUnit: 'UTF-16 code units', ...(completeItems ? {completeItems: relocatedItems} : {})};
}

export function writeRecordRuleLibrary(emitted, name, { check = false, sourceStyles = null } = {}) {
  assert.match(name, /^[a-z][a-z_0-9]*$/u);
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(header, 'the complete checked output retains its existing generated module header');
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const relocation = readPortableLibraryRelocation(raw).actions;
  const relocated = new DecoratorSet(relocation.length ? [{ id: name + '-library-owner', level: 'emitter', order: 0, when: [['format', 'Rust']], actions: relocation }] : [])
    .decorate('emitter', { format: 'Rust', source: raw }).source;
  const publicNames = [...relocated.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const loans = readPortableSourceParameterStyles(relocated, { publicNames });
  const reviewed = readPortableSourceLintStyles(loans.source);
  assert.ok(sourceStyles === null || typeof sourceStyles === 'function', 'an optional family style is an actual source proof producer');
  const additional = sourceStyles === null ? {source: reviewed.source, actions: []} : sourceStyles(reviewed.source);
  const feedback = readNativeCompilerSourceStyles(additional.source, name);
  const numeric = readNativeNumericPatternSourceStyles(feedback.source, name);
  const names = readNumericModuleBindingSourceStyles(numeric.source, name);
  const branches = readDeclarationDependencyBranchStyles(names.source, name);
  const actions = [...relocation, ...loans.actions, ...reviewed.actions, ...additional.actions, ...feedback.actions, ...numeric.actions, ...names.actions, ...branches.actions];
  const styles = new DecoratorSet(actions.length ? [{ id: name + '-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }] : []);
  const styleFile = new URL('../../parity/self-translation/' + name.replaceAll('_', '-') + '-decorators.lino', import.meta.url);
  if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'record family library styles retain every actual declaration');
  else writeFileSync(styleFile, styles.toLino());
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.equal(decorated, branches.source, 'the recorded ownership, loan and lint styles replay exactly');
  const formatted = formatProductRuleSource(decorated);
  const files = partitionPortableRustLibrary(formatted, {
    notice: '// Generated from the complete checked JavaScript source and recorded dependencies.\n// Do not edit by hand.\n',
    directory: name,
  });
  for (const file of files) {
    const output = new URL('../../rust/src/translation/' + (file.path ?? name + '.rs'), import.meta.url);
    if (check) assert.equal(readFileSync(output, 'utf8'), file.source, 'complete native record family library is fresh');
    else {
      mkdirSync(new URL('.', output), { recursive: true });
      writeFileSync(output, file.source);
    }
  }
  return files;
}
