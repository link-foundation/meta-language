// Package complete checked source output. The recorded styles change only
// declaration ownership and immutable host parameter loans; algorithms stay
// in the original complete generated declarations and retained artifact.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { partitionPortableRustLibrary, readPortableSourceLintStyles, readPortableSourceParameterStyles } from './portable-source-library-style.mjs';

export function writeRecordRuleLibrary(emitted, name, { check = false } = {}) {
  assert.match(name, /^[a-z][a-z_0-9]*$/u);
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(header, 'the complete checked output retains its existing generated module header');
  const raw = [header, ...emitted.preludes, ...emitted.definitions].join('\n\n');
  const declarations = new Set([...raw.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
  const relocation = [...declarations].filter(declaration => raw.includes('crate::' + declaration))
    .map(declaration => ({ op: 'replace', field: 'source', from: 'crate::' + declaration, to: 'self::' + declaration }));
  const relocated = new DecoratorSet([{ id: name + '-library-owner', level: 'emitter', order: 0, when: [['format', 'Rust']], actions: relocation }])
    .decorate('emitter', { format: 'Rust', source: raw }).source;
  const publicNames = [...relocated.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const loans = readPortableSourceParameterStyles(relocated, { publicNames });
  const reviewed = readPortableSourceLintStyles(loans.source);
  const styles = new DecoratorSet([{ id: name + '-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions: [...relocation, ...loans.actions, ...reviewed.actions] }]);
  const styleFile = new URL('../../parity/self-translation/' + name.replaceAll('_', '-') + '-decorators.lino', import.meta.url);
  if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'record family library styles retain every actual declaration');
  else writeFileSync(styleFile, styles.toLino());
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.equal(decorated, reviewed.source, 'the recorded ownership, loan and lint styles replay exactly');
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
