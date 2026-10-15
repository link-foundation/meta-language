import assert from 'node:assert/strict';
import {writeProjectData,checkProjectData} from './project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,TYPED_PROGRAM_LINKS_SCHEMA as schema} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';

const destination=new URL('../../parity/fixtures/source-span-numbers.lino',import.meta.url);
const label=name=>schema.fields[name]??name.replace(/[A-Z]/gu,letter=>` ${letter.toLowerCase()}`);
const field=(network,owner,name)=>network.fieldRelations().find(row=>row.parent.equals(owner)&&row.label===label(name))?.child;

/** Mutate only the requested offset scalar in an actual serialized graph. */
export function sourceSpanNumberArchive(entry,edit) {
  const network=LinkNetwork.fromLino(entry.archive),node=network.link(edit.scalar);
  node.setMetadata(node.metadata().withDefinition(edit.spelling));
  return {network:LinkNetwork.fromLino(network.toLino()),root:entry.root};
}

export function sourceSpanNumberFixture() {
  const sources=[
    ['primitive','fn keep(value:f64)->f64{value}fn main(){println!("{}",keep(-0f64)==0f64);}'],
    ['datatype','#[derive(Clone)]enum Choice{Only}mod nested{fn keep(value:crate::Choice)->crate::Choice{value.clone()}}fn main(){let value=nested::keep(Choice::Only);println!("{}",match value{Choice::Only=>true});}'],
    ['default','fn keep(value:f64)->f64{value}fn main(){println!("{}",keep(-0f64)==0f64);}'],
  ];
  const cases=sources.map(([id,source])=>{
    const encoded=createTypedProgramLinks(source,'Rust',id==='default'?{}:{rustTypeNamespaceModel:'resolved Rust type namespace 1'});
    const network=LinkNetwork.fromLino(encoded.network.toLino()),archive=network.toLino(),root=encoded.root.asU64();
    const spans=network.fieldRelations().filter(row=>row.label===label('span')&&field(network,row.child,'start'));
    const aliases=[];
    for(const [spanIndex,span] of spans.entries())for(const offset of ['start','end']){
      const scalar=field(network,span.child,offset),value=Number(network.link(scalar).metadata().definition);
      const spellings=[`${value}.0`,`${value}e0`,`${value/10}e1`];
      for(const spelling of spanIndex===0?spellings:[spellings[(spanIndex+(offset==='end'?1:0)+(id==='default'?2:0))%spellings.length]]) {
        const edit={id:`${id}-${spanIndex}-${offset}-${spelling}`,scalar:scalar.asU64(),offset,value,spelling};
        assert.doesNotThrow(()=>decodeTypedProgramLinks(...Object.values(sourceSpanNumberArchive({archive,root},edit))),edit.id);
        aliases.push(edit);
      }
    }
    const scalar=field(network,spans[0].child,'start');
    const invalid=['-0','-0.0','-0e0','-1','0.5','9007199254740992','1e309'].map(spelling=>({id:`${id}-refuse-${spelling}`,scalar:scalar.asU64(),spelling}));
    for(const edit of invalid)assert.throws(()=>decodeTypedProgramLinks(...Object.values(sourceSpanNumberArchive({archive,root},edit))),undefined,edit.id);
    return{id,source,archive,root,output:['true'],aliases,invalid};
  });
  return{schemaVersion:1,scope:'equivalent finite integral number spellings on actual named source spans; source offset sign, safe range and ordering remain required',cases};
}
if(process.argv[1]===new URL(import.meta.url).pathname) {
  const fixture=sourceSpanNumberFixture();
  if(process.argv.includes('--check'))checkProjectData(destination,fixture);
  else writeProjectData(destination,fixture);
  console.log(`source-span numbers: ${fixture.cases.reduce((count,entry)=>count+entry.aliases.length,0)} actual offset aliases and ${fixture.cases.reduce((count,entry)=>count+entry.invalid.length,0)} refusals`);
}
