// Actual argument/type adapters; shared source rules select text and windows.
import {BOOL,FLOAT,STRING,UNIT,option} from './types.js';
import {unsupported,typeError} from './diagnostics.js';
import {acceptJavaScriptNumberConversionType,materializeJavaScriptNumberConversion} from './javascript-number-conversion.js';
import {readJavaScriptPrimitiveTextForm,readJavaScriptStringSearchDefaultPosition,planJavaScriptStringSearchWindow,readJavaScriptStringSearchResultKind} from './javascript-property-call-rules.js';
const number=value=>({k:'lit',value,type:FLOAT});
const variable=(name,type)=>({k:'var',name,type});
const undefinedValue=()=>({k:'none',absence:'undefined',type:option(UNIT,'undefined')});
function accepts(type){const form=readJavaScriptPrimitiveTextForm(type.kind);return form==='optional'?accepts(type.element):form!=='refuse';}
export function materializeJavaScriptPrimitiveText(value,span){
 if(!accepts(value.type))throw unsupported('primitive text conversion','the actual object or symbol needs its complete primitive conversion',span);
 const form=readJavaScriptPrimitiveTextForm(value.type.kind);
 if(form==='identity')return value;
 if(form==='undefined')return{k:'lit',value:'undefined',type:STRING};
 return{k:form==='optional'?'optionalText':'toString',arg:value,console:false,type:STRING};
}
export function checkJavaScriptPrimitiveTextBuiltin(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.some(argument=>argument.spread))throw unsupported('String arguments','the complete actual iterator effects must be represented',node.span);
 const values=node.args.map(argument=>checker.expr(argument.value,env,path,undefined));
 const names=values.map(()=>checker.freshName('ml_string_argument'));
 let body=values.length?materializeJavaScriptPrimitiveText(variable(names[0],values[0].type),node.span):{k:'lit',value:'',type:STRING};
 for(let at=values.length-1;at>=0;at-=1)body={k:'let',name:names[at],value:values[at],body,type:STRING};
 return body;
}
export function checkJavaScriptStringSearch(checker,node,receiver,env,path){
 const values=[receiver,...node.args.map(argument=>{if(!argument)throw typeError('a string search must retain every actual argument',node.span);return checker.expr(argument,env,path,undefined);})];
 const names=values.map(()=>checker.freshName('ml_string_search_operand'));
 const bound=values.map((value,index)=>variable(names[index],value.type));
 const search=materializeJavaScriptPrimitiveText(bound[1]??undefinedValue(),node.span),searchName=checker.freshName('ml_string_search_text');
 const defaultValue=number(readJavaScriptStringSearchDefaultPosition(node.op));
 let position=defaultValue;
 if(bound[2]){
  if(!acceptJavaScriptNumberConversionType(bound[2].type))throw unsupported('string search position','the actual object or symbol needs its complete numeric conversion',node.span);
  position=bound[2].type.kind==='unit'?defaultValue:materializeJavaScriptNumberConversion(checker,bound[2],'coerceJavaScriptNumber');
  if(bound[2].type.kind==='option'&&bound[2].type.absence!=='null')position={k:'if',cond:{k:'strictEqual',left:bound[2],right:{k:'none',absence:'undefined',type:bound[2].type},negate:false,type:BOOL},then:defaultValue,else:position,type:FLOAT};
 }
 const positionName=checker.freshName('ml_string_search_position'),at=variable(positionName,FLOAT);
 const nonnegative={k:'math',op:'max',args:[at,number('0')],type:FLOAT};
 const args=planJavaScriptStringSearchWindow(node.op).map(value=>value==='position'?nonnegative:number(value));
 const sliced={k:'method',op:'slice',object:bound[0],args,type:STRING};
 const type=readJavaScriptStringSearchResultKind(node.op)==='float'?FLOAT:BOOL;
 let body=type===FLOAT?{k:'method',op:node.op,object:bound[0],args:[variable(searchName,STRING),at],type}:{k:'stringTest',op:node.op,string:sliced,search:variable(searchName,STRING),type};
 body={k:'let',name:positionName,value:position,body,type};
 body={k:'let',name:searchName,value:search,body,type};
 for(let index=values.length-1;index>=0;index-=1)body={k:'let',name:names[index],value:values[index],body,type};
 return body;
}
export function checkJavaScriptStringMapCall(checker,node,receiver,env,path){
 const values=[receiver,...node.args.map(argument=>{if(!argument)throw typeError('a string map must retain every actual argument',node.span);return checker.expr(argument,env,path,undefined);})];
 const names=values.map(()=>checker.freshName('ml_string_map_operand'));
 let body={k:'stringMap',op:node.op,string:variable(names[0],receiver.type),type:STRING};
 for(let at=values.length-1;at>=0;at-=1)body={k:'let',name:names[at],value:values[at],body,type:STRING};
 return body;
}
