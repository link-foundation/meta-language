import {isDeepStrictEqual} from 'node:util';
import {fileURLToPath} from 'node:url';
import {readProjectData, writeProjectData} from './project-data.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';

export const JAVA_SCRIPT_SCOPED_BINDING_MODEL = 'owned JavaScript scoped bindings 1';
const loopModel = 'closed declaration scalar loop 1';
export function javaScriptScopedBindingEmissionFixture() {
  const specifications = [
    {id:'declaration-loop', language:'Rust', source:'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}', entry:'run', capture:{rustScalarLoopModel:loopModel}, inputs:[[0,3],[4,3],['-0',0]], expected:[3,4,'-0'], output:[]},
    {id:'print-discard-order', language:'JavaScript', source:'function marker(x){console.log("called");return x;} console.log("before");marker(2);console.log("after");', entry:'marker', capture:{}, inputs:[[2],[4]], expected:[2,4], output:['before','called','after']},
    {id:'declaration-alias-collision', language:'JavaScript', source:'function binding_value(x){return x+10;} function choose(value){const shadow=value+1;return binding_value(shadow)+value;}choose(2);', entry:'choose', capture:{}, inputs:[[2],[3]], expected:[15,17], output:[]},
    {id:'lexical-shadow', language:'JavaScript', source:'function choose(value){const previous=value;{const value=previous+1;return value+previous;}}choose(2);', entry:'choose', capture:{}, inputs:[[2],[0]], expected:[5,1], output:[]},
    {id:'scalar-snapshot-declaration-collision', language:'Rust', source:'fn first(mut x:f64,mut at:f64,limit:f64)->f64{loop{return if at<limit{(x,at)=(x+1f64,at+1f64);continue;}else{x};}}fn second(mut x:f64,mut at:f64,limit:f64)->f64{loop{return if at<limit{(x,at)=(x+1f64,at+1f64);continue;}else{x};}}fn main(){}', entry:'first', capture:{rustScalarLoopModel:loopModel}, inputs:[[0,0,3],[4,4,3]], expected:[3,4], output:[], declarationEdit:{from:'second',to:'ml_scalar_write0'}, additionalObservations:[{entry:'ml_scalar_write0',inputs:[[0,0,3],[4,4,3]],expected:[3,4]}]},
  ];
  const cases=specifications.map(specification => {
    let archive=createTypedProgramLinks(specification.source,specification.language,specification.capture);
    if(specification.declarationEdit){
      const {from,to}=specification.declarationEdit;
      const program=decodeTypedProgramLinks(archive.network,archive.root);
      const declaration=program.declarations.get(from);
      declaration.name=to;declaration.fullName=to;declaration.sourceLoop.sourceName=to;
      declaration.sourceLoop.continuation.path=[to];
      const pending=[declaration.body];
      for(let at=0;at<pending.length;at+=1){
        const value=pending[at];
        if(value.k==='call'&&value.fn===from)value.fn=to;
        for(const [key,child] of Object.entries(value))if(!['type','span','domain'].includes(key)){
          if(Array.isArray(child)){for(const nested of child)if(nested&&typeof nested==='object')pending.push(nested);}
          else if(child&&typeof child==='object')pending.push(child);
        }
      }
      program.declarations.delete(from);program.declarations.set(to,declaration);
      archive=encodeTypedProgramLinks(program);
    }
    return {...specification, links:archive.network.toLino(), root:archive.root.asU64()};
  });
  const replayArchives=[];
  let current=createTypedProgramLinks(specifications[0].source,'Rust',specifications[0].capture);
  for(let cycle=0;cycle<3;cycle+=1){
    const source=emitTypedProgramLinks(current.network,current.root,'JavaScript',{javaScriptScopedBindingEmissionModel:JAVA_SCRIPT_SCOPED_BINDING_MODEL}).code;
    current=createTypedProgramLinks(source,'JavaScript',{javaScriptScalarLoopModel:loopModel});
    replayArchives.push({cycle,source,links:current.network.toLino(),root:current.root.asU64()});
  }
  return {schemaVersion:1, model:JAVA_SCRIPT_SCOPED_BINDING_MODEL, loopModel, scope:'Five concrete complete modules and repeated source-free target replay, not a universal runtime-module claim.', cases,replayArchives};
}
if(process.argv[1]===fileURLToPath(import.meta.url)) {
  const file=new URL('../../parity/fixtures/javascript-scoped-binding-emission.lino',import.meta.url);
  const fixture=javaScriptScopedBindingEmissionFixture();
  if(process.argv.includes('--check')) {if(!isDeepStrictEqual(readProjectData(file),fixture))throw new Error('JavaScript scoped binding fixtures are stale');}
  else writeProjectData(file,fixture);
  console.log('JavaScript scoped binding emission: five actual complete-module fixtures');
}
