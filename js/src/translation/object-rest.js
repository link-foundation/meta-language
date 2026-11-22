// Original source operands and selected actual fields into existing record IR.
import {typeError,unsupported} from './diagnostics.js';
import {FLOAT,array} from './types.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
import {readJavaScriptObjectRestOperation,readJavaScriptObjectRestOwnerForm,selectJavaScriptObjectRestFields,acceptJavaScriptNominalObjectRest} from './object-rest-rules.js';
export function constructJavaScriptObjectRestSource(object,excluded,span){
 return {k:'math',op:readJavaScriptObjectRestOperation(),name:'',args:[{value:object,spread:false},{value:{k:'array',items:excluded.map(token=>({value:{k:'str',value:token.value,span:{start:token.start,end:token.end}},spread:false})),span},spread:false}],span};
}
export function readJavaScriptObjectRestOperands(node,language='JavaScript'){
 const args=node.args;
 if(language!=='JavaScript'||args.length!==2||args.some(argument=>argument.spread)||args[1].value.k!=='array'||args[1].value.items.some(item=>item.spread||item.value.k!=='str'))throw typeError('object rest needs its actual receiver and exact original excluded property names',node.span);
 return {object:args[0].value,excluded:args[1].value.items.map(item=>item.value.value)};
}
export function selectJavaScriptObjectRestActualFields(fields,excluded){
 const units=name=>Array.from({length:name.length},(_,index)=>name.charCodeAt(index));
 return selectJavaScriptObjectRestFields(fields.map(field=>units(field.name)),excluded.map(units)).map(position=>fields[position]);
}
export function acceptJavaScriptNominalObjectRestConstructors(constructors,excluded){
 if(excluded.includes('$'))return false;
 return acceptJavaScriptNominalObjectRest(constructors.map(ctor=>excluded.map(name=>{
  const field=ctor.fields.find(field=>field.name===name);
  return field===undefined?-1:field.optional===true?1:0;
 })));
}
function checkJavaScriptNominalObjectRest(checker,node,value,excluded){
 const type=value.type,declaration=checker.items.get(type.name);
 if(declaration?.k!=='data'||!acceptJavaScriptNominalObjectRestConstructors(declaration.ctors,excluded))
  throw unsupported('nominal object rest receiver','the actual tag and every required constructor field must be retained',node.span);
 const removed=new Set(excluded),cases=declaration.ctors.map(ctor=>{
  const binds=ctor.fields.map(field=>removed.has(field.name)?null:checker.freshName('ml_nominal_rest_field'));
  const args=ctor.fields.map((field,index)=>binds[index]===null
   ?{k:'none',absence:'undefined',type:field.type}
   :{k:'var',name:binds[index],type:field.type});
  return {pattern:{k:'ctor',data:type.name,ctor:ctor.name,binds},body:{k:'ctor',data:type.name,ctor:ctor.name,args,type}};
 });
 return {k:'match',scrutinee:value,cases,type,...(node.span?{span:node.span}:{})};
}
export function checkJavaScriptObjectRest(checker,node,env,path){
 const operands=readJavaScriptObjectRestOperands(node,checker.language),value=checker.referenceValue(checker.expr(operands.object,env,path,undefined));
 if(value.type.kind==='data')return checkJavaScriptNominalObjectRest(checker,node,value,operands.excluded);
 if(readJavaScriptObjectRestOwnerForm(value.type.kind,value.type.open===true)!=='copy'||value.type.ordered!==true)throw unsupported('object rest receiver','every actual enumerable own field and its source order must be represented',node.span);
 const fields=selectJavaScriptObjectRestActualFields(value.type.fields,operands.excluded),name=checker.freshName('ml_object_rest_receiver'),object={k:'var',name,type:value.type},type={kind:'record',fields,ordered:true};
 const entries=fields.map(field=>({name:field.name,value:{k:'recordField',object,field:field.name,index:value.type.fields.indexOf(field),type:field.type}}));
 const operation=value.type.fields.some(field=>field.optional)?'copyJavaScriptPresentRecordPropertyOrder':'copyJavaScriptRecordPropertyOrder';
 const order={k:'math',op:operation,args:[constructRecordPropertyNameRows(fields),constructRecordPropertyNameRows(value.type.fields),{k:'method',op:'readJavaScriptRecordPropertyOrder',object,args:[],type:array(FLOAT)}],type:array(FLOAT)};
 return {k:'let',name,value,body:{k:'record',entries,order,type},type,...(node.span?{span:node.span}:{})};
}
