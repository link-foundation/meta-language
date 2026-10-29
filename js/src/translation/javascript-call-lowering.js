// Transport actual checked core occurrences to the shared source compiler.
// Runtime operations remain the automatically translated call machine's code.
import {validateLinkedTypedProgram} from '../typed-program-links-validation.js';
import {recheckLinkedTypedProgram} from '../typed-program-checked-projection.js';
import {compileJavaScriptCallSource} from './javascript-call-source.js';
import {unsupported} from './diagnostics.js';
import {tailLoop} from './ir.js';
import {readRecordFieldWriteOperator,readRecordFieldWriteName} from './record-write-rules.js';
import {readJavaScriptBigIntegerResultOperation} from './big-integer-conversion-rules.js';
import {readBigIntegerBitwiseOperationCode} from './big-integer-bitwise-rules.js';
import {readJavaScriptNumberBitwiseOperationCode} from './number-bitwise-rules.js';
import {readJavaScriptSourceExceptionClass} from './source-exception-rules.js';
import {constructJavaScriptLiteralErrorSource,constructJavaScriptNaturalGuardSource,constructJavaScriptNaturalCastSource,constructJavaScriptNaturalDifferenceSource,constructJavaScriptNaturalPredecessorSource,constructJavaScriptNaturalMatchSource,testJavaScriptNaturalSourceRealm,planJavaScriptNaturalGuardOrder,readJavaScriptNaturalCastPlan,readJavaScriptNaturalPatternBinding,planJavaScriptNaturalPatternBranches} from './javascript-call-natural-source.js';
import {readCheckedJavaScriptIntegerResult} from './javascript-call-integer-lowering.js';

function refuse(node,message){throw unsupported('registered call source lowering',message,node?.span);}
function acceptType(type,depth=0){
 if(!type||depth>128)return false;
 if(['float','bool','string','int','nat','unit'].includes(type.kind))return true;
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
 const constant=value=>({$:'sourceConstant',value}),undefinedSource=()=>constant({$:'undefined'}),primitive=(operation,arguments_)=>({$:'sourcePrimitive',operation,arguments:arguments_});
 const sourceError=constructJavaScriptLiteralErrorSource;
 const oneRealm=testJavaScriptNaturalSourceRealm(functions.map(fn=>String(fn.realm)));
 const compiled=[],sourceUnits=[];let instructionCount=0;
 for(const [name,entry]of program.declarations){
  if(entry.k!=='fn')refuse(entry,'semantic data declarations need explicit ordinary-object allocation lowering');
  const owner=directory.get(name);if(!owner)refuse(entry,'every actual function requires its supplied global callable identity');
  if(!entry.params.every(param=>acceptType(param.type)&&!param.default)||!acceptType(entry.ret))refuse(entry,'function signatures need their complete represented primitive/callable value profile');
  const guarded=entry.params.filter(param=>param.guard);
  if(guarded.some(param=>typeof param.guard.message!=='string'))refuse(entry,'natural guards retain their actual literal Error messages');
  const guardOrder=planJavaScriptNaturalGuardOrder(guarded.map(param=>param.type.kind),guarded.map(param=>param.guard.order??-1),oneRealm);
  if(guardOrder[0]===-1)refuse(entry,'natural guards retain complete source order, domains and one actual source realm');
  const guards=guardOrder.map(index=>guarded[index]);
  let nextBinding=0,visits=0;const bindings=new Map(),parameters=[];
  for(const param of entry.params){const identity=nextBinding++;if(bindings.has(param.name))refuse(entry,'actual parameter owners must be distinct');bindings.set(param.name,identity);parameters.push(identity);}
  function expression(node,environment,depth=0){
   if(!node||depth>128||++visits>65536||!acceptType(node.type))refuse(node,'source expressions exceed the represented value/structural profile');
   const integerResult=readCheckedJavaScriptIntegerResult(node);
   if(integerResult)return primitive(integerResult.operation,integerResult.arguments.map(arg=>expression(arg,environment,depth+1)));
   switch(node.k){
    case 'lit':{
     const type=node.type.kind,value=node.value;
     if(type==='float')return constant({$:'number',value:Number(value)});
     if(type==='int'||type==='nat')return constant({$:'integer',value:BigInt(value)});
     if(type==='bool')return constant({$:'boolean',value});
     if(type==='string')return constant({$:'text',units:Array.from({length:value.length},(_,index)=>value.charCodeAt(index))});
     return refuse(node,'this literal needs its actual semantic value mapping');
    }
    case 'unit':return undefinedSource();
    case 'abort':{
     if(typeof node.message!=='string')return refuse(node,'a legacy source Error requires its actual literal message');
     return sourceError(node.message);
    }
    case 'none':if(node.absence!=='undefined'&&node.absence!=='null')return refuse(node,'absent literals must retain their exact source absence');return constant({$:node.absence});
    case 'some':return expression(node.value,environment,depth+1);
    case 'flattenOption':return expression(node.value,environment,depth+1);
    case 'optionalIndexNumber':return expression(node.arg,environment,depth+1);
    case 'unwrapOption':return expression(node.arg,environment,depth+1);
    case 'nullishNumber':{
     if(node.type.kind!=='float'||node.arg.type.kind!=='option'||!['float','bool','unit'].includes(node.arg.type.element.kind))return refuse(node,'nullable arithmetic requires its actual admitted Number conversion');
     return primitive('coerce primitive number',[expression(node.arg,environment,depth+1)]);
    }
    case 'toString':case 'optionalText':{
     if(node.console===true)return refuse(node,'console text requires its actual console formatting profile');
     const type=node.arg.type.kind==='option'?node.arg.type.element:node.arg.type;
     if(!['float','bool','string','int','nat','unit'].includes(type.kind))return refuse(node,'text conversion requires its actual primitive operand semantics');
     return primitive('convert primitive text',[expression(node.arg,environment,depth+1)]);
    }
    case 'optionalTypeOf':{
     const identity=nextBinding++,value={$:'sourceBinding',identity},text=word=>constant({$:'text',units:Array.from({length:word.length},(_,index)=>word.charCodeAt(index))});
     const body={$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:undefinedSource()},yes:text('undefined'),no:{$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:'null'})},yes:text('object'),no:text(node.present)}};
     return {$:'sourceLet',identity,value:expression(node.arg,environment,depth+1),body};
    }
    case 'optionalField':case 'optionalProject':{
     if(node.object.type.kind!=='option')return refuse(node,'optional projections require their actual checked receiver type');
     const identity=nextBinding++,value={$:'sourceBinding',identity};let body;
     if(node.k==='optionalField'&&node.object.type.element.kind==='record')body={$:'sourceGet',object:value,key:constant({$:'text',units:Array.from({length:node.field.length},(_,index)=>node.field.charCodeAt(index))})};
     else if(node.k==='optionalProject'&&node.object.type.element.kind==='string'&&node.field==='length')body=primitive('read text length',[value]);
     else return refuse(node,'this optional property requires its complete allocation or property model');
     for(const absence of ['undefined','null'])body={$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:absence})},yes:undefinedSource(),no:body};
     return {$:'sourceLet',identity,value:expression(node.object,environment,depth+1),body};
    }
    case 'coalesce':{
     if(!['undefined','null','both'].includes(node.left.type.absence))return refuse(node,'coalescing requires the actual source absence policy');
     const identity=nextBinding++,value={$:'sourceBinding',identity},fallback=expression(node.right,environment,depth+1);let body=value;
     for(const absence of node.left.type.absence==='both'?['undefined','null']:[node.left.type.absence])body={$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:absence})},yes:fallback,no:body};
     return {$:'sourceLet',identity,value:expression(node.left,environment,depth+1),body};
    }
    case 'length':if(node.array.type.kind==='string'&&node.type.kind==='float')return primitive('read text length',[expression(node.array,environment,depth+1)]);return refuse(node,'this length requires the actual array allocation model');
    case 'stringMap':{
     const operation={trim:'trim text',trimStart:'trim text start',trimEnd:'trim text end'}[node.op];if(!operation)return refuse(node,'this string mapping requires its complete source definition');
     return primitive(operation,[expression(node.string,environment,depth+1)]);
    }
    case 'stringTest':{
     const left=expression(node.string,environment,depth+1),right=expression(node.search,environment,depth+1),operation={startsWith:'test text prefix',endsWith:'test text suffix',includes:'test text inclusion'}[node.op];
     if(operation)return primitive(operation,[left,right]);
     if(!['unicodeCodeUnitLessThan','unicodeCodeUnitLessThanOrEqual','unicodeCodeUnitGreaterThan','unicodeCodeUnitGreaterThanOrEqual'].includes(node.op))return refuse(node,'this text test requires its complete source operation');
     const identity=nextBinding++,value={$:'sourceBinding',identity},zero=constant({$:'number',value:0}),greater=node.op==='unicodeCodeUnitGreaterThan'||node.op==='unicodeCodeUnitGreaterThanOrEqual';
     const less={$:'sourceNumberBinary',operator:'lessThan',left:greater?zero:value,right:greater?value:zero};
     return {$:'sourceLet',identity,value:primitive('compare text',[left,right]),body:node.op.endsWith('OrEqual')?{$:'sourceIf',condition:less,yes:constant({$:'boolean',value:true}),no:{$:'sourceStrictEqual',left:value,right:zero}}:less};
    }

    case 'var':{const identity=environment.get(node.name);if(identity===undefined)return refuse(node,'a variable use has no actual lexical owner');return {$:'sourceBinding',identity};}
    case 'let':{
     const value=expression(node.value,environment,depth+1),identity=nextBinding++,nested=new Map(environment);nested.set(node.name,identity);
     return {$:'sourceLet',identity,value,body:expression(node.body,nested,depth+1)};
    }
    case 'if':return {$:'sourceIf',condition:expression(node.cond,environment,depth+1),yes:expression(node.then,environment,depth+1),no:expression(node.else,environment,depth+1)};
    case 'binary':{
     if(['int','nat'].includes(node.left.type.kind)&&['int','nat'].includes(node.right.type.kind)){
      if(node.op==='sub'&&node.semantics==='truncated'){
       const left=nextBinding++,right=nextBinding++;
       return {$:'sourceLet',identity:left,value:expression(node.left,environment,depth+1),body:{$:'sourceLet',identity:right,value:expression(node.right,environment,depth+1),body:constructJavaScriptNaturalDifferenceSource(left,right)}};
      }
      const operation={add:'add integers',sub:'subtract integers',mul:'multiply integers',div:'divide integers',rem:'read integer remainder'}[node.op];
      if(operation)return primitive(operation,[expression(node.left,environment,depth+1),expression(node.right,environment,depth+1)]);
      if(['lt','gt','le','ge'].includes(node.op)){
       const identity=nextBinding++,value={$:'sourceBinding',identity},zero=constant({$:'number',value:0}),greater=node.op==='gt'||node.op==='ge';
       const less={$:'sourceNumberBinary',operator:'lessThan',left:greater?zero:value,right:greater?value:zero};
       return {$:'sourceLet',identity,value:primitive('compare integers',[expression(node.left,environment,depth+1),expression(node.right,environment,depth+1)]),body:node.op==='le'||node.op==='ge'?{$:'sourceIf',condition:less,yes:constant({$:'boolean',value:true}),no:{$:'sourceStrictEqual',left:value,right:zero}}:less};
      }
     }
     if(node.op==='concat'&&node.left.type.kind==='string'&&node.right.type.kind==='string')return primitive('concatenate text',[expression(node.left,environment,depth+1),expression(node.right,environment,depth+1)]);
     if(['and','or'].includes(node.op)&&node.left.type.kind==='bool'&&node.right.type.kind==='bool')return {$:'sourceIf',condition:expression(node.left,environment,depth+1),yes:node.op==='and'?expression(node.right,environment,depth+1):constant({$:'boolean',value:true}),no:node.op==='or'?expression(node.right,environment,depth+1):constant({$:'boolean',value:false})};
     if(node.op==='eq'||node.op==='ne'){
      const value={$:'sourceStrictEqual',left:expression(node.left,environment,depth+1),right:expression(node.right,environment,depth+1)};
      return node.op==='ne'?{$:'sourceIf',condition:value,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})}:value;
     }
     if(['gt','le','ge'].includes(node.op)&&node.left.type.kind==='float'&&node.right.type.kind==='float'){
      const left=nextBinding++,right=nextBinding++,a={$:'sourceBinding',identity:left},b={$:'sourceBinding',identity:right};
      const comparison={$:'sourceNumberBinary',operator:'lessThan',left:node.op==='le'?a:b,right:node.op==='le'?b:a};
      const value=node.op==='gt'?comparison:{$:'sourceIf',condition:comparison,yes:constant({$:'boolean',value:true}),no:{$:'sourceStrictEqual',left:a,right:b}};
      return {$:'sourceLet',identity:left,value:expression(node.left,environment,depth+1),body:{$:'sourceLet',identity:right,value:expression(node.right,environment,depth+1),body:value}};
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
    case 'cast':{
     const plan=readJavaScriptNaturalCastPlan(node.from.kind,node.to.kind,node.flavor,typeof node.message==='string',node.order??-1,oneRealm);
     if(plan===1)return expression(node.arg,environment,depth+1);
     if(plan===0)return refuse(node,'this conversion needs its complete checked source guard and realm');
     const identity=nextBinding++;
     return {$:'sourceLet',identity,value:expression(node.arg,environment,depth+1),body:constructJavaScriptNaturalCastSource(identity,node.message??'',plan===2)};
    }
    case 'match':{
     const branches=planJavaScriptNaturalPatternBranches(node.scrutinee.type.kind,node.cases.map(arm=>arm.pattern.k));
     if(branches.length!==2)return refuse(node,'only complete checked natural pattern branches are represented here');
     const identity=nextBinding++,value={$:'sourceBinding',identity},zero=node.cases[branches[0]],succ=node.cases[branches[1]];
     function branch(arm){
      const mode=readJavaScriptNaturalPatternBinding(arm.pattern.k);
      if(![0,1,2].includes(mode))return refuse(node,'the natural binder must retain its actual source rule');
      if(mode===0)return expression(arm.body,environment,depth+1);
      const binding=nextBinding++,inner=new Map(environment);inner.set(arm.pattern.name,binding);
      const bound=mode===2?constructJavaScriptNaturalPredecessorSource(identity):value;
      return {$:'sourceLet',identity:binding,value:bound,body:expression(arm.body,inner,depth+1)};
     }
     return {$:'sourceLet',identity,value:expression(node.scrutinee,environment,depth+1),body:constructJavaScriptNaturalMatchSource(identity,branch(zero),branch(succ))};
    }
    case 'truthy':return {$:'sourceIf',condition:expression(node.arg,environment,depth+1),yes:constant({$:'boolean',value:true}),no:constant({$:'boolean',value:false})};
    case 'unary':{
     const value=expression(node.arg,environment,depth+1);
     if(node.op==='neg'&&['int','nat'].includes(node.arg.type.kind))return primitive('negate integer',[value]);
     if(node.op==='not'&&node.arg.type.kind==='bool')return {$:'sourceIf',condition:value,yes:constant({$:'boolean',value:false}),no:constant({$:'boolean',value:true})};
     if(node.op==='neg'&&node.arg.type.kind==='float')return {$:'sourceNumberBinary',operator:'multiply',left:constant({$:'number',value:-1}),right:value};
     return refuse(node,'this unary operand requires additional exact source lowering');
    }
    case 'functionRef':{const actual=directory.get(node.fn);if(!actual)return refuse(node,'a function reference requires its actual supplied callable');return constant({$:'object',identity:actual.identity,callable:true});}
    case 'call':{
     const actual=directory.get(node.fn);if(!actual)return refuse(node,'a source call requires its actual supplied callable');
     return {$:'sourceCall',callee:constant({$:'object',identity:actual.identity,callable:true}),receiver:undefinedSource(),arguments:node.args.map(arg=>expression(arg,environment,depth+1))};
    }
    case 'math':case 'optionalPredicate':{
     if(node.k==='math'){
      if(readJavaScriptBigIntegerResultOperation(node.op)==='boolean'&&node.args.length===1&&node.args[0].type.kind==='bool')return {$:'sourceIf',condition:expression(node.args[0],environment,depth+1),yes:constant({$:'integer',value:1n}),no:constant({$:'integer',value:0n})};
      if(readJavaScriptSourceExceptionClass(node.op)!==''&&node.args.length===2&&node.args[0].type.kind==='string'&&node.args[1].type.kind==='bool')return primitive(node.op,node.args.map(arg=>expression(arg,environment,depth+1)));
      const comparison=readJavaScriptBigIntegerResultOperation(node.op);
      if((comparison==='number-compare'||comparison==='string-compare')&&node.args.length===2&&['int','nat'].includes(node.args[1].type.kind)&&node.args[0].type.kind===(comparison==='number-compare'?'float':'string'))return primitive(comparison==='number-compare'?'compare number integer':'compare text integer',node.args.map(arg=>expression(arg,environment,depth+1)));
      const integerCode=readBigIntegerBitwiseOperationCode(node.op),numberCode=readJavaScriptNumberBitwiseOperationCode(node.op);
      const integerOperations=['conjoin integer bits','disjoin integer bits','exclude equal integer bits','shift integer bits left','shift integer bits right','','complement integer bits'];
      const numberOperations=['conjoin number bits','disjoin number bits','exclude equal number bits','shift number bits left','shift number bits right','shift number bits right unsigned','complement number bits'];
      if(integerCode>=0&&node.args.every(arg=>arg.type.kind==='int'))return primitive(integerOperations[integerCode],node.args.map(arg=>expression(arg,environment,depth+1)));
      if(numberCode>=0&&node.args.every(arg=>arg.type.kind==='float'))return primitive(numberOperations[numberCode],node.args.map(arg=>expression(arg,environment,depth+1)));
     }
     if(node.k==='math'&&['coerceJavaScriptNumber','convertJavaScriptNumber'].includes(node.op)&&node.args.length===1){
      const value=expression(node.args[0],environment,depth+1);
      return node.args[0].type.kind==='float'?value:primitive(node.op==='convertJavaScriptNumber'?'convert explicit number':'coerce primitive number',[value]);
     }
     const numericOperation=node.k==='math'?{trunc:'truncate number',min:'read minimum number',max:'read maximum number',abs:'read absolute number',floor:'floor number',ceil:'ceil number',round:'round number',sign:'read number sign',sqrt:'read square root',isInteger:'test integer number',isSafeInteger:'test safe integer number'}[node.op]:undefined;
     if(numericOperation&&node.args.every(arg=>arg.type.kind==='float'))return primitive(numericOperation,node.args.map(arg=>expression(arg,environment,depth+1)));
     if(node.k==='optionalPredicate'&&['isInteger','isSafeInteger'].includes(node.op)&&node.arg.type.kind==='option'&&node.arg.type.element.kind==='float'){
      const identity=nextBinding++,value={$:'sourceBinding',identity};let body=primitive(node.op==='isInteger'?'test integer number':'test safe integer number',[value]);
      for(const absence of ['undefined','null'])body={$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:absence})},yes:constant({$:'boolean',value:false}),no:body};
      return {$:'sourceLet',identity,value:expression(node.arg,environment,depth+1),body};
     }

     const argument=node.k==='math'?node.args[0]:node.arg;
     if(node.k==='math'&&readJavaScriptBigIntegerResultOperation(node.op)==='number-power'&&node.args.length===2&&node.args.every(arg=>arg.type.kind==='float'))return {$:'sourceNumberBinary',operator:'power',left:expression(node.args[0],environment,depth+1),right:expression(node.args[1],environment,depth+1)};
     if(!argument||!['isNaN','isFinite'].includes(node.op)||node.k==='math'&&(node.args.length!==1||argument.type.kind!=='float')||node.k==='optionalPredicate'&&(argument.type.kind!=='option'||argument.type.element.kind!=='float'))return refuse(node,'this intrinsic requires its complete source lowering');
     const identity=nextBinding++,value={$:'sourceBinding',identity},yes=constant({$:'boolean',value:true}),no=constant({$:'boolean',value:false});
     const same={$:'sourceStrictEqual',left:value,right:value};
     let body=node.op==='isNaN'?{$:'sourceIf',condition:same,yes:no,no:yes}:{$:'sourceIf',condition:same,yes:{$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:'number',value:Infinity})},yes:no,no:{$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:'number',value:-Infinity})},yes:no,no:yes}},no};
     if(node.k==='optionalPredicate'&&node.op==='isFinite')for(const absence of ['undefined','null'])body={$:'sourceIf',condition:{$:'sourceStrictEqual',left:value,right:constant({$:absence})},yes:no,no:body};
     return {$:'sourceLet',identity,value:expression(argument,environment,depth+1),body};
    }
    case 'method':{
     if(node.object.type.kind==='string'){
      const operation={slice:'slice text',charAt:'read text character',at:'read relative text character',codePointAt:'read text code point',indexOf:'find text',lastIndexOf:'find last text'}[node.op];
      if(!operation)return refuse(node,'this text method requires its complete source operation');
      return primitive(operation,[expression(node.object,environment,depth+1),...node.args.map(arg=>expression(arg,environment,depth+1))]);
     }

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
     if(node.fn.k==='lambda'){
      if(node.fn.params.length!==node.args.length||node.fn.params.some(param=>param.default||param.guard||!acceptType(param.type)))return refuse(node,'immediate checked functions require complete actual parameter binding');
      const scope=new Map(environment),captures=[];
      for(const [index,param]of node.fn.params.entries()){const identity=nextBinding++;captures.push({identity,value:expression(node.args[index],environment,depth+1)});scope.set(param.name,identity);}
      let body=expression(node.fn.body,scope,depth+1);
      for(const capture of captures.toReversed())body={$:'sourceLet',identity:capture.identity,value:capture.value,body};
      return body;
     }
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
  let source=expression(tailLoop(entry,entry.body)??entry.body,bindings);
  for(const param of [...guards].reverse()){
   source=constructJavaScriptNaturalGuardSource(bindings.get(param.name),param.guard.message,source);
  }
  const result=compileJavaScriptCallSource(source,parameters);
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
