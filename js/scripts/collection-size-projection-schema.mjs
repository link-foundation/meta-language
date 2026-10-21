// Exact finite schema succession; current projection remains shared data and the existing checker owns meaning.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readProjectData,serializeProjectData} from './project-data.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
export const COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY=Object.freeze({
 revision:'7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3',
 files:Object.freeze(['js/src/data/typed-program-links-schema.lino','rust/src/data/typed-program-links-schema.lino']),
 predecessor:Object.freeze({bytes:153938,sha256:'211055fda6211cf98bbe46e3f7a176b0b26bb4b033b93f0966a96c3daeda9d1f'}),
 recipe:'parity/self-translation/collection-size-projection-schema.lino',
});
const freeze=value=>{if(value&&typeof value==='object')for(const child of Object.values(value))freeze(child);return Object.freeze(value);};
export const COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES=freeze({
 expressionProjections:{collectionSize:{construct:'field',fields:[['object','expression','object'],['field','constant','size']]}},
 expressionFields:{collectionSize:['k','object','type','span']},
});
export const COLLECTION_SIZE_PROJECTION_FIXTURE_INPUTS=freeze([["javascript-set-size","JavaScript","/** @param {Set<number>} values */ function count(values){return values.size;}console.log(count(new Set([1,1,2])));",["2"]],["javascript-map-size","JavaScript","/** @param {Map<string,number>} values */ function count(values){return values.size;}console.log(count(new Map([[\"second\",2],[\"first\",1],[\"second\",3]])));",["2"]]]);

export function deriveCollectionSizeProjectionFixtureSource(predecessor) {
 for(const row of COLLECTION_SIZE_PROJECTION_FIXTURE_INPUTS)assert.equal(predecessor.includes(JSON.stringify(row[0])),false,'new fixture identity must not duplicate an existing input');
 const before='\n];\n\nfunction execute(code) {';
 const after='\n'+COLLECTION_SIZE_PROJECTION_FIXTURE_INPUTS.map(row=>'  '+JSON.stringify(row)+',').join('\n')+before;
 assert.equal(predecessor.split(before).length,2,'one complete original fixture input-list boundary');
 const current=predecessor.replace(before,after);assert.equal(current.split(after).length,2);
 assert.equal(current.replace(after,before),predecessor,'every original fixture input and generator byte stays exact');return current;
}
export function deriveCollectionSizeProjectionValidationSource(predecessor) {
 const before="    if (['tuple', 'tupleIndex'].includes(node.k)\n      && Object.keys(node).some(field => !acceptProgramExpressionField(node.k, field))) throw new TypeError('a linked tuple expression has an unrecognized projection field');",after="    if (acceptProgramExpressionField(node.k ?? '', 'k')\n      && Object.keys(node).some(field => !acceptProgramExpressionField(node.k, field))) throw new TypeError(\n        ['tuple', 'tupleIndex'].includes(node.k) ? 'a linked tuple expression has an unrecognized projection field'\n          : 'a linked expression contains an unrecognized semantic field');";
 assert.equal(predecessor.split(before).length,2,'one complete original tuple-only closure boundary');
 const current=predecessor.replace(before,after);assert.equal(current.split(after).length,2);
 assert.equal(current.replace(after,before),predecessor,'every original validator byte and tuple refusal stays exact');return current;
}

const root=new URL('../../',import.meta.url);
const recipeSha256='3dc57ac68653607f0f95fcea65164d83fe5fb2325d9984878da751023da9ab85';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const canonical=value=>serializeProjectData(value,'collection-size-projection-schema.lino');
let sourcePair;

/** Exactly two additive entries; inverse conservation covers the entire earlier schema. */
export function deriveCollectionSizeProjectionSchema(predecessor) {
 const before=readStructuredDataLino(predecessor),after=structuredClone(before);
 for(const [table,entries]of Object.entries(COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES)){
  assert.ok(Object.hasOwn(after,table));
  for(const [key,value]of Object.entries(entries)){assert.equal(Object.hasOwn(after[table],key),false);after[table][key]=structuredClone(value);}
 }
 const current=serializeProjectData(after,'schema.lino'),inverse=structuredClone(after);
 for(const [table,entries]of Object.entries(COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES))for(const [key,value]of Object.entries(entries)){
  assert.equal(canonical(inverse[table][key]),canonical(value));delete inverse[table][key];
 }
 assert.equal(serializeProjectData(inverse,'schema.lino'),predecessor,'every original schema byte and every prior table entry remains exact');
 return current;
}

export function verifyCollectionSizeProjectionSchemaPacket(packet) {
 const policy=COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY;
 assert.deepEqual(Object.keys(packet).sort(),['boundary','current','extensions','owners','predecessor','revision','schemaVersion']);
 assert.equal(packet.schemaVersion,1);assert.equal(packet.revision,policy.revision);
 assert.equal(canonical(packet.extensions),canonical(COLLECTION_SIZE_PROJECTION_SCHEMA_ENTRIES));
 assert.deepEqual(Object.keys(packet.predecessor).sort(),['bytes','certificates','sha256','source']);
 assert.deepEqual(pin(packet.predecessor.source),policy.predecessor);
 assert.equal(packet.predecessor.bytes,policy.predecessor.bytes);assert.equal(packet.predecessor.sha256,policy.predecessor.sha256);
 assert.deepEqual(packet.predecessor.certificates.map(row=>row.file),policy.files);
 for(const row of packet.predecessor.certificates){
  assert.deepEqual(Object.keys(row).sort(),['certificate','file']);
  verifyGitArtifactCertificate(row.certificate,Buffer.from(packet.predecessor.source),{revision:policy.revision,path:row.file});
 }
 const owners=['js/src/translation/check.js','js/src/typed-program-expression-projection.js','rust/src/typed_program_links/projection.rs','js/scripts/generate-typed-program-links-fixtures.mjs','js/src/typed-program-links-validation.js'];
 assert.deepEqual(packet.owners.map(row=>row.file),owners);
 for(const row of packet.owners){
  assert.deepEqual(Object.keys(row).sort(),['bytes','certificate','current','file','sha256','source']);
  assert.deepEqual(pin(row.source),{bytes:row.bytes,sha256:row.sha256});
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:policy.revision,path:row.file});
  const current=row.file==='js/scripts/generate-typed-program-links-fixtures.mjs'?deriveCollectionSizeProjectionFixtureSource(row.source)
   :row.file==='js/src/typed-program-links-validation.js'?deriveCollectionSizeProjectionValidationSource(row.source):row.source;
  assert.deepEqual(pin(current),row.current,'fixture rows are additive; validator closure is exactly shared-data-driven; every other complete owner stays exact');
 }
 const source=deriveCollectionSizeProjectionSchema(packet.predecessor.source);assert.deepEqual(pin(source),packet.current);
 return{predecessor:packet.predecessor.source,current:source,extensions:structuredClone(packet.extensions)};
}

export function readCollectionSizeProjectionSchemaPacket() {
 const bytes=readFileSync(new URL(COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY.recipe,root));assert.equal(pin(bytes).sha256,recipeSha256);
 const packet=readProjectData(new URL(COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY.recipe,root));verifyCollectionSizeProjectionSchemaPacket(packet);return packet;
}

/** Use only within explicit historical schema checks. Live classifiers and projectors read current files directly. */
export function restoreCollectionSizeProjectionSchema(bytes,file) {
 const policy=COLLECTION_SIZE_PROJECTION_SCHEMA_POLICY;assert.ok(policy.files.includes(file));
 assert.ok(Buffer.isBuffer(bytes)&&bytes.length<=256*1024);const source=bytes.toString('utf8');assert.equal(Buffer.from(source).equals(bytes),true);
 if(pin(bytes).sha256===policy.predecessor.sha256){assert.deepEqual(pin(bytes),policy.predecessor);return source;}
 if(!sourcePair)sourcePair=verifyCollectionSizeProjectionSchemaPacket(readCollectionSizeProjectionSchemaPacket());
 assert.equal(source,sourcePair.current,'unknown complete current schema changes refuse historical source authority');
 return sourcePair.predecessor;
}
