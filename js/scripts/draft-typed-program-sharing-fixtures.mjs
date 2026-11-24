// Exact source and retained predecessor data only, without program execution.
import {readProjectData,writeProjectData} from './project-data.mjs';
import {existsSync,lstatSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),programs=[
 {name:'equal scalar spellings do not merge distinct parameter owners',source:'function first(value){return value;}function second(value){return value;}function compute(){return first(2)+second(3);}',expected:5,parameter:'value'},
 {name:'exact array type values share without sharing lexical binders',source:'/** @param {number[]} values @returns {number} */\nfunction first(values){return values.length;}\n/** @param {number[]} values @returns {number} */\nfunction second(values){return values.length;}\nfunction compute(){return first([1])+second([2,3]);}',expected:3,parameter:'values'}];
const revision=execFileSync('git',['rev-parse','b3c01a96'],{cwd:root,encoding:'utf8'}).trim();
const predecessors=['js/src/typed-program-links.js','js/scripts/draft-source-library.mjs','js/scripts/draft-native-model-codecs.mjs',
 'rust/src/typed_program_links.rs','rust/src/typed_program_links/codec.rs','rust/src/typed_program_links/codec_encoding.rs'].map(path=>{
 const bytes=execFileSync('git',['show',revision+':'+path],{cwd:root,maxBuffer:4*1024*1024}),retained='parity/fixtures/history/typed-sharing-before-'+path.replaceAll('/','-')+'.lino',file=new URL(retained,root);
 const record={path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
 if(existsSync(file)){if(!lstatSync(file).isFile()||lstatSync(file).isSymbolicLink())throw new TypeError('retained source must remain a regular file');const prior=readProjectData(file);if(Object.keys(prior).length!==Object.keys(record).length||Object.keys(record).some(key=>prior[key]!==record[key]))throw new TypeError('preceding complete typed source differs');}else writeProjectData(file,record);
 return {path,revision,retained,bytes:record.bytes,sha256:record.sha256};
});
writeProjectData(new URL('parity/fixtures/typed-program-value-sharing.lino',root),{schemaVersion:1,model:'exact scalars, types and source spans 1',versions:{node:['>=22.15 <23','>=24 <25'],rust:['>=1.90 <1.100; edition 2024']},programs,predecessors,
 boundary:'Exact existing encoded values only. Separate expression/declaration/binder occurrences and their references retain ownership; matching names alone never prove concept equivalence.'});
