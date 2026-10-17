import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { translateProgram, readTranslationProvenance } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { STRING_PRELUDE } from '../src/translation/string-runtime.js';
import { readStringCodeUnit } from '../src/translation/string-code-unit-rules.js';
import { createStringCodeUnitTemplateExpression } from '../src/translation/string-code-unit-template.js';
import { readPrimitiveMethodMinimumArguments, readPrimitiveMethodMaximumArguments,
  readPrimitiveMethodDefaultForm, readPrimitiveMethodDefaultValue,
  readPrimitiveMethodCheckedArgumentKind, readPrimitiveMethodCheckedResultForm,
  readPrimitiveMethodLoweringForm } from '../src/translation/primitive-method-rules.js';
import { readProjectData } from '../scripts/project-data.mjs';
import { readStringCodeUnitTemplateGeneration } from '../scripts/generate-string-code-unit-template.mjs';

function observe(source) {
  const lines = [];
  vm.runInNewContext(source, { console: { log: (...values) => lines.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return lines;
}

const escapedSource = `/** @param {string} text @param {number} position @returns {number} */
function unit(text, position) { return text.charCodeAt(position); }
/** @param {string} text @param {number|null|undefined} position @returns {number} */
function nullableUnit(text, position) { return text.charCodeAt(position); }
console.log('ABC'.charCodeAt(), 'ABC'.charCodeAt(undefined), 'ABC'.charCodeAt(null));
console.log(unit('A😀B', 1), unit('A😀B', 2), unit('A😀B', 3));
console.log(unit('\\ud800x\\udfff', 0), unit('\\ud800x\\udfff', 2));
console.log(unit('ABC', -0.9), unit('ABC', 1.9), unit('ABC', NaN));
console.log(unit('ABC', -1), unit('ABC', 3), unit('', 0));
console.log(unit('ABC', -Infinity), unit('ABC', Infinity));
console.log(nullableUnit('ABC', null), nullableUnit('ABC', undefined), nullableUnit('ABC', 2));
`;
const escapedExpected = ['65 65 65', '55357 56832 66', '55296 57343', '65 66 65', 'NaN NaN NaN', 'NaN NaN', '65 65 67'];
// Retain the exact escaped-source fixture and all of its original VM
// observations below. Its unsupported literal decoder is distinct from
// passing the same UTF-16 values to a represented String parameter.
const escapedLiteralLine = escapedSource.split('\n').find(line => line.includes('ud800'));
assert.ok(escapedLiteralLine);
const source = escapedSource.split('\n').filter(line => line !== escapedLiteralLine).join('\n');
const expected = escapedExpected.filter((_, index) => index !== 2);

test('portable code unit source preserves numeric ToIntegerOrInfinity and individual UTF-16 surrogates', () => {
  const texts = ['', 'ABC', '\u0000\uffff', 'A😀B', '\ud800x\udfff', '\ud800\udfff'];
  const positions = [-Infinity, -9007199254740992, -2, -1, -0.9, -Number.MIN_VALUE, -0,
    0, Number.MIN_VALUE, 0.9, 1, 1.9, 2, 3, 4, 9007199254740992, Infinity, NaN];
  for (const text of texts) for (const position of positions) {
    assert.equal(readStringCodeUnit(text, position), text.charCodeAt(position), `${JSON.stringify(text)} at ${String(position)}`);
  }
  assert.equal(readStringCodeUnit('😀', 0), 0xd83d);
  assert.equal(readStringCodeUnit('😀', 1), 0xde00);
  assert.notEqual(readStringCodeUnit('😀', 0), '😀'.codePointAt(0));
});

test('charCodeAt checking preserves numeric defaults and surrogate parameters while source escapes remain explicit refusals', () => {
  assert.equal(readPrimitiveMethodMinimumArguments('string', 'charCodeAt'), 0);
  assert.equal(readPrimitiveMethodMaximumArguments('string', 'charCodeAt'), 1);
  assert.equal(readPrimitiveMethodDefaultForm('string', 'charCodeAt', 0), 'float');
  assert.equal(readPrimitiveMethodDefaultValue('string', 'charCodeAt', 0), '0');
  assert.equal(readPrimitiveMethodCheckedArgumentKind('string', 'charCodeAt', 0), 'number-value');
  assert.equal(readPrimitiveMethodCheckedResultForm('string', 'charCodeAt'), 'float');
  assert.equal(readPrimitiveMethodMinimumArguments('string', 'codePointAt'), 1);
  assert.equal(readPrimitiveMethodCheckedResultForm('string', 'codePointAt'), 'optional-float');
  for (const receiver of ['string', 'array', '', 'record']) for (const operation of ['charCodeAt', 'codePointAt', 'slice', '']) {
    assert.equal(readPrimitiveMethodLoweringForm(receiver, operation), receiver === 'string' && operation === 'charCodeAt' ? 'code-unit-template' : 'method');
  }
  assert.deepEqual(observe(escapedSource), escapedExpected);
  assert.throws(() => parseJavaScript(escapedSource), error => error.kind === 'unsupported'
    && error.reason.includes('not a scalar value') && error.span.start === escapedSource.indexOf('\\ud800'));
  const program = checkProgram(parseJavaScript(source));
  assert.deepEqual(observe(source), expected);
  assert.deepEqual(observe(emitJavaScript(program).text), expected);
  assert.ok(!emitJavaScript(program).text.includes('.charCodeAt('), 'checked emission uses the actual portable lambda operations');
  const original = vm.runInNewContext(source + ';unit;');
  const translated = vm.runInNewContext(emitJavaScript(program).text + ';unit;');
  for (const text of ['\ud800x\udfff', '\ud800', '\udfff', '\ud800\udfff', 'A😀B']) for (const position of [0, 1, 2, -0.9, Infinity, NaN]) {
    assert.equal(original(text, position), text.charCodeAt(position));
    assert.equal(translated(text, position), text.charCodeAt(position));
  }
});

test('charCodeAt preserves receiver and index call order while standalone absence annotations remain explicit refusals', () => {
  const annotated = `/** @returns {string} */function receiver(){console.log('receiver');return 'A😀B';}
/** @returns {number} */function index(){console.log('index');return 1.9;}
/** @returns {undefined} */function absent(){console.log('absent');return undefined;}
console.log(receiver().charCodeAt(index()));
console.log(receiver().charCodeAt(absent()));
console.log(receiver().charCodeAt());`;
  const lines = ['receiver', 'index', '55357', 'receiver', 'absent', '65', 'receiver', '65'];
  assert.deepEqual(observe(annotated), lines);
  assert.throws(() => checkProgram(parseJavaScript(annotated)), error => error.kind === 'type' && error.reason.includes('absence'));
  const ordered = annotated.replace('/** @returns {undefined} */', '');
  assert.deepEqual(observe(ordered), lines);
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(ordered))).text), lines);
  const shadowed = `/** @param {string} text @param {number} position @param {number} index @param {number} Math @param {number} Number @returns {number} */
function collision(text,position,index,Math,Number){const readStringCodeUnit=999;return text.charCodeAt(position)+index+readStringCodeUnit+Math+Number;}
console.log(collision('A😀B',1,7,4,5));`;
  assert.deepEqual(observe(shadowed), ['56372']);
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(shadowed))).text), ['56372']);
});

test('charCodeAt retains explicit conversion refusals outside the represented numeric domain', () => {
  for (const index of ['true', 'false', "'1'", '{}', '[1]']) {
    assert.throws(() => checkProgram(parseJavaScript("console.log('ABC'.charCodeAt(" + index + '));')),
      error => error.kind === 'type' || error.kind === 'unsupported');
  }
  assert.throws(() => checkProgram(parseJavaScript("console.log('ABC'.charCodeAt(0,1));")),
    error => error.kind === 'unsupported' && error.reason.includes('0 to 1 arguments'));
  const program = checkProgram(parseJavaScript("console.log('ABC'.charCodeAt(0));"));
  for (const target of ['Lean', 'Rocq']) {
    const result = translateProgram("console.log('ABC'.charCodeAt(0));", 'JavaScript', target);
    assert.match(result.diagnostic.message, /UTF-16 operational string encoding/u);
  }
  assert.equal(program.sourceLanguage, 'JavaScript');
});

test('charCodeAt source-free Links reload retains defaults, absence conversion and numeric observations', () => {
  const archive = createTypedProgramLinks(source, 'JavaScript');
  for (const network of [LinkNetwork.fromLino(archive.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(archive.network))]) {
    const program = decodeTypedProgramLinks(network, archive.root);
    recheckLinkedTypedProgram(program, { wholeProgram: true });
    const emitted = emitTypedProgramLinks(network, archive.root, 'JavaScript').code;
    assert.deepEqual(observe(emitted), expected);
    const replayed = vm.runInNewContext(emitted + ';unit;');
    for (const text of ['\ud800x\udfff', '\ud800', '\udfff', '\ud800\udfff', 'A😀B']) for (const position of [0, 1, 2, -0.9, Infinity, NaN]) {
      assert.equal(replayed(text, position), text.charCodeAt(position));
    }
    assert.ok(!JSON.stringify(program.items).includes('"op":"charCodeAt"'), 'the existing graph operations retain the expanded kernel without a new operation enum');
    assert.ok(network.links().every(link => !link.metadata().definition?.includes(source)), 'semantic replay stores no source program');
  }
  const numericSurface = parseJavaScript('/** @returns {number} */function converted(){return 0;}');
  for (const arg of [{ k: 'str', value: '1' }, { k: 'bool', value: true }, { k: 'unit' },
    { k: 'some', value: { k: 'str', value: '1' }, type: { kind: 'option', element: { kind: 'string' }, absence: 'undefined' } }]) {
    const invalid = structuredClone(numericSurface);
    invalid.items[0].body = { k: 'numberValue', arg };
    assert.throws(() => checkProgram(invalid), error => error.kind === 'type' || error.kind === 'unsupported',
      'the internal numeric projection retains the existing represented coercion domain');
  }
  const malformed = structuredClone(numericSurface);
  malformed.items[0].body = { k: 'numberValue' };
  assert.throws(() => checkProgram(malformed), TypeError, 'an internal numeric projection requires its actual argument');
});

test('the complete code unit kernel translates using only the retained UTF-16 runtime', () => {
  const complete = readFileSync(new URL('../src/translation/string-code-unit-rules.js', import.meta.url), 'utf8');
  const surface = parseJavaScript(complete, { referenceSemantics: false });
  const program = checkProgram(surface);
  const emitted = emitRust(program, { arrayConsumption: true });
  assert.deepEqual(emitted.preludes, [STRING_PRELUDE]);
  assert.equal(surface.items.length, 1);
  assert.equal(emitted.definitions.length, surface.items.length);
  assert.match(emitted.definitions[0], /pub fn read_string_code_unit/u);
  assert.match(emitted.definitions[0], /\.slice\(/u);
  assert.match(emitted.definitions[0], /\.code_point_at\(/u);
  const original = vm.runInNewContext(complete.replace('export function', 'function') + ';readStringCodeUnit;');
  const translated = vm.runInNewContext(emitJavaScript(program).text + ';readStringCodeUnit;');
  for (const text of ['', 'A😀B', '\ud800x\udfff']) for (const position of [-Infinity, -0.5, 0, 1.9, 2, Infinity, NaN]) {
    assert.equal(translated(text, position), original(text, position));
  }
});

test('generated code unit template pins its whole source, declarations, binders and unchanged base runtime', () => {
  const policy = readStringCodeUnitTemplateGeneration();
  const stored = readProjectData(new URL('../src/translation/data/string-code-unit-template.lino', import.meta.url));
  assert.deepEqual(stored, policy);
  assert.equal(readFileSync(new URL('../src/translation/data/string-code-unit-template.lino', import.meta.url), 'utf8'),
    readFileSync(new URL('../../rust/src/data/string-code-unit-template.lino', import.meta.url), 'utf8'));
  assert.match(readFileSync(new URL('../../rust/Cargo.toml', import.meta.url), 'utf8'),
    /^    "src\/data\/string-code-unit-template\.lino",$/mu);
  const digest = text => createHash('sha256').update(text).digest('hex');
  assert.equal(policy.sourceSha256, digest(policy.source));
  assert.equal(policy.basePreludeSha256, digest(STRING_PRELUDE));
  assert.deepEqual(policy.template.params, policy.originalDeclaration.params);
  assert.deepEqual(policy.template.body, policy.originalDeclaration.body);
  assert.deepEqual(policy.template.ret, policy.originalDeclaration.ret);
  assert.deepEqual(policy.checkedDeclaration.params.map(parameter => parameter.type.kind), ['string', 'float']);
  assert.equal(policy.checkedDeclaration.ret.kind, 'float');
  assert.equal(policy.inventory.length, 1);
});

test('charCodeAt Rust emission reuses only existing operations and refuses unproved reverse runtime interpretation', () => {
  const emitted = emitRust(checkProgram(parseJavaScript(source)));
  assert.ok(emitted.preludes.includes(STRING_PRELUDE));
  assert.ok(!emitted.text.includes('ml_string_code_unit'));
  assert.match(emitted.definitions.join('\n'), /\.code_point_at\(/u);
  assert.match(emitted.definitions.join('\n'), /\.slice\(/u);
  const ordinary = emitRust(checkProgram(parseJavaScript("console.log('ABC'.slice(1));")));
  assert.ok(ordinary.preludes.includes(STRING_PRELUDE));
  assert.ok(!ordinary.text.includes('ml_string_code_unit'));
  const archive = createTypedProgramLinks(source, 'JavaScript');
  const network = LinkNetwork.fromLino(archive.network.toLino());
  const linked = emitTypedProgramLinks(network, archive.root, 'Rust');
  const translated = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(translated.diagnostic, null);
  assert.equal(readTranslationProvenance(translated.code, 'Rust').sourceLanguage, 'JavaScript');
  assert.equal(linked.code, translated.code.slice(translated.code.indexOf('\n') + 1), 'only the authenticated provenance header differs');
  assert.match(linked.code, /\.code_point_at\(/u);
  for (const rust of [emitted.text,
    'fn forged(text: String, position: f64)->f64 { ml_string_code_unit::read_string_code_unit(text, position) } fn main(){}']) {
    assert.throws(() => checkProgram(parseRust(rust)), error => error.kind === 'type' || error.kind === 'unsupported');
    assert.ok(translateProgram(rust, 'Rust', 'JavaScript').diagnostic, 'a helper name never substitutes for its actual Rust semantics');
  }
});

function instantiateCapturedCodeUnitTemplate(policy, span) {
  // Execute the actual production instantiator. Only its module-loader edges
  // are supplied by this test; its recursive field/span decisions stay intact.
  const moduleUrl = new URL('../src/translation/string-code-unit-template.js', import.meta.url);
  const source = readFileSync(moduleUrl, 'utf8');
  const imported = "import { readRuntimeProjectData } from '../project-data-loader.js';";
  const declared = 'export function createStringCodeUnitTemplateExpression(span) {';
  for (const fragment of [imported, declared, 'import.meta.url']) {
    assert.equal(source.split(fragment).length, 2, 'one exact production module transport edge');
  }
  const executable = source.replace(imported, '').replace(declared,
    'function createStringCodeUnitTemplateExpression(span) {').replace('import.meta.url', 'moduleUrl');
  let reads = 0;
  const result = vm.runInNewContext(executable + '\ncreateStringCodeUnitTemplateExpression(callerSpan);', {
    URL, moduleUrl: moduleUrl.href, callerSpan: span,
    readRuntimeProjectData(address) {
      reads++;
      assert.equal(address.href, new URL('../src/translation/data/string-code-unit-template.lino', import.meta.url).href);
      return policy;
    },
  }, { timeout: 1000 });
  assert.equal(reads, 1, 'the complete captured template is the only supplied data');
  return structuredClone(result);
}

test('historical code unit templates preserve complete metadata and agree through actual caller span instantiation', () => {
  const historicalPath = new URL('../../parity/fixtures/historical/frontend-const-lists/string-code-unit-template.lino', import.meta.url);
  const historicalBytes = readFileSync(historicalPath);
  const digest = value => createHash('sha256').update(value).digest('hex');
  // Exact complete Git blob extracted from this original revision; this is
  // retained independently of the newly generated runtime copies.
  const originalRevision = '858665058648a3f1f9ce05ff3b0668b9ccfd66eb';
  assert.equal(historicalBytes.length, 110810);
  assert.equal(digest(historicalBytes), '549da9c740846c976a077aed192918e62a1f8ecd9b63109739763b196b553c4f',
    'the whole original template at ' + originalRevision);
  const historical = readProjectData(historicalPath);
  const current = readStringCodeUnitTemplateGeneration();
  const sourceSha256 = '171561add319a13fc6b28af7c9121db192e9983f53fca209ff3bbb0c17c35d9d';
  const basePreludeSha256 = 'dcc6da567430fa71a318f5d5d9beb873f23e009dfc7d9a7c1bfb66a0e9231ea5';
  assert.equal(digest(historical.source), sourceSha256);
  assert.equal(historical.sourceSha256, sourceSha256);
  assert.equal(current.sourceSha256, sourceSha256);
  assert.equal(readFileSync(new URL('../src/translation/string-code-unit-rules.js', import.meta.url), 'utf8'), historical.source);
  assert.equal(historical.basePreludeSha256, basePreludeSha256);
  assert.equal(current.basePreludeSha256, basePreludeSha256);
  assert.equal(digest(STRING_PRELUDE), basePreludeSha256);
  assert.deepEqual(historical.inventory, [{ kind: 'fn', name: 'readStringCodeUnit', span: { start: 227, end: 233 } }]);
  assert.deepEqual(current.inventory, historical.inventory);
  assert.equal(historical.source[345], ';');
  assert.equal(historical.source.slice(345, 349), ';\n  ');
  assert.equal(historical.source.slice(349, 351), 'if');
  const expectedCurrent = structuredClone(historical);
  for (const field of ['originalDeclaration', 'checkedDeclaration', 'template']) {
    assert.deepEqual(historical[field].body.span, { start: 282, end: 349 });
    assert.deepEqual(current[field].body.span, { start: 282, end: 345 });
    expectedCurrent[field].body.span.end = 345;
  }
  assert.deepEqual(current, expectedCurrent,
    'every source, declaration, operation, binder, type and metadata field is retained except exactly three authenticated endpoints');
  const caller = parseJavaScript(source, { referenceSemantics: false }).items[0].body;
  assert.equal(caller.k, 'method');
  assert.equal(caller.op, 'charCodeAt');
  assert.ok(caller.span.end > caller.span.start);
  const oldExpression = instantiateCapturedCodeUnitTemplate(historical, caller.span);
  const currentExpression = instantiateCapturedCodeUnitTemplate(current, caller.span);
  assert.deepEqual(currentExpression, oldExpression,
    'the actual production instantiator sets caller spans without deleting or changing any executable field');
  assert.deepEqual(currentExpression, createStringCodeUnitTemplateExpression(caller.span));
  assert.deepEqual(oldExpression.span, caller.span);
  assert.deepEqual(oldExpression.body.span, caller.span);
  const emitted = [oldExpression, currentExpression].map(template => {
    const surface = parseJavaScript(historical.source, { referenceSemantics: false });
    surface.items[0] = { ...surface.items[0], params: template.params, body: template.body, ret: template.ret };
    return emitJavaScript(checkProgram(surface));
  });
  assert.deepEqual(emitted[0], emitted[1], 'both complete instantiated templates emit identical programs and provenance');
  const original = vm.runInNewContext(emitted[0].text + ';readStringCodeUnit;');
  const translated = vm.runInNewContext(emitted[1].text + ';readStringCodeUnit;');
  for (const text of ['', 'ABC', 'A😀B', '\ud800x\udfff']) for (const position of [-Infinity, -0.9, 0, 1.9, 2, Infinity, NaN]) {
    assert.equal(original(text, position), text.charCodeAt(position));
    assert.equal(translated(text, position), text.charCodeAt(position));
  }
  assert.deepEqual(historical.template.body.span, { start: 282, end: 349 }, 'the historical template was not rewritten');
});
