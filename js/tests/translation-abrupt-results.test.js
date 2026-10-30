// Deferred actual source errors, constructor effects, graph roles and codecs.
import assert from 'node:assert/strict';
import test from 'node:test';
import {coerceAbruptResult} from '../src/translation/abrupt-results.js';
import {UNIT,FLOAT,BOOL} from '../src/translation/types.js';
test('ordinary returning results and possibly failing calls cannot become nonreturning',()=>{
 const unit={k:'lit',value:null,type:UNIT},failure={k:'math',op:'raiseJavaScriptTypeError',args:[],type:UNIT};
 for(const value of [unit,{k:'app',name:'unknown',args:[],type:UNIT},{k:'math',op:'unknownOperation',args:[],type:UNIT},{k:'if',cond:{k:'lit',value:true,type:BOOL},then:failure,else:unit,type:UNIT},{k:'let',name:'value',value:failure,body:unit,type:UNIT}])assert.equal(coerceAbruptResult(value,FLOAT),null);
});
test('retagging preserves original operands and every terminal wrapper effect',()=>{
 const message={k:'lit',value:'bad',type:{kind:'string'}},failure={k:'math',op:'raiseJavaScriptTypeError',args:[message,{k:'lit',value:true,type:BOOL}],type:UNIT};
 const operand={k:'lit',value:3,type:FLOAT},value={k:'let',name:'actual',value:operand,body:{k:'print',text:message,body:failure,type:UNIT},type:UNIT},actual=coerceAbruptResult(value,FLOAT);
 assert.equal(actual.value,operand);assert.equal(actual.body.text,message);assert.equal(actual.body.body.args,failure.args);assert.equal(actual.type,FLOAT);assert.equal(actual.body.type,FLOAT);assert.equal(actual.body.body.type,FLOAT);assert.equal(value.type,UNIT);
});
