// Genuine raw Links edits retain the existing JavaScript Number interpretation.
import assert from 'node:assert/strict';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';

const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(network,owner,name)=>network.fieldRelations().find(row=>row.parent.equals(owner)&&row.label===label(name))?.child;
export function numericMetadataArchive(entry,edit){
 const network=LinkNetwork.fromLino(entry.archive),scalar=network.link(edit.scalar);
 assert.ok(scalar);scalar.setMetadata(scalar.metadata().withDefinition(edit.spelling));
 return{network:LinkNetwork.fromLino(network.toLino()),root:entry.root};
}
export function numericMetadataFixture(){
 const source='fn keep(value:i32)->i32{value}fn main(){println!("{}",keep(7i32)==7i32);}';
 const encoded=createTypedProgramLinks(source,'Rust');
 const text=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url)).cases[0];
 const cases=[{id:'fixed type',source,archive:encoded.network.toLino(),root:encoded.root.asU64(),output:['true']},
  {id:'owned text proof',source:text.source,archive:text.links,root:text.root,observations:text.observations,name:text.name}].map(entry=>{
  const network=LinkNetwork.fromLino(entry.archive),scalars=new Map();
  for(const row of network.fieldRelations()){
   if(row.label!==label('bits'))continue;
   const kind=field(network,row.parent,'kind');
   if(!kind||network.link(kind).metadata().definition!=='fixed')continue;
   const value=Number(network.link(row.child).metadata().definition);scalars.set(row.child.asU64(),value);
  }
  assert.ok(scalars.size);
  const expected=decodeTypedProgramLinks(network,entry.root),aliases=[],invalid=[];
  for(const[scalar,value]of scalars){
   for(const spelling of [`${value}.0`,`${value}e0`,`${value/10}e1`]){
    const edit={id:`${entry.id}-${scalar}-${spelling}`,scalar,value,spelling};
    const loaded=numericMetadataArchive(entry,edit);assert.deepEqual(decodeTypedProgramLinks(loaded.network,loaded.root),expected,edit.id);aliases.push(edit);
   }
  }
  const scalar=[...scalars.keys()][0];
  for(const spelling of ['-0','-0.0','0','-32','32.5','9007199254740992','1e309']){
   const edit={id:`${entry.id}-refuse-${spelling}`,scalar,spelling},loaded=numericMetadataArchive(entry,edit);
   assert.throws(()=>decodeTypedProgramLinks(loaded.network,loaded.root),undefined,edit.id);invalid.push(edit);
  }
  if(entry.id==='owned text proof'){
   const proof=network.fieldRelations().find(row=>row.label===label('sourceLocalState')).child;
   const bindings=network.link(field(network,proof,'bindings')).references();
   const binding=bindings.find(id=>{const type=field(network,id,'type');return type&&network.link(field(network,type,'kind')).metadata().definition==='fixed';});
   const bindingType=field(network,binding,'type'),bindingBits=field(network,bindingType,'bits');
   assert.ok(aliases.some(edit=>edit.scalar===bindingBits.asU64()));entry.proofBindingScalar=bindingBits.asU64();
  }
  return{...entry,aliases,invalid};
 });
 return{schemaVersion:1,scope:'only finite equivalent Number spellings of existing fixed type bit widths, including actual owned text proof binding types; scalar floating meaning is unchanged',cases};
}
if(process.argv[1]===new URL(import.meta.url).pathname){
 const value=numericMetadataFixture(),file=new URL('./program-numeric-metadata-fixture.lino',import.meta.url);
 if(process.argv.includes('--check')){if(!checkProjectData(file,value))throw new Error('numeric metadata fixtures are stale');}else writeProjectData(file,value);
 console.log(`numeric metadata: ${value.cases.reduce((count,entry)=>count+entry.aliases.length,0)} actual raw aliases and ${value.cases.reduce((count,entry)=>count+entry.invalid.length,0)} refusals`);
}
