// Complete source-only bridge before existing historical native reader chains.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {replayRequiredFieldNativeHostIntervals} from './required-field-native-host-succession.mjs';
const root=new URL('../../',import.meta.url);
export const CURRENT_NATIVE_HOST_REVISION='8d77e476d097448eb381ebe03e2aa173402bc03d';
export const CURRENT_NATIVE_HOST_OUTPUT='parity/self-translation/ci-39539903548-native-host-source-history.lino';
export const CURRENT_NATIVE_HOST_FILES=Object.freeze(['rust/src/self_translation.rs','rust/src/translation/mod.rs','rust/tests/unit/mod.rs','rust/src/translation/javascript/expressions/values.rs']);
const packetSha256='63430f51b2757e02d165dd1523219a7adb4f0703abe11692b310014bd414a137';
let cached;
export const pinCurrentNativeHostSource=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),[...keys].sort());};

export function deriveCurrentNativeHostSourceIntervals(previous,current){
  assert.equal(Buffer.from(previous).toString('utf8'),previous);assert.equal(Buffer.from(current).toString('utf8'),current);
  const before=[...previous],after=[...current];let start=0,oldEnd=before.length,end=after.length;
  while(start<oldEnd&&start<end&&before[start]===after[start])start++;
  while(oldEnd>start&&end>start&&before[oldEnd-1]===after[end-1]){oldEnd--;end--;}
  const edits=[{start:before.slice(0,start).join('').length,removed:before.slice(start,oldEnd).join(''),inserted:after.slice(start,end).join('')}];
  assert.equal(replayRequiredFieldNativeHostIntervals(previous,edits),current);return edits;
}

export function verifyCurrentNativeHostSourceHistory(packet){
  exactKeys(packet,['schemaVersion','model','revision','sources','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'complete current native historical host sources 1');
  assert.equal(packet.revision,CURRENT_NATIVE_HOST_REVISION);
  assert.deepEqual(packet.sources.map(row=>row.file),CURRENT_NATIVE_HOST_FILES);
  for(const row of packet.sources){
    exactKeys(row,['file','previous','current','coordinateUnit','edits']);
    exactKeys(row.previous,['source','bytes','sha256','certificate']);exactKeys(row.current,['source','bytes','sha256']);
    assert.deepEqual(pinCurrentNativeHostSource(row.previous.source),{bytes:row.previous.bytes,sha256:row.previous.sha256});
    verifyGitArtifactCertificate(row.previous.certificate,Buffer.from(row.previous.source),{revision:CURRENT_NATIVE_HOST_REVISION,path:row.file});
    assert.deepEqual(pinCurrentNativeHostSource(row.current.source),{bytes:row.current.bytes,sha256:row.current.sha256});
    assert.equal(row.current.source,readFileSync(new URL(row.file,root),'utf8'),'the entire actual current native host supplies this view');
    assert.equal(row.coordinateUnit,'UTF-16 code units');
    assert.deepEqual(row.edits,deriveCurrentNativeHostSourceIntervals(row.previous.source,row.current.source));
    assert.equal(replayRequiredFieldNativeHostIntervals(row.previous.source,row.edits),row.current.source);
    let inverse=row.current.source;
    for(const edit of row.edits.toReversed()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,row.previous.source);
  }
  return packet;
}

export function readCurrentNativeHostSourceHistory(){
  if(!cached){const bytes=readFileSync(new URL(CURRENT_NATIVE_HOST_OUTPUT,root));assert.equal(pinCurrentNativeHostSource(bytes).sha256,packetSha256);cached=verifyCurrentNativeHostSourceHistory(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
  return structuredClone(cached);
}

/** Unknown sources remain unchanged for the original strict reader to refuse.
 * No unknown source is assigned a historical identity; runtime files stay current. */
export function restoreCurrentNativeHostSource(source,file){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  source=restoreCheckedExternalBindingSource(source,file);
  if (file === 'rust/src/translation/ir.rs'
    && pinCurrentNativeHostSource(source).sha256 === 'e2bcdac4be92d67015211f97b26f6b69f90f2bf4796a8ccdde3d28cc1a5149dd') {
    const previous = source.replace('            | Node::CollectionSize { object: arg }\n', '').replace(
      '            Node::CollectionSize { object } => Node::CollectionSize {\n                object: Box::new(visit(object)),\n            },\n', '',
    );
    assert.equal(pinCurrentNativeHostSource(previous).sha256, '29ef362f4b1d0129791687bd6bd52f5a68655136f4018d6981292c087961bbd7');
    return previous;
  }
  if (file === 'rust/tests/unit/translation_array_relative_element.rs'
    && pinCurrentNativeHostSource(source).sha256 === '347e32acb006b544b12841ed36f81d58a2ed9c9372c45ceb13eeec6125ced762') {
    const previous = source.replace('        assert_eq!(array_relative_native(&translated.code, index), expected);\n', '');
    assert.equal(pinCurrentNativeHostSource(previous).sha256, 'f46fe72f7f80edc180118bfba44fda54d1418a6ad23da34343a55149e67d53f3');
    return previous;
  }
  if(!CURRENT_NATIVE_HOST_FILES.includes(file))return source;
  const row=readCurrentNativeHostSourceHistory().sources.find(row=>row.file===file);
  return source===row.current.source?row.previous.source:source;
}

const externalBindingSourcePins = new Map([
  ['js/src/typed-program-links.js', {
    current: '0f591a4718ba78855313539f593a306aa5207d5fcb859419b527c8c7549f9e8e',
    previous: 'a0f46c4fdca3ed9bb729da7a701266bf8d7609ba142bc3009cd6eafe39983458',
  }],
  ['js/src/typed-program-links-validation.js', {
    current: 'b1f87907f68cc24dfc6d84676d2b0f939318c48b79d92aaf9becc29d1bdd35e2',
    previous: '6c2ea9bcb6266ef98af1b86708561dd8fd991e002f46c22dbfa047d90de39051',
  }],
]);

/** Preserve the exact checked codec sources used by the older proof readers. */
export function restoreCheckedExternalBindingSource(source, file) {
  const pins = externalBindingSourcePins.get(file);
  if (pins && pinCurrentNativeHostSource(source).sha256 === pins.current) {
    const previous = source.replace(
      '  for (const external of program.externals ?? []) {\n    if (!program.declarations.has(external.name)) declarations.set(external.name, external);\n  }',
      '  for (const external of program.externals ?? []) declarations.set(external.name, external);',
    );
    assert.equal(pinCurrentNativeHostSource(previous).sha256, pins.previous);
    return previous;
  }
  if (file === 'rust/src/typed_program_links/checked.rs'
    && pinCurrentNativeHostSource(source).sha256 === '653747d3c71c1237fbd659574c834c47286262b66e742959e940c4296a483acc') {
    const previous = source.replace(
      '    if !program.imports.is_empty() || program.output_threaded || program.aborts_threaded {',
      '    if !program.imports.is_empty()\n        || !program.externals.is_empty()\n        || program.output_threaded\n        || program.aborts_threaded\n    {',
    ).replace('        externals: program.externals.clone(),', '        externals: Vec::new(),');
    assert.equal(pinCurrentNativeHostSource(previous).sha256, 'ea6b9054929ad83be10af922b26238f5b2f19066c26b435773f99218306dcd10');
    return previous;
  }
  return source;
}
