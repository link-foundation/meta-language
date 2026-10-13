#!/usr/bin/env node
// Generate structural admission from the JavaScript policy, without compiling.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { DecoratorSet } from '../src/decorators.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const input = new URL('../src/concept-view-rules.js', import.meta.url);
const output = new URL('../../rust/src/concept_ontology/view_rules.rs', import.meta.url);
const program = checkProgram(parseJavaScript(readFileSync(input, 'utf8')));
const translated = emitRust(program);
const styleFile = new URL('../../parity/self-translation/concept-view-rules-decorators.lino', import.meta.url);
if (program.main.effects.length !== 0 || program.main.result !== undefined) throw new Error('a concept view library policy has executable main code');
if (process.argv.includes('--update-styles')) {
  const boundary = translated.text.indexOf('\nfn ml_main() {\n\n}\n');
  if (boundary < 0 || translated.definitions.some(definition => !translated.text.slice(0, boundary).includes(definition))) throw new Error('the empty library harness cannot be extracted without retaining all policy functions');
  let styled = translated.text;
  const actions = [];
  const replace = (from, to) => {
    if (!styled.includes(from)) throw new Error('a reviewed concept view style source is missing');
    actions.push({ op: 'replace', field: 'source', from, to }); styled = styled.split(from).join(to);
  };
  for (const name of ['role', 'arity', 'view', 'kind']) replace(`${name}: String`, `${name}: &str`);
  replace(translated.text.slice(boundary), '\n');
  // Reuse the discovery generator's balanced else-if extraction. Expression
  // parentheses and both branch bodies remain unchanged.
  for (;;) {
    const match = /\} else \{\n\s+if /u.exec(styled);
    if (!match) break;
    const opening = match.index + '} else '.length;
    let depth = 1; let cursor = opening + 1; let quoted = false;
    for (; cursor < styled.length && depth; cursor += 1) {
      const character = styled[cursor];
      if (quoted && character === '\\') { cursor += 1; continue; }
      if (character === '"') quoted = !quoted;
      if (!quoted && character === '{') depth += 1;
      if (!quoted && character === '}') depth -= 1;
    }
    if (depth) throw new Error('a translated concept view decision has an unclosed branch');
    replace(styled.slice(match.index, cursor), `} else ${styled.slice(opening + 1, cursor - 1).trim()}`);
  }
  for (const name of ['accept_concept_view_shape', 'accept_concept_view_relation']) replace(`pub fn ${name}`, `#[must_use]\npub fn ${name}`);
  writeFileSync(styleFile, new DecoratorSet([{ id: 'concept-view-library-boundary', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
const styles = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
let reviewed = translated.text;
for (const entry of styles.forLevel('emitter')) {
  for (const action of entry.actions) {
    if (action.op !== 'replace' || action.field !== 'source' || !reviewed.includes(action.from)) throw new Error('concept view boundary styles require review after source drift');
    reviewed = reviewed.split(action.from).join(action.to);
  }
}
const decorated = styles.decorate('emitter', { format: 'Rust', source: translated.text }).source;
if (decorated !== reviewed || /\bfn (?:ml_main|main)\(/u.test(decorated)) throw new Error('concept view library extraction did not preserve its reviewed boundary');
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/concept-view-rules.js by\n// js/scripts/generate-concept-view-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('concept view rules are stale; run node js/scripts/generate-concept-view-rules.mjs');
  console.log('generated concept view rules match JavaScript');
} else writeFileSync(output, text);
