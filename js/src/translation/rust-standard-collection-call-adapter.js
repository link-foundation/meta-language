// Executable packet transport over the actual collection kernel. Source text
// admission remains separate from this prechecked interface and library seam.
import {readRustStandardCollectionStateError,createRustStandardCollectionState} from './rust-standard-collection-state.js';
import {executeRustStandardCollectionValuePacket} from './rust-standard-collection-value-packet.js';
export {executeRustStandardCollectionValuePacket} from './rust-standard-collection-value-packet.js';
import {readRustStandardCollectionCallOriginError,readRustStandardCollectionSourceOriginError,resolveRustStandardCollectionOwner,testRustStandardCollectionMutation} from './rust-standard-collection-rules.js';
import {readRustExclusiveSourceLoanContextError,readRustExclusiveSourceLoanStateError,accessRustExclusiveSourcePlace,executeRustExclusiveSourceCollectionOperation} from './rust-exclusive-source-loans.js';
import {readSourceLibraryDeclarationOwnership,readSuppliedSourceLibraryClosure} from './source-library-closure.js';
import {parseJavaScript} from './javascript.js';
import {checkProgram} from './check.js';

// Source contexts and declarations are compiler metadata, not represented Map
// values. Their structural comparisons never implement collection key equality.
function sameRustStandardCollectionSourceTree(left,right){return JSON.stringify(left)===JSON.stringify(right);}
// Complete graph correspondence includes declaration Maps, BigInts, signed
// zero, data-only own fields and shared-node identity. JSON loses these.
export function testRustStandardCollectionProjectedGraphEquality(left,right){
 const forward=new WeakMap(),backward=new WeakMap();let work=0;
 function same(actual,expected){
  if(++work>1000000)return false;
  if(actual===null||expected===null||typeof actual!=='object'||typeof expected!=='object')return Object.is(actual,expected);
  if(forward.has(actual)||backward.has(expected))return forward.get(actual)===expected&&backward.get(expected)===actual;
  forward.set(actual,expected);backward.set(expected,actual);
  if(Array.isArray(actual)!==Array.isArray(expected)||Object.getPrototypeOf(actual)!==Object.getPrototypeOf(expected))return false;
  const keys=Reflect.ownKeys(actual),other=Reflect.ownKeys(expected);if(keys.length!==other.length||keys.some(key=>!other.includes(key)))return false;
  for(const key of keys){const first=Object.getOwnPropertyDescriptor(actual,key),second=Object.getOwnPropertyDescriptor(expected,key);if(!Object.hasOwn(first,'value')||!Object.hasOwn(second,'value')||!same(first.value,second.value))return false;}
  if(actual instanceof Map||expected instanceof Map){if(!(actual instanceof Map)||!(expected instanceof Map)||actual.size!==expected.size)return false;const rows=[...expected];let index=0;for(const [key,value] of actual){const row=rows[index++];if(!same(key,row[0])||!same(value,row[1]))return false;}return true;}
  if(actual instanceof Set||expected instanceof Set){if(!(actual instanceof Set)||!(expected instanceof Set)||actual.size!==expected.size)return false;return [...actual].every((value,index)=>same(value,[...expected][index]));}
  return true;
 }
 return same(left,right);
}
function registeredRustStandardCollectionSource(context,binderIdentity,collection){
 const invalid=readRustExclusiveSourceLoanContextError(context);if(invalid)return invalid;
 const binder=context.binders.find(row=>row.identity===binderIdentity),position=binder?.position,entry=(context.collectionTypes??[]).find(row=>row.typeIdentity===position?.collectionTypeIdentity);
 return binder&&entry&&position.wrappers.length===0&&binder.lexical===collection.root&&entry.typeIdentity===collection.origin.typeIdentity&&entry.pointerWidth===collection.pointerWidth&&entry.keyDomain===collection.keyDomain&&entry.valueDomain===collection.valueDomain&&sameRustStandardCollectionSourceTree(entry.origin,collection.origin)?'':'collection packets require the actual registered whole carrier, concrete type and lexical source root';
}
/** Actual lexical loans authorize access before represented kernel execution.
 * The caller must already have proved the immediate standard Option projection
 * and complete ownership transfers from its real source expression interfaces.
 * @param {object} context @param {object} loanState @param {number} binderIdentity
 * @param {number} authority @param {RustStandardCollectionState} collection
 * @param {string} method @param {RustStandardCollectionScalar[]} arguments_
 * @param {RustStandardCollectionCallOrigin} call @param {string} projection */
export function executeRustStandardCollectionSourcePacket(context,loanState,binderIdentity,authority,collection,method,arguments_,call,projection=''){
 const invalid=registeredRustStandardCollectionSource(context,binderIdentity,collection)||readRustExclusiveSourceLoanStateError(context,loanState);if(invalid)return {$:'rustCollectionInvalid',reason:invalid};
 if(method!=='get'&&projection!=='')return {$:'rustCollectionInvalid',reason:'only actual get source packets select an immediate Option projection'};
 if(method==='get'||method==='clone'){
  const actual=readRustStandardCollectionCallOriginError(collection.origin,collection.root,method,arguments_.length,call);if(actual)return {$:'rustCollectionInvalid',reason:actual};
  const access=accessRustExclusiveSourcePlace(context,loanState,binderIdentity,[],'read',authority);if(access.$!=='rustExclusiveSourceContinue')return {$:'rustCollectionInvalid',reason:access.reason};
  if(method==='clone'){
   const destination=context.binders.find(row=>row.lexical===call.outputRoot);
   if(!destination||destination.mode!=='owned'||destination.identity===binderIdentity||destination.position.collectionTypeIdentity!==collection.origin.typeIdentity)return {$:'rustCollectionInvalid',reason:'collection clone source result needs its actual independent registered destination carrier'};
  }
  const packet=executeRustStandardCollectionValuePacket(collection,method,arguments_,call,projection);return packet.$==='rustCollectionPacketNormal'?{...packet,loanState}:packet;
 }
 const result=executeRustExclusiveSourceCollectionOperation(context,loanState,binderIdentity,authority,collection,method,arguments_,call);
 if(result.$==='rustCollectionNormal')return {$:'rustCollectionPacketNormal',values:[result.value,result.state],writeback:result.writeback,loanState:result.loanState};
 return result.$==='rustExclusiveSourceLoweringInvalid'?{$:'rustCollectionInvalid',reason:result.reason}:result;
}
/** @param {object} context @param {number} binderIdentity @param {RustStandardCollectionCallOrigin} call */
export function createRustStandardCollectionSourcePacket(context,binderIdentity,call){
 const invalid=readRustExclusiveSourceLoanContextError(context);if(invalid)return {$:'rustCollectionInvalid',reason:invalid};
 const binder=context.binders.find(row=>row.identity===binderIdentity),entry=(context.collectionTypes??[]).find(row=>row.typeIdentity===binder?.position?.collectionTypeIdentity);
 if(!entry||!binder||binder.lexical!==call.receiverRoot)return {$:'rustCollectionInvalid',reason:'collection creation packets need the actual registered concrete destination carrier'};
 const created=createRustStandardCollectionState(entry.origin,entry.keyDomain,entry.valueDomain,entry.pointerWidth,binder.lexical,call);
 return created.$==='rustCollectionCreated'?{$:'rustCollectionPacketNormal',values:[created.state,created.state],writeback:false}:created;
}
/** Publish whole-root state before invoking the ordinary continuation. Reads
 * retain the prior caller root; clone returns its new root as result zero.
 * @param {object} packet @param {{binder:number,collection:RustStandardCollectionState}[]} roots
 * @param {number} binderIdentity @param {Function} continuation */
export function resumeRustStandardCollectionSourcePacket(packet,roots,binderIdentity,continuation){
 const selected=roots.find(row=>row.binder===binderIdentity);
 if(packet?.$!=='rustCollectionPacketNormal'||packet.values.length!==2||typeof packet.writeback!=='boolean'||!selected||typeof continuation!=='function')return {$:'rustCollectionInvalid',reason:'collection packet writeback retains actual result/state positions and caller root continuation'};
 const updated=packet.values[1],invalid=readRustStandardCollectionStateError(updated);if(invalid)return {$:'rustCollectionInvalid',reason:invalid};
 if(updated.root!==selected.collection.root||updated.origin.typeIdentity!==selected.collection.origin.typeIdentity)return {$:'rustCollectionInvalid',reason:'collection writeback cannot replace the selected actual caller root or concrete type'};
 const published=roots.map(row=>row.binder===binderIdentity&&packet.writeback?{...row,collection:updated}:row);
 return continuation(packet.values[0],published,packet.loanState);
}

function rustStandardCollectionSourceDeclarations(program){const rows=[];function visit(items,path){for(const declaration of items){if(declaration.k==='module')visit(declaration.items,[...path,declaration.name]);else rows.push({path:['crate',...path,declaration.name],declaration});}}visit(program.items,[]);return rows;}
function rustStandardCollectionSourceCalls(body){const calls=[],pending=[body],seen=new Set();let count=0;while(pending.length){const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;if(++count>100000)throw new TypeError('collection source library body exceeds its traversal profile');seen.add(value);if(value.k==='call')calls.push(value.fn);for(const child of Object.values(value))if(child&&typeof child==='object')pending.push(child);}return calls;}
/** A bridge seam for a REAL automatically projected library graph. It requires
 * both complete checked declarations with actual source-module ownership and
 * complete source declarations installed in the caller's source program.
 * It never returns success for a name/flag-only body promise.
 * library.functions: [{checkedName,sourcePath}], library.packetType:
 * {checkedName,sourcePath,constructors:{normal:{checkedName,sourceName},
 * invalid:{checkedName,sourceName},resource:{checkedName,sourceName}}}.
 */
export function readRustStandardCollectionProjectedLibraryError(library){
 if(!library?.closure||!library.program?.declarations||!library.sourceProgram?.items||!Array.isArray(library.functions)||library.functions.length!==3||new Set(library.functions.map(row=>row.role)).size!==3||!['packet','kernel','equality'].every(role=>library.functions.some(row=>row.role===role))||typeof library.entryPath!=='string'||!library.entryPath.endsWith('/rust-standard-collection-value-packet.js'))return 'collection source linkage requires complete actual packet kernel equality and source declaration graphs';
 let owners;
 try{
  const actualClosure=readSuppliedSourceLibraryClosure(library.entryPath,library.closure.inputs.map(row=>({path:row.path,source:row.source})),{aliases:true}),surface=parseJavaScript(actualClosure.source),checked=checkProgram(surface),projected=checkProgram(structuredClone(library.sourceProgram));
  if(actualClosure.source!==library.closure.source||actualClosure.sourceSha256!==library.closure.sourceSha256||actualClosure.bytes!==library.closure.bytes)return 'collection closure must match its actual recomposed complete source inputs';
  if(!testRustStandardCollectionProjectedGraphEquality(checked,library.program)||!testRustStandardCollectionProjectedGraphEquality(projected,checked))return 'collection source and library bodies must recheck to the same complete actual checked graph; unsupported projection remapping refuses';
  // An identity projection still checks actual bodies; it is not permission to
  // attach arbitrary source declarations to a genuine checked kernel graph.
  for(const row of library.functions)if(!sameRustStandardCollectionSourceTree(row.sourcePath,['crate',...row.checkedName.split('.')]))return 'collection source function remapping requires a separate exact checked projection interpretation';
  if(!sameRustStandardCollectionSourceTree(library.packetType?.sourcePath,['crate',...(library.packetType?.checkedName??'').split('.')])||Object.values(library.packetType?.constructors??{}).some(row=>row.checkedName!==row.sourceName))return 'collection source datatype or constructor remapping requires its separate exact checked projection interpretation';
  owners=readSourceLibraryDeclarationOwnership(actualClosure,checked);
 }catch(error){return 'collection source graph regeneration refused: '+String(error?.message??error);}
 const source=rustStandardCollectionSourceDeclarations(library.sourceProgram),binding=library.functions.find(row=>row.role==='packet'),entry=binding?library.program.declarations.get(binding.checkedName):null,owner=binding?owners.find(row=>row.name===binding.checkedName):null,projected=binding?source.find(row=>sameRustStandardCollectionSourceTree(row.path,binding.sourcePath))?.declaration:null;
 if(entry?.k!=='fn'||!entry.body||entry.params.length!==5||!projected?.body||projected.k!=='fn'||projected.params.length!==5||!projected.ret||(owner?.sourceName??owner?.name)!=='executeRustStandardCollectionValuePacket'||!owner?.path?.endsWith('/rust-standard-collection-value-packet.js'))return 'collection packet binding requires its actual complete automatically projected source callable body and original module owner';
 const kernel=library.functions.find(row=>row.role==='kernel'),actualKernel=kernel?library.program.declarations.get(kernel.checkedName):null,kernelOwner=kernel?owners.find(row=>row.name===kernel.checkedName):null;
 if(actualKernel?.k!=='fn'||!actualKernel.body||(kernelOwner?.sourceName??kernelOwner?.name)!=='executeRustStandardCollectionOperation'||!kernelOwner?.path?.endsWith('/rust-standard-collection-state.js')||!rustStandardCollectionSourceCalls(entry.body).includes(kernel.checkedName))return 'collection packet callable must invoke the actual complete projected shared kernel body';
 const ordered=owners.filter(row=>(row.sourceName??row.name)==='executeRustStandardCollectionOrderedSnapshot'&&row.path?.endsWith('/rust-standard-collection-ordered-snapshot.js'));
 if(ordered.length!==1||library.program.declarations.get(ordered[0].name)?.k!=='fn'||!library.program.declarations.get(ordered[0].name)?.body||!rustStandardCollectionSourceCalls(entry.body).includes(ordered[0].name))return 'ordered collection projection retains its actual shared BTree snapshot body rather than Hash insertion-order substitution';
 const borrowed=owners.filter(row=>(row.sourceName??row.name)==='executeRustStandardCollectionOrderedBorrow'&&row.path?.endsWith('/rust-standard-collection-ordered-borrow.js'));
 if(borrowed.length!==1||!library.program.declarations.get(borrowed[0].name)?.body||!rustStandardCollectionSourceCalls(entry.body).includes(borrowed[0].name))return 'shared ordered source iteration retains its actual complete projected borrowed-entry callable body';
 const equality=library.functions.find(row=>row.role==='equality'),actualEquality=library.program.declarations.get(equality.checkedName),equalityOwner=owners.find(row=>row.name===equality.checkedName),equalityKernels=owners.filter(row=>(row.sourceName??row.name)==='executeRustStandardCollectionEquality'&&row.path?.endsWith('/rust-standard-collection-equality.js'));
 if(actualEquality?.k!=='fn'||!actualEquality.body||actualEquality.params.length!==4||actualEquality.params.slice(0,2).some(parameter=>parameter.type.kind!=='data'||parameter.type.name!==entry.params[0].type.name)||(equalityOwner?.sourceName??equalityOwner?.name)!=='compareRustStandardCollectionValuePackets'||!equalityOwner?.path?.endsWith('/rust-standard-collection-value-packet.js')||equalityKernels.length!==1||library.program.declarations.get(equalityKernels[0].name)?.k!=='fn'||!library.program.declarations.get(equalityKernels[0].name)?.body||!rustStandardCollectionSourceCalls(actualEquality.body).includes(equalityKernels[0].name))return 'collection equality requires its actual complete shared wrapper and order-independent kernel bodies';
 const ordering=owners.filter(row=>(row.sourceName??row.name)==='compareRustStandardCollectionOrderedValuePackets'&&row.path?.endsWith('/rust-standard-collection-value-packet.js')),orderingKernels=owners.filter(row=>(row.sourceName??row.name)==='executeRustStandardCollectionOrdering'&&row.path?.endsWith('/rust-standard-collection-ordering.js')),orderingEntry=library.program.declarations.get('compareRustStandardCollectionOrderedValuePackets');
 if(ordering.length!==1||ordering[0].name!=='compareRustStandardCollectionOrderedValuePackets'||orderingEntry?.k!=='fn'||!orderingEntry.body||orderingEntry.params.length!==4||orderingEntry.params.slice(0,2).some(parameter=>parameter.type.kind!=='data'||parameter.type.name!==entry.params[0].type.name)||orderingKernels.length!==1||!library.program.declarations.get(orderingKernels[0].name)?.body||!rustStandardCollectionSourceCalls(orderingEntry.body).includes(orderingKernels[0].name))return 'ordered comparison retains its actual shared wrapper and lexicographic BTree kernel bodies';
 const type=library.packetType,checked=type?library.program.declarations.get(type.checkedName):null,declared=type?source.find(row=>sameRustStandardCollectionSourceTree(row.path,type.sourcePath))?.declaration:null;
 if(checked?.k!=='data'||declared?.k!=='data'||checked.ctors.length!==3||declared.ctors.length!==3||entry.ret?.kind!=='data'||entry.ret.name!==type.checkedName||projected.ret.kind!=='named'||!sameRustStandardCollectionSourceTree(projected.ret.path,type.sourcePath))return 'collection packet linkage retains every actual projected transition constructor and callable result type';
 for(const [role,fields] of [['normal',['values','writeback']],['invalid',['reason']],['resource',['state','reason']]]){
  const names=type.constructors?.[role],actual=declared.ctors.find(row=>row.name===names?.sourceName),original=checked.ctors.find(row=>row.name===names?.checkedName);
  if(!actual||!original||actual.fields.length!==fields.length||original.fields.length!==fields.length||original.fields.some((field,index)=>field.name!==fields[index]))return 'collection packet transition fields must match the actual complete projected constructor inventory';
 }
 return '';
}
/** Lower a prechecked source interface into real source call/match/tuple/let
 * terms. heldOperands are already evaluated names; handlers preserve actual
 * Invalid/Resource continuations and decode the concrete typed normal result.
 * The caller applies this through its existing owned-record effect completion
 * continuation, including branch, return and residual state publication.
 */
export function lowerRustStandardCollectionProjectedSourcePacket(parser,library,request,heldOperands,roots,handlers,continuation){
 const error=readRustStandardCollectionProjectedLibraryError(library);if(error)return {$:'rustCollectionSourceLoweringInvalid',reason:error};
 const originError=readRustStandardCollectionSourceOriginError(request.origin,request.keyDomain,request.valueDomain,request.pointerWidth);if(originError)return {$:'rustCollectionSourceLoweringInvalid',reason:originError};
 const invalid=readRustStandardCollectionCallOriginError(request.origin,request.root,request.method,request.call.arguments.length,request.call);if(invalid)return {$:'rustCollectionSourceLoweringInvalid',reason:invalid};
 if(parser?.origins?.collecting!==false||!(parser.reservedNames instanceof Set)||typeof parser.freshProductBinding!=='function'||heldOperands.length!==5||heldOperands.some(node=>node.k!=='name'||node.path.length!==1)||typeof handlers?.decode!=='function'||typeof handlers.invalid!=='function'||typeof handlers.resource!=='function'||typeof continuation!=='function')return {$:'rustCollectionSourceLoweringInvalid',reason:'collection lowering requires actual final source interfaces held operands and complete typed continuations'};
 const receiver=roots.find(row=>row.root===request.root);if(!receiver||receiver.origin.typeIdentity!==request.origin.typeIdentity||!sameRustStandardCollectionSourceTree(receiver.origin,request.origin))return {$:'rustCollectionSourceLoweringInvalid',reason:'collection source packet lowering retains its actual concrete caller root'};
 const selected=parser.origins.origin(heldOperands[0],request.path);
 if(!selected?.proven||selected.sourceCollectionTypeIdentity!==request.origin.typeIdentity||selected.sourceBindingIdentity!==request.root&&selected.sourcePlaceBindingIdentity!==request.root)return {$:'rustCollectionSourceLoweringInvalid',reason:'collection source operand retains its actual parser-proven concrete carrier and binding root'};
 const functionBinding=library.functions.find(row=>row.role==='packet'),packetType=library.packetType,actualCallable=parser.origins.lookupValueOwner(functionBinding.sourcePath,request.path),actualType=parser.origins.lookupTypeOwner(packetType.sourcePath,request.path),actualResult=actualCallable?.result?.type;
 if(actualCallable?.k!=='fn'||actualCallable.parameters.length!==5||actualCallable.parameters.some(origin=>!origin.proven||!origin.type)||actualType?.k!=='data'||actualResult?.kind!=='named'||parser.origins.lookupTypeOwner(actualResult.path,actualCallable.result.path)?.fullName!==actualType.fullName)return {$:'rustCollectionSourceLoweringInvalid',reason:'collection library callable and result datatype must already exist as complete actual parser-owned source interfaces'};
 for(const [role,count] of [['normal',2],['invalid',1],['resource',2]]){const constructor=parser.origins.lookupValueOwner([...packetType.sourcePath,packetType.constructors[role].sourceName],request.path);if(constructor?.k!=='ctor'||constructor.owner!==actualType.fullName||constructor.fields.length!==count)return {$:'rustCollectionSourceLoweringInvalid',reason:'collection source patterns retain actual registered transition constructor owners and fields'};}
 for(const row of rustStandardCollectionSourceDeclarations(library.sourceProgram))for(const segment of row.path.slice(1))parser.reservedNames.add(segment);
 const fresh=()=>{const name=parser.freshProductBinding();parser.reservedNames.add(name);return name;},binding=library.functions.find(row=>row.role==='packet'),type=library.packetType,normalNames=[fresh(),fresh()],invalidName=fresh(),resourceNames=[fresh(),fresh()],reference=name=>({k:'name',path:[name]}),values=reference(normalNames[0]),writeback=reference(normalNames[1]),nextName=fresh(),packetName=fresh(),expected=testRustStandardCollectionMutation(resolveRustStandardCollectionOwner(request.origin.resolvedOwner,true,false),request.method);
 const result=handlers.decode({k:'tupleIndex',object:values,index:0},request),state={k:'tupleIndex',object:values,index:1},packet={k:'tuple',items:[result,state]},normal={k:'if',cond:expected?writeback:{k:'unary',op:'not',arg:writeback},then:packet,else:handlers.invalid({k:'str',value:'collection packet writeback contradicts its actual source method'})};
 const call={k:'app',fn:{k:'name',path:[...binding.sourcePath]},args:heldOperands},rows=[{patterns:[{k:'ctor',path:[...type.sourcePath,type.constructors.normal.sourceName],args:normalNames.map(name=>({k:'bind',name}))}],body:normal},{patterns:[{k:'ctor',path:[...type.sourcePath,type.constructors.invalid.sourceName],args:[{k:'bind',name:invalidName}]}],body:handlers.invalid(reference(invalidName))},{patterns:[{k:'ctor',path:[...type.sourcePath,type.constructors.resource.sourceName],args:resourceNames.map(name=>({k:'bind',name}))}],body:handlers.resource(reference(resourceNames[0]),reference(resourceNames[1]))}],match={k:'match',scrutinees:[call],rows},packetReference=reference(packetName),published=roots.map(row=>row.root===request.root&&expected?{...row,value:reference(nextName)}:row);
 const continued=continuation({k:'tupleIndex',object:packetReference,index:0},published),body=expected?{k:'let',name:nextName,value:{k:'tupleIndex',object:packetReference,index:1},body:continued}:continued;
 return {$:'rustCollectionSourceLowered',expression:{k:'let',name:packetName,value:match,body},roots:published};
}
