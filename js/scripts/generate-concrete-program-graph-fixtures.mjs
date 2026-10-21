// Independent finite source and result inputs for CST-preserving graph imports.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
const root=new URL('../../',import.meta.url);

export function generateConcreteProgramGraphFixtures({check=false}={}) {
  const profiles={schemaVersion:1,model:'concrete-program-source-v1',languages:[
    {language:'JavaScript',aliases:['javascript','js','mjs'],grammar:'native-javascript',adapter:'checked source'},
    {language:'Rust',aliases:['rust','rs'],grammar:'native-rust',adapter:'checked source'},
    {language:'TypeScript',aliases:['typescript','ts'],grammar:'native-typescript',adapter:'typed source'},
  ]};
  const fixture={schemaVersion:1,cases:[
    {language:'JavaScript',source:'// origin π\r\nfunction add(a,b) { return a+b; }\r\nconsole.log(add(2,3));\r\n',expectedLines:['5']},
    {language:'Rust',source:'// origin λ\r\nfn add(a: i32,b: i32)->i32 { a+b }\r\nfn main() { println!("{}", add(2,3)); }\r\n',expectedLines:['5']},
    {language:'TypeScript',source:'// origin π\r\nfunction count(value: string): number {return value.length;}\r\nconsole.log(count("😀"));\r\n',expectedLines:['2']},
  ],sourcePins:['js/src/data/native-grammars/javascript.lino','js/src/data/native-grammars/rust.lino',
    'js/src/data/native-grammars/typescript.lino','js/src/typed-program-links.js',
    'experiments/typescript-typed-graph/projection.mjs'].map(file=>{const bytes=readFileSync(new URL(file,root));
      return{file,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};}),
    boundary:'Finite experimental CST-preserving JavaScript, Rust and TypeScript graph controls. Expected output is authored independently. Semantic coverage remains that of the existing checked frontends; no native execution, complete grammar union, public API or global meaning-equivalence evidence.'};
  for(const[path,value]of [['concrete-program-source-profiles',profiles],['concrete-program-graph',fixture]]){
    const file=new URL('parity/fixtures/'+path+'.lino',root);
    if(check)checkProjectData(file,value);else writeProjectData(file,value);
  }
  return{profiles,fixture};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  generateConcreteProgramGraphFixtures({check:process.argv.includes('--check')});
  console.log('Retained three concrete source profiles and independent finite graph inputs.');
}
