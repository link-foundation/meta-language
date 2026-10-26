// Thin owned-IR projection; decisions and every interface syntax operation are
// shared translated definitions. This never edits or reparses source strings.
import {unsupported} from './diagnostics.js';
import {acceptRustTextualSourceInterfaceModel,readRustTextualSourceType,acceptRustTextualSourceOperation,acceptRustTextualSourceBinary,renderRustTextualSourceInput,renderRustTextualSourceSliceArgument} from './rust-textual-source-interface-rules.js';
import {acceptRustScalarMathematicalInterfaceModel,acceptRustScalarMathematicalExpression,acceptRustScalarMathematicalExpressionField,acceptRustScalarMathematicalOperation} from './rust-scalar-mathematical-interface-rules.js';
import {typeKey} from './types.js';
import {checkRustPrimitiveOwnerProof} from './rust-primitive-owners.js';
import {validateRustTypeNamespace} from './rust-type-namespace.js';
import {checkRustSourceSignature} from './rust-source-signatures.js';
import {RUST_SOURCE_HEADER_EMISSION_POLICY} from './rust-source-header-emission.js';
import {renderRustSourceHeaderParameter,renderRustSourceHeaderLifetime,renderRustSourceHeaderDereference,renderRustSourceHeaderCopy,renderRustSourceHeaderFunction} from './rust-source-header-emission-rules.js';
import {renderRustBorrowedCallHeaderReference,renderRustBorrowedCallReference,renderRustBorrowedCallInvocation,renderRustBorrowedCallBlock} from './rust-borrowed-call-interface-rules.js';
import {acceptRustScalarSourceInterfaceModel,readRustScalarValueType,readRustScalarSourceType,acceptRustScalarSourceContext,acceptRustScalarSourceDeclaration,acceptRustScalarSourceDeclarationField,acceptRustScalarSourcePosition,acceptRustScalarSourceExpression,acceptRustScalarSourceExpressionField,acceptRustScalarSourceCall,acceptRustScalarSourceBudget,renderRustScalarSourceConversion} from './rust-scalar-source-interface-rules.js';
import {acceptRustSourceHeaderLifetimeCharacter} from './rust-source-header-emission-rules.js';
const fail=detail=>{throw unsupported('Rust scalar source interface',detail);};
function valueType(type,strings=false){
 const fields=type?.kind==='fixed'?['kind','bits','signed']:['kind'];
 const name=type&&(strings&&type.kind==='string'?'String':readRustScalarValueType(type.kind,type.bits??0,type.signed??false));
 if(!name||Object.keys(type).some(field=>!fields.includes(field)))fail('every interface and expression needs an actual scalar value type');
 return name;
}
function sourceType(position,strings=false){const value=valueType(position.type,strings);return value==='String'?readRustTextualSourceType(position.owner.kind,position.owner.path.join('::'),position.owner.arguments.length):readRustScalarSourceType(value,position.owner.kind,position.owner.path[0]??'',position.owner.path.length,position.owner.arguments.length);}
/** Validate the whole owned call graph before any target interface changes. */
export function readRustScalarSourceInterface(program,model){
 const strings=acceptRustTextualSourceInterfaceModel(model),mathematical=strings||acceptRustScalarMathematicalInterfaceModel(model);
 if(!acceptRustScalarSourceInterfaceModel(model)&&!mathematical)fail('select retained scalar Rust function interface 1');
 checkRustPrimitiveOwnerProof(program);validateRustTypeNamespace(program);
 const forbidden=Boolean(program.imports?.length||program.externals?.length||program.main?.effects?.length||program.main?.sequentialAsync||program.main?.sourceRuntimeWorker||program.referenceSemantics||program.utf16Strings||program.outputThreaded||program.abortsThreaded||program.sourceRuntimeShell||program.sourcePureUnitCalls);
 if(!acceptRustScalarSourceContext(program.sourceLanguage,forbidden))fail('only closed synchronous Rust scalar modules without imports or runtime effects are admitted');
 const headers=new Map(),policy=RUST_SOURCE_HEADER_EMISSION_POLICY,pending=[];
 const enqueue=children=>{if(!acceptRustScalarSourceBudget(pending.length+children.length))fail('the closed scalar call graph exceeds its expression budget');for(const child of children)pending.push(child);};
 for(const [key,declaration] of program.declarations){
  if(!acceptRustScalarSourceDeclaration(declaration.k,declaration.generated===true,key===declaration.fullName))fail('every interface needs its actual original owned function declaration');
  if(Object.keys(declaration).some(field=>!acceptRustScalarSourceDeclarationField(field,policy.declarationFields.includes(field))))fail('the declaration contains unrepresented source interface qualifiers');
  if(!declaration.sourceSignature)fail('every original function needs its retained version-two source signature');
  const signature=checkRustSourceSignature(declaration.sourceSignature,declaration.params.map(parameter=>parameter.type),declaration.ret,program.sourceLanguage);
  for(const lifetime of signature.lifetimeBindings)if(!lifetime.length||[...lifetime].some((character,index)=>!acceptRustSourceHeaderLifetimeCharacter(character.charCodeAt(0),index===0)))fail('source lifetimes need represented ASCII identifiers');
  for(const parameter of declaration.params)if(parameter.guard!=null||parameter.default!=null||Object.keys(parameter).some(field=>!policy.parameterFields.includes(field)))fail('parameter guards, defaults and additional qualifiers need their own interface interpretation');
  for(const [position,result] of [...signature.parameters.map(position=>[position,false]),[signature.result,true]])if(!acceptRustScalarSourcePosition(signature.model,sourceType(position,strings),position.wrappers.some(wrapper=>wrapper!=='reference'),result&&position.wrappers.length>0))fail('version-two scalar owners, immutable-reference inputs and direct results are required');
  headers.set(key,signature);enqueue([declaration.body]);
 }
 const visited=new WeakSet();let visits=0;
 for(let at=0;at<pending.length;at+=1){
  const node=pending[at];if(!node||typeof node!=='object')fail('a complete checked expression is required');
  if(visited.has(node))continue;visited.add(node);visits+=1;
  if(!acceptRustScalarSourceBudget(visits))fail('the closed scalar call graph exceeds its expression budget');
  valueType(node.type,strings);
  if(!(mathematical?acceptRustScalarMathematicalExpression(node.k):acceptRustScalarSourceExpression(node.k))||Object.keys(node).some(field=>!(mathematical?acceptRustScalarMathematicalExpressionField(node.k,field):acceptRustScalarSourceExpressionField(node.k,field))))fail('the expression requires an additional source interface interpretation');
  if(node.k==='math'){
   if(!Array.isArray(node.args)||!(strings?acceptRustTextualSourceOperation(node.op,node.args.length,node.type.kind):acceptRustScalarMathematicalOperation(node.op,node.args.length,node.type.kind)))fail('the mathematical operation needs its admitted scalar result and complete operands');
   enqueue(node.args);
  }
  else if(node.k==='call'){
   const target=program.declarations.get(node.fn),signature=headers.get(node.fn);
   const argumentTypes=Boolean(target&&Array.isArray(node.args)&&node.args.length===target.params.length&&node.args.every((argument,index)=>typeKey(argument.type)===typeKey(target.params[index].type)));
   const staticBorrow=signature?.parameters.some(position=>position.lifetimes.includes('static'))??false;
   if(!acceptRustScalarSourceCall(Boolean(target&&signature&&target.fullName===node.fn),argumentTypes,Boolean(target&&typeKey(node.type)===typeKey(target.ret)),staticBorrow))fail('calls need owned scalar targets with matching checked types and nonstatic copied borrows');
   enqueue(node.args);
  }else if(node.k==='let')enqueue([node.value,node.body]);
  else if(node.k==='if')enqueue([node.cond,node.then,node.else]);
  else if(node.k==='unary')enqueue([node.arg]);
  else if(node.k==='cast'){valueType(node.from);valueType(node.to);enqueue([node.arg]);}
  else if(node.k==='binary'){valueType(node.domain,strings);if(strings&&!acceptRustTextualSourceBinary(node.domain.kind,node.op))fail('string ordering needs its own source interface interpretation');enqueue([node.left,node.right]);}
 }
 return headers;
}
/** Copy borrowed inputs into core values and convert only pointer-sized slots. */
export function renderRustScalarSourceHeader(signature,params,name,body,indent){
 const copies=[],parameters=params.map((parameter,index)=>{
  const position=signature.parameters[index],source=sourceType(position,true);let type=source,value=parameter.name;
  for(let layer=position.wrappers.length-1;layer>=0;layer-=1){type=renderRustBorrowedCallHeaderReference(position.lifetimes[layer]??'',type,layer<position.wrappers.length-1);value=renderRustSourceHeaderDereference(value);}
  const converted=position.type.kind==='string'&&position.wrappers.length?renderRustTextualSourceInput(value):renderRustScalarSourceConversion(value,source,false);
  if(position.wrappers.length||converted!==value)copies.push(renderRustSourceHeaderCopy(parameter.name,converted));
  return renderRustSourceHeaderParameter(parameter.name,type);
 });
 const result=sourceType(signature.result,true),value=renderRustScalarSourceConversion(body,result,true);
 return renderRustSourceHeaderFunction(name,signature.lifetimeBindings.map(renderRustSourceHeaderLifetime).join(', '),parameters.join(', '),result,indent([...copies,value].join('\n'),1));
}
/** Evaluate in order, convert into source slots, borrow, then return to core. */
export function renderRustScalarSourceCall(signature,head,values,names,indent){
 const bindings=values.map((value,index)=>renderRustSourceHeaderCopy(names[index],renderRustScalarSourceConversion(value,sourceType(signature.parameters[index],true),true)));
 const arguments_=names.map((name,index)=>{const position=signature.parameters[index],slice=sourceType(position,true)==='str';let value=slice?renderRustTextualSourceSliceArgument(name):name;for(let layer=slice?1:0;layer<position.wrappers.length;layer+=1)value=renderRustBorrowedCallReference(value);return value;});
 const invocation=renderRustScalarSourceConversion(renderRustBorrowedCallInvocation(head,arguments_.join(', ')),sourceType(signature.result,true),false);
 return renderRustBorrowedCallBlock(indent([...bindings,invocation].join('\n'),1));
}
