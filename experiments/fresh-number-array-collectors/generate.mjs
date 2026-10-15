import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { DecoratorSet } from '../../js/src/decorators.js';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { freshNumberArrayCollectorPolicy as policy } from './policy.mjs';

export function freshNumberArrayCollectorRuleSource() {
  return `// Generated from the complete linked collector grammar. Do not edit.
/** @param {number[]} tokens @param {number[]} pattern @returns {boolean} */
export function acceptFreshNumberArrayCollectorPattern(tokens, pattern) {
  if (tokens.length !== pattern.length || tokens.length > ${policy.maximumTokens}) return false;
  let fn = -1;
  let input = -1;
  let collector = -1;
  let index = -1;
  for (let position = 0; position < tokens.length; position += 1) {
    const actual = tokens[position] ?? -99;
    const expected = pattern[position] ?? -99;
    if (!Number.isSafeInteger(actual) || actual < 0 || actual >= ${policy.nameCodeMinimum + policy.maximumTokens}) return false;
    if (expected >= 0) {
      if (actual !== expected) return false;
    } else {
      if (actual < ${policy.nameCodeMinimum}) return false;
      if (expected === -1) { if (fn < 0) fn = actual; else if (actual !== fn) return false; }
      else if (expected === -2) { if (input < 0) input = actual; else if (actual !== input) return false; }
      else if (expected === -3) { if (collector < 0) collector = actual; else if (actual !== collector) return false; }
      else if (expected === -4) { if (index < 0) index = actual; else if (actual !== index) return false; }
      else return false;
    }
  }
  return fn >= ${policy.nameCodeMinimum} && input >= ${policy.nameCodeMinimum}
    && collector >= ${policy.nameCodeMinimum} && index >= ${policy.nameCodeMinimum}
    && fn !== input && fn !== collector && fn !== index
    && input !== collector && input !== index && collector !== index;
}
`;
}

export function freshNumberArrayCollectorDecorators() {
  return new DecoratorSet([{
    id: 'lower-certified-fresh-number-array-collector', level: 'transformation', order: 0,
    when: [['model', policy.model], ['role', policy.lowering.role]],
    actions: ['bindingForm', 'operation', 'argumentEvaluation'].map(field =>
      ({ op: 'set', field, value: policy.lowering[field] })),
  }]).toLino();
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const check = process.argv.includes('--check');
  const source = freshNumberArrayCollectorRuleSource();
  const files = [
    [new URL('./data/policy.lino', import.meta.url), policy],
  ];
  for (const [file, value] of files) {
    if (check) checkProjectData(file, value); else writeProjectData(file, value);
  }
  for (const [file, value] of [
    [new URL('./rules.mjs', import.meta.url), source],
    [new URL('./data/lowering.lino', import.meta.url), freshNumberArrayCollectorDecorators()],
  ]) {
    if (check) { if (readFileSync(file, 'utf8') !== value) throw new Error('collector source or decorators are stale'); }
    else writeFileSync(file, value);
  }
  const checked = checkProgram(parseJavaScript(source));
  const rust = emitRust(checked);
  const translation = { model: policy.model, source, declarations: checked.items.map(entry => entry.fullName),
    definitions: rust.definitions, completeRust: rust.text };
  const file = new URL('./data/rule-translation.lino', import.meta.url);
  if (check) checkProjectData(file, translation); else writeProjectData(file, translation);
  console.log('Complete collector admission rule translated through the production JavaScript parser/checker/Rust emitter.');
}
