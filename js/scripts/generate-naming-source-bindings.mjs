// Automatically relocate the exact naming table into shared Links data.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync,existsSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {NAMING_SOURCE_BINDING_STORAGE as specification,namingSourceBindingLiteralChunks,verifyNamingSourceBindingData,readNamingSourceBindings} from './naming-source-bindings.mjs';
const root=new URL('../../',import.meta.url),pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

function namingSourceLiteralRange(source) {
  const marker=Buffer.from('const '+specification.table+'='),at=source.indexOf(marker);
  assert.ok(at>=0&&source.indexOf(marker,at+1)<0);const start=at+marker.length;assert.equal(source[start],91);
  let quoted=false,escaped=false,depth=0;
  for(let at=start;at<source.length;at++){
    const byte=source[at];if(quoted){if(escaped)escaped=false;else if(byte===92)escaped=true;else if(byte===34)quoted=false;continue;}
    if(byte===34)quoted=true;else if(byte===91)depth++;else if(byte===93&&--depth===0){assert.equal(source[at+1],59);return{start,end:at+1};}
  }
  throw new Error('complete original naming source array required');
}

// Only scalar strings are interned. Equal independent containers stay distinct.
function shareNamingSourceScalarValues(value,strings=new Map()) {
  if(typeof value==='string'){
    if(strings.has(value))return strings.get(value);
    const copy=Buffer.from(value,'utf16le').toString('utf16le');assert.equal(copy,value);strings.set(copy,copy);return copy;
  }
  if(Array.isArray(value))return value.map(item=>shareNamingSourceScalarValues(item,strings));
  if(value!==null&&typeof value==='object')return Object.fromEntries(Object.entries(value).map(([key,item])=>{
    assert.notEqual(key,'__proto__');return[key,shareNamingSourceScalarValues(item,strings)];
  }));
  return value;
}

/** Preserve the whole trusted source while moving its data into Links. */
export function generateNamingSourceBindingData({check=false}={}) {
  const modulePath=new URL(specification.module,root),dataPath=new URL(specification.data,root),migrationPath=new URL(specification.migration,root);
  if(check){
    const rows=readNamingSourceBindings(),migration=readProjectData(migrationPath);
    verifyNamingSourceBindingData(rows,migration,readFileSync(modulePath));
    checkProjectData(dataPath,rows);checkProjectData(migrationPath,migration);
    return{rows:rows.length,source:pin(readFileSync(modulePath)),data:pin(readFileSync(dataPath))};
  }
  // Re-running after conversion can validate, never reset appended bindings.
  if(existsSync(migrationPath)&&existsSync(dataPath)&&readFileSync(modulePath).includes(Buffer.from(specification.initializer)))return generateNamingSourceBindingData({check:true});
  assert.ok(!existsSync(migrationPath)&&!existsSync(dataPath),'refuse partially migrated data; preserve existing files for review');
  const original=captureGitArtifactCertificate(specification.revision,specification.module),range=namingSourceLiteralRange(original.blob);
  assert.deepEqual(readFileSync(modulePath),original.blob,'refuse unrelated naming source work');
  let rows=JSON.parse(original.blob.subarray(range.start,range.end).toString('utf8'));assert.equal(rows.length,specification.baselineRows);
  let at=range.start;
  for(const chunk of namingSourceBindingLiteralChunks(rows)){assert.ok(original.blob.subarray(at,at+chunk.length).equals(chunk),'every original JSON row and whitespace byte');at+=chunk.length;}
  assert.equal(at,range.end);
  let converted=Buffer.concat([original.blob.subarray(0,range.start),Buffer.from(specification.initializer),original.blob.subarray(range.end)]);
  const anchor=Buffer.from("import { readStructuredDataLino } from '../src/structured-data-links.js';\n"),anchorAt=converted.indexOf(anchor);
  assert.ok(anchorAt>=0&&converted.indexOf(anchor,anchorAt+1)<0);
  converted=Buffer.concat([converted.subarray(0,anchorAt+anchor.length),Buffer.from(specification.importSource),converted.subarray(anchorAt+anchor.length)]);
  const migration={schemaVersion:1,revision:specification.revision,module:specification.module,table:specification.table,
    baselineRows:specification.baselineRows,original:pin(original.blob),literal:pin(original.blob.subarray(range.start,range.end)),
    certificate:original.certificate,converted:pin(converted),
    boundary:'Lossless data relocation only. Every original naming-module byte, ordered row, field, source context and input remains reconstructible from converted code and the original data prefix. Identical scalar values share links; independent ordered containers remain distinct. Current appended bindings retain separate identities. No naming requirement, vocabulary, fixture, oracle, native behavior or gate is weakened.'};
  verifyNamingSourceBindingData(rows,migration,converted);
  rows=shareNamingSourceScalarValues(rows);global.gc?.();
  writeProjectData(dataPath,rows);writeProjectData(migrationPath,migration);writeFileSync(modulePath,converted);
  return{rows:rows.length,source:migration.converted,data:pin(readFileSync(dataPath))};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  console.log(JSON.stringify(generateNamingSourceBindingData({check:process.argv.includes('--check')})));
}
