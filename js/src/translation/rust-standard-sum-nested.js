// Actual nested source owners into existing complete constructor/match terms.
import {unsupported} from './diagnostics.js';
import {readRustStandardSumSourceOrigin,createRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {readRustStandardSumSourceArgumentCount,readRustStandardSumSourceConstructors,readRustStandardSumSourceFieldArgument,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {readRustStandardSumNestedArity,readRustStandardSumNestedProfile,planRustStandardSumNestedResult,planRustStandardSumNestedBranch,readRustStandardSumNestedConstructionPosition,readRustStandardSumNestedConstructionVariant,readRustStandardSumNestedSourceVersionError} from './rust-standard-sum-nested-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust nested standard sum',reason,span);};

function resultOrigin(parser,source,inner,profile,span){
 const stack=[],outer=source.standardSumSource,nested=inner.standardSumSource;
 for(const term of planRustStandardSumNestedResult(profile)){
  if(term==='inner')stack.push(inner);else if(term==='inner success')stack.push(nested.arguments[0]);else if(term==='inner error')stack.push(nested.arguments[1]);else if(term==='outer error')stack.push(outer.arguments[1]);
  else{const count=readRustStandardSumSourceArgumentCount(term);if(count<0||stack.length<count)fail('the shared nested result plan lost an actual type argument',span);const arguments_=stack.splice(stack.length-count,count);stack.push(createRustStandardSumSourceOrigin(parser,term,arguments_,span));}
 }
 if(stack.length!==1)fail('the shared nested result plan needs one complete actual sum',span);return stack[0];
}

function branchValue(parser,result,plan,payload,span){
 const stack=[];
 for(const term of plan){
  if(term==='payload'){if(!payload)fail('the nested branch has no actual payload',span);stack.push(payload);continue;}
  const position=readRustStandardSumNestedConstructionPosition(term),variant=readRustStandardSumNestedConstructionVariant(term);
  const actual=position===-1?result:position===0?readRustStandardSumSourceOrigin(parser,result.standardSumSource.arguments[0]):null;
  if(!actual)fail('the shared nested construction needs its actual target sum owner',span);
  const sum=actual.standardSumSource,count=readRustStandardSumSourceConstructorArity(sum.owner,variant),path=sum.constructorPaths[variant];
  if(count<0||!path||stack.length<count)fail('the shared nested construction lost its actual constructor fields',span);
  const args=stack.splice(stack.length-count,count),node=count?{k:'app',fn:{k:'name',path,span},args,span}:{k:'name',path,span};
  stack.push(parser.origins.mark(node,actual));
 }
 if(stack.length!==1)fail('the shared nested branch needs one complete result',span);return stack[0];
}

export function readRustStandardSumSourceNested(parser,receiver,method,path){
 if(!parser.ordinaryCompletionContext())return null;
 const source=readRustStandardSumSourceOrigin(parser,parser.origins.origin(receiver,path));if(!source)return null;
 const outer=source.standardSumSource,arity=readRustStandardSumNestedArity(outer.owner,method.value);if(arity<0)return null;
 const versionError=readRustStandardSumNestedSourceVersionError(outer.owner,method.value,parser.sourceVersionMinor);if(versionError)fail(versionError,method);
 const inner=readRustStandardSumSourceOrigin(parser,outer.arguments[0]);
 if(source.wrappers.length||!parser.origins.immutableProduct(source)||!inner||inner.wrappers.length||!parser.origins.immutableProduct(inner))fail('nested operations require actual complete direct Copy inner and outer standard sums',receiver.span);
 const profile=readRustStandardSumNestedProfile(outer.owner,method.value,inner.standardSumSource.owner);if(!profile)fail('the actual inner standard sum does not match this nested operation',receiver.span);
 const c=parser.cursor;c.expect('(','nested standard sum');const close=c.expect(')','nested standard sum'),span={start:receiver.span?.start??method.start,end:close.end};
 const result=resultOrigin(parser,source,inner,profile,span),capture=parser.freshProductBinding();
 const local=parser.origins.mark({k:'name',path:[capture],span},source);
 function matched(origin,scrutinee,isInner){
  const sum=origin.standardSumSource,rows=[];
  for(const variant of readRustStandardSumSourceConstructors(sum.owner)){
   const argument=readRustStandardSumSourceFieldArgument(sum.owner,variant),count=readRustStandardSumSourceConstructorArity(sum.owner,variant),binding=count===1?parser.freshProductBinding():null;
   const payload=binding?parser.origins.mark({k:'name',path:[binding],span},sum.arguments[argument]):null;
   const plan=planRustStandardSumNestedBranch(profile,sum.owner,variant,isInner);if(!plan.length)fail('the shared nested constructor branch is absent',span);
   const body=plan.length===1&&plan[0]==='match inner'?matched(inner,payload,true):branchValue(parser,result,plan,payload,span);
   rows.push({patterns:[{k:'ctor',path:sum.constructorPaths[variant],args:binding?[{k:'bind',name:binding,span}]:[],span}],body,span});
  }
  return parser.origins.mark({k:'match',scrutinees:[scrutinee],rows,span},result);
 }
 const body={k:'let',name:capture,type:source.type,value:receiver,body:matched(source,local,false),span};
 return parser.origins.mark(body,result);
}
