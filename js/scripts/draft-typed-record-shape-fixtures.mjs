// Literal source and exact predecessor captures only; never parse/run inputs.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),revision='9fa55ebd';
const paths=['js/src/typed-program-links.js','js/src/index.js','js/src/index.d.ts','js/scripts/draft-source-library.mjs','js/scripts/draft-javascript-call-machine.mjs','js/scripts/draft-javascript-call-source.mjs',
 'js/src/formal-concept-definition.js','js/src/formal-function-links.js','rust/src/formal_concept_definition.rs','rust/src/formal_function_links.rs','rust/src/typed_program_links.rs','rust/src/typed_program_links/codec.rs','rust/src/lib.rs'];
const predecessors=paths.map(path=>{const source=execFileSync('git',['show',revision+':'+path],{cwd:root,encoding:'utf8',maxBuffer:4*1024*1024}),retained='parity/fixtures/history/record-shape-before-'+path.replaceAll('/','-')+'.lino',record={path,revision,source};
 writeProjectData(new URL(retained,root),record);return {path,revision,retained,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};});
writeProjectData(new URL('parity/fixtures/typed-program-record-shapes.lino',root),{schemaVersion:1,model:'ordered typed program record fields 1',nodeVersionRange:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rustVersionRange:'>=1.90.0 <1.100.0',predecessors,programs:[
 {name:'shared field layout keeps lexical and literal occurrences separate',source:'/** @param {number} value @returns {number} */ export function first(value){return value+1;} /** @param {number} value @returns {number} */ export function second(value){return value+1;}',declarations:['first','second'],edit:true},
 {name:'formal Boolean declaration selection ignores storage layout',source:'/** @param {boolean} value @returns {boolean} */ export function choose(value){return !value;}',declarations:['choose'],boolean:true},
]});
