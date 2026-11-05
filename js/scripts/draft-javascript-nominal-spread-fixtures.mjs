// Literal source programs/expectations and immutable preceding source capture.
import {writeProjectData,readProjectData} from './project-data.mjs';
import {existsSync,lstatSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),type='/** @typedef {{$:"state",value:number,active:boolean}} State */\n';
const programs=[
 {name:'complete nominal copies preserve the old value and apply the override',source:type+'/** @param {State} value @returns {State} */\nfunction copy(value){return {...value,value:9};}\nfunction compute(){const original={$:"state",value:3,active:true};const result=copy(original);return original.value+result.value;}',expected:12},
 {name:'every overwritten spread and override evaluates exactly once in order',source:type+'/** @param {number[]} events @returns {State} */\nfunction first(events){events.push(1);return {$:"state",value:1,active:false};}\n/** @param {number[]} events @returns {number} */\nfunction second(events){events.push(2);return 2;}\n/** @param {number[]} events @returns {State} */\nfunction third(events){events.push(3);return {$:"state",value:7,active:true};}\nfunction compute(){const events=[];const value={...first(events),value:second(events),...third(events)};return (events[0]??0)*1000+(events[1]??0)*100+(events[2]??0)*10+value.value;}',expected:1237},
 {name:'an exact presence guard protects the same optional nominal binder',source:type+'/** @param {State[]} values @returns {number} */\nfunction read(values){const first=values[0];if(first===undefined)return -1;const copy={...first,value:9};return copy.value;}\nfunction compute(){return read([{$:"state",value:1,active:true}])+read([]);}',expected:8},
];
const refusals=[
 {name:'extra own fields cannot disappear',source:type+'/** @param {State} value @returns {State} */\nfunction copy(value){return {...value,extra:2};}'},
 {name:'an unguarded optional spread cannot invent absent fields',source:type+'/** @param {State[]} values @returns {State} */\nfunction copy(values){return {...values[0],value:9};}'},
 {name:'a leading override needs a distinct own-field order interpretation',source:type+'/** @param {State} value @returns {State} */\nfunction copy(value){return {value:2,...value};}'},
 {name:'nominal owner names cannot be substituted by a matching field shape',source:type+'/** @typedef {{$:"other",value:number,active:boolean}} Other */\n/** @param {State} left @param {Other} right @returns {State} */\nfunction copy(left,right){return {...left,...right};}'},
 {name:'multiple constructors require their complete tag-dependent copy model',source:'/** @typedef {{$:"left",value:number}|{$:"right",value:number}} Choice */\n/** @param {Choice} value @returns {Choice} */\nfunction copy(value){return {...value,value:9};}'},
 {name:'a new shadowed binder cannot inherit another binders absence proof',source:type+'/** @param {State[]} values @returns {number} */\nfunction copy(values){const first=values[0];if(first===undefined)return -1;{const first=values[1];const result={...first,value:9};return result.value;}}'},
];
const base={language:'JavaScript',constructors:1,declared:['value','active'],kinds:['spread','field'],names:['','value'],owners:['State',''],owner:'State',prototype:false};
const policies=[{name:'a complete actual owner and declared override',...base,expected:''},
 {name:'different owner',...base,owners:['Other',''],expected:'nominal object copies retain their actual complete source owner'},
 {name:'extra field',...base,names:['','extra'],expected:'nominal object copies cannot add fields or replace their constructor tag'},
 {name:'tag replacement',...base,names:['','$'],expected:'nominal object copies cannot add fields or replace their constructor tag'},
 {name:'multiple constructors',...base,constructors:2,expected:'nominal object copies need one complete actual constructor'},
 {name:'prototype replacement',...base,prototype:true,expected:'nominal object copies cannot replace their complete prototype interpretation'},
 {name:'duplicate declaration',...base,declared:['value','value'],expected:'nominal object copies require complete unique declared fields'},
 {name:'incomplete directory',...base,names:[''],expected:'nominal object copies start with a complete actual source value'}];
const guards=[{kind:'binary',operation:'eq',absence:'undefined',expected:2},{kind:'binary',operation:'ne',absence:'undefined',expected:1},
 {kind:'binary',operation:'eq',absence:'null',expected:2},{kind:'binary',operation:'ne',absence:'null',expected:1},
 {kind:'math',operation:'eq',absence:'null',expected:0},{kind:'binary',operation:'lt',absence:'undefined',expected:0},{kind:'binary',operation:'ne',absence:'both',expected:0}];
const revision=execFileSync('git',['rev-parse','b3c01a96'],{cwd:root,encoding:'utf8'}).trim();
const predecessors=['js/src/translation/check.js','js/src/translation/javascript-infer.js','js/scripts/portable-source-library-style.mjs',
 'rust/src/translation/check.rs','rust/src/translation/check/expressions.rs','rust/src/translation/check/expressions/branches.rs','rust/src/translation/check/items.rs','rust/src/translation/check/records.rs',
 'rust/src/translation/javascript/infer.rs','rust/src/translation/javascript/infer/records.rs','rust/src/translation/javascript/infer/solver.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{cwd:root,maxBuffer:4*1024*1024}),retained='parity/fixtures/history/nominal-spread-before-'+path.replaceAll('/','-')+'.lino',file=new URL(retained,root);
 const record={path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
 if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink())throw new TypeError('retained source must remain a regular file');const prior=readProjectData(file);if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(key=>prior[key]!==record[key]))throw new TypeError('preceding complete source differs');}else writeProjectData(file,record);
 return {path,revision,retained,bytes:record.bytes,sha256:record.sha256};
});
writeProjectData(new URL('parity/fixtures/javascript-nominal-spreads.lino',root),{schemaVersion:1,model:'owned nominal JavaScript complete field copies 1',versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024']},programs,refusals,policies,guards,predecessors,
 boundary:'Complete closed single-constructor owned values and unchanged guarded binders only. General CopyDataProperties, optional/multiple constructor layouts, prototype/exotic behavior and frontend completeness remain required.'});
