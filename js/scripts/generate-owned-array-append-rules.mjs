// Translate the complete checked ownership decisions with their actual module
// dependency. The common library writer records only ownership/loan styles.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';

const sourceFile = new URL('../src/translation/owned-array-append-rules.js', import.meta.url);
const factsFile = new URL('../src/translation/fact-column-rules.js', import.meta.url);
const requiredImport = "import { readRequiredNumberFact } from './fact-column-rules.js';\n";
const hash = source => createHash('sha256').update(source).digest('hex');

// Complete original inputs. Values are independent source-oracle expectations;
// normal generation stores this inventory as Links for both runtime tests.
export function readOwnedArraySelfSliceFixtures() {
  const declarations = [
    ['one bound', '/** @returns {string} */ function selfSliceOne(){let values=[1,2,3];values=values.slice(1);values.push(4);return values.join(",");}', 'selfSliceOne()', '2,3,4\n'],
    ['no bounds', '/** @returns {string} */ function selfSliceAll(){let values=[1];values=values.slice();values.push(2);return values.join(",");}', 'selfSliceAll()', '1,2\n'],
    ['stack loop', '/** @returns {string} */ function selfSliceStack(){let values=[1];for(let index=0;index<4;index+=1){const next=values[values.length-1]??0;values=values.slice(0,values.length-1);values.push(next+1);}return values.join(",");}', 'selfSliceStack()', '5\n'],
    ['branch join', '/** @returns {string} */ function selfSliceBranch(){let values=[1,2,3];for(let index=0;index<3;index+=1){if(index%2===0){values=values.slice(1);}else{values=values.slice(0,values.length-1);}values.push(index+4);}return values.join(",");}', 'selfSliceBranch()', '3,5,6\n'],
    ['nested loops', '/** @returns {string} */ function selfSliceNested(){let values=[0];for(let outer=0;outer<3;outer+=1){for(let inner=0;inner<3;inner+=1){values=values.slice(1);values.push(outer*10+inner);}}return values.join(",");}', 'selfSliceNested()', '22\n'],
    ['lexical shadow', '/** @returns {string} */ function selfSliceShadow(){let values=[1];{let values=[9,8];values=values.slice(1);values.push(7);}values.push(2);return values.join(",");}', 'selfSliceShadow()', '1,2\n'],
    ['bound effects exactly once', '/** @param {number} value @returns {number} */ function selfSliceBound(value){console.log("bound",value);return value;}\n/** @returns {string} */ function selfSliceEffects(){let values=[1,2,3];values=values.slice(selfSliceBound(1),selfSliceBound(3));values.push(4);return values.join(",");}', 'selfSliceEffects()', 'bound 1\nbound 3\n2,3,4\n'],
  ];
  return declarations.map(([id, source, call, expected]) => ({ id, source: source + '\nconsole.log(' + call + ');', expected }));
}

export function readOwnedArraySelfSliceRefusals() {
  return [
    ['prior alias', 'const alias=values;values=values.slice(1);values.push(4);return alias.join(",")+"|"+values.join(",");', '1,2,3|2,3,4\n'],
    ['later loop alias', 'for(let index=0;index<2;index+=1){values=values.slice(1);values.push(index+4);const alias=values;}return values.join(",");', '3,4,5\n'],
    ['foreign assignment', 'const other=[8];values=other;values.push(4);return other.join(",");', '8,4\n'],
    ['foreign slice', 'const other=[8,9];values=other.slice(1);values.push(4);return values.join(",");', '9,4\n'],
    ['concat remains outside profile', 'values=values.concat([4]);values.push(5);return values.join(",");', '1,2,3,4,5\n'],
    ['identity observation', 'const same=values===values;values=values.slice(1);values.push(4);return same;', 'true\n'],
    ['captured owner', 'const capture=()=>values;values=values.slice(capture().length);values.push(4);return values.join(",");', '4\n'],
    ['escaping bound argument', 'values=values.slice(selfSliceObserve(values));values.push(4);return values.join(",");', '4\n'],
  ].map(([id, body, expected]) => ({ id,
    source: '/** @param {number[]} input @returns {number} */ function selfSliceObserve(input){const alias=input;return alias.length;}\nfunction selfSliceRefusal(){let values=[1,2,3];' + body + '}\nconsole.log(selfSliceRefusal());', expected }));
}

export function readOwnedArrayAppendSourceModule() {
  const source = readFileSync(sourceFile, 'utf8');
  assert.equal(source.split(requiredImport).length, 2, 'one actual required-column module edge is resolved');
  return readFileSync(factsFile, 'utf8') + '\n' + source.replace(requiredImport, '');
}

export async function generateOwnedArrayAppendRules({ check = false, native = false } = {}) {
  const source = readFileSync(sourceFile, 'utf8');
  const facts = readFileSync(factsFile, 'utf8');
  const closure = readOwnedArrayAppendSourceModule();
  const policy = {
    schemaVersion: 1,
    model: 'fresh local array construction and final binding consumption 1',
    source: 'js/src/translation/owned-array-append-rules.js',
    sourceSha256: hash(source),
    decisions: ['acceptOwnedArrayAppendShape', 'acceptOwnedArrayRemovalShape', 'readOwnedArrayRemovalSliceBounds', 'acceptOwnedArrayRead', 'acceptOwnedArrayReaderResult',
      'acceptOwnedArrayAppendUses', 'acceptOwnedArrayFinalUse', 'acceptOwnedArraySelfSliceRebinding'],
    assertionVersions: {Node: '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0'},
    selfSliceFixtures: readOwnedArraySelfSliceFixtures(), selfSliceRefusals: readOwnedArraySelfSliceRefusals(),
    sourceClosure: { source: closure, sourceSha256: hash(closure), removedImport: requiredImport,
      inputs: [{ path: 'js/src/translation/owned-array-append-rules.js', sourceSha256: hash(source) },
        { path: 'js/src/translation/fact-column-rules.js', sourceSha256: hash(facts) }] },
    boundary: 'An exact self-slice assignment (zero to two arguments) may retain its current fresh lexical owner only after the complete RHS read traversal; every existing escape remains. Other assignments remain escapes. Only actual fresh source allocations with no earlier or loop-carried escape lower to immutable array value rebinding. A native storage move additionally requires no reachable later read of the same actual renamed binding. Links reload retains ordinary immutable value semantics.',
  };
  for (const file of [new URL('../src/translation/data/owned-array-append.lino', import.meta.url),
    new URL('../../rust/src/data/owned-array-append.lino', import.meta.url)]) {
    if (check) checkProjectData(file, policy);
    else writeProjectData(file, policy);
  }
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const program = checkProgram(parseJavaScript(closure, { referenceSemantics: false }));
    const emitted = emitRust(program, { arrayConsumption: true });
    assert.equal(emitted.definitions.length, program.declarations.size, 'every actual declaration and generated loop helper is retained');
    for (const decision of policy.decisions) assert.ok(program.declarations.has(decision), decision);
    writeRecordRuleLibrary(emitted, 'owned_array_append_rules', { check });
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateOwnedArrayAppendRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('Owned array construction: actual fresh allocations, escape relations and final lexical consumption');
}
