// Actual declaration/type owners into existing exhaustive match/option IR.
import {readJavaScriptDataFieldPlan} from './javascript-data-field-rules.js';
import {readJavaScriptRecordFormalDefaultFieldForm} from './javascript-record-parameter-rules.js';
import {UNIT,STRING,option,typeKey} from './types.js';
import {javaScriptConstructorFieldValueType} from './javascript-constructor-presence.js';
import {unsupported} from './diagnostics.js';
import {coerce} from './check.js';
export function checkJavaScriptDataField(checker,object,field,span,preservePresence=false,{guard,own=false}={}){
  if(checker.language!=='JavaScript'||object.type.kind!=='data')return null;
  const entry=checker.items.get(object.type.name);
  if(entry?.k!=='data')throw unsupported('datatype property','the actual nominal declaration is unavailable',span);
  const variants=field!=='$'&&guard?.owner===entry.fullName&&guard.names.length>0?
    entry.ctors.filter(ctor=>guard.names.includes(ctor.name)):entry.ctors;
  const positions=variants.map(ctor=>ctor.fields.findIndex(candidate=>candidate.name===field));
  const types=variants.map((ctor,index)=>positions[index]<0?'':typeKey(javaScriptConstructorFieldValueType(ctor.fields[positions[index]])));
  const inherited=!own&&readJavaScriptRecordFormalDefaultFieldForm('record',false,false,field)==='prototype';
  if(!preservePresence&&inherited&&variants.some((ctor,index)=>ctor.fields[positions[index]]?.optional)){
    throw unsupported('datatype property','an absent own field can expose an inherited property',span);
  }
  const plan=readJavaScriptDataFieldPlan(checker.language,field,inherited,positions,types);
  if(plan.error)throw unsupported('datatype property',plan.error,span);
  if(plan.mode==='absent'){
    const type=option(UNIT,'undefined');
    return{k:'let',name:checker.freshName('ml_absent_data_property_receiver'),value:object,body:{k:'none',absence:'undefined',type},type};
  }
  const first=positions.findIndex(position=>position>=0);
  const declaration=first<0?null:variants[first].fields[positions[first]];
  const selected=!declaration?UNIT:preservePresence?declaration.type:javaScriptConstructorFieldValueType(declaration);
  const type=plan.mode==='tag'?STRING:plan.optional?option(selected,'undefined'):selected;
  const restricted=variants.length!==entry.ctors.length;
  const product=restricted?{kind:'tuple',elements:[type]}:undefined;
  const matchedType=product?option(product,'undefined'):type;
  const fieldPositions=new Map(variants.map((ctor,index)=>[ctor.name,positions[index]]));
  const cases=entry.ctors.map(ctor=>{
    const position=fieldPositions.get(ctor.name);
    if(position===undefined)return {pattern:{k:'ctor',data:entry.fullName,ctor:ctor.name,binds:ctor.fields.map(()=>null)},body:{k:'none',absence:'undefined',type:matchedType}};
    const name=position>=0&&plan.mode!=='tag'?checker.freshName('ml_data_property_value'):null;
    const pattern={k:'ctor',data:entry.fullName,ctor:ctor.name,binds:ctor.fields.map((_,at)=>at===position?name:null)};
    const declaration=ctor.fields[position];
    let value=plan.mode==='tag'?{k:'lit',value:ctor.name,type:STRING}:position<0?{k:'none',absence:'undefined',type}:{k:'var',name,type:declaration.type};
    if(!preservePresence&&position>=0&&plan.mode!=='tag'&&declaration.optional){
      const product=declaration.type.element,element=javaScriptConstructorFieldValueType(declaration);
      const fallback={k:'tuple',items:[{k:'none',absence:'undefined',type:element}],type:product};
      value={k:'tupleIndex',index:0,object:{k:'coalesce',left:value,right:fallback,type:product},type:element};
    }
    const body=coerce(value,type,checker.language,span);
    return{pattern,body:product?{k:'some',value:{k:'tuple',items:[body],type:product},type:matchedType}:body};
  });
  const value={k:'match',scrutinee:object,cases,type:matchedType};
  return product?{k:'tupleIndex',index:0,object:{k:'unwrapOption',arg:value,type:product},type}:value;
}
