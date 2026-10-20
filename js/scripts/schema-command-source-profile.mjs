// Exact check-only source adapters for immutable schema authoring commands.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

export const SCHEMA_COMMAND_SOURCE_PROFILE=Object.freeze({
  revision:'90636f9fb52b1b9095defbac041c8089608b8576',
  archive:'parity/self-translation/schema-command-source-profile.lino',
  sourceGate:'js/scripts/check-bulk-translation-rules.mjs',
  files:Object.freeze(['js/scripts/extend-typed-program-schema-second-stage.mjs','js/scripts/extend-typed-program-schema-sixth-stage.mjs']),
  nodeRange:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',
});
const root=new URL('../../',import.meta.url);
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let sources;

/** The complete original scripts remain independently pinned Git objects. */
export function readSchemaCommandSourceProfile(file) {
  const specification=SCHEMA_COMMAND_SOURCE_PROFILE;assert.ok(specification.files.includes(file),'registered schema command source required');
  if(!sources){
    const packet=readProjectData(new URL(specification.archive,root));
    assert.equal(packet.schemaVersion,1);assert.equal(packet.revision,specification.revision);
    assert.deepEqual(packet.sources.map(row=>row.file),specification.files);
    const checked=new Map();
    for(const row of packet.sources){
      const bytes=Buffer.from(row.source);assert.ok(bytes.length<=131072);
      verifyGitArtifactCertificate(row.certificate,bytes,{revision:specification.revision,path:row.file});
      assert.deepEqual(pin(bytes),row.original);
      assert.deepEqual(row.view,deriveSchemaCommandSourceView(row.file,row.source,row.certificate).receipt);
      checked.set(row.file,row);
    }
    sources=checked;
  }
  return structuredClone(sources.get(file));
}

/** Only the schema read and its explicit validating import change. Every other
 * original source byte, assertion, immutable source owner and refusal remains. */
export function deriveSchemaCommandSourceView(file,source,certificate) {
  assert.ok(SCHEMA_COMMAND_SOURCE_PROFILE.files.includes(file));assert.equal(typeof source,'string');assert.ok(Buffer.byteLength(source)<=131072);
  verifyGitArtifactCertificate(certificate,Buffer.from(source),{revision:SCHEMA_COMMAND_SOURCE_PROFILE.revision,path:file});
  const anchor="import {readStructuredDataLino} from '../src/structured-data-links.js';\n";
  const addition="import {readStringOrderSchemaPredecessor} from './check-typed-program-schema-history.mjs';\n";
  const second=file===SCHEMA_COMMAND_SOURCE_PROFILE.files[0];
  const actions=[{before:anchor,after:anchor+addition},second?
    {before:'let actualBytes = readFileSync(schemaFile);',after:'let actualBytes = readStringOrderSchemaPredecessor();'}:
    {before:"const actualBytes=readFileSync(schemaFile),oracle=readFileSync(oracleFile,'utf8');",after:"const actualBytes=readStringOrderSchemaPredecessor(),oracle=readFileSync(oracleFile,'utf8');"}];
  let current=source;
  for(const action of actions){assert.equal(current.split(action.before).length,2);current=current.replace(action.before,action.after);}
  let inverse=current;for(const action of actions.toReversed()){assert.equal(inverse.split(action.after).length,2);inverse=inverse.replace(action.after,action.before);}
  assert.equal(inverse,source,'whole immutable schema command byte inverse');
  return{file,source:current,receipt:{schemaVersion:1,profile:'authenticated predecessor schema command view 1',file,
    revision:SCHEMA_COMMAND_SOURCE_PROFILE.revision,nodeRange:SCHEMA_COMMAND_SOURCE_PROFILE.nodeRange,original:pin(source),executed:pin(current),actions,
    input:'The complete current JavaScript and Rust schemas, string-order model, generated declarations and fixtures pass the existing string-order generator before the original sixth schema bytes are supplied.',
    inverseVerified:true,boundary:'Check-only source adaptation. Original command files, all receipts, schema bytes and assertions stay exact. No native compilation or execution evidence.'}};
}

export function renderSchemaCommandSourceView(file,source) {
  const original=readSchemaCommandSourceProfile(file);assert.equal(source,original.source,'unknown immutable schema command edits refuse');
  const result=deriveSchemaCommandSourceView(file,source,original.certificate);assert.deepEqual(result.receipt,original.view);
  return result;
}
