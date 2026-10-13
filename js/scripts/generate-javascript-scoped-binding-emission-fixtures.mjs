import {isDeepStrictEqual} from 'node:util';
import {fileURLToPath} from 'node:url';
import {readProjectData, writeProjectData} from './project-data.mjs';
import {createTypedProgramLinks} from '../src/typed-program-links.js';

export const JAVA_SCRIPT_SCOPED_BINDING_MODEL = 'owned JavaScript scoped bindings 1';
const loopModel = 'closed declaration scalar loop 1';
export function javaScriptScopedBindingEmissionFixture() {
  const specifications = [
    {id:'declaration-loop', language:'Rust', source:'fn run(mut run:f64,limit:f64)->f64{loop{return if run<limit{(run,)=(run+1f64,);continue;}else{run};}}fn main(){}', entry:'run', capture:{rustScalarLoopModel:loopModel}, inputs:[[0,3],[4,3],['-0',0]], expected:[3,4,'-0'], output:[]},
    {id:'print-discard-order', language:'JavaScript', source:'function marker(x){console.log("called");return x;} console.log("before");marker(2);console.log("after");', entry:'marker', capture:{}, inputs:[[2],[4]], expected:[2,4], output:['before','called','after']},
    {id:'declaration-alias-collision', language:'JavaScript', source:'function binding_value(x){return x+10;} function choose(value){const shadow=value+1;return binding_value(shadow)+value;}choose(2);', entry:'choose', capture:{}, inputs:[[2],[3]], expected:[15,17], output:[]},
    {id:'lexical-shadow', language:'JavaScript', source:'function choose(value){const previous=value;{const value=previous+1;return value+previous;}}choose(2);', entry:'choose', capture:{}, inputs:[[2],[0]], expected:[5,1], output:[]},
  ];
  return {schemaVersion:1, model:JAVA_SCRIPT_SCOPED_BINDING_MODEL, loopModel, scope:'Four concrete complete modules and repeated source-free target replay, not a universal runtime-module claim.', cases:specifications.map(specification => {
    const archive=createTypedProgramLinks(specification.source,specification.language,specification.capture);
    return {...specification, links:archive.network.toLino(), root:archive.root.asU64()};
  })};
}
if(process.argv[1]===fileURLToPath(import.meta.url)) {
  const file=new URL('../../parity/fixtures/javascript-scoped-binding-emission.lino',import.meta.url);
  const fixture=javaScriptScopedBindingEmissionFixture();
  if(process.argv.includes('--check')) {if(!isDeepStrictEqual(readProjectData(file),fixture))throw new Error('JavaScript scoped binding fixtures are stale');}
  else writeProjectData(file,fixture);
  console.log('JavaScript scoped binding emission: four actual complete-module fixtures');
}
