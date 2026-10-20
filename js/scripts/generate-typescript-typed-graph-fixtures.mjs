// Authored inputs and independent results; never derive an oracle from emission.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url);

/** Store finite TypeScript graph controls as ordinary inspectable Links data. */
export function generateTypeScriptTypedGraphFixtures({check=false}={}) {
  const value={schemaVersion:1,id:'typescript-typed-semantic-graph',model:'typescript-portable-typing-v1',
    boundary:'Finite JavaScript draft controls under the exact recorded grammar and erasure sources. Complete original and lowered concrete trees, editable checked semantics, text and binary reload, and refusal inputs only. No TypeScript compiler, native runtime, full language coverage, clean installed consumer or global meaning-equivalence claim.',
    cases:[
      {source:'// π\r\nfunction add(a: number,b: number): number {return a+b;}\r\nconsole.log(add(2,3));\r\n',expectedLines:['5']},
      {source:'type Word = string;\nfunction echo(value: Word): Word {return value;}\nconsole.log(echo("λ"));\n',expectedLines:['λ']},
      {source:'function count(value: string): number {return value.length;}\nconsole.log(count("😀"));\n',expectedLines:['2']},
    ],
    edit:{source:'function add(a: number,b: number): number {return a+b;}\nconsole.log(add(2,3));\n',
      before:'add',after:'mul',expectedLines:['6']},
    refusals:['enum Entry { One }','namespace Entry { export const value = 1; }',
      'function answer(): number {return "wrong";}'],
    nonScalarSource:{prefix:'// ',units:[55296],suffix:'\nfunction answer(): number {return 1;}'},
    sourcePins:[
      'js/src/data/native-grammars/typescript.lino','js/src/data/native-grammars/javascript.lino',
      'js/src/data/typescript-erasure.lino','js/src/data/typescript-portable-typing.lino',
      'js/src/typescript-erasure.js','js/src/typescript-portable-typing.js',
    ].map(file=>{const bytes=readFileSync(new URL(file,root));return{file,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};}),
  };
  const file=new URL('parity/fixtures/typescript-typed-semantic-graph.lino',root);
  if(check)checkProjectData(file,value);else writeProjectData(file,value);
  return value;
}

/** Prepare real program networks for the native package's existing graph API.
 * Graphs are inputs, not independent semantic oracles. Result lines remain the
 * separately authored fixture values above. No native tool runs here.
 */
export async function generateTypeScriptTypedGraphArchives({check=false}={}) {
  const fixture=generateTypeScriptTypedGraphFixtures({check});
  const [{createTypeScriptSemanticGraph},{TYPED_PROGRAM_LINKS_SCHEMA}]=await Promise.all([
    import('../../experiments/typescript-typed-graph/projection.mjs'),import('../src/typed-program-links.js'),
  ]);
  const directory='parity/fixtures/typescript-typed-graph';
  if(!check)mkdirSync(new URL(directory+'/',root),{recursive:true});
  const cases=[...fixture.cases.map(row=>({...row,edit:false})),{...fixture.edit,edit:true}],records=[];
  for(const [index,row]of cases.entries()){
    const packet=createTypeScriptSemanticGraph(row.source,{utf16Strings:true});
    if(row.edit){
      const label=TYPED_PROGRAM_LINKS_SCHEMA.fields.op??'op';
      const operation=packet.network.fieldRelations().find(field=>field.label===label
        &&packet.network.link(field.child).metadata().definition===row.before);
      assert.ok(operation);const link=packet.network.link(operation.child);
      link.setMetadata(link.metadata().withDefinition(row.after));
    }
    const source=packet.network.toLino(),path=directory+'/case-'+index+'.lino';
    if(check)assert.equal(readFileSync(new URL(path,root),'utf8'),source);else writeFileSync(new URL(path,root),source);
    records.push({index,root:String(packet.root.asU64()),path,bytes:Buffer.byteLength(source),
      sha256:createHash('sha256').update(source).digest('hex'),expectedLines:row.expectedLines});
  }
  const manifest={schemaVersion:1,sourceFixture:'parity/fixtures/typescript-typed-semantic-graph.lino',cases:records,
    boundary:'JavaScript-produced Links graph inputs with both complete CSTs for the native package. Expected lines are independent authored values. These prepared inputs are not native execution evidence.'};
  const path=new URL(directory+'/manifest.lino',root);
  if(check)checkProjectData(path,manifest);else writeProjectData(path,manifest);
  return manifest;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  const value=generateTypeScriptTypedGraphFixtures({check:process.argv.includes('--check')});
  await generateTypeScriptTypedGraphArchives({check:process.argv.includes('--check')});
  console.log('Retained '+value.cases.length+' original TypeScript inputs and independently authored outputs.');
}
