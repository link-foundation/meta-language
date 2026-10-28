// Transport actual checked core occurrences to the shared source compiler.
// Runtime operations remain the automatically translated call machine's code.
import {validateLinkedTypedProgram} from '../typed-program-links-validation.js';
import {recheckLinkedTypedProgram} from '../typed-program-checked-projection.js';
import {compileJavaScriptCallSource} from './javascript-call-source.js';
import {unsupported} from './diagnostics.js';
import {tailLoop} from './ir.js';
import {readRecordFieldWriteOperator,readRecordFieldWriteName} from './record-write-rules.js';

function refuse(node,message){throw unsupported('registered call source lowering',message,node?.span);}
function acceptType(type,depth=0){
 if(!type||depth>128)return false;
 if(['float','bool','string','int','unit'].includes(type.kind))return true;
 if(type.kind==='record')return type.fields.every(field=>acceptType(field.type,depth+1));
 if(type.kind==='option')return ['undefined','null','both'].includes(type.absence)&&acceptType(type.element,depth+1);
 return type.kind==='function'&&type.params.every(param=>acceptType(param.type??param,depth+1))&&acceptType(type.ret,depth+1);
}

/** The registry supplies existing global function/realm identities. No object,
 * Symbol, realm or callable is allocated by lexical source lowering. The heap
 * executor subsequently checks those actual identities against its context.
 * This initial profile lowers complete pure primitive expression functions;
 * it refuses a whole program if any function needs an unrepresented operation.
 */
export function lowerCheckedJavaScriptCallModule(program,{functions,realms,exoticKey}){
 validateLinkedTypedProgram(program);
 recheckLinkedTypedProgram(program,{wholeProgram:true});
 if(program.sourceLanguage!=='JavaScript')refuse(null,'this profile retains actual JavaScript source semantics');
 if(program.main?.effects.length||program.externals?.length||program.imports?.length||program.sourceRuntimeShell)refuse(null,'source modules require explicit main, import, external and runtime-shell lowering');
 if(!Array.isArray(functions)||functions.length>4096||!Array.isArray(realms)||realms.length>4096)refuse(null,'complete bounded function and realm directories are required');
 let previous=0n;for(const realm of realms){if(typeof realm!=='bigint'||realm<=previous||realm>18446744073709551615n)refuse(null,'realm identities must be actual ordered unique unsigned64 links');previous=realm;}
 if(exoticKey?.$!=='symbol'||typeof exoticKey.identity!=='bigint'||exoticKey.identity<=0n||exoticKey.identity>18446744073709551615n)refuse(null,'the actual primitive conversion Symbol link is required');
 const directory=new Map(),identities=new Set();
 for(const row of functions){
  if(typeof row.name!=='string'||directory.has(row.name)||typeof row.identity!=='bigint'||row.identity<=0n||row.identity>18446744073709551615n||identities.has(row.identity)||!realms.includes(row.realm))refuse(null,'function names, global identities and source realms require complete unique actual owners');
  const entry=program.declarations.get(row.name);if(entry?.k!=='fn')refuse(null,'a callable registry entry has no actual checked function declaration');
  directory.set(row.name,row);identities.add(row.identity);
 }
 const constant=value=>({$:'sourceConstant',value}),undefinedSource=()=>constant({$:'undefined'});
 const compiled=[],sourceUnits=[];let instructionCount=0;
 for(const [name,entry]of program.declarations){
  if(entry.k!=='fn')refuse(entry,'semantic data declarations need explicit ordinary-object allocation lowering');
  const owner=directory.get(name);if(!owner)refuse(entry,'every actual function requires its supplied global callable identity');
  if(!entry.params.every(param=>acceptType(param.type)&&!param.default&&!param.guard)||!acceptType(entry.ret))refuse(entry,'function signatures need their complete represented primitive/callable value profile');
  let nextBinding=0,visits=0;const bindings=new Map(),parameters=[];
  for(const param of entry.params){const identity=nextBinding++;if(bindings.has(param.name))refuse(entry,'actual parameter owners must be distinct');bindings.set(param.name,identity);parameters.push(identity);}
  function expression(node,environment,depth=0){
   if(!node||depth>128||++visits>65536||!acceptType(node.type))refuse(node,'source expressions exceed the represented value/structural profile');
   switch(node.k){
    case 'lit':{
     const type=node.type.kind,value=node.value;
     if(type==='float')return constant({$:'number',value:Number(value)});
     if(type==='int')return constant({$:'integer',value:BigInt(value)});
     if(type==='bool')return constant({$:'boolean',value});
     if(type==='string')return constant({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
     return refuse(node,'this literal needs its actual semantic value mapping');
    }
    case 'unit':return undefinedSource();
    case 'none':if(node.absence!=='undefined'&&node.absence!=='null')return refuse(node,'absent literals must retain their exact source absence');return constant({$:node.absence});
    case 'some':return expression(node.value,environment,depth+1);
    case 'var':{const identity=environment.get(node.name);if(identity===undefined)return refuse(node,'a variable use has no actual lexical owner');return {$:'sourceBinding',identity};}
    case 'let':{
     const value=expression(node.value,environment,depth+1),identity=nextBinding++,nested=new Map(environment);nested.set(node.name,identity);
     return {$:'sourceLet',identity,value,body:expression(node.body,nested,depth+1)};
    }
    case 'if':return {$:'sourceIf',condition:expression(node.cond,environment,depth+1),yes:expression(node.then,environment,depth+1),no:expression(node.else,environment,depth+1)};
    case 'binary':{
     if(['and','or'].includes(node.op)&&node.left.type.kind==='bool'&&node.right.type.kind==='bool')return {$:'sourceIf',condition:expression(node.left,environment,depth+1),yes:node.op==='and'?expression(node.right,environment,depth+1):constant({$:'boolean',value:true}),no:node.op==='or'?expression(node.right,environment,depth+1):constant({$:'boolean',value:false})};
     if(node.op==='eq'||node.op==='ne'){
      const value={$:'sourceStrictEqual',left:expression(node.left,environment,depth+1),right:expression(node.right,environment,depth+1)};
      return node.op==='ne'?{$:'sourceIf',condition:value,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})}:value;
     }
     const operation={add:'add',sub:'subtract',mul:'multiply',div:'divide',rem:'remainder',lt:'lessThan',eq:'equal'}[node.op];
     if(!operation||node.left.type.kind!=='float'||node.right.type.kind!=='float')return refuse(node,'this operator requires additional exact source lowering');
     return {$:'sourceNumberBinary',operator:operation,left:expression(node.left,environment,depth+1),right:expression(node.right,environment,depth+1)};
    }
    case 'recordField':return {$:'sourceGet',object:expression(node.object,environment,depth+1),key:constant({$:'text',units:Array.from({length:node.field.length},(_,index)=>node.field.charCodeAt(index))})};
    case 'strictEqual':{
     const value={$:'sourceStrictEqual',left:expression(node.left,environment,depth+1),right:expression(node.right,environment,depth+1)};
     return node.negate?{$:'sourceIf',condition:value,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})}:value;
    }
    case 'truthy':return {$:'sourceIf',condition:expression(node.arg,environment,depth+1),yes:constant({$:'boolean',value:true}),no:constant({$:'boolean',value:false})};
    case 'unary':{
     const value=expression(node.arg,environment,depth+1);
     if(node.op==='not'&&node.arg.type.kind==='bool')return {$:'sourceIf',condition:value,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})};
     if(node.op==='neg'&&node.arg.type.kind==='float')return {$:'sourceNumberBinary',operator:'multiply',left:constant({$:'number',value:-1}),right:value};
     return refuse(node,'this unary operand requires additional exact source lowering');
    }
    case 'functionRef':{const actual=directory.get(node.fn);if(!actual)return refuse(node,'a function reference requires its actual supplied callable');return constant({$:'object',identity:actual.identity,callable:true});}
    case 'call':{
     const actual=directory.get(node.fn);if(!actual)return refuse(node,'a source call requires its actual supplied callable');
     return {$:'sourceCall',callee:constant({$:'object',identity:actual.identity,callable:true}),receiver:undefinedSource(),arguments:node.args.map(arg=>expression(arg,environment,depth+1))};
    }
    case 'method':{
     const operation=readRecordFieldWriteOperator(node.op),field=readRecordFieldWriteName(node.op);
     if(!operation||node.object.type.kind!=='record'||node.object.type.reference!==true)return refuse(node,'this method requires its actual source operation');
     const identity=nextBinding++,saved={$:'sourceBinding',identity},key=constant({$:'text',units:Array.from({length:field.length},(_,index)=>field.charCodeAt(index))});
     let value;
     if(operation==='=')value=expression(node.args[0],environment,depth+1);
     else{
      const operator={'+=':'add','-=':'subtract','*=':'multiply','/=':'divide','%=':'remainder','**=':'power','++':'add','--':'subtract'}[operation];
      if(!operator)return refuse(node,'the exact compound write operator is not represented');
      value={$:'sourceNumberBinary',operator,left:{$:'sourceGet',object:saved,key},right:operation==='++'||operation==='--'?constant({$:'number',value:1}):expression(node.args[0],environment,depth+1)};
     }
     const ignored=nextBinding++;
     return {$:'sourceLet',identity,value:expression(node.object,environment,depth+1),body:{$:'sourceLet',identity:ignored,value:{$:'sourceSet',object:saved,key,value},body:undefinedSource()}};
    }
    case 'recur':return {$:'sourceRecur',arguments:node.args.map(arg=>expression(arg,environment,depth+1))};
    case 'apply':{
     if(node.fn.k==='recordField'){
      const receiver=expression(node.fn.object,environment,depth+1),identity=nextBinding++,saved={$:'sourceBinding',identity};
      const key=constant({$:'text',units:Array.from({length:node.fn.field.length},(_,index)=>node.fn.field.charCodeAt(index))});
      return {$:'sourceLet',identity,value:receiver,body:{$:'sourceCall',callee:{$:'sourceGet',object:saved,key},receiver:saved,arguments:node.args.map(arg=>expression(arg,environment,depth+1))}};
     }
     return {$:'sourceCall',callee:expression(node.fn,environment,depth+1),receiver:undefinedSource(),arguments:node.args.map(arg=>expression(arg,environment,depth+1))};
    }
    default:return refuse(node,'the actual expression '+node.k+' needs further source lowering');
   }
  }
  const source=expression(tailLoop(entry,entry.body)??entry.body,bindings),result=compileJavaScriptCallSource(source,parameters);
  if(result.error)refuse(entry,result.error);
  sourceUnits.push({name,source,parameters,compilation:result});
  instructionCount+=result.instructions.length;if(instructionCount>65536)refuse(entry,'the complete program exceeds its instruction budget');
  compiled.push({$:'callFunction',identity:owner.identity,realm:owner.realm,receiverMode:'strict',lexicalReceiver:{$:'undefined'},parameters:parameters.length,registers:result.registers,captures:[],instructions:result.instructions});
 }
 if(directory.size!==compiled.length)refuse(null,'the supplied directory must exactly cover the actual function declarations');
 compiled.sort((left,right)=>left.identity<right.identity?-1:left.identity>right.identity?1:0);
 return {$:'loweredCallModule',program:{$:'callProgram',functions:compiled,realms:[...realms],exoticKey},sourceUnits};
}

export function lowerCheckedJavaScriptCallProgram(program,registry){return lowerCheckedJavaScriptCallModule(program,registry).program;}
