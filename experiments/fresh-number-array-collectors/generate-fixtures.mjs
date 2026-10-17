import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import { checkProjectData, writeProjectData } from '../../js/scripts/project-data.mjs';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { createFreshNumberArrayCollector, FRESH_NUMBER_ARRAY_COLLECTOR_POLICY as policy } from './model.mjs';

// Independent original inputs and expected Number values, authored before the
// selected lowering is run. No expected value comes from an emitted program.
export function freshNumberArrayCollectorFixtures() {
  return [
    { id: 'input-order', name: 'collect', input: [4, -2, 0], expected: [4, -2, 0],
      source: '/** @param {number[]} values @returns {number[]} */\nfunction collect(values){const result=[];for(let index=0;index<values.length;index+=1){result.push(values[index]??-1);}return result;}' },
    { id: 'input-zero-fallback', name: 'copy', input: [9, 3], expected: [9, 3],
      source: '/** @param {number[]} numbers @returns {number[]} */\nfunction copy(numbers) { const output = []; for (let at = 0; at < numbers.length; at += 1) { output.push(numbers[at] ?? 0); } return output; }\n' },
    { id: 'argument-before-append', name: 'previousLengths', input: [8, 7, 6, 5], expected: [0, 1, 2, 3],
      source: '/** @param {number[]} input @returns {number[]} */\nfunction previousLengths(input){const output=[];for(let at=0;at<input.length;at+=1){output.push(output.length);}return output;}' },
    { id: 'source-index-scope', name: 'indices', input: [5, 5, 5], expected: [0, 1, 2],
      source: '/** @param {number[]} xs @returns {number[]} */\nfunction indices(xs){const ys=[];for(let position=0;position<xs.length;position+=1){ys.push(position);}return ys;}' },
    { id: 'constant-with-trivia', name: 'ones', input: [2, 3, 4], expected: [1, 1, 1],
      source: '/** @param {number[]} data @returns {number[]} */\n// α 🦉 original trivia remains in its own code-unit records.\nfunction ones(data) {\n const list = [];\n for(let cursor=0;cursor<data.length;cursor+=1) { list.push(1); }\n return list;\n}\n' },
  ];
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const fixture = { model: policy.model, examples: freshNumberArrayCollectorFixtures().map(entry => {
    const { program } = createFreshNumberArrayCollector(entry.source, { model: policy.model });
    const rust = emitRust(program);
    return { ...entry, sourceSha256: createHash('sha256').update(entry.source).digest('hex'),
      declarations: program.items.map(item => item.fullName), completeRust: rust.text, definitions: rust.definitions };
  }) };
  const file = new URL('./data/fixtures.lino', import.meta.url);
  if (process.argv.includes('--check')) checkProjectData(file, fixture); else writeProjectData(file, fixture);
  console.log('Five independent complete original collectors retain their complete automatically emitted Rust artifacts.');
}
