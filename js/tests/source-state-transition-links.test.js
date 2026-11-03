// Deferred additive vocabulary/structural controls, not source-origin evidence.
import test from 'node:test';import assert from 'node:assert/strict';
import {extendSourcePositionSchema} from '../scripts/source-position-schema-extension.mjs';
import {projectLinkedProgramExpression} from '../src/typed-program-expression-projection.js';
import {checkRustSourceStateTransition} from '../src/translation/rust-source-state-transitions.js';
import {BOOL} from '../src/translation/types.js';
const facet={model:'Rust source state transition 1',operation:'initialize',binder:{name:'flag',identity:'declared structural fixture binder',start:0,end:4},position:{type:BOOL,wrappers:[],lifetimes:[],owner:{kind:'primitive',path:['bool'],arguments:[]}}};
test('state transitions add optional expression fields while preserving all predecessor vocabulary',()=>{
 const before={fields:{sourcePosition:'source expression position',name:'name'},expressionFields:{lit:['type','value'],let:['name','value','body']}},after=extendSourcePositionSchema(before);
 assert.deepEqual(before,{fields:{sourcePosition:'source expression position',name:'name'},expressionFields:{lit:['type','value'],let:['name','value','body']}});
 assert.equal(after.fields.sourceStateTransition,'source state transition');assert.equal(after.fields.sourceLoopFold,'source loop fold');assert.equal(after.fields.sourceStateTransform,'source state transformation');assert.equal(after.fields.sourceFiniteRecordLoopHelper,'source finite record loop helper');for(const [kind,fields]of Object.entries(before.expressionFields))assert.deepEqual(after.expressionFields[kind],[...fields,'sourcePosition','sourceStateTransition','sourceLoopFold']);assert.deepEqual(extendSourcePositionSchema(after),after);
 for(const fields of [{sourceStateTransition:'another meaning'},{another:'source state transition'}])assert.throws(()=>extendSourcePositionSchema({...before,fields}));
});
test('checked projection preserves source-root facts without adding serialized proof revisions',()=>{
 const normalized=checkRustSourceStateTransition(facet,BOOL,'Rust'),projected=projectLinkedProgramExpression({k:'lit',value:true,type:BOOL,sourceStateTransition:normalized});
 assert.deepEqual(projected.sourceStateTransition,normalized);assert.notEqual(projected.sourceStateTransition,normalized);assert.equal(projected.sourceStateTransition.binder.identity,facet.binder.identity);
 assert.throws(()=>checkRustSourceStateTransition({...facet,revision:1},BOOL,'Rust'));assert.throws(()=>checkRustSourceStateTransition({...facet,operation:'inferred write'},BOOL,'Rust'));assert.throws(()=>checkRustSourceStateTransition(facet,BOOL,'JavaScript'));
});
