// Deliberate failures for the diagnostic transport control, never evidence for
// translation behavior. Every callback/target remains independently attempted.
import {produceCollectionControlFixture} from './collection-control-producer.mjs';
import {createControlDiagnostics,finishControlDiagnostics} from '../tests/support/control-diagnostics.js';
const rows=[{name:'first fixture',input:1},{name:'middle fixture',input:2},{name:'last fixture',input:3}];
await produceCollectionControlFixture({model:'deliberately failing diagnostic transport 1',originals:[],refusals:[],additionalGroups:[{key:'injectedControls',rows,read(row){
 const diagnostic=createControlDiagnostics(),measurements={};
 for(const target of ['JavaScript','Rust','Lean','Rocq']){const value=diagnostic.attempt(target,()=>{if(row.input===1&&target==='Lean'||row.input===3&&target==='Rust')throw new Error(row.name+'/'+target+'/intentional failure');return row.input;});if(value!==undefined)measurements[target]=value;}
 return finishControlDiagnostics({...row,measurements,diagnostics:{assertions:diagnostic.assertions,failures:diagnostic.failures}});
}}]});
