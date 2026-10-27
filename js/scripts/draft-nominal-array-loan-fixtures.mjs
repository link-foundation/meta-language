// Literal policy boundaries and exact previous source records only.
import {readProjectData,writeProjectData} from './project-data.mjs';
import {existsSync,readFileSync,writeFileSync,lstatSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),base={result:'array',reference:false,receiver:'var',constructors:1,cases:1,owner:true,fields:2,bindings:2,slot:0,matches:1,direct:true};
const cases=[];function add(name,changes,expected){cases.push({name,...base,...changes,expected});}
add('the actual direct immutable field may be borrowed',{},true);
add('the final declared field remains selectable',{slot:1},true);
add('an effectful scrutinee cannot be evaluated as a borrowed variable',{receiver:'call'},false);
add('a shared mutable array keeps its existing reference loan',{reference:true},false);
add('nonarray payloads do not acquire slice semantics',{result:'string'},false);
add('multiple constructors retain their complete match',{constructors:2},false);
add('multiple cases retain their complete match',{cases:2},false);
add('a different actual declaration owner cannot be borrowed',{owner:false},false);
add('an incomplete pattern cannot acquire a field position',{bindings:1},false);
add('missing field coordinates refuse',{slot:-1},false);
add('out of range field coordinates refuse',{slot:2},false);
add('fractional field coordinates refuse',{slot:0.5},false);
add('duplicate bindings do not identify one actual field',{matches:2},false);
add('computed case bodies retain their complete operation',{direct:false},false);
add('the declared field budget is inclusive',{fields:65536,bindings:65536,slot:65535},true);
add('the declared field budget cannot be exceeded',{fields:65537,bindings:65537},false);
const revision=execFileSync('git',['rev-parse','9053da35'],{cwd:root,encoding:'utf8'}).trim();
const predecessors=['js/src/translation/emit-rust.js','rust/src/translation/emit_rust.rs','rust/src/translation/emit_rust/declarations.rs','rust/src/translation/emit_rust/arrays.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{cwd:root,maxBuffer:4*1024*1024}),retained='parity/fixtures/history/nominal-array-loans-before-'+path.replaceAll('/','-')+'.lino',file=new URL(retained,root);
 const record={path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};
 if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink())throw new TypeError('retained source must be regular');const prior=readProjectData(file);if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(key=>prior[key]!==record[key]))throw new TypeError('preceding emitter source changed');}
 else writeProjectData(file,record);return {path,revision,retained,bytes:record.bytes,sha256:record.sha256};
});
const path='parity/self-translation/javascript-ordinary-heap-native.lino',bytes=execFileSync('git',['show',revision+':'+path],{cwd:root,maxBuffer:16*1024*1024}),retained='parity/fixtures/history/javascript-ordinary-heap-before-nominal-loans-native.lino',file=new URL(retained,root);
if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink()||!readFileSync(file).equals(bytes))throw new TypeError('preceding complete heap recipe changed');}else writeFileSync(file,bytes,{flag:'wx'});
predecessors.push({path,revision,retained,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),raw:true});
writeProjectData(new URL('parity/fixtures/nominal-array-loans.lino',root),{schemaVersion:1,model:'checked immutable nominal array field loans 1',cases,predecessors});
