// Source-only exact reference-validator succession before unchanged historical owner gates.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readCollectionSizeProjectionSchemaPacket,deriveCollectionSizeProjectionValidationSource} from './collection-size-projection-schema.mjs';
import {restoreRawConsumingSnapshotSource} from './raw-consuming-snapshot-source-profile.mjs';
import {truthyProjectionHostPins,restoreTruthyProjectionHistoricalSource} from './truthy-projection-schema.mjs';
const validator='js/src/typed-program-links-validation.js',checker='js/src/translation/check.js';
const changes=[
  {
    "before": "import { fixedBounds, sameType } from './translation/types.js';",
    "after": "import { fixedBounds, sameType } from './translation/types.js';\nimport {readReferenceIdentityComparisonError} from './translation/reference-identity-rules.js';"
  },
  {
    "before": "      if (node.domain) same(node.domain, node.left.type, 'binary operation domain');\n      const comparison =",
    "after": "      if (node.domain) same(node.domain, node.left.type, 'binary operation domain');\n      const referenceError = readReferenceIdentityComparisonError(node.op, node.left.type.kind, node.right.type.kind, node.left.type.reference === true, node.right.type.reference === true, sameType(node.left.type, node.right.type), node.domain !== undefined, node.domain?.reference === true, node.domain !== undefined && sameType(node.domain, node.left.type));\n      if (referenceError) throw new TypeError(referenceError);\n      const comparison ="
  }
];
const pin=source=>({bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')});

function originalOwner(file){
 const packet=readCollectionSizeProjectionSchemaPacket(),owner=packet.owners.find(row=>row.file===file);assert.ok(owner);
 return owner;
}
/** The entire previous validator is independently certified by the unchanged collection packet. */
function deriveWithOwner(owner,previous){
 const expected=deriveCollectionSizeProjectionValidationSource(owner.source);
 assert.equal(previous,expected,'only the complete certified collection validator is a predecessor');
 assert.deepEqual(pin(previous),owner.current);
 let current=previous;
 for(const change of changes){assert.equal(current.split(change.before).length,2,'one actual reference-validation insertion');current=current.replace(change.before,change.after);}
 let inverse=current;
 for(const change of [...changes].reverse()){assert.equal(inverse.split(change.after).length,2);inverse=inverse.replace(change.after,change.before);}
 assert.equal(inverse,previous,'every previous validator byte and assertion survives');
 return current;
}
export function deriveReferenceIdentityValidationSource(previous){return deriveWithOwner(originalOwner(validator),previous);}
/** Live runtime/codec/checker use never invokes this scoped historical adapter. */
export function restoreReferenceIdentityHistoricalSource(source,file,scope){
 assert.equal(typeof source,'string');assert.equal(typeof file,'string');assert.equal(typeof scope,'string');
 if(file===validator&&['collection size projection','program string registration'].includes(scope)){
  const owner=originalOwner(file),previous=deriveCollectionSizeProjectionValidationSource(owner.source);
  assert.deepEqual(pin(previous),owner.current);
  return source===deriveWithOwner(owner,previous)?previous:source;
 }
 if(file===checker&&scope==='collection size projection'){
  const owner=originalOwner(file);assert.deepEqual(pin(owner.source),owner.current);
  if(source===owner.source)return source;
  const old=truthyProjectionHostPins.find(row=>row.file===file);assert.ok(old);
  if(pin(source).sha256===old.current.sha256){const previous=restoreTruthyProjectionHistoricalSource(Buffer.from(source),file);assert.equal(previous,owner.source);return previous;}
  // Existing normal snapshot authority authenticates all actual source bytes.
  // Unknown selected source stays unknown; a stale snapshot still refuses.
  const historical=restoreRawConsumingSnapshotSource(Buffer.from(source),file,true).toString('utf8');
  if(historical!==source){assert.equal(historical,owner.source,'the whole existing checker authority must select the exact certified collection owner');return historical;}
 }
 return source;
}
