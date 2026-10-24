// Actual checked type/argument adapters. Source operations live in shared data.
import {STRING,FLOAT,BOOL,array,option} from './types.js';
import {unsupported,typeError} from './diagnostics.js';
import {materializeJavaScriptNumberConversion,acceptJavaScriptNumberConversionType} from './javascript-number-conversion.js';
import {readJavaScriptStringConstructionForm} from './string-construction-rules.js';

const number=value=>({k:'lit',value:String(value),type:FLOAT});
const variable=(name,type)=>({k:'var',name,type});
export function checkJavaScriptStringConstruction(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.some(item=>item.spread))throw unsupported('spread string construction arguments','the complete source iterator effects require their represented operations',node.span);
 const values=node.args.map(item=>checker.expr(item.value,env,path,undefined));
 if(values.some(value=>!acceptJavaScriptNumberConversionType(value.type)))throw unsupported('string construction primitive conversion','the actual object or symbol primitive conversion needs its represented operation',node.span);
 const names=values.map(()=>checker.freshName('ml_character_argument'));
 const converted=values.map((value,index)=>materializeJavaScriptNumberConversion(checker,variable(names[index],value.type),'coerceJavaScriptNumber'));
 const column={k:'array',items:converted,type:array(FLOAT)};
 let body;
 if(node.op==='constructJavaScriptStringFromCharacterCodes')body={k:'math',op:'convertJavaScriptCharacterCodesToString',args:[column],type:STRING};
 else{
  body={k:'lit',value:'',type:STRING};
  // Argument expressions have all run, while each primitive conversion and
  // range check precedes the next conversion, exactly as the source loop.
  for(let index=values.length-1;index>=0;index-=1){
   const inputName=checker.freshName('ml_character_number'),input=variable(inputName,FLOAT);
   const resultName=checker.freshName('ml_character_units'),result=variable(resultName,array(FLOAT));
   const at={k:'coalesce',left:{k:'optionalIndex',array:result,index:number(0),type:option(FLOAT,'undefined')},right:number(0),type:FLOAT};
   const message={k:'binary',op:'concat',left:{k:'lit',value:'Invalid code point ',type:STRING},right:{k:'toString',arg:input,type:STRING},type:STRING};
   const failure={k:'math',op:'raiseJavaScriptRangeError',args:[message,{k:'lit',value:true,type:BOOL}],type:STRING};
   const suffix={k:'method',op:'slice',object:result,args:[number(1),number('Infinity')],type:array(FLOAT)};
   const character={k:'math',op:'convertJavaScriptCharacterCodesToString',args:[suffix],type:STRING};
   const success={k:'binary',op:'concat',left:character,right:body,type:STRING};
   const convertedPoint={k:'math',op:'convertJavaScriptCodePointUnitsResult',args:[{k:'array',items:[input],type:array(FLOAT)}],type:array(FLOAT)};
   body={k:'let',name:inputName,value:converted[index],body:{k:'let',name:resultName,value:convertedPoint,
    body:{k:'if',cond:{k:'binary',op:'eq',left:at,right:number(-1),type:BOOL,domain:FLOAT},then:success,else:failure,type:STRING},type:STRING},type:STRING};
  }
 }

 for(let at=values.length-1;at>=0;at-=1)body={k:'let',name:names[at],value:values[at],body,type:STRING};
 return body;
}
export function checkJavaScriptStringConstructionValue(checker,node,env,path){
 if(checker.language!=='JavaScript'||node.args.length!==1||node.args[0].spread)throw typeError('a JavaScript string construction intrinsic needs one Number column',node.span);
 const value=checker.expr(node.args[0].value,env,path,array(FLOAT));
 if(value.type.kind!=='array'||value.type.element.kind!=='float')throw typeError('a JavaScript string construction intrinsic needs one Number column',node.span);
 return{k:'math',op:node.op,args:[value],type:readJavaScriptStringConstructionForm(node.op)==='characters'?STRING:array(FLOAT)};
}
