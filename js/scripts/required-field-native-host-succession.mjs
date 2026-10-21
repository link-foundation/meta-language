// One finite source-only successor for three existing historical native readers.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {restoreCurrentNativeHostSource} from './current-native-host-source-history.mjs';
import {restoreScopedIterationNativeHostSource} from './scoped-iteration-native-host-source-history.mjs';
import {replayAsciiHostEdits,verifyAsciiTextPredicateHosts} from './ascii-text-predicate-host-history.mjs';
const root=new URL('../../',import.meta.url);
export const REQUIRED_FIELD_NATIVE_HOST_REVISION='a8a789afb0ed569340d9079279f534f0bf7c4e72';
export const REQUIRED_FIELD_NATIVE_HOST_OUTPUT='parity/self-translation/required-field-native-host-succession.lino';
export const REQUIRED_FIELD_NATIVE_HOST_SOURCES=Object.freeze([
  {file:'rust/src/translation/mod.rs',changes:[{
    before:'mod javascript_line_terminator_rules;\n',
    after:'mod javascript_line_terminator_rules;\npub(crate) mod javascript_record_parameter_rules;\n',
  }]},
  {file:'rust/tests/unit/mod.rs',changes:[{
    before:'mod translation_javascript_array_bindings;\n',
    after:'mod translation_javascript_array_bindings;\nmod translation_javascript_record_parameters;\n',
  }]},
  {file:'rust/src/translation/javascript/expressions/values.rs',changes:[{
    before:'            let place = self.to_here(&token);\n            let items = arguments.into_iter().next().unwrap_or_else(|| {\n',
    after:'            if !self.has_standard_builtin_owner(&constructor) {\n'
      +'                return Err(unsupported(\n'
      +'                    "shadowed standard collection owner",\n'
      +'                    "a local, module or supplied context constructor cannot be replaced with the standard Map or Set constructor",\n'
      +'                    Some(constructor.span()),\n'
      +'                ));\n'
      +'            }\n'
      +'            let place = self.to_here(&token);\n            let items = arguments.into_iter().next().unwrap_or_else(|| {\n',
  }]},
  {file:'rust/src/translation/javascript/declarations.rs',current:{bytes:38280,sha256:'573ac52363f87b48eb9526293990a2640a32be93df600bfbbd6867bb3c5cf2b0'}},
  {file:'rust/src/translation/javascript/lexical_bindings.rs',current:{bytes:7733,sha256:'b25723726c36ea5f267e643f71183a03eb14c70e76dc963a0d09b56bc360280f'}},
].map(row=>Object.freeze({...row,...(row.changes?{changes:Object.freeze(row.changes.map(change=>Object.freeze(change)))}:{current:Object.freeze(row.current)})})));
const readers={
  'program string registration':['rust/src/translation/mod.rs','rust/tests/unit/mod.rs'],
  'ASCII character host':['rust/src/translation/javascript/expressions/values.rs'],
  'mutable array binding host':['rust/src/translation/javascript/declarations.rs','rust/src/translation/javascript/lexical_bindings.rs'],
};
const frozen=[
  ['parity/self-translation/program-string-host-registration-history.lino','ba99ac1be8c218a374b5f014d666f6985b7bd706a0b395b28f7aecaa30ee6f0f'],
  ['parity/self-translation/ascii-text-predicate-hosts.lino','3e270ec750f423f2cce0576e8f05834970a31be45ec7bf094880b901c7c00a7a'],
  ['parity/self-translation/javascript-mutable-array-binding-hosts.lino','bb2f5ca4e690014ed0a08c984854b39e13cbc7a8a7dbc9be8f7d292b30a21e7b'],
];
const packetSha256='b4390b4ed5e837dc81eda30ed3c42ba88bec8d383fb57de5426f913b3b651cd7';
let cached;
export const pinRequiredFieldNativeHostSource=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});
const exactKeys=(value,keys)=>assert.deepEqual(Object.keys(value).sort(),[...keys].sort(),'the finite complete source layout stays exact');

function readFrozenNativeHostSources(){
  const [registration,ascii,mutable]=frozen.map(([path,sha256])=>{
    const bytes=readFileSync(new URL(path,root));assert.equal(pinRequiredFieldNativeHostSource(bytes).sha256,sha256,'every original packet remains byte exact');
    return readBootstrapStructuredDataLino(bytes.toString('utf8'));
  });
  assert.equal(registration.revision,'7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3');
  assert.equal(registration.sources.length,2);assert.equal(registration.registrations.length,5);
  const previous=new Map(registration.sources.map(row=>[row.path,row.current.source]));
  const values=verifyAsciiTextPredicateHosts(ascii).find(row=>row.path===REQUIRED_FIELD_NATIVE_HOST_SOURCES[2].file);
  assert.ok(values);previous.set(values.path,values.source);
  assert.equal(mutable.revision,'897bfc8b2dcc5ec561feffa455ddeb9f95d24335');assert.equal(mutable.hosts.length,6);
  for(const specification of REQUIRED_FIELD_NATIVE_HOST_SOURCES.slice(3)){
    const row=mutable.hosts.find(row=>row.path===specification.file);assert.ok(row);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:mutable.revision,path:row.path});
    const source=replayRequiredFieldNativeHostIntervals(row.source,row.edits);assert.equal(pinRequiredFieldNativeHostSource(source).sha256,row.currentSha256);
    previous.set(row.path,source);
  }
  return previous;
}

/** Existing policies and whole committed predecessors remain distinct from the
 * actual finite successor. This provides no execution or semantic-equivalence proof. */
export function verifyRequiredFieldNativeHostSuccession(packet){
  exactKeys(packet,['schemaVersion','model','revision','coordinateUnit','sources','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'required field native host source succession 1');
  assert.equal(packet.revision,REQUIRED_FIELD_NATIVE_HOST_REVISION);assert.equal(packet.coordinateUnit,'UTF-16 code units');
  assert.deepEqual(packet.sources.map(row=>row.file),REQUIRED_FIELD_NATIVE_HOST_SOURCES.map(row=>row.file));
  const frozenSources=readFrozenNativeHostSources();
  for(const[index,row]of packet.sources.entries()){
    exactKeys(row,['file','previous','current','edits']);
    exactKeys(row.previous,['source','bytes','sha256','certificate']);exactKeys(row.current,['source','bytes','sha256']);
    assert.equal(typeof row.previous.source,'string');assert.equal(typeof row.current.source,'string');
    const original=Buffer.from(row.previous.source);assert.equal(original.toString('utf8'),row.previous.source);
    verifyGitArtifactCertificate(row.previous.certificate,original,{revision:REQUIRED_FIELD_NATIVE_HOST_REVISION,path:row.file});
    assert.deepEqual(pinRequiredFieldNativeHostSource(original),{bytes:row.previous.bytes,sha256:row.previous.sha256});
    assert.equal(row.previous.source,frozenSources.get(row.file),'the complete a8 source equals the separately frozen historical reader input');
    const replay=deriveRequiredFieldNativeHostSource(row.previous.source,row.current.source,REQUIRED_FIELD_NATIVE_HOST_SOURCES[index]);
    assert.equal(row.current.source,replay.source);assert.deepEqual(row.edits,replay.edits);
    assert.equal(replayRequiredFieldNativeHostIntervals(row.previous.source,row.edits),row.current.source);
    assert.deepEqual(pinRequiredFieldNativeHostSource(replay.source),{bytes:row.current.bytes,sha256:row.current.sha256});
    let inverse=replay.source;
    for(const edit of row.edits.toReversed()){
      assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
      inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
    }
    assert.equal(inverse,row.previous.source,'every original complete source byte survives');
    const actual=readFileSync(new URL(row.file,root),'utf8');
    const scoped=restoreScopedIterationNativeHostSource(actual,row.file,'required field native host');
    assert.equal(restoreCurrentNativeHostSource(scoped,row.file),row.current.source,'the complete actual current host retains the new registration or builtin-owner guard');
  }
  return packet;
}

export function readRequiredFieldNativeHostSuccession(){
  if(!cached){const bytes=readFileSync(new URL(REQUIRED_FIELD_NATIVE_HOST_OUTPUT,root));assert.equal(pinRequiredFieldNativeHostSource(bytes).sha256,packetSha256);
    cached=verifyRequiredFieldNativeHostSuccession(readBootstrapStructuredDataLino(bytes.toString('utf8')));}
  return cached;
}

/** Only the three explicitly named old readers can request these five exact
 * source inverses. Unknown paths/scopes/bytes remain visible to their strict gate. */
export function restoreRequiredFieldNativeHostSource(source,file,reader){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof reader,'string');
  if(!Object.hasOwn(readers,reader)||!readers[reader].includes(file))return source;
  source=restoreScopedIterationNativeHostSource(source,file,'required field native host');
  source=restoreCurrentNativeHostSource(source,file);
  const row=readRequiredFieldNativeHostSuccession().sources.find(row=>row.file===file);assert.ok(row);
  if(source!==row.current.source)return source;
  assert.deepEqual(pinRequiredFieldNativeHostSource(source),{bytes:row.current.bytes,sha256:row.current.sha256});
  return row.previous.source;
}


// Metadata transport only: source symbols retain complete lossless UTF-8 bytes,
// while recorded intervals use the same UTF-16 coordinates as the old receipts.
export function replayRequiredFieldNativeHostIntervals(source,edits){
  let current=source;
  for(const edit of edits){
    assert.ok(Number.isSafeInteger(edit.start)&&edit.start>=0&&edit.start<=current.length);
    assert.equal(typeof edit.removed,'string');assert.equal(typeof edit.inserted,'string');
    assert.equal(current.slice(edit.start,edit.start+edit.removed.length),edit.removed);
    current=current.slice(0,edit.start)+edit.inserted+current.slice(edit.start+edit.removed.length);
  }
  let inverse=current;
  for(const edit of edits.toReversed()){
    assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
    inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
  }
  assert.equal(inverse,source,'the entire source predecessor survives every recorded interval');return current;
}

export function deriveRequiredFieldNativeHostSource(previous,current,specification){
  if(specification.changes)return replayAsciiHostEdits(previous,specification.changes);
  assert.deepEqual(pinRequiredFieldNativeHostSource(current),specification.current,'one complete reviewed native transport successor');
  assert.equal(Buffer.from(previous).toString('utf8'),previous);assert.equal(Buffer.from(current).toString('utf8'),current);
  const before=[...previous],after=[...current];let start=0,oldEnd=before.length,end=after.length;
  while(start<oldEnd&&start<end&&before[start]===after[start])start+=1;
  while(oldEnd>start&&end>start&&before[oldEnd-1]===after[end-1]){oldEnd-=1;end-=1;}
  const edits=[{start:before.slice(0,start).join('').length,removed:before.slice(start,oldEnd).join(''),inserted:after.slice(start,end).join('')}];
  assert.equal(replayRequiredFieldNativeHostIntervals(previous,edits),current);
  return{source:current,edits};
}
