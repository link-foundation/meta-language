// Package complete checked source output. The recorded styles change only
// declaration ownership and immutable host parameter loans; algorithms stay
// in the original complete generated declarations and retained artifact.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { tokenize } from '../src/translation/lexer.js';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { partitionPortableRustLibrary, readPortableSourceLintStyles, readPortableSourceParameterStyles } from './portable-source-library-style.mjs';
import {readNativeCompilerSourceStyles} from './native-compiler-source-styles.mjs';

/** Relocate actual module paths while retaining every literal and comment byte. */
export function readPortableLibraryRelocation(source) {
  const declarations = new Set([...source.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
  const tokens = tokenize(source, 'Rust').tokens, edits = [];
  for (let index = 0; index < tokens.length; index++) {
    const token = tokens[index];
    if (token.kind === 'identifier' && token.value === 'crate' && tokens[index + 1]?.value === '::'
      && tokens[index + 2]?.kind === 'identifier' && declarations.has(tokens[index + 2].value)) {
      edits.push({start: token.start, end: token.end, from: token.raw, to: 'self'});
    }
  }
  let relocated = source;
  for (const edit of [...edits].reverse()) relocated = relocated.slice(0, edit.start) + edit.to + relocated.slice(edit.end);
  const legacy = [...declarations].filter(declaration => source.includes('crate::' + declaration))
    .map(declaration => ({op: 'replace', field: 'source', from: 'crate::' + declaration, to: 'self::' + declaration}));
  const global = legacy.reduce((text, action) => text.replaceAll(action.from, action.to), source);
  // Retain every existing receipt when its lexical meaning was already correct.
  const actions = global === relocated ? legacy : [{op: 'replace', field: 'source', from: source, to: relocated}];
  return {source: relocated, actions, edits, offsetUnit: 'UTF-16 code units'};
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
  const actions = [...relocation, ...loans.actions, ...reviewed.actions, ...additional.actions, ...feedback.actions];
  const styles = new DecoratorSet(actions.length ? [{ id: name + '-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }] : []);
  const styleFile = new URL('../../parity/self-translation/' + name.replaceAll('_', '-') + '-decorators.lino', import.meta.url);
  if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'record family library styles retain every actual declaration');
  else writeFileSync(styleFile, styles.toLino());
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.equal(decorated, feedback.source, 'the recorded ownership, loan and lint styles replay exactly');
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
