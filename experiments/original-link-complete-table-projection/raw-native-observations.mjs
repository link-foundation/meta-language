// CI-only complete-artifact observations. Importing creates no graph or process.
// Planning and injected contracts require CI; real execution also requires the native flag.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import vm from 'node:vm';
import {createSourceNativeObservationRunner} from '../source-native-observations.mjs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {verifyOriginalLinkRawSchemaSourceClosure} from './raw-schema-source-closure.mjs';
import {reloadCanonicalArchive, readCanonicalArchiveTransportSource} from './archive-transport.mjs';

// Actual immutable producer and source closure; no capability declaration substitutes for execution.
export const ORIGINAL_LINK_RAW_MODEL_COMMIT = '9123dd5d19fc26c95a9d31eff47dd9655c33da53';
const sourceDigests = {
  "policy.mjs": "e869b0164d92e28c4e34277ec0ac9a24e0476ee5e104042748f2f35a3e959970",
  "rules.mjs": "da3815634f0ec09afdefd432c817e0368c73749fd6ca88fbc86db82f8beb5952",
  "generate.mjs": "92afd9fb275c3e3c04a84728f99a224d0ea2226d658a1484efac0a9112856d4f",
  "transport.mjs": "6975fd4431fa0631197c9af72a5be1abef92a62c250e42d5dc20ea3d89f5d7b2",
  "data/policy.lino": "d5414508fe555a1f87a732fd2fd3b3e32b6fed5134ed30268b7e8c10df5ff212",
  "data/native-definitions.lino": "8a844644c5e0e1f4211a270e23245ffa0b5d36344cea4d26d29460fe50bde224",
  "../original-link-table-projection/policy.mjs": "cb0e58a2f878f753f36e7b9ac0755277c9634cb90aeb9d67269342c471e5ea20",
  "../original-link-table-projection/rules.mjs": "7cb4eef983d851a4fa9da784c931cb826c33aad992eb166462a8679b267802f7",
  "../original-link-table-projection/transport.mjs": "fbe742bd90f9e3a3ed0d550060b2bf156a978c632f6f3fe5f652623c6c8d6212",
  "../original-link-table-projection/data/policy.lino": "1babbc5bb563be80887013cd531e3d56d60daea5a53d307445edd2052033f508",
  "../original-link-role-projection/rules.mjs": "74db55cf5fff7d9268c16b6dbfe93504081a614f5308a2ad731619765ce74944",
  "../original-link-role-projection/transport.mjs": "c265f3d6c732369eb00632d95e5dafb1b02b484e7e68e87cfce81d02f20df22b",
  "../original-link-role-projection/data/policy.lino": "be7e8a265748909af5bd92f6272510ab81a8947df7e0d21c3217233fd6f39b01",
  "../original-link-identity/policy.mjs": "080d379c8c97696fb611f841b1d7dba220864e4cd82984febb5c550e7bdd46e5",
  "../original-link-identity/rules.mjs": "ac1d9dae9835c1fe5b089d96a163ad3aaf0ac89ef6f29faccf0fc19c700727ba",
  "../original-link-symbol-projection/policy.mjs": "b88a1c3b2f51d2f28411ab43559b22cae585267192d70790a54457bbc21c4428",
  "../original-link-symbol-projection/rules.mjs": "dea761345d761f6d58ecb564893134ccdc149b19eaf89d51ef7dcbe78e4a9448",
  "../original-link-symbol-projection/data/policy.lino": "ece8ece9be52b5917944ebd53d604d956ad4e973880ee7f719e8b5c620db152f",
  "../rust-text-certificate/bridge.mjs": "c3121d3206ec0c6fafa1ac4506acc73012e245509b76c14bb77d696d11808e4d",
  "../rust-text-certificate/rules.mjs": "30f2e26a2f2b8d9b30de14a9ffed93a72c2b1e1edd6a95e717dafa36226e1f19",
  "../rust-text-certificate/data/policy.lino": "ee66558495d3eea1c022b1ca68f4e0a3ebba9e7ebb501001f8dff5a432ce8e14",
  "../rust-text-certificate-adapter/raw-identity.mjs": "09da17255d14999d85eaae3abae4c9789a082d636a3d30d9493a9704557eed94",
  "../rust-text-value-projection/model.mjs": "d2be6a924798984c3393adf1050ebbb74b8159be79e8392e38e84b5bbcf7db56",
  "../rust-text-value-projection/source.mjs": "6d64305d0a5b65de68e36d6e62db25b5f6bf6822b7ed3d0ca47ba7790a6565f5",
  "../rust-text-value-projection/rules.mjs": "890472bec11e6095ffcca50fff363ae0d5538bf1cd639f61f645051d970f952b",
  "../rust-text-value-projection/data/policy.lino": "0287e8b72b3b73fd82c5deac38f78a191e9426b3f5f7ed47c043f15c68da7272",
  "../rust-text-value-projection/data/fixtures.lino": "6fd937e41d61f0fcd2bae6fb9b2fd9d0d358e775234408cf434843e853f8bce4",
  "../generated-owned-rust/model.mjs": "482a8ffecc9e17c3bf3ae0eb4ef883453e61391204d9e5310741e9a3c9389524",
  "../generated-owned-rust/codec.mjs": "b9364434b7d521f3bd2457d7e2313b63a94ef303c03fc88f1f3a2d9a02904413",
  "../generated-owned-rust/parser.mjs": "0b2c4d49209fb41e54c43ba4389c76d3aba14307bf56684c4f93e84f39ea9d3f",
  "../generated-owned-rust/rules.mjs": "422d5af1ba85942184297516a49612c015c7e78379a28f7a206ca4e875264ee2",
  "../generated-owned-rust/value-edit-rules.mjs": "863031615810d43f5a4ca91c434f65dbc8f8b739f19b4347948d9e1b7abf558f",
  "../generated-owned-rust/trivia.mjs": "4674afab71a49a5016415a0e9875e7bdd89c48e29f7b6f446d7ea2e32d745d09",
  "../generated-owned-rust/data/policy.lino": "7ba85399443765c98641ef9b570a91a42822622a8b038891dc89bc471e42c13c",
  "../source-native-observations.mjs": "7375572e830c9d7a686efe84fb57111cec7861a334851ecad8b5604cf86440a8",
  "raw-policy.mjs": "f4707a1b4b82148b21e21addaf872ab206e6e858be78643ba197f3d9651ef3b2",
  "raw-rules.mjs": "e4bafbec028815e973f2d3c0c683db6347d77d60b64548845f7669d01a0a35c5",
  "raw-transport.mjs": "2906032bcadb0573d1895f08918d8cdc2458b94ec9680ac2c9cacbd00e0997b1",
  "raw-fixtures.mjs": "07c44ecbc555abd98a0a31be2966fb97d3988971fc229a50ed48dbd298c019a6",
  "raw-corpus.mjs": "4efd775098d853ce1e5ef61ceadea90b34bc25cb863cfd1b0c9a2184fbbe6784",
  "generate-raw.mjs": "ba0794065bbbb8e96b4b24f16e977d1d137068fb6af2f61e6df5d2b8b1791f94",
  "data/raw-policy.lino": "3c2ccc49d80f69530987a63440882f1af507181364a891c13a9080ed2cc8ba2d",
  "data/raw-native-definitions.lino": "d4b2a0c42bba5ae8f29137ead5bc264b633fcb6fad953b13d2adb5fed2d9d6cd",
  "data/raw-translation-attempts.lino": "be3f51ee0ed2eba2366ddb01886e0025165378fdf867959eec63a0572e437f89",
  "../../js/src/data/typed-program-links-schema.lino": "377101200a12b4ada15e0a34a583900e8b6d62adb1fb685f9b2d6f161cacafdc",
  "../generated-owned-rust/data/value-edit-policy.lino": "74ee5344d0c05192f497b7672c0a26f193debce64a9ed338d986e5ebeb3fd8bb",
  "../issue-199-rust-string-continuation-adapter.mjs": "d28bfe1dca0c6b87a82bd6b2680cebebef5e3242ae42399f142025fec8e64de2"
};
// All forty-seven producer bindings above remain unchanged. The receipt keeps
// the original schema bytes while authenticating only an additive host schema.
const schemaClosureReceiptSha256 = '4279200cc6d7f1aa36aa7090461b3d2f09d3e711c42cee6c86d29e8bf8873375';
// Ordered inventory derived from the frozen fixture/corpus sources, without constructing graphs.
const frozenFixtureIds = [
  "authentic-owned-clone",
  "authentic-same-name-independent-parameters",
  "same-name-foreign-binding-retarget",
  "duplicate-selected-body-facet",
  "misplaced-sole-main-facet",
  "additional-main-facet",
  "additional-envelope-facet",
  "body-claimed-field-arity-2",
  "body-claimed-field-arity-4",
  "ordinary-field-label-self-link",
  "legacy-non-field-label-target",
  "proof-scalar-positional-child",
  "proof-scalar-named-child",
  "harmless-unrelated-root",
  "harmless-second-archive-root",
  "unselected-user-provenance-marker",
  "selected-user-provenance-marker",
  "harmless-metadata-flag-isError",
  "harmless-metadata-flag-hasError",
  "harmless-metadata-flag-isMissing",
  "harmless-metadata-flag-isExtra",
  "harmless-metadata-named-flag",
  "harmless-metadata-span",
  "harmless-metadata-flag-isError-with-hasError",
  "harmless-metadata-flag-isMissing-with-hasError",
  "ordinary-camel-bindingReference-structural-cycle",
  "ordinary-camel-declarationReference-structural-cycle",
  "ordinary-camel-datatypeReference-structural-cycle",
  "ordinary-camel-constructorReference-structural-cycle",
  "legacy-used-label-non-self-arity-1",
  "legacy-used-label-non-self-arity-2",
  "legacy-used-label-non-self-arity-4",
  "used-label-three-reference-dual-role",
  "shared-module-path-dag",
  "harmless-unrelated-dangling-reference",
  "empty-string-definition-undefined",
  "empty-string-definition-empty",
  "empty-string-definition-null",
  "duplicate-empty-map-key-null-undefined",
  "duplicate-empty-map-key-null-empty",
  "duplicate-empty-map-key-undefined-empty",
  "compatible-empty-list-cross-role-alias",
  "incompatible-nonempty-list-cross-role-alias",
  "original-source-owned-move",
  "original-source-owned-clone",
  "original-source-shared-string-clone",
  "original-source-borrowed-str-construction",
  "original-source-borrowed-str-owned",
  "original-source-borrowed-str-display",
  "original-source-cooked-literal-construction",
  "original-source-raw-literal-construction",
  "original-source-literal-owned",
  "original-source-raw-literal-display",
  "original-source-valid-clone-retarget",
  "original-source-invalid-slice-retarget",
  "original-source-equal-valued-distinct-owners",
  "coherent-edit-retarget",
  "coherent-edit-declaration rename",
  "coherent-edit-parameter rename",
  "coherent-edit-literal value",
  "foreign-owner-declaration",
  "cloned-owner-declaration",
  "foreign-owner-signature",
  "cloned-owner-signature",
  "foreign-owner-resultPosition",
  "cloned-owner-resultPosition",
  "foreign-owner-operationNode",
  "cloned-owner-operationNode",
  "foreign-owner-coreValue",
  "cloned-owner-coreValue",
  "foreign-owner-input",
  "cloned-owner-input",
  "foreign-owner-position",
  "cloned-owner-position",
  "actual-direct-number-metadata-NaN",
  "actual-direct-number-metadata-positive infinity",
  "actual-direct-number-metadata-negative infinity",
  "actual-direct-number-metadata-negative zero"
];
const frozenInventory = {fixtures: 78, formats: 195, numericPremises: 190, serializationRefusals: 5};
const independentJavaScriptControls = {
  testFile: 'experiments/original-link-complete-table-projection/raw-projection.test.mjs',
  controls: ['Number lexical slice adjacency', 'legacy Var names and semantic-reference conflicts', 'declaration mapping order and inventory mutations', 'normalized duplicate, unsafe and foreign role fields', 'transport materialization, sparse resource excess and active depth bounds', 'SameValueZero membership and strict positions', 'automatic generation freshness and authentic translation attempts'],
  scope: 'The required JavaScript suite runs all 55 groups independently. These additional standalone controls are not native observations of this oracle; its native graph scope is exactly the 78 shared lazy rows and their declared formats.',
};
const frozenNative = {
  model: 'original selected Rust text graph shape component 1', definitionCount: 106,
  sourceBytes: 351746, sourceSha256: '24db774177e047de0c0ea22adb2464e4e8481d03547f5a810b2d9fe317fb3046',
  functionName: 'check_original_link_raw_shape', parameterModes: ['f64', 'f64', 'Vec<Vec<f64>>'], resultMode: 'Vec<f64>',
};
// Read directly from the complete automatic artifact at the pinned producer commit.
const observedInterface = ['pub', 'fn', 'check_original_link_raw_shape', '(', 'root_high', ':', 'f64', ',', 'root_low', ':', 'f64', ',', 'columns', ':', 'Vec', '<', 'Vec', '<', 'f64', '>', '>', ')', '->', 'Vec', '<', 'f64', '>', '{'];
const checkedPlans = new WeakSet();
const sha256 = value => createHash('sha256').update(value).digest('hex');
const maximumProtocolBytes = 1024 * 1024, maximumConsumerBytes = 4 * 1024 * 1024;
const blockWidth = 256;

function textBytes(value) {
  assert.ok(typeof value === 'string' && value.isWellFormed(), 'native source is primitive scalar Unicode');
  return Buffer.from(value, 'utf8');
}
function freeze(value) {
  if (value && typeof value === 'object') { Object.values(value).forEach(freeze); Object.freeze(value); }
  return value;
}
function numberBits(value) {
  assert.equal(typeof value, 'number');
  const bytes = Buffer.alloc(8); bytes.writeDoubleBE(value); return bytes.toString('hex');
}
const argumentBits = args => [numberBits(args[0]), numberBits(args[1]), args[2].map(row => row.map(numberBits))];

export function requireOriginalLinkRawPlanning(environment = process.env) {
  if (environment.CI !== 'true') throw new TypeError('complete raw planning and injected contracts require CI=true');
}
export function originalLinkRawNativeMode(environment = process.env) {
  if (environment.ORIGINAL_LINK_RAW_NATIVE !== '1') return false;
  requireOriginalLinkRawPlanning(environment); return true;
}
export function verifyOriginalLinkRawNativeModel(read = readFileSync) {
  assert.match(ORIGINAL_LINK_RAW_MODEL_COMMIT ?? '', /^[0-9a-f]{40}$/u, 'raw producer commit is not frozen');
  return Object.entries(sourceDigests).map(([file, expected]) => {
    const bytes = read(new URL(file, import.meta.url)), actual = sha256(bytes);
    if (file === '../../js/src/data/typed-program-links-schema.lino') {
      const receiptBytes = read(new URL('./data/raw-schema-source-closure.lino', import.meta.url));
      assert.equal(sha256(receiptBytes), schemaClosureReceiptSha256, 'the frozen raw schema source-closure receipt changed');
      const closure = verifyOriginalLinkRawSchemaSourceClosure(readStructuredDataLino(receiptBytes.toString('utf8')), bytes, sourceDigests);
      assert.equal(closure.baselineSha256, expected);
      return {file, sha256: actual, producerSha256: expected, schemaSourceClosure: closure, sourceClosureReceiptSha256: schemaClosureReceiptSha256};
    }
    assert.equal(actual, expected, 'the frozen raw component changed: ' + file); return {file, sha256: actual};
  });
}

// Exact numeric transport only. No graph labels, roles or semantic fields are projected.
function createCarrier(observations) {
  const words = [], blocks = [], vectors = [], wordIndex = new Map(), blockIndex = new Map(), vectorIndex = new Map();
  const vector = bits => {
    const key = bits.join(''); if (vectorIndex.has(key)) return vectorIndex.get(key);
    const cells = bits.map(word => { assert.match(word, /^[0-9a-f]{16}$/u); if (!wordIndex.has(word)) { wordIndex.set(word, words.length); words.push(word); } return wordIndex.get(word); });
    const sequence = [];
    for (let start = 0; start < cells.length; start += blockWidth) {
      const block = cells.slice(start, start + blockWidth), key = block.join(',');
      if (!blockIndex.has(key)) { blockIndex.set(key, blocks.length); blocks.push(block); }
      sequence.push(blockIndex.get(key));
    }
    const index = vectors.length; vectorIndex.set(key, index); vectors.push(sequence);
    assert.deepEqual(sequence.flatMap(index => blocks[index].map(index => words[index])), bits, 'carrier reconstruction preserves every complete original vector bit');
    return index;
  };
  const invocations = observations.map(row => ({columns: row.inputBits[2].map(vector), result: vector(row.resultBits)}));
  const carrier = {words, blocks, vectors, invocations};
  observations.forEach((row, index) => {
    assert.deepEqual(invocations[index].columns.map(id => vectorBits(carrier, id)), row.inputBits[2], 'every reused complete input vector retains all original bits');
    assert.deepEqual(vectorBits(carrier, invocations[index].result), row.resultBits, 'every reused complete result retains all original bits');
  });
  return carrier;
}
const vectorBits = (carrier, index) => carrier.vectors[index].flatMap(index => carrier.blocks[index].map(index => carrier.words[index]));
const protocol = (observations, carrier) => observations.map((row, index) => [index, row.resultBits.length, ...carrier.vectors[carrier.invocations[index].result].map(value => value.toString(16))].join(':')).join('\n') + '\n';
function archiveCompleteRustSource(source, LinkNetwork, encodeBinary, decodeBinary) {
  const graph = LinkNetwork.parse(source, 'Rust');
  const textReplay = reloadCanonicalArchive(graph.toLino()).reconstructText(); assert.equal(textReplay, source);
  const binaryReplay = decodeBinary(encodeBinary(graph)).reconstructText(); assert.equal(binaryReplay, source);
  // The large temporary source graphs are not retained in the observation plan.
  return {sourceSha256: sha256(source), textReplaySha256: sha256(textReplay), binaryReplaySha256: sha256(binaryReplay)};
}

function nativeInterface(source, tokenize) {
  const tokens = tokenize(source, 'Rust').tokens, found = []; let depth = 0;
  for (let index = 0; index < tokens.length; index++) {
    if (depth === 0 && tokens[index].value === 'pub' && tokens[index + 1]?.value === 'fn' && tokens[index + 2]?.value === frozenNative.functionName) found.push(index);
    if (tokens[index].kind === 'punct' && tokens[index].value === '{') depth++;
    if (tokens[index].kind === 'punct' && tokens[index].value === '}') depth--;
  }
  assert.equal(depth, 0); assert.equal(found.length, 1, 'one actual complete raw entry point exists');
  assert.deepEqual(tokens.slice(found[0], found[0] + observedInterface.length).map(token => token.value), observedInterface, 'the actual emitted native ABI is exact');
  // This consumer uses the actual inspected value-owned nested-vector ABI.
  assert.deepEqual(frozenNative.parameterModes, ['f64', 'f64', 'Vec<Vec<f64>>']);
  assert.equal(frozenNative.resultMode, 'Vec<f64>');
  return {functionName: frozenNative.functionName, parameterOrder: ['root_high', 'root_low', 'columns'], parameterModes: frozenNative.parameterModes, resultMode: frozenNative.resultMode, interfaceSha256: sha256(observedInterface.join('\0'))};
}

function consumerSource(observations, carrier, header) {
  const vector = values => 'vec![' + values.join(',') + ']';
  const blocks = vector(carrier.blocks.map(vector)), vectors = vector(carrier.vectors.map(vector));
  const calls = observations.map((row, index) => {
    const invocation = carrier.invocations[index];
    return `{
let column_ids:Vec<usize>=${vector(invocation.columns)};
let columns:Vec<Vec<f64>>=column_ids.iter().map(|index| vector_bits(&words,&blocks,&vectors,*index).into_iter().map(f64::from_bits).collect()).collect();
for (column,index) in columns.iter().zip(&column_ids) {assert_eq!(column.iter().map(|value|value.to_bits()).collect::<Vec<_>>(),vector_bits(&words,&blocks,&vectors,*index),"complete carrier input differs");}
let root_high=f64::from_bits(0x${row.inputBits[0]}u64);let root_low=f64::from_bits(0x${row.inputBits[1]}u64);
let result=oracle_source::${header.functionName}(root_high,root_low,columns.clone());
let observed:Vec<u64>=result.iter().map(|value|value.to_bits()).collect();
assert_eq!(observed,vector_bits(&words,&blocks,&vectors,${invocation.result}),"complete raw decision differs");
assert_eq!(root_high.to_bits(),0x${row.inputBits[0]}u64);assert_eq!(root_low.to_bits(),0x${row.inputBits[1]}u64);
for (column,index) in columns.iter().zip(&column_ids) {assert_eq!(column.iter().map(|value|value.to_bits()).collect::<Vec<_>>(),vector_bits(&words,&blocks,&vectors,*index),"caller input changed");}
write!(&mut output,"${index}:{}",observed.len()).expect("invocation length");
for chunk in observed.chunks(${blockWidth}) {let block=blocks.iter().position(|block|block.len()==chunk.len()&&block.iter().zip(chunk).all(|(word,actual)|words[*word]==*actual)).expect("actual output block");write!(&mut output,":{block:x}").expect("output block");}
output.push('\\n');
}`;
  });
  return `extern crate oracle_source;
use std::fmt::Write as _;
// Reconstruct the authenticated exact-bit carrier, independently of the decision algorithm.
fn vector_bits(words:&[u64],blocks:&[Vec<usize>],vectors:&[Vec<usize>],index:usize)->Vec<u64>{let mut result=Vec::new();for block in &vectors[index]{for word in &blocks[*block]{result.push(words[*word]);}}result}
fn main(){let words:Vec<u64>=${vector(carrier.words.map(word => '0x' + word + 'u64'))};let blocks:Vec<Vec<usize>>=${blocks};let vectors:Vec<Vec<usize>>=${vectors};let mut output=String::new();
${calls.join('\n')}
print!("{output}");}
`;
}

// Only immutable texts, their interface, and declaration identities escape this
// scope. The checked surface/IR and duplicate artifact definitions are temporary.
function prepareOriginalLinkRawSource({readProjectData, parseJavaScript, checkProgram, emitRust, emitJavaScript, tokenize}) {
  const artifact = readProjectData(new URL('./data/raw-native-definitions.lino', import.meta.url)), source = readFileSync(new URL('./raw-rules.mjs', import.meta.url), 'utf8');
  const program = checkProgram(parseJavaScript(source, {referenceSemantics: false})), rust = emitRust(program);
  assert.equal(artifact.version, 1); assert.equal(artifact.model, frozenNative.model); assert.equal(rust.definitions.length, frozenNative.definitionCount);
  assert.equal(artifact.sourceSha256, sha256(source)); assert.deepEqual(artifact.declarationNames, program.items.map(row => row.fullName)); assert.deepEqual(artifact.declarations, rust.definitions); assert.equal(artifact.completeRust, rust.text);
  assert.equal(textBytes(rust.text).length, frozenNative.sourceBytes); assert.equal(sha256(rust.text), frozenNative.sourceSha256);
  const header = nativeInterface(rust.text, tokenize), emitted = emitJavaScript(program);
  const target = emitted.mappings.find(row => row.kind === 'function' && row.source === 'checkOriginalLinkRawShape')?.target;
  return freeze({model: artifact.model, source: rust.text, javascript: emitted.text, target, header,
    declarationNames: artifact.declarationNames, definitionCount: rust.definitions.length, preludeCount: rust.preludes.length});
}

/** Complete source replay before allocating any observation-matrix state. */
export function runOriginalLinkRawPlanningPhases({prepareSource, replaySource, buildMatrix}) {
  const prepared = prepareSource();
  const originalReceipt = replaySource(prepared.source);
  return buildMatrix(prepared, originalReceipt);
}

function buildOriginalLinkRawPlan(prepared, originalReceipt, modelSources, transportSources, {LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary, fixtures, corpus, transport, rules, oldDecoder, oldProjector, textProjection}) {
  const {header, target} = prepared, context = {};
  vm.runInNewContext(prepared.javascript, context, {timeout: 2000}); assert.equal(typeof context[target], 'function');
  const inventory = [...fixtures.originalLinkRawAdversarialFixtures(), ...corpus.originalLinkRawCertificateCorpus(), ...corpus.originalLinkRawNumericMetadataCorpus()];
  assert.deepEqual(inventory.map(row => row.id), frozenFixtureIds); assert.equal(new Set(frozenFixtureIds).size, frozenFixtureIds.length); assert.equal(inventory.length, frozenInventory.fixtures);
  const observations = [], observationIndex = new Map(), premises = [], serializationRefusals = [], hostChecks = [];
  const outcome = operation => { try { const result = operation(); return {kind: result === null || result?.decision?.[0] === 0 ? 'unselected' : 'accepted'}; } catch (error) { assert.ok(error instanceof Error); return {kind: 'refused', name: error.name, message: error.message}; } };
  const add = (id, args, premise, expectedKind) => {
    const inputBits = argumentBits(args), result = rules.checkOriginalLinkRawShape(...args), resultBits = result.map(numberBits);
    assert.deepEqual(argumentBits(args), inputBits, 'the original complete kernel preserves every numeric input bit');
    assert.deepEqual(Array.from(context[target](...args)).map(numberBits), resultBits, id); assert.deepEqual(argumentBits(args), inputBits, 'the complete emitted JavaScript kernel preserves every numeric input bit');
    const kind = result[0] === 0 ? 'unselected' : result[0] === 1 ? 'accepted' : 'refused'; if (expectedKind) assert.equal(kind, expectedKind, id);
    // Every genuine archive premise is authenticated before exact-bit deduplication.
    if (premise) premises.push({...premise, id, kind, inputSha256: sha256(JSON.stringify(inputBits)), resultBits});
    const key = JSON.stringify(inputBits);
    if (observationIndex.has(key)) { const previous = observations[observationIndex.get(key)]; assert.deepEqual(previous.resultBits, resultBits); if (premise) previous.premiseIndices.push(premises.length - 1); return; }
    observationIndex.set(key, observations.length); observations.push({id, kind, inputBits, resultBits, premiseIndices: premise ? [premises.length - 1] : []});
  };
  let authentic;
  let declaredFormatCount = 0;
  for (const fixture of inventory) {
    const original = fixture.create();
    const formats = fixture.formats ?? ['direct', 'text', 'binary'];
    assert.ok(formats.length > 0 && new Set(formats).size === formats.length); assert.ok(formats.every(format => ['direct', 'text', 'binary'].includes(format)));
    declaredFormatCount += formats.length;
    for (const format of formats) {
      const override = fixture.expected.archives?.[format];
      if (override === 'serialization-refused') {
        assert.equal(format, 'binary'); let refusal;
        assert.throws(() => encodeLinkNetworkBinary(original.network), error => { assert.ok(error instanceof Error); refusal = {name: error.name, message: error.message}; return true; });
        const originalInput = transport.readOriginalLinkRawTransport(original.network, original.root);
        serializationRefusals.push({fixtureId: fixture.id, format, originalRoot: original.root.asU64().toString(), inputSha256: sha256(JSON.stringify(argumentBits(originalInput.arguments))), refusal, contract: fixture.contract, scope: 'existing archive serializer refusal; no fabricated native transport or observation'});
        continue;
      }
      assert.ok(override === undefined || override === 'accepted' || override && typeof override === 'object', 'every format-specific contract is interpreted explicitly');
      const expected = {...fixture.expected, ...(override && typeof override === 'object' ? override : {})};
      const network = format === 'direct' ? original.network : format === 'text' ? LinkNetwork.fromLino(original.network.toLino()) : decodeLinkNetworkBinary(encodeLinkNetworkBinary(original.network));
      const before = network.toLino(), captured = transport.readOriginalLinkRawTransport(network, original.root);
      // Original graph capture precedes all pre-existing object decoders/projectors.
      const decoder = outcome(() => oldDecoder.decodeRustTextCertificateFacet(network, original.root));
      if (expected.oldDecoder) assert.equal(decoder.kind, expected.oldDecoder, fixture.id + ' ' + format);
      const projector = outcome(() => oldProjector.projectOriginalLinkCompleteTables(network, original.root));
      let selectedValue;
      const selected = outcome(() => { selectedValue = transport.checkOriginalLinkRawSelectedGraph(network, original.root); return selectedValue; });
      if (expected.selectedGraph) assert.equal(selected.kind, expected.selectedGraph, fixture.id + ' ' + format);
      if (expected.selectedGraph === 'accepted') { assert.equal(decoder.kind, 'accepted'); assert.equal(projector.kind, 'accepted'); }
      let execution = null;
      if (original.witness?.execution) {
        assert.equal(selected.kind, 'accepted');
        const witness = original.witness.execution, javascript = textProjection.emitRustTextValueProjection(selectedValue.checked.program, selectedValue.checked.proof, 'JavaScript').text, context = {};
        vm.runInNewContext(javascript, context, {timeout: 2000}); assert.equal(typeof context[witness.name], 'function');
        const actual = context[witness.name](...witness.args); assert.equal(actual, witness.expected, fixture.id + ' ' + format);
        execution = {...witness, actual, javaScriptSha256: sha256(javascript), scope: 'independent existing complete JavaScript emission after this actual graph reload'};
      }
      assert.equal(network.toLino(), before, 'independent existing checks preserve the original graph');
      add(fixture.id + ' ' + format, captured.arguments, {fixtureId: fixture.id, sourceFixtureId: fixture.sourceFixtureId ?? null, sourceSha256: fixture.sourceSha256 ?? null, witness: original.witness ?? null, execution, format, originalRoot: original.root.asU64().toString(), rootHigh: captured.arguments[0], rootLow: captured.arguments[1], originalIds: captured.anchors.map(id => id.asU64().toString()), networkSha256: sha256(before), decoder, projector, selected, contract: fixture.contract, scope: format === 'direct' ? 'actual direct JavaScript graph metadata; no archive preservation or native graph capture claim' : 'actual JavaScript LinkNetwork archive capture; no native graph capture claim'}, expected.rawShape);
      if (!authentic && fixture.id === 'authentic-owned-clone') authentic = captured.arguments;
    }
  }
  assert.ok(authentic); assert.equal(declaredFormatCount, frozenInventory.formats);
  assert.equal(premises.length, frozenInventory.numericPremises); assert.equal(serializationRefusals.length, frozenInventory.serializationRefusals);
  assert.equal(premises.length + serializationRefusals.length, declaredFormatCount);
  const genuineFormatPremiseCount = premises.length;
  for (let column = 0; column < authentic[2].length; column++) for (const mutation of ['String', 'hole']) {
    const columns = authentic[2].map(row => [...row]); assert.ok(columns[column].length);
    if (mutation === 'hole') delete columns[column][0]; else columns[column][0] = 'false';
    const result = rules.checkOriginalLinkRawShape(authentic[0], authentic[1], columns); assert.ok(result[0] < 0); assert.deepEqual(Array.from(context[target](authentic[0], authentic[1], columns)), result);
    hostChecks.push({column, mutation, resultBits: result.map(numberBits), scope: 'JavaScript host check only; Strings and sparse cells cannot inhabit native Vec<f64>'});
  }
  // Synthetic word translation tests arithmetic over the whole primitive graph.
  // It does not represent capture from a native high-ID graph implementation.
  assert.ok(authentic[2][0].every(value => value === 0)); assert.equal(authentic[0], 0);
  for (const high of [1, 4294967295]) {
    const columns = authentic[2].map(row => [...row]);
    for (const slot of [0, 2, 4, 14]) columns[slot] = columns[slot].map(() => high);
    add('synthetic high-word translation ' + high, [high, authentic[1], columns], {scope: 'synthetic full-u64 arithmetic only; no native graph capture', high}, 'accepted');
  }
  for (const column of [11, 12, 13]) for (const value of [NaN, Infinity, -1, 0.5, 10]) {
    const columns = authentic[2].map(row => [...row]); columns[column][0] = value;
    add('numeric metadata-tag corruption ' + column + ' ' + numberBits(value), [authentic[0], authentic[1], columns], {scope: 'synthetic numeric primitive transport; no actual graph capture', column, payloadBits: numberBits(value)}, 'refused');
  }
  for (const value of [NaN, -1, 0.5, 4294967296]) add('numeric root-word corruption ' + numberBits(value), [value, authentic[1], authentic[2].map(row => [...row])], {scope: 'synthetic numeric root-word transport only', payloadBits: numberBits(value)}, 'refused');
  const carrier = createCarrier(observations), consumer = consumerSource(observations, carrier, header), stream = protocol(observations, carrier);
  const sizes = {sourceBytes: textBytes(prepared.source).length, consumerBytes: textBytes(consumer).length, protocolBytes: Buffer.byteLength(stream), originalInputCells: observations.reduce((sum, row) => sum + row.inputBits[2].reduce((sum, cells) => sum + cells.length, 0), 0), uniqueWords: carrier.words.length, uniqueBlocks: carrier.blocks.length, uniqueBlockCells: carrier.blocks.reduce((sum, block) => sum + block.length, 0), vectors: carrier.vectors.length};
  process.stdout.write('Raw native oracle planned sizes: ' + JSON.stringify(sizes) + '\n');
  assert.ok(sizes.consumerBytes <= maximumConsumerBytes, 'complete consumer exceeds unchanged budget'); assert.ok(sizes.protocolBytes <= maximumProtocolBytes, 'complete result protocol exceeds unchanged budget');
  const entry = {id: 'complete original raw graph component', kind: 'whole automatically emitted raw component', fixtureId: 'experiments/original-link-complete-table-projection/data/raw-native-definitions.lino', source: prepared.source, sourceBytes: sizes.sourceBytes, sourceSha256: sha256(prepared.source), replaySha256: originalReceipt.binaryReplaySha256, ...header, declarationNames: prepared.declarationNames, definitionCount: prepared.definitionCount, preludeCount: prepared.preludeCount, fixtureInventory: frozenFixtureIds, inventoryCounts: frozenInventory, independentJavaScriptControls, originalReceipt, consumerSource: consumer, consumerSha256: sha256(consumer), javaScriptSha256: sha256(prepared.javascript), carrier, carrierSha256: sha256(JSON.stringify(carrier)), sizes, genuineFormatPremiseCount, serializationRefusals, premises, hostChecks, observations, observationScope: 'Bounded complete primitive decisions over actual JavaScript direct/archive transports and separately labeled synthetic u64 word controls. Failed serialization is separately observed, never converted into a native vector. Old decoders/projectors remain independently observed. No native LinkNetwork capture, public selector, source signature proof, whole-crate parity, completion or performance claim.'};
  const plan = freeze({model: prepared.model, modelCommit: ORIGINAL_LINK_RAW_MODEL_COMMIT, modelSources, transportSources, originalCount: 1, fixtureInventory: frozenFixtureIds, inventoryCounts: frozenInventory, entries: [entry]}); checkedPlans.add(plan); return plan;
}

export async function planOriginalLinkRawNative(environment = process.env) {
  requireOriginalLinkRawPlanning(environment);
  const modelSources = verifyOriginalLinkRawNativeModel();
  const transportSources = freeze([readCanonicalArchiveTransportSource()]);
  // Nothing above this point loads grammars, creates fixtures or reconstructs a CST.
  const [{readProjectData}, {LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary}, {parseJavaScript}, {checkProgram}, {emitRust}, {emitJavaScript}, {tokenize}, fixtures, corpus, transport, rules, oldDecoder, oldProjector, textProjection] = await Promise.all([
    import('../../js/scripts/project-data.mjs'), import('../../js/src/index.js'), import('../../js/src/translation/javascript.js'), import('../../js/src/translation/check.js'), import('../../js/src/translation/emit-rust.js'), import('../../js/src/translation/emit-javascript.js'), import('../../js/src/translation/lexer.js'), import('./raw-fixtures.mjs'), import('./raw-corpus.mjs'), import('./raw-transport.mjs'), import('./raw-rules.mjs'), import('../rust-text-certificate-adapter/raw-identity.mjs'), import('./transport.mjs'), import('../rust-text-value-projection/model.mjs'),
  ]);
  return runOriginalLinkRawPlanningPhases({
    prepareSource: () => prepareOriginalLinkRawSource({readProjectData, parseJavaScript, checkProgram, emitRust, emitJavaScript, tokenize}),
    replaySource: source => archiveCompleteRustSource(source, LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary),
    buildMatrix: (prepared, receipt) => buildOriginalLinkRawPlan(prepared, receipt, modelSources, transportSources, {LinkNetwork, encodeLinkNetworkBinary, decodeLinkNetworkBinary, fixtures, corpus, transport, rules, oldDecoder, oldProjector, textProjection}),
  });
}

function parseObservations(output, expected, carrier) {
  assert.ok(Buffer.byteLength(output) <= maximumProtocolBytes);
  const lines = output.split('\n'); assert.equal(lines.pop(), ''); assert.equal(lines.length, expected.length);
  return lines.map((line, index) => {
    const parts = line.split(':'); assert.equal(parts[0], String(index)); assert.equal(parts[1], String(expected[index].resultBits.length));
    const bits = parts.slice(2).flatMap((part, block) => { assert.match(part, /^(?:0|[1-9a-f][0-9a-f]*)$/u); const id = Number.parseInt(part, 16); assert.ok(Number.isSafeInteger(id) && id < carrier.blocks.length); assert.equal(id, carrier.vectors[carrier.invocations[index].result][block], 'actual block order is exact'); return carrier.blocks[id].map(index => carrier.words[index]); });
    assert.deepEqual(bits, expected[index].resultBits, 'every full actual result bit is preserved');
    return {id: expected[index].id, kind: expected[index].kind, inputSha256: sha256(JSON.stringify(expected[index].inputBits)), rootBits: expected[index].inputBits.slice(0, 2), resultBits: bits, premiseIndices: expected[index].premiseIndices};
  });
}
const runChecked = createSourceNativeObservationRunner({nativeMode: originalLinkRawNativeMode, checkedPlan: plan => checkedPlans.has(plan), textBytes, parseObservations: (output, expected) => { const carrier = expectedCarriers.get(expected); assert.ok(carrier); return parseObservations(output, expected, carrier); }, entryReceipt: entry => ({parameterOrder: entry.parameterOrder, parameterModes: entry.parameterModes, resultMode: entry.resultMode, interfaceSha256: entry.interfaceSha256, declarationNames: entry.declarationNames, definitionCount: entry.definitionCount, preludeCount: entry.preludeCount, fixtureInventory: entry.fixtureInventory, inventoryCounts: entry.inventoryCounts, independentJavaScriptControls: entry.independentJavaScriptControls, carrierSha256: entry.carrierSha256, sizes: entry.sizes, genuineFormatPremiseCount: entry.genuineFormatPremiseCount, serializationRefusals: entry.serializationRefusals, premises: entry.premises, hostChecks: entry.hostChecks, observationScope: entry.observationScope}), temporaryPrefix: 'original-raw-native-', failureLabel: 'Complete original raw component observations failed'});
const expectedCarriers = new WeakMap();
export function runOriginalLinkRawNative(plan, options) {
  requireOriginalLinkRawPlanning(options?.environment ?? process.env);
  assert.ok(checkedPlans.has(plan), 'only the exact checked raw plan may execute');
  assert.deepEqual(plan.transportSources, [readCanonicalArchiveTransportSource()], 'the raw archive transport source changed before execution');
  for (const entry of plan.entries) expectedCarriers.set(entry.observations, entry.carrier);
  // An immutable environment keeps both CI authorizations true for every runner phase.
  const environment = Object.freeze({...options?.environment ?? process.env});
  const receipt = runChecked(plan, {...options, environment});
  assert.deepEqual(plan.transportSources, [readCanonicalArchiveTransportSource()], 'the raw archive transport source changed during execution');
  return {...receipt, transportSources: plan.transportSources};
}
