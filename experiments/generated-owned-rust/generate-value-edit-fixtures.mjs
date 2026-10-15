import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {generatedOwnedRustValueEditPolicy} from './generate-value-edit-rules.mjs';

export function generatedOwnedRustValueEditFixtures(){return{
 version:1,model:generatedOwnedRustValueEditPolicy.model,
 quoterSourceSha256:'e14fc708624025b29b5d51284d53244a35dc919d6e6e6e4310aaf5a9e70e0c87',
 protectedArtifacts:['data/policy.lino','data/fixtures.lino','rules.mjs','data/rules.rs','data/library-styles.lino'].map(path=>({path,sha256:createHash('sha256').update(readFileSync(new URL(path,import.meta.url))).digest('hex')})),
 strings:[
  {name:'every C0 and delete control',value:Array.from({length:32},(_,unit)=>String.fromCharCode(unit)).join('')+'7Ω😀\x7f'},
  {name:'every seven bit scalar',value:Array.from({length:128},(_,unit)=>String.fromCharCode(unit)).join('')},
  {name:'null immediately before digits',value:'\0'+'123'},
  {name:'quotation and reverse solidus with line breaks',value:'"\\\r\n\tΩ😀'},
  {name:'empty text',value:''},
  {name:'non ASCII scalar boundaries',value:'\u0085\u2028\u2029\ufeff\u{10ffff}'},
 ],
 integers:[
  {source:'-1',value:'1'},{source:'-2147483648',value:'2147483647'},{source:'2147483647',value:'-2147483648'},
  {source:'-1',value:'0'},{source:'0',value:'-1'},{source:'-0',value:'0'},{source:'-0',value:'-1'},{source:'00',value:'-2147483648'},
 ],
 signContexts:[
  {source:'if-1i32==n',value:'1',retained:'if 1i32==n'},
  {source:'if-/* retained Ω😀 */1i32==n',value:'1',retained:'if /* retained Ω😀 */1i32==n'},
  {source:'if(n==(-/* retained sign trivia */1i32))',value:'1',retained:' /* retained sign trivia */1i32'},
 ],
 originalRefusals:{
  control:{reason:'generated whole owned Rust: the literal requires Unicode scalar text and its supported Rust quoting domain',span:{start:717,end:723}},
  sign:{reason:'generated whole owned Rust: a negative source operation cannot become a positive value without a new sign interpretation',span:{start:614,end:618}},
 },
};}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('./data/value-edit-fixtures.lino',import.meta.url),data=generatedOwnedRustValueEditFixtures();
 if(process.argv.includes('--check'))checkProjectData(file,data);else writeProjectData(file,data);
}
