// Actual declaration/type owners into existing exhaustive match/option IR.
import {readJavaScriptDataFieldPlan} from './javascript-data-field-rules.js';
import {readJavaScriptRecordFormalDefaultFieldForm} from './javascript-record-parameter-rules.js';
import {UNIT,STRING,option,typeKey} from './types.js';
import {unsupported} from './diagnostics.js';
import {coerce} from './check.js';
export function checkJavaScriptDataField(checker,object,field,span){
  if(checker.language!=='JavaScript'||object.type.kind!=='data')return null;
  const entry=checker.items.get(object.type.name);
  if(entry?.k!=='data')throw unsupported('datatype property','the actual nominal declaration is unavailable',span);
  const positions=entry.ctors.map(ctor=>ctor.fields.findIndex(candidate=>candidate.name===field));
  const types=entry.ctors.map((ctor,index)=>positions[index]<0?'':typeKey(ctor.fields[positions[index]].type));
  const inherited=readJavaScriptRecordFormalDefaultFieldForm('record',false,false,field)==='prototype';
  const plan=readJavaScriptDataFieldPlan(checker.language,field,inherited,positions,types);
  if(plan.error)throw unsupported('datatype property',plan.error,span);
  if(plan.mode==='absent'){
    const type=option(UNIT,'undefined');
    return{k:'let',name:checker.freshName('ml_absent_data_property_receiver'),value:object,body:{k:'none',absence:'undefined',type},type};
  }
  const first=positions.findIndex(position=>position>=0);
  const selected=first<0?UNIT:entry.ctors[first].fields[positions[first]].type;
  const type=plan.mode==='tag'?STRING:plan.optional?option(selected,'undefined'):selected;
  const cases=entry.ctors.map((ctor,index)=>{
    const position=positions[index];
    const name=position>=0&&plan.mode!=='tag'?checker.freshName('ml_data_property_value'):null;
    const pattern={k:'ctor',data:entry.fullName,ctor:ctor.name,binds:ctor.fields.map((_,at)=>at===position?name:null)};
    const value=plan.mode==='tag'?{k:'lit',value:ctor.name,type:STRING}:position<0?{k:'none',absence:'undefined',type}:{k:'var',name,type:ctor.fields[position].type};
    return{pattern,body:coerce(value,type,checker.language,span)};
  });
  return{k:'match',scrutinee:object,cases,type};
}
