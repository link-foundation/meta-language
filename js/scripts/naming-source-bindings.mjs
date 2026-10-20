// Exact source binding data, stored as Links instead of a large code literal.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {readProjectData} from './project-data.mjs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';

export const NAMING_SOURCE_BINDING_STORAGE=Object.freeze({
  revision:'5a57b666fb54b151249896d90f1e63d7a8866fcf',
  module:'js/scripts/issue-195-naming.mjs',table:'B70_SOURCE_BINDINGS',baselineRows:3727,
  data:'parity/naming/source-bindings.lino',migration:'parity/naming/source-binding-data-migration.lino',
  importSource:"import { readNamingSourceBindings } from './naming-source-bindings.mjs';\n",
  initializer:'readNamingSourceBindings()',
});
const root=new URL('../../',import.meta.url),pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
let cachedBindings;

/** Serialize each original row in its exact existing JSON literal layout. No
 * JSON file is produced; these bytes only check the complete predecessor. */
export function* namingSourceBindingLiteralChunks(rows) {
  assert.ok(Array.isArray(rows));if(!rows.length){yield Buffer.from('[]');return;}
  for(const[index,row]of rows.entries()){
    const one=JSON.stringify([row],null,2);assert.ok(one.startsWith('[\n')&&one.endsWith('\n]'));
    yield Buffer.from((index===0?'[\n':',\n')+one.slice(2,-2));
  }
  yield Buffer.from('\n]');
}

function uniqueNamingSourceBytes(source,pattern) {
  const bytes=Buffer.from(pattern),start=source.indexOf(bytes);assert.ok(start>=0&&source.indexOf(bytes,start+1)<0,'one exact naming source edit');
  return{start,end:start+bytes.length};
}

/** Recreate every original module byte using only converted code and the
 * original data prefix; current appended bindings remain separate inputs. */
export function verifyNamingSourceBindingData(rows,migration,currentModule) {
  const specification=NAMING_SOURCE_BINDING_STORAGE;
  assert.ok(Array.isArray(rows)&&rows.length>=specification.baselineRows);
  assert.equal(migration.schemaVersion,1);assert.equal(migration.revision,specification.revision);
  assert.equal(migration.module,specification.module);assert.equal(migration.table,specification.table);
  assert.equal(migration.baselineRows,specification.baselineRows);
  assert.ok(Number.isSafeInteger(migration.original.bytes)&&migration.original.bytes>0&&migration.original.bytes<=64*1024*1024,'bounded complete original source');
  assert.equal(migration.original.bytes,migration.certificate.blob.bytes);
  assert.deepEqual(pin(currentModule),migration.converted,'unknown naming module edits cannot acquire an original source view');
  const addition=uniqueNamingSourceBytes(currentModule,specification.importSource);
  const withoutImport=Buffer.concat([currentModule.subarray(0,addition.start),currentModule.subarray(addition.end)]);
  const initializer=uniqueNamingSourceBytes(withoutImport,'const '+specification.table+'='+specification.initializer+';');
  const prefix=Buffer.from('const '+specification.table+'='),original=Buffer.alloc(migration.original.bytes);
  let cursor=withoutImport.copy(original,0,0,initializer.start+prefix.length);
  const literalHash=createHash('sha256');let literalBytes=0;
  for(const chunk of namingSourceBindingLiteralChunks(rows.slice(0,specification.baselineRows))){
    assert.ok(cursor+chunk.length<=original.length);cursor+=chunk.copy(original,cursor);literalHash.update(chunk);literalBytes+=chunk.length;
  }
  assert.deepEqual({bytes:literalBytes,sha256:literalHash.digest('hex')},migration.literal,'every original ordered binding and JSON source spelling remains exact');
  cursor+=withoutImport.copy(original,cursor,initializer.end-1);assert.equal(cursor,original.length);
  assert.deepEqual(pin(original),migration.original,'complete original naming module byte inverse');
  verifyGitArtifactCertificate(migration.certificate,original,{revision:specification.revision,path:specification.module});
  return{rows:rows.length,baselineRows:specification.baselineRows,original:migration.original,converted:migration.converted};
}

// Copies preserve ordered object identities and share immutable scalar values.
// A caller cannot mutate the private decoded data through its returned rows.
function copyNamingSourceBindingValues(value,seen=new WeakMap(),strings=null) {
  if(strings&&typeof value==='string'){
    if(!strings.has(value))strings.set(value,Buffer.from(value,'utf16le').toString('utf16le'));
    return strings.get(value);
  }
  if(value===null||typeof value!=='object')return value;
  if(seen.has(value))return seen.get(value);
  const result=Array.isArray(value)?[]:{};seen.set(value,result);
  for(const[key,item]of Object.entries(value)){
    assert.notEqual(key,'__proto__','source bindings are ordinary JSON data');
    result[key]=copyNamingSourceBindingValues(item,seen,strings);
  }
  return result;
}

export function readNamingSourceBindings() {
  if(!cachedBindings){
    const specification=NAMING_SOURCE_BINDING_STORAGE;
    const rows=readBootstrapStructuredDataLino(readFileSync(new URL(specification.data,root),'utf8'));
    const migration=readProjectData(new URL(specification.migration,root));
    verifyNamingSourceBindingData(rows,migration,readFileSync(new URL(specification.module,root)));
    // Detach scalar substrings from the complete serialized input. Only strings
    // share identity; ordered arrays and objects retain their actual owners.
    cachedBindings=copyNamingSourceBindingValues(rows,new WeakMap(),new Map());
  }
  return copyNamingSourceBindingValues(cachedBindings);
}
