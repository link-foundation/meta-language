import assert from 'node:assert/strict';
import {readProjectData,writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {LinkNetwork} from '../js/src/network.js';
import {decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../js/src/typed-program-links.js';
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>' '+letter.toLowerCase());
const field=(network,owner,name)=>network.fieldRelations().find(row=>row.parent.equals(owner)&&row.label===label(name)).child;
export function ownedOccurrencePositionArchive(entry,edit){
 const network=LinkNetwork.fromLino(entry.archive),node=network.link(edit.scalar);
 let metadata=node.metadata().withDefinition(edit.spelling);
 if(edit.scalarKind)metadata=metadata.withTerm(schema.scalars[edit.scalarKind]);node.setMetadata(metadata);
 return{network:LinkNetwork.fromLino(network.toLino()),root:entry.root};
}
export function ownedOccurrencePositionFixture(){
 const originals=readProjectData(new URL('./rust-owned-local-state-fixture.lino',import.meta.url));
 const cases=originals.cases.map(original=>{
  const entry={id:original.name,name:original.name,archive:original.links,root:original.root,observations:original.observations};
  const network=LinkNetwork.fromLino(entry.archive),proof=network.fieldRelations().find(row=>row.label===label('sourceLocalState')).child;
  const reads=network.link(field(network,proof,'reads')).references();let scalar;
  for(const[readIndex,read]of reads.entries()){
   const path=network.link(field(network,read,'path')).references();
   const pathPartIndex=path.findIndex(id=>network.link(id).metadata().term===schema.scalars.number);
   if(pathPartIndex>=0){scalar=path[pathPartIndex];entry.readIndex=readIndex;entry.pathPartIndex=pathPartIndex;break;}
  }
  assert.ok(scalar);assert.equal(network.link(scalar).metadata().definition,'0');
  const expected=decodeTypedProgramLinks(network,entry.root),aliases=['0.0','0e0','-0','-0.0','-0e0'].map(spelling=>({id:entry.id+'-position-'+spelling,scalar:scalar.asU64(),spelling}));
  const invalid=['0.5','1.0','2.0','-1','9007199254740992','1e309'].map(spelling=>({id:entry.id+'-refuse-'+spelling,scalar:scalar.asU64(),spelling}));invalid.push({id:entry.id+'-refuse-string-zero',scalar:scalar.asU64(),spelling:'0',scalarKind:'string'});
  for(const edit of aliases){const packet=ownedOccurrencePositionArchive(entry,edit),observed=decodeTypedProgramLinks(packet.network,packet.root),comparison=structuredClone(expected);comparison.items[0].sourceLocalState.reads[entry.readIndex].path[entry.pathPartIndex]=Number(edit.spelling);assert.deepEqual(observed,comparison,edit.id);}
  for(const edit of invalid){const packet=ownedOccurrencePositionArchive(entry,edit);assert.throws(()=>decodeTypedProgramLinks(packet.network,packet.root),undefined,edit.id);}
  return{...entry,aliases,invalid};
 });
 return{schemaVersion:1,scope:'actual Number positions in owned source-free occurrence paths; integral aliases and either sign of zero preserve one position while strings and wrong occurrences refuse',cases};
}
if(process.argv[1]===new URL(import.meta.url).pathname){
 const value=ownedOccurrencePositionFixture(),file=new URL('./owned-occurrence-position-fixture.lino',import.meta.url);
 if(process.argv.includes('--check')){if(!checkProjectData(file,value))throw new Error('owned occurrence position fixtures are stale');}else writeProjectData(file,value);
 console.log('owned occurrence positions: 25 actual aliases and 35 refusal controls over five frozen sources');
}
