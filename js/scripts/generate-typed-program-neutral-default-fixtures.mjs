// Authored source programs and their real checked text/binary archives, in normal Links data.
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {readProgramTypeNeutralDefaults} from './read-program-type-neutral-defaults.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {typeKey} from '../src/translation/types.js';
import {encodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../src/typed-program-links.js';
import {encodeLinkNetworkBinary} from '../src/link-network-binary.js';

export function generateTypedProgramNeutralDefaultFixture({check=false}={}){
  const guarded=readProgramTypeNeutralDefaults();
  const source=[
    '/** @param {number[]} value @returns {number[]} */ function arrayIdentity(value){return value;}',
    '/** @param {{value:number}} value @returns {{value:number}} */ function recordIdentity(value){return value;}',
    '/** @param {Map<string,number>} value @returns {Map<string,number>} */ function mapIdentity(value){return value;}',
    '/** @param {Set<number>} value @returns {Set<number>} */ function setIdentity(value){return value;}',
  ].join('\n');
  const program=structuredClone(checkProgram(parseJavaScript(source,{referenceSemantics:false})));
  const seen=new WeakSet(),types=[];
  const walk=value=>{
    if(!value||typeof value!=='object'||seen.has(value))return;seen.add(value);
    if(Object.hasOwn(guarded.defaults,value.kind??'')){
      const key=typeKey(value);
      for(const name of Object.keys(guarded.defaults[value.kind]))value[name]=false;
      assert.equal(typeKey(value),key);types.push(value);
    }
    for(const child of value instanceof Map?[...value.values()]:Object.values(value))walk(child);
  };
  walk(program);assert.deepEqual([...new Set(types.map(type=>type.kind))].sort(),['array','map','record','set']);
  const archive=encodeTypedProgramLinks(program),text=archive.network.toLino(),binary=encodeLinkNetworkBinary(archive.network);
  const kinds=['array','record','map','set','float','tuple','function','','unregistered','expression:array','declaration:record'];
  const fields=['reference','open','frozen','span','unknown','__proto__'];
  const decisions=kinds.flatMap(kind=>fields.map(field=>({kind,field,expected:Object.hasOwn(guarded.defaults,kind)&&Object.hasOwn(guarded.defaults[kind],field)?guarded.defaults[kind][field]:''})));
  const scalar=id=>archive.network.link(id).metadata().definition;
  const label=key=>schema.fields[key]??key;
  const typedRecord=kind=>{const rows=archive.network.fieldRelations().filter(row=>row.label===label('kind')&&scalar(row.child)===kind);assert.ok(rows.length>0);return rows[0].parent;};
  const construct=kind=>{const row=archive.network.fieldRelations().find(row=>row.label===label('k')&&scalar(row.child)===kind);assert.ok(row);return row.parent;};
  const negativeTargets=[
    {id:'array-open-is-not-a-type-default',target:typedRecord('array'),field:'open'},
    {id:'primitive-reference-is-not-a-type-default',target:typedRecord('float'),field:'reference'},
    {id:'program-is-not-a-type-record',target:archive.network.link(archive.root).references()[0],field:'reference'},
    {id:'expression-is-not-a-type-record',target:construct('var'),field:'reference'},
    {id:'declaration-is-not-a-type-record',target:construct('fn'),field:'reference'},
    {id:'unknown-type-field-is-not-neutral',target:typedRecord('record'),field:'unknown'},
  ].map(row=>({...row,target:row.target.asU64().toString()}));
  const typeCases=Object.entries(guarded.defaults).flatMap(([kind,fields])=>Object.keys(fields).map(field=>({kind,field,type:structuredClone(types.find(type=>type.kind===kind))})));
  const fixture={schemaVersion:1,id:'typed-program-neutral-type-defaults',sourceGuard:guarded,decisions,typeCases,
    cases:[{id:'four-owned-type-identities',source,expected:'[[3],4,5,true]',
      probe:'JSON.stringify([arrayIdentity([3]),recordIdentity({value:4}).value,mapIdentity(new Map([["a",5]])).get("a"),setIdentity(new Set([6])).has(6)])',
      negativeTargets,archive:{root:archive.root.asU64().toString(),text,textSha256:createHash('sha256').update(text).digest('hex'),binaryHex:Buffer.from(binary).toString('hex'),binarySha256:createHash('sha256').update(binary).digest('hex')}}]};
  const target=new URL('../../parity/fixtures/typed-program-neutral-type-defaults.lino',import.meta.url);
  if(check)checkProjectData(target,fixture);else writeProjectData(target,fixture);return fixture;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.length===0||args.length===1&&args[0]==='--check');generateTypedProgramNeutralDefaultFixture({check:args[0]==='--check'});
}
