// Actual scalar lookup source facts into the existing standard Option data.
// This route never replaces an already-admitted immediate consumer pipeline.
import {TranslationError,unsupported} from './diagnostics.js';
import {rustFixedType} from './types.js';
import {readRustArrayLookupArity,readRustArrayLookupReceiverError,readRustArrayLookupElementOwnerError,
 planRustArrayLookupCondition,planRustArrayLookupIndex} from './rust-array-lookup-rules.js';
import {readRustArrayLookupOptionDisposition,readRustArrayLookupOptionCaptureRoles,
 readRustArrayLookupOptionOwner,readRustArrayLookupOptionVariant} from './rust-array-lookup-option-rules.js';
import {projectRustStaticByteArrayReadOrigin} from './rust-byte-string-expressions.js';
import {projectRustArrayLookupElementOwner} from './rust-array-lookup-source-owners.js';
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
import {readRustUsizeIndexExpression} from './rust-usize-index-expressions.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {createRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';

export function readRustArrayLookupOption(parser,receiver,method,args,range,path){
 const c=parser.cursor;
 const copied=c.is('.')&&c.peek(1).kind==='identifier'&&c.is('copied',1)&&c.is('(',2)&&c.is(')',3);
 const next=c.is('.',4)&&c.peek(5).kind==='identifier'?c.peek(5).value:'';
 if(readRustArrayLookupOptionDisposition(method.value,parser.ordinaryCompletionContext(),copied,next)!=='owned scalar option')return null;
 const arity=readRustArrayLookupArity(method.value,true);
 if(args.length!==arity)throw new TranslationError('type',method.value+' needs '+arity+' actual array lookup arguments',range);
 c.expect('.','copied lookup');c.identifier('copied lookup');c.expect('(','copied lookup');const close=c.expect(')','copied lookup');
 range={start:receiver.span?.start??range.start,end:close.end};
 const actual=parser.origins.origin(receiver,path),source=projectRustStaticByteArrayReadOrigin(actual)??actual;
 const element=source.type?.kind==='array'?source.type.element:null;
 const error=readRustArrayLookupReceiverError(source.type?.kind??'',element?.kind??'',source.arraySourceModel??'',source.proven===true,
  source.wrappers.every(layer=>layer==='reference'||layer==='box'),source.arrayElementCopyEvidence===true,true);
 if(error)throw unsupported('Rust array lookup receiver',error,range);
 if(!element)throw unsupported('Rust array lookup result','a copied value needs its complete actual scalar element type',range);
 if(arity&&readRustUsizeIndexExpression(parser,args[0],path))throw unsupported('Rust array lookup index','the actual index needs its complete direct usize source meaning',args[0].span??range);
 checkRustOwnedOrigin(parser.origins,actual,range);
 const owner=readRustPointerIntegerContractOwner(source.expandedSourceOwner??source.sourceOwner);
 const ownerError=readRustArrayLookupElementOwnerError(element.kind,String(element.bits??''),element.signed===true,owner);
 if(ownerError)throw unsupported('Rust array lookup result',ownerError,range);
 const leaf=projectRustArrayLookupElementOwner(source.expandedSourceOwner??source.sourceOwner);
 const payloadOrigin={...parser.origins.typeOrigin(element,path),sourceOwner:element.kind==='unit'?{kind:'unit',path:[],arguments:[]}:structuredClone(leaf)};
 const resultOrigin=createRustStandardSumSourceOrigin(parser,readRustArrayLookupOptionOwner(),[payloadOrigin],range);
 const sum=resultOrigin.standardSumSource,usize=rustFixedType('usize');
 const captures=readRustArrayLookupOptionCaptureRoles(method.value).map(role=>({role,name:parser.freshProductBinding(),value:role==='receiver'?receiver:args[0],type:role==='receiver'?source.type:usize}));
 const local=role=>{const capture=captures.find(entry=>entry.role===role);if(!capture)throw new TypeError('an owned lookup plan lost its actual capture');return {k:'name',path:[capture.name],span:capture.value.span??range};};
 const length={k:'cast',arg:{k:'length',object:local('receiver'),integer:true,span:range},to:usize,flavor:'exact',span:range};
 const number=value=>({k:'num',value,type:usize,span:range});
 const values={length,zero:number('0'),one:number('1'),...(arity?{index:local('index')}:{})};
 const project=plan=>{
  const stack=[],pop=()=>{if(!stack.length)throw new TypeError('an owned lookup plan lost an earlier term');return stack.pop();};
  for(const term of plan){if(values[term])stack.push(values[term]);else if(term==='lt'||term==='gt'||term==='sub'){const right=pop(),left=pop();stack.push({k:'binary',op:term,left,right,span:range});}else throw new TypeError('unknown owned array lookup source term');}
  if(stack.length!==1)throw new TypeError('an owned lookup plan needs one complete result');return stack[0];
 };
 const index=project(planRustArrayLookupIndex(method.value));
 const selected={k:'index',object:local('receiver'),index:{k:'cast',arg:index,to:{kind:'nat'},flavor:'exact',span:range},span:range};
 parser.origins.mark(selected,payloadOrigin);
 const construct=(present,args)=>({k:'app',fn:{k:'name',path:sum.constructorPaths[readRustArrayLookupOptionVariant(present)],span:range},args,span:range});
 let body={k:'if',cond:project(planRustArrayLookupCondition(method.value)),then:construct(true,[selected]),else:construct(false,[]),span:range};
 for(let at=captures.length-1;at>=0;at-=1){const capture=captures[at];body={k:'let',name:capture.name,type:capture.type,value:capture.value,body,span:range};}
 return parser.origins.mark(body,resultOrigin);
}
