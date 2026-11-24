// Run with Node's heap bounded to 512 MiB. The native regression's original
// 512 graphs are finite. Compare the default and an explicit finite node limit.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {runInNewContext} from 'node:vm';
import {readDeclarationDependencySourceClosure} from '../../js/scripts/generate-declaration-dependency-rules.mjs';
import {checkProgram} from '../../js/src/translation/check.js';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks} from '../../js/src/typed-program-links.js';

const nativeRegressionSource = () => {
  let source = readDeclarationDependencySourceClosure().source;
  for (let mask = 0; mask < 512; mask += 1) {
    const starts = [], counts = [], targets = [];
    for (let owner = 0; owner < 3; owner += 1) {
      const start = targets.length;
      starts.push(start);
      for (let target = 0; target < 3; target += 1)
        if (mask & (1 << (owner * 3 + target))) targets.push(target);
      counts.push(targets.length - start);
    }
    const rustArray = values => '[' + values.join(', ') + ']';
    source += '\nconst case' + mask + '=readDeclarationDependencyComponents('
      + [starts, counts, targets].map(rustArray).join(',') + ');console.log(case'
      + mask + '[0],case' + mask + '[1],case' + mask + '[2]);\n';
  }
  source += "\nconsole.log(acceptDeclarationModuleScope(['comment','named function declaration','named import','primitive literal binding'],false));\n";
  for (const form of ['var declaration', 'assignment', 'eval', 'namespace import', 'const binding list', 'class declaration', 'unknown module form']) {
    const literal = JSON.stringify(form);
    source += "\nconsole.log(acceptDeclarationModuleScope(['named function declaration'," + literal
      + "],false));console.log(acceptDeclarationModuleScope(['named function declaration'," + literal + "],true));\n";
  }
  source += "\nconsole.log(acceptDeclarationModuleScope(['empty class declaration'],false));console.log(acceptDeclarationSignatureBinding(true,1));\n";
  for (const count of [0, 2])
    source += '\nconsole.log(acceptDeclarationSignatureBinding(true,' + count
      + '));console.log(acceptDeclarationSignatureBinding(false,' + count + '));\n';
  source += '\nconsole.log(acceptDeclarationSignatureBinding(false,1));\n';
  return source;
};

const source = nativeRegressionSource();
assert.ok(Buffer.byteLength(source) < 160_000);
function observe(sourceText) {
  const lines = [];
  runInNewContext(sourceText.replace(/^export /gmu, ''), {
    console: {log(...values) { lines.push(values.map(String).join(' ')); }},
  }, {timeout: 5000});
  return lines;
}
const observed = observe(source);
assert.equal(observed.length, 534);
const checked = checkProgram(parseJavaScript(source));
const pending = [checked];
let values = 0, containers = 0, scalars = 0;
while (pending.length) {
  assert.ok(++values <= 500_000 && pending.length < 10_000, 'finite checked source measurement');
  const value = pending.pop();
  if (value && typeof value === 'object') {
    containers += 1;
    pending.push(...(value instanceof Map ? [...value].flat() : Object.values(value)));
  } else {
    scalars += 1;
  }
}
console.log(JSON.stringify({source: createHash('sha256').update(source).digest('hex'),
  bytes: Buffer.byteLength(source), logs: observed.length, values, containers, scalars}));
const limits = {nodeLimit: 250_000};
const encoded = createTypedProgramLinks(source, 'JavaScript', limits);
assert.throws(() => decodeTypedProgramLinks(encoded.network, encoded.root), /structural resource limit/u);
const decoded = decodeTypedProgramLinks(encoded.network, encoded.root, limits);
assert.ok(isDeepStrictEqual(decoded, checked), 'the complete decoded program equals the independently checked original');
const code = emitTypedProgramLinks(encoded.network, encoded.root, 'JavaScript', limits).code;
assert.deepEqual(observe(code), observed);
console.log('The original complete source and all 534 output lines survive the explicit 250,000-node JavaScript limit.');
