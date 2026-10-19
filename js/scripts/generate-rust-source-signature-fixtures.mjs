import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
import {readProjectData,checkProjectData,writeProjectData} from './project-data.mjs';
import {parseRust} from '../src/translation/rust.js';
import {checkProgram} from '../src/translation/check.js';
import {createTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
export const sourceSignatureModel='Rust source function signature 1';
const owner=(kind,path,arguments_=[])=>({kind,path,arguments:arguments_});
const position=(type,owner_,wrappers=[],lifetimes=[])=>({type,owner:owner_,wrappers,lifetimes});
const primitive=(path,type)=>position(type,owner('primitive',[path]));
const float=()=>primitive('f64',{kind:'float'}),boolean=()=>primitive('bool',{kind:'bool'});
const reference=(value,lifetime=null)=>({...value,wrappers:['reference',...value.wrappers],lifetimes:[lifetime,...value.lifetimes]});
const box=(value,path=['std','boxed','Box'])=>({...value,owner:owner('library',path,[value.owner]),wrappers:['box',...value.wrappers]});
const signature=(parameters,result,lifetimeBindings=[])=>({model:sourceSignatureModel,language:'Rust',parameters,result,lifetimeBindings});
const sourceCases=[
 ['direct-float','fn keep(value:f64)->f64{value}fn main(){}',signature([float()],float())],
 ['direct-boolean','fn keep(value:bool)->bool{value}fn main(){}',signature([boolean()],boolean())],
 ['borrowed-float','fn keep(value:&f64)->f64{*value}fn main(){let value=5.0;println!("{}",keep(&value)==5.0);}',signature([reference(float())],float()),['true']],
 ['borrowed-result','fn keep(value:&f64)->&f64{value}fn main(){}',signature([reference(float())],reference(float()))],
 ['standard-box','fn keep(value:Box<f64>)->Box<f64>{value}fn main(){}',signature([box(float())],box(float()))],
 ['alloc-box','extern crate alloc;fn keep(value:alloc::boxed::Box<f64>)->alloc::boxed::Box<f64>{value}fn main(){}',signature([box(float(),['alloc','boxed','Box'])],box(float(),['alloc','boxed','Box']))],
 ['borrowed-string','fn keep(value:&str)->&str{value}fn main(){}',signature([reference(primitive('str',{kind:'string'}))],reference(primitive('str',{kind:'string'})))],
 ['owned-string','fn keep(value:String)->String{value}fn main(){}',signature([position({kind:'string'},owner('library',['std','string','String']))],position({kind:'string'},owner('library',['std','string','String'])))],
 ['unsigned-size','fn keep(value:usize)->usize{value}fn main(){}',signature([primitive('usize',{kind:'fixed',bits:64,signed:false})],primitive('usize',{kind:'fixed',bits:64,signed:false}))],
 ['signed-size','fn keep(value:isize)->isize{value}fn main(){}',signature([primitive('isize',{kind:'fixed',bits:64,signed:true})],primitive('isize',{kind:'fixed',bits:64,signed:true}))],
 ['copy-tuple','fn keep(value:(f64,bool))->(f64,bool){value}fn main(){}',signature([position({kind:'tuple',elements:[{kind:'float'},{kind:'bool'}]},owner('tuple',[],[float().owner,boolean().owner]))],position({kind:'tuple',elements:[{kind:'float'},{kind:'bool'}]},owner('tuple',[],[float().owner,boolean().owner])))],
 ['unit','fn keep(value:())->(){value}fn main(){}',signature([position({kind:'unit'},owner('unit',[]))],position({kind:'unit'},owner('unit',[])))],
 ['named-data','enum Choice{A}fn keep(value:Choice)->Choice{value}fn main(){}',signature([position({kind:'data',name:'Choice'},owner('declaration',['Choice']))],position({kind:'data',name:'Choice'},owner('declaration',['Choice'])))],
 ['named-lifetimes',"fn keep<'a,'b>(value:&'a f64,other:&'b bool)->bool{*other}fn main(){}",signature([reference(float(),'a'),reference(boolean(),'b')],boolean(),['a','b'])],
 ['slice','fn keep(value:&[f64])->bool{value.is_empty()}fn main(){}',signature([position({kind:'array',element:{kind:'float'}},owner('slice',[],[float().owner]),['reference'],[null])],boolean())],
 ['vector','fn keep(value:Vec<f64>)->bool{value.is_empty()}fn main(){}',signature([position({kind:'array',element:{kind:'float'}},owner('library',['std','vec','Vec'],[float().owner]))],boolean())],
 ['value-namespace-function','fn f64(value:f64)->f64{value}fn main(){}',signature([float()],float()),[], 'f64'],
 ['value-namespace-boolean','fn bool(value:bool)->bool{value}fn main(){}',signature([boolean()],boolean()),[], 'bool'],
 ['value-namespace-constant','const f64:f64=1.0;fn keep(value:f64)->f64{value}fn main(){}',signature([float()],float())],
];
const refused=[
 ['shadowed-float','enum f64{Empty}fn keep(value:f64)->f64{value}fn main(){}'],
 ['shadowed-boolean','enum bool{Empty}fn keep(value:bool)->bool{value}fn main(){}'],
 ['shadowed-str','enum str{Empty}fn keep(value:&str)->&str{value}fn main(){}'],
 ['nested-type-shadow','enum f64{Empty}mod nested{fn f64(value:f64)->f64{value}}fn main(){}'],
 ['mutable-reference','fn keep(value:&mut f64)->f64{*value}fn main(){}'],
 ['bounded-lifetime',"fn keep<'a:'static>(value:&'a f64)->f64{*value}fn main(){}"],
 ['generic-type','fn keep<T>(value:T)->T{value}fn main(){}'],
];
export function rustSourceSignatureFixture(){
 const cases=sourceCases.map(([id,source,expected,output=[],functionName='keep'])=>{
  const surface=parseRust(source,{sourceSignatureModel});const checked=checkProgram(surface);
  assert.deepEqual(checked.declarations.get(functionName).sourceSignature,expected,id+' independent header oracle');
  const archive=createTypedProgramLinks(source,'Rust',{rustSourceSignatureModel:sourceSignatureModel,provenance:{project:id}});
  return {id,source,functionName,expected,output,archive:archive.network.toLino(),root:archive.root.asU64()};
 });
 const invalid=refused.map(([id,source])=>{
  try{checkProgram(parseRust(source,{sourceSignatureModel}));}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
  throw new Error('source signature refusal was accepted: '+id);
 });
 const schema=readProjectData(new URL('../src/data/typed-program-links-schema.lino',import.meta.url));
 const field=(network,parent,name)=>network.fieldRelations().find(row=>row.parent.equals(parent)&&row.label===(schema.fields[name]??name))?.child;
 const malformed=[];
 function mutate(id,caseId,edit){
  const entry=cases.find(value=>value.id===caseId),network=LinkNetwork.fromLino(entry.archive);
  const header=network.fieldRelations().find(row=>row.label===schema.fields.sourceSignature).child;
  edit(network,header);
  for(const target of ['JavaScript','Rust'])assert.throws(()=>emitTypedProgramLinks(network,entry.root,target),undefined,id);
  malformed.push({id,archive:network.toLino(),root:entry.root});
 }
 const set=(network,id,text)=>network.link(id).setMetadata(network.link(id).metadata().withDefinition(text));
 mutate('wrong-model','direct-float',(network,header)=>set(network,field(network,header,'model'),'unknown source signature'));
 mutate('wrong-language','direct-float',(network,header)=>set(network,field(network,header,'language'),'JavaScript'));
 mutate('missing-parameter','direct-float',(network,header)=>network.link(field(network,header,'parameters')).setReferences([]));
 mutate('result-type','direct-float',(network,header)=>set(network,field(network,field(network,field(network,header,'result'),'type'),'kind'),'bool'));
 mutate('owner-path','direct-float',(network,header)=>set(network,network.link(field(network,field(network,field(network,header,'result'),'owner'),'path')).references()[0],'bool'));
 mutate('unknown-owner','direct-float',(network,header)=>set(network,field(network,field(network,field(network,header,'result'),'owner'),'kind'),'foreign owner'));
 mutate('missing-reference-lifetime','borrowed-float',(network,header)=>network.link(field(network,network.link(field(network,header,'parameters')).references()[0],'lifetimes')).setReferences([]));
 mutate('removed-box-witness','standard-box',(network,header)=>network.link(field(network,network.link(field(network,header,'parameters')).references()[0],'wrappers')).setReferences([]));
 mutate('unknown-type-fact','direct-float',(network,header)=>network.insertFieldRelation(field(network,field(network,header,'result'),'type'),'unrecognizedSourceFact',field(network,header,'model')));
 mutate('unknown-position-fact','direct-float',(network,header)=>network.insertFieldRelation(field(network,header,'result'),'unrecognizedSourceFact',field(network,header,'model')));
 const witness=cases.find(value=>value.id==='borrowed-float'),edited=LinkNetwork.fromLino(witness.archive);
 const literals=edited.fieldRelations().filter(row=>row.label===schema.fields.k&&edited.link(row.child).metadata().definition==='lit');
 const literal=literals.find(row=>edited.link(field(edited,row.parent,'value')).metadata().definition==='5');
 assert.ok(literal,'the actual first source literal is linked');set(edited,field(edited,literal.parent,'value'),'7');
 const valueEdit={archive:edited.toLino(),root:witness.root,output:['false'],expected:witness.expected};
 return{schemaVersion:1,sourceSignatureModel,malformed,valueEdit,scope:'owned source function headers; qualifier-preserving target emission remains unproved',cases,invalid};
}
if(process.argv[1]&&import.meta.url===pathToFileURL(process.argv[1]).href){
 const file=new URL('../../parity/fixtures/rust-source-signatures.lino',import.meta.url),fixture=rustSourceSignatureFixture();
 if(process.argv.includes('--check'))checkProjectData(file,fixture);else writeProjectData(file,fixture);
 console.log(`Rust source signatures: ${fixture.cases.length} independently specified headers and ${fixture.invalid.length} precise refusals`);
}
