// Exact finite successor for one already-checked Boolean coercion operation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {serializeProjectData,readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
function freezeTruthyProjectionData(value){if(value&&typeof value==='object')for(const child of Object.values(value))freezeTruthyProjectionData(child);return Object.freeze(value);}
export const truthyProjectionHostRecipes = freezeTruthyProjectionData({
  "js/src/translation/check.js": [
    [
      "      case 'numberValue':\n",
      "      case 'booleanValue':\n        return this.booleanValue(this.expr(node.arg, env, path, undefined), node.span);\n      case 'numberValue':\n"
    ]
  ],
  "rust/src/translation/surface/nodes.rs": [
    [
      "    /// Internal reconstruction of the represented JavaScript numeric conversion.\n",
      "    /// Internal reconstruction of the checked source Boolean conversion.\n    BooleanValue {\n        arg: Box<SExpr>,\n    },\n    /// Internal reconstruction of the represented JavaScript numeric conversion.\n"
    ],
    [
      "            Self::NumberValue { .. } => \"numberValue\",\n",
      "            Self::BooleanValue { .. } => \"booleanValue\",\n            Self::NumberValue { .. } => \"numberValue\",\n"
    ]
  ],
  "rust/src/translation/check/expressions.rs": [
    [
      "            SNode::NumberValue { arg } => {\n",
      "            SNode::BooleanValue { arg } => {\n                let value = self.expr(arg, env, path, None, false)?;\n                self.boolean_value(value, span)\n            }\n            SNode::NumberValue { arg } => {\n"
    ]
  ]
});
export const truthyProjectionHostPins = freezeTruthyProjectionData([
  {
    "file": "js/src/translation/check.js",
    "previous": {
      "bytes": 131170,
      "sha256": "5219d03742b1227486a0e0daa71225891586f5f7815a1a558c1f9c0228ff9bbd"
    },
    "current": {
      "bytes": 131285,
      "sha256": "343e36fb0ff567340d08a14c325b7ef527c724b77a114f8f0d02f09924fa164a"
    }
  },
  {
    "file": "rust/src/translation/surface/nodes.rs",
    "previous": {
      "bytes": 12159,
      "sha256": "4679f10f66dbca121618d07c5c2c4174d492239d7e2bf590ce7e2fb23fcb9a51"
    },
    "current": {
      "bytes": 12341,
      "sha256": "2cbedaef1a59a71691154b604240aac5fe4a72468e1b74ad892a0d3c8acf6d24"
    }
  },
  {
    "file": "rust/src/translation/check/expressions.rs",
    "previous": {
      "bytes": 32425,
      "sha256": "9508d9f4b5e9fa94d63aaa5a79ad044ea7ba9d33797beab2c8b2b5c8408eaf19"
    },
    "current": {
      "bytes": 32601,
      "sha256": "a908e31e50900430757dea6138ab5a7bf0f5083efb04e9e3d5eb01641cc4ce1a"
    }
  }
]);
export const inspectedTruthyPreimages = freezeTruthyProjectionData([
  {
    "file": "js/src/data/typed-program-links-schema.lino",
    "bytes": 155087,
    "sha256": "d34bf62c4fd6114e5cdaa8a78558de01b83d373175185ee6a94b56114b80599f"
  },
  {
    "file": "rust/src/data/typed-program-links-schema.lino",
    "bytes": 155087,
    "sha256": "d34bf62c4fd6114e5cdaa8a78558de01b83d373175185ee6a94b56114b80599f"
  },
  {
    "file": "js/src/translation/check.js",
    "bytes": 131170,
    "sha256": "5219d03742b1227486a0e0daa71225891586f5f7815a1a558c1f9c0228ff9bbd"
  },
  {
    "file": "rust/src/translation/surface/nodes.rs",
    "bytes": 12159,
    "sha256": "4679f10f66dbca121618d07c5c2c4174d492239d7e2bf590ce7e2fb23fcb9a51"
  },
  {
    "file": "rust/src/translation/check/expressions.rs",
    "bytes": 32425,
    "sha256": "9508d9f4b5e9fa94d63aaa5a79ad044ea7ba9d33797beab2c8b2b5c8408eaf19"
  }
]);

export const TRUTHY_PROJECTION_REVISION='8d77e476d097448eb381ebe03e2aa173402bc03d';
export const TRUTHY_PROJECTION_OUTPUT='parity/self-translation/truthy-projection-schema.lino';
export const TRUTHY_PROJECTION_ENTRIES=freezeTruthyProjectionData({
 expressionProjections:{truthy:{construct:'booleanValue',fields:[['arg','expression','arg']]}},
 expressionFields:{truthy:['k','arg','type','span']},
});
const root=new URL('../../',import.meta.url);
const recipeSha256='be21041ccddae30ae29f4d183d4b9edc0a280af9fd2501aebda6460403105552';
export const pinTruthyProjectionSource=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const canonical=value=>serializeProjectData(value,'truthy-projection-schema.lino');

export function deriveTruthyProjectionHostSource(file,previous) {
 const recipes=truthyProjectionHostRecipes[file];assert.ok(recipes,'one exact Boolean projection host');
 let current=previous;
 for(const [before,after]of recipes){assert.equal(current.split(before).length,2,'one complete original host boundary');current=current.replace(before,after);}
 let inverse=current;
 for(const [before,after]of [...recipes].reverse()){assert.equal(inverse.split(after).length,2);inverse=inverse.replace(after,before);}
 assert.equal(inverse,previous,'every original host byte stays exact');return current;
}

export function deriveTruthyProjectionSchema(previous) {
 const original=readStructuredDataLino(previous),current=structuredClone(original);
 assert.deepEqual(current.expressionOperands.truthy,['arg']);
 for(const [table,entries]of Object.entries(TRUTHY_PROJECTION_ENTRIES))for(const [key,value]of Object.entries(entries)){
  assert.equal(Object.hasOwn(current[table],key),false,'append only the genuinely absent projection/field entries');
  current[table][key]=structuredClone(value);
 }
 const inverse=structuredClone(current);
 for(const [table,entries]of Object.entries(TRUTHY_PROJECTION_ENTRIES))for(const [key,value]of Object.entries(entries)){
  assert.equal(canonical(inverse[table][key]),canonical(value));delete inverse[table][key];
 }
 assert.equal(serializeProjectData(inverse,'schema.lino'),previous,'the complete old schema is byte-exact');
 return serializeProjectData(current,'schema.lino');
}

export function verifyTruthyProjectionSchemaPacket(packet) {
 assert.deepEqual(Object.keys(packet).sort(),['boundary','extensions','revision','schemaVersion','sources']);
 assert.equal(packet.schemaVersion,1);assert.equal(packet.revision,TRUTHY_PROJECTION_REVISION);
 assert.equal(canonical(packet.extensions),canonical(TRUTHY_PROJECTION_ENTRIES));
 assert.deepEqual(packet.sources.map(row=>row.file),inspectedTruthyPreimages.map(row=>row.file));
 for(const [index,row]of packet.sources.entries()){
  assert.deepEqual(Object.keys(row).sort(),['certificate','current','file','previous']);
  assert.deepEqual(Object.keys(row.previous).sort(),['bytes','sha256','source']);
  assert.deepEqual(Object.keys(row.current).sort(),['bytes','sha256','source']);
  const expected=inspectedTruthyPreimages[index];
  assert.deepEqual(pinTruthyProjectionSource(row.previous.source),{bytes:expected.bytes,sha256:expected.sha256});
  assert.deepEqual(pinTruthyProjectionSource(row.previous.source),{bytes:row.previous.bytes,sha256:row.previous.sha256});
  verifyGitArtifactCertificate(row.certificate,Buffer.from(row.previous.source),{revision:TRUTHY_PROJECTION_REVISION,path:row.file});
  const derived=index<2?deriveTruthyProjectionSchema(row.previous.source):deriveTruthyProjectionHostSource(row.file,row.previous.source);
  assert.equal(row.current.source,derived);assert.deepEqual(pinTruthyProjectionSource(derived),{bytes:row.current.bytes,sha256:row.current.sha256});
 }
 assert.equal(packet.sources[0].previous.source,packet.sources[1].previous.source);
 assert.equal(packet.sources[0].current.source,packet.sources[1].current.source);
 return structuredClone(packet);
}

export function readTruthyProjectionSchemaPacket() {
 const bytes=readFileSync(new URL(TRUTHY_PROJECTION_OUTPUT,root));
 assert.equal(pinTruthyProjectionSource(bytes).sha256,recipeSha256,'complete normal-author packet is pinned');
 return verifyTruthyProjectionSchemaPacket(readProjectData(new URL(TRUTHY_PROJECTION_OUTPUT,root)));
}

// Explicit historical readers only. Runtime schema/checkers always use actual
// current bytes; unknown complete schema or host successors still refuse.
export function restoreTruthyProjectionHistoricalSource(bytes,file) {
 assert.ok(Buffer.isBuffer(bytes)&&bytes.length<=256*1024);
 const expected=inspectedTruthyPreimages.find(row=>row.file===file);assert.ok(expected,'closed five-source historical view');
 const text=bytes.toString('utf8');assert.equal(Buffer.from(text).equals(bytes),true);
 if(pinTruthyProjectionSource(bytes).sha256===expected.sha256){assert.equal(bytes.length,expected.bytes);return text;}
 const packet=readTruthyProjectionSchemaPacket(),row=packet.sources.find(row=>row.file===file);
 assert.equal(text,row.current.source,'unknown complete current source refuses historical authority');
 assert.deepEqual(pinTruthyProjectionSource(bytes),{bytes:row.current.bytes,sha256:row.current.sha256});
 return row.previous.source;
}
