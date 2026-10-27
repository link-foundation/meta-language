// Independent literal inputs/expectations; no parser or represented operation.
import {writeProjectData,readProjectData} from './project-data.mjs';
import {execFileSync} from 'node:child_process';
import {existsSync,lstatSync} from 'node:fs';
import {createHash} from 'node:crypto';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const prefix='/** @typedef {{ $:"none" } | { $:"text", units:number[] }} TextChoice */\n';
const cases=[
 {name:'tagged loops keep complete original field expressions after loop lifting',source:prefix+'/** @param {TextChoice} value */ function sum(value){switch(value.$){case "none":return 0;case "text":{let total=0;for(let at=0;at<value.units.length;at+=1)total+=value.units[at]??0;return total;}}} function compute(){return sum({$:"text",units:[2,3,4]});}',value:9},
 {name:'missing own fields are undefined',source:prefix+'/** @param {TextChoice} value */ function absent(value){return value.units===undefined;} function compute(){return absent({$:"none"});}',value:true},
 {name:'unrefined tags retain the original constructor spelling',source:prefix+'/** @param {TextChoice} value */ function tag(value){return value.$;} function compute(){return tag({$:"text",units:[]});}',value:'text'},
 {name:'common fields use actual matching nominal types',source:'/** @typedef {{ $:"first", amount:number } | { $:"second", amount:number }} Choice */\n/** @param {Choice} value */ function amount(value){return value.amount;} function compute(){return amount({$:"second",amount:19});}',value:19},
 {name:'absent projection evaluates its receiver exactly once',source:prefix+'function compute(){let calls=0;/** @returns {TextChoice} */ const next=()=>{calls+=1;return {$:"none"};};const result=next().units;return result===undefined&&calls===1;}',value:true}
];
const refusals=[
 {name:'heterogeneous constructor fields need a complete union value representation',source:'/** @typedef {{ $:"first", amount:number } | { $:"second", amount:string }} Choice */\n/** @param {Choice} value */ function amount(value){return value.amount;}'},
 {name:'inherited datatype properties retain complete prototype requirements',source:prefix+'/** @param {TextChoice} value */ function inherited(value){return value.toString;}'}
];
cases.push(
 {name:'declared callback record owners settle before nominal field spellings',source:'/** @typedef {{ $:"none" } | { $:"reference", identity:bigint }} Datum */\n/** @typedef {{identity:number,prototype:number}} RecordOwner */\n/** @param {RecordOwner[]} records @returns {number[]} */ function project(records){return records.map(record=>record.identity+record.prototype);} function compute(){return project([{identity:3,prototype:5}])[0]??0;}',value:8},
 {name:'tagged while conditions retain actual constructor fields',source:'/** @typedef {{ $:"stop" } | { $:"step", identity:number }} Step */\n/** @param {Step} value */ function walk(value){let current=value;let result=0;while(current.$==="step"){result+=current.identity;current={$:"stop"};}return result;} function compute(){return walk({$:"step",identity:9});}',value:9},
 {name:'tagged for conditions retain the complete continuation',source:'/** @typedef {{ $:"stop" } | { $:"step", identity:number }} Step */\n/** @param {Step} value */ function walk(value){let current=value;let result=0;for(let at=0;current.$==="step";at+=1){result+=current.identity;current={$:"stop"};}return result;} function compute(){return walk({$:"step",identity:7});}',value:7},
 {name:'tagged do-while continuation preserves termination',source:'/** @typedef {{ $:"stop" } | { $:"step", identity:number }} Step */\n/** @param {Step} value */ function walk(value){let current=value;let count=0;do{count+=1;if(count>=3)current={$:"stop"};}while(current.$==="step");return count;} function compute(){return walk({$:"step",identity:7});}',value:3}
);
const plans=[];
function plan(name,field,positions,types,mode,optional,inference,error='',inherited=false,language='JavaScript'){plans.push({name,language,field,positions,types,inherited,expected:{mode,error,optional},inference});}
plan('common fields retain exact resolved type keys','value',[0,1],['nominal:A','nominal:A'],'field',false,'present');
plan('partial constructor fields retain own undefined','units',[-1,0],['','array:float'],'field',true,'optional');
plan('absent own fields retain undefined','absent',[-1,-1],['',''],'absent',true,'absent');
plan('tag projection is explicit','$',[-1,-1],['',''],'tag',false,'tag');
plan('different nominal owners are not equal by spelling','value',[0,0],['nominal:A','nominal:B'],'refuse',false,'present','different constructor field types need their explicit union property model');
plan('inherited absence needs a complete prototype operation','toString',[-1,0],['','function:0'],'refuse',false,'optional','an inherited datatype property needs its complete prototype operation',true);
plan('empty constructor directories refuse','value',[],[],'refuse',false,'refuse','a datatype property needs its complete bounded constructor directory');
plan('malformed coordinates refuse','value',[-2],[''],'refuse',false,'refuse','a datatype property needs actual matching field coordinates and types');
plan('unpaired field type columns refuse','value',[0],[],'refuse',false,'present','a datatype property needs its complete bounded constructor directory');
plan('other source languages keep their original path','value',[0],['float'],'unchanged',false,'present','',false,'Rust');
const revision='06a124d2d5e1445fc83f4c35a22f08768891d01c';
const predecessors=['js/src/translation/check.js','js/src/translation/javascript-infer.js','js/src/translation/reference-array-projections.js',
 'rust/src/translation/check/records.rs','rust/src/translation/javascript/infer/fields.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});
 const retained='parity/fixtures/history/javascript-data-fields-before-'+path.replaceAll('/','-')+'.lino',target=new URL('../../'+retained,import.meta.url);
 const record={path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
 // Serialize an immutable source record, retaining exact original UTF8 bytes.
 if(existsSync(target)){if(!lstatSync(target).isFile()||lstatSync(target).isSymbolicLink())throw new TypeError('retained source must be a regular file');
  const prior=readProjectData(target);
  if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(key=>prior[key]!==record[key]))throw new TypeError('complete retained predecessor source changed: '+retained);
 }else writeProjectData(target,record);
 return {path,revision,retained,bytes:record.bytes,sha256:record.sha256};
});
const ownerFallbackCases=[{kind:'var',owners:1,settled:false,expected:false},{kind:'var',owners:1,settled:true,expected:true},
 {kind:'var',owners:2,settled:true,expected:false},{kind:'record',owners:1,settled:true,expected:false},{kind:'var',owners:0,settled:true,expected:false}];
const continuationRevision='0b64364aeee2225c2a9c0fb3450cbb15c7e7cf8b';
const continuationPredecessors=['js/src/translation/javascript-infer.js','js/src/translation/javascript-lower.js','js/src/translation/javascript-data-field-rules.js',
 'rust/src/translation/javascript/infer/fields.rs','rust/src/translation/javascript/infer/solver.rs','rust/src/translation/javascript/imperative/loops.rs'].map(path=>{
 const bytes=execFileSync('git',['show',continuationRevision+':'+path],{maxBuffer:4*1024*1024}),retained='parity/fixtures/history/javascript-heap-before-'+path.replaceAll('/','-')+'.lino',target=new URL('../../'+retained,import.meta.url);
 const record={path,revision:continuationRevision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
 if(existsSync(target)){if(!lstatSync(target).isFile()||lstatSync(target).isSymbolicLink())throw new TypeError('retained source must be a regular file');const prior=readProjectData(target);if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(key=>prior[key]!==record[key]))throw new TypeError('complete retained predecessor source changed: '+retained);}
 else writeProjectData(target,record);
 return {path,revision:record.revision,retained,bytes:record.bytes,sha256:record.sha256};
});
writeProjectData(new URL('../../parity/fixtures/javascript-data-fields.lino',import.meta.url),{schemaVersion:1,model:'JavaScript common datatype fields and own absence source projection 1',cases,refusals,plans,predecessors,ownerFallbackCases,continuationPredecessors});
