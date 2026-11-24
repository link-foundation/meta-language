// Generic shared-plan AST transport. No native/host numeric conversion runs.
import {BOOL,FLOAT} from './types.js';
import {planRustIntegerFloatConversion,readRustIntegerFloatPlanOperation} from './rust-integer-float-plan-rules.js';
export function projectRustIntegerFloatPlan(parser,input,sourceType,range){
 if(sourceType.kind!=='fixed')throw new TypeError('the exact integer floating source plan requires an actual fixed source type');
 const plan=planRustIntegerFloatConversion(sourceType.bits),values=[],names=[];
 if(!plan.length||plan.length%5||plan.length>4096)throw new TypeError('the exact integer floating source plan exceeds its declared domain');
 const name=slot=>{if(!Number.isSafeInteger(slot)||slot<0||slot>128)throw new TypeError('invalid shared floating source binding slot');return names[slot]??=(parser.freshProductBinding());};
 const read=index=>{if(!Number.isSafeInteger(index)||index<0||index>=values.length)throw new TypeError('the shared floating source plan lost an earlier term');return values[index];};
 for(let at=0;at<plan.length;at+=5){
  const operation=readRustIntegerFloatPlanOperation(plan[at]),one=plan[at+1],two=plan[at+2],three=plan[at+3];let value;
  if(operation==='input')value={node:input,type:sourceType};
  else if(['fixed zero','fixed two','floating zero','floating one','floating two'].includes(operation)){
   const type=operation.startsWith('fixed ')?sourceType:FLOAT,magnitude=operation.endsWith('zero')?'0':operation.endsWith('one')?'1':'2';value={node:{k:'num',value:magnitude,type,span:range},type};
  }else if(operation==='fixed reference'||operation==='floating reference')value={node:{k:'name',path:[name(one)],span:range},type:operation==='fixed reference'?sourceType:FLOAT};
  else if(['equal','less','remainder','division','addition','multiplication'].includes(operation)){
   const left=read(one),right=read(two),op={equal:'eq',less:'lt',remainder:'rem',division:'div',addition:'add',multiplication:'mul'}[operation],type=operation==='equal'||operation==='less'?BOOL:operation==='addition'||operation==='multiplication'?FLOAT:sourceType;
   value={node:{k:'binary',op,left:left.node,right:right.node,span:range},type};
  }else if(operation==='negation')value={node:{k:'unary',op:'neg',arg:read(one).node,span:range},type:FLOAT};
  else if(operation==='conditional'){const cond=read(one),then=read(two),otherwise=read(three);value={node:{k:'if',cond:cond.node,then:then.node,else:otherwise.node,span:range},type:then.type};}
  else if(operation==='binding'){const item=read(two),body=read(three);value={node:{k:'let',name:name(one),type:item.type,value:item.node,body:body.node,span:range},type:body.type};}
  else throw new TypeError('the shared floating source plan has an unknown source term');
  values.push(value);
 }
 const result=values.at(-1);if(result.type.kind!=='float')throw new TypeError('the shared floating source plan lost its actual binary64 result');return result.node;
}
