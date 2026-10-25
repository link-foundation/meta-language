// Original source operands and selected actual fields into existing record IR.
import {typeError,unsupported} from './diagnostics.js';
import {FLOAT,array} from './types.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
import {readJavaScriptObjectRestOwnerForm,selectJavaScriptObjectRestFields} from './object-rest-rules.js';
export function readJavaScriptObjectRestOperands(node,language='JavaScript'){
 const args=node.args;
 if(language!=='JavaScript'||args.length!==2||args.some(argument=>argument.spread)||args[1].value.k!=='array'||args[1].value.items.some(item=>item.spread||item.value.k!=='str'))throw typeError('object rest needs its actual receiver and exact original excluded property names',node.span);
 return {object:args[0].value,excluded:args[1].value.items.map(item=>item.value.value)};
}
export function selectJavaScriptObjectRestActualFields(fields,excluded){
 const units=name=>Array.from({length:name.length},(_,index)=>name.charCodeAt(index));
 return selectJavaScriptObjectRestFields(fields.map(field=>units(field.name)),excluded.map(units)).map(position=>fields[position]);
}
export function checkJavaScriptObjectRest(checker,node,env,path){
 const operands=readJavaScriptObjectRestOperands(node,checker.language),value=checker.referenceValue(checker.expr(operands.object,env,path,undefined));
 if(readJavaScriptObjectRestOwnerForm(value.type.kind,value.type.open===true)!=='copy'||value.type.ordered!==true)throw unsupported('object rest receiver','every actual enumerable own field and its source order must be represented',node.span);
 const fields=selectJavaScriptObjectRestActualFields(value.type.fields,operands.excluded),name=checker.freshName('ml_object_rest_receiver'),object={k:'var',name,type:value.type},type={kind:'record',fields,ordered:true};
 const entries=fields.map(field=>({name:field.name,value:{k:'recordField',object,field:field.name,index:value.type.fields.indexOf(field),type:field.type}}));
 const order={k:'math',op:'copyJavaScriptRecordPropertyOrder',args:[constructRecordPropertyNameRows(fields),constructRecordPropertyNameRows(value.type.fields),{k:'method',op:'readJavaScriptRecordPropertyOrder',object,args:[],type:array(FLOAT)}],type:array(FLOAT)};
 return {k:'let',name,value,body:{k:'record',entries,order,type},type,...(node.span?{span:node.span}:{})};
}
