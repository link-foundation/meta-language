// TYPE-only source positions. Runtime value loans belong to actual vector
// producers and bindings and are never copied into these records.
import {typeError} from './diagnostics.js';
import {measureRustStandardSumSourceMetadata} from './rust-standard-sum-source-metadata.js';
import {readRustBorrowedVectorPositionBudgetError} from './rust-borrowed-vector-rules.js';
import {readRustOwnedContainerFacts,reserveRustOwnedContainerCopies} from './rust-owned-containers.js';
import {acceptRustBorrowedVectorElement,readRustBorrowedVectorElementPhase,acceptRustBorrowedVectorShape,acceptRustBorrowedVectorPosition,planRustBorrowedVectorResultParameters,readRustBorrowedVectorArgumentError} from './rust-borrowed-vector-rules.js';
import {readRustCopyClosureValueBorrows,validateRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {mergeRustOwnedLoanBindings,projectRustOwnedLoanBindings} from './rust-owned-binding-rules.js';
import {checkRustOwnedOrigin} from './rust-owned-bindings.js';
import {unsupported} from './diagnostics.js';
import {readRustBorrowedVectorElementCountError,readRustBorrowedVectorProductionError,readRustBorrowedVectorResultError,readRustBorrowedVectorObservationError,acceptRustBorrowedVectorProducedValue,readRustBorrowedVectorCallRegionError,readRustBorrowedVectorCallLifetime} from './rust-borrowed-vector-rules.js';
import {qualifyRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {captureRustMutableRecordValueLoans} from './rust-mutable-record-loans.js';
import {acceptRustBorrowedVectorLiteralContext,readRustBorrowedVectorEmptyConstructor} from './rust-borrowed-vector-rules.js';
import {acceptRustBorrowedSourceBoxReference} from './rust-borrowed-source-region-rules.js';
import {acceptRustOwnedBoxTypeNode} from './rust-owned-recursive-datatype-rules.js';
import {noteRustBorrowedExpressionSourcePosition} from './rust-borrowed-expression-positions.js';
function sameElementOwner(left,right){return left.kind===right.kind&&left.path.length===right.path.length&&left.path.every((part,index)=>part===right.path[index])&&left.arguments.length===right.arguments.length&&left.arguments.every((child,index)=>sameElementOwner(child,right.arguments[index]));}
export function projectRustBorrowedVectorElement(parser,node,path,expected){
 const table=parser.origins,actual=table.origin(node,path);if(!expected||actual.wrappers[0]!=='reference'||expected.wrappers[0]!=='reference'||actual.wrappers.length<=expected.wrappers.length)return actual;
 reserveRustOwnedContainerCopies(table,[actual,expected]);const qualified=qualifyRustStandardSumSourceOrigin(table,actual),target=qualifyRustStandardSumSourceOrigin(table,expected);if(!qualified?.sourceOwner||!target?.sourceOwner)return actual;
 const drop=qualified.wrappers.length-target.wrappers.length,project=source=>{let owner=source;for(let index=0;index<drop;index+=1){if(!owner||!acceptRustOwnedBoxTypeNode(['box'],owner.kind,owner.path.join('::'),owner.arguments.length))return null;owner=owner.arguments[0];}return owner;},owner=project(qualified.sourceOwner),expanded=qualified.expandedSourceOwner?project(qualified.expandedSourceOwner):null;
 if(!acceptRustBorrowedSourceBoxReference(qualified.wrappers,target.wrappers,Boolean(owner)&&(!qualified.expandedSourceOwner||Boolean(expanded)),Boolean(owner)&&sameElementOwner(owner,target.sourceOwner)))return actual;
 const projected={...qualified,wrappers:[...target.wrappers],sourceOwner:owner,...(expanded?{expandedSourceOwner:expanded}:{}),sourceBoxLayers:(qualified.sourceBoxLayers??[]).slice(drop)};
 if(!table.same(projected,target))fail('a contextual Box reference needs its actual matching referent type',node.span);
 table.mark(node,projected);noteRustBorrowedExpressionSourcePosition(parser,node,path);return projected;
}
export function rustBorrowedVectorPositionOrigin(position,path){return {type:position.type,path:[...path],wrappers:[...position.wrappers],sourceLifetimes:[...position.lifetimes],sourceOwner:position.owner,arrayElementPosition:position.arrayElementPosition,proven:true,copyEvidence:position.wrappers[0]==='reference',fields:null};}
export function readRustBorrowedVectorElementAdmission(parser,origin){
 const reference=acceptRustBorrowedVectorElement(parser.origins.borrowedVectorSourceEnabled,parser.sourceVersionMinor,origin.wrappers,(origin.sourceLifetimes??[]).map(value=>value??''),origin.proven===true,false);
 if(!reference)return '';
 // Remove the source reference only for the referent's target representation
 // proof. No source Copy/Clone or owned producer fact is created or installed.
 const referent={...origin,wrappers:[],sourceLifetimes:[],copyEvidence:false},facts=readRustOwnedContainerFacts(parser.origins,referent);
 return readRustBorrowedVectorElementPhase(parser.origins.borrowedVectorSourceEnabled,reference,parser.headerDiscovery===true,parser.origins.nominalOwnershipScopeComplete===true,facts.closed);
}
export function isRustBorrowedVectorSourceOrigin(table,origin){const owner=origin?.expandedSourceOwner??origin?.sourceOwner,position=origin?.arrayElementPosition;return acceptRustBorrowedVectorShape(table.borrowedVectorSourceEnabled,origin?.type?.kind??'',owner?.kind??'',owner?.path?.join('::')??'',owner?.arguments?.length??-1,origin?.wrappers?.length??-1,Boolean(position)&&acceptRustBorrowedVectorPosition(position.wrappers,position.lifetimes.length,Boolean(position.arrayElementPosition)));}
export function isRustBorrowedVectorProducedValue(table,origin){return acceptRustBorrowedVectorProducedValue(table.borrowedVectorSourceEnabled,isRustBorrowedVectorSourceOrigin(table,origin),origin?.proven===true,origin?.sourceOwnedResultProof===true);}
export function sameRustBorrowedVectorType(table,left,right){return isRustBorrowedVectorSourceOrigin(table,left)&&isRustBorrowedVectorSourceOrigin(table,right)&&left.type.reference===right.type.reference&&table.same(rustBorrowedVectorPositionOrigin(left.arrayElementPosition,left.path),rustBorrowedVectorPositionOrigin(right.arrayElementPosition,right.path));}
export function readRustBorrowedVectorSignature(parser,origin){return isRustBorrowedVectorSourceOrigin(parser.origins,origin)?readRustBorrowedVectorElementAdmission(parser,rustBorrowedVectorPositionOrigin(origin.arrayElementPosition,origin.path)):'';}
export function readRustBorrowedVectorResultParameters(result,parameters){return planRustBorrowedVectorResultParameters(result.arrayElementPosition?.lifetimes?.[0]??'',parameters.map(value=>value.wrappers.length===1&&value.wrappers[0]==='reference'||Boolean(value.arrayElementPosition)),parameters.map(value=>value.arrayElementPosition?.lifetimes?.[0]??value.sourceLifetimes?.[0]??''));}
export function validateRustBorrowedVectorArgument(table,actual,expected,span){
 const left=isRustBorrowedVectorSourceOrigin(table,actual),right=isRustBorrowedVectorSourceOrigin(table,expected);if(!left&&!right)return;
 const a=actual.arrayElementPosition,b=expected.arrayElementPosition;
 const error=readRustBorrowedVectorArgumentError(left,right,Boolean(a&&b)&&table.same(rustBorrowedVectorPositionOrigin(a,actual.path),rustBorrowedVectorPositionOrigin(b,expected.path)),Boolean(a&&b)&&JSON.stringify(a.owner)===JSON.stringify(b.owner),a?.lifetimes?.[0]??'',b?.lifetimes?.[0]??'');if(error)throw typeError(error,span);
 // A transferred vector container still carries live referent loans. Recheck
 // those after later call arguments without rereading its moved owner root.
 validateRustCopyClosureValueBorrows(table,actual,span);
}
const fail=(message,span)=>{throw unsupported('Rust borrowed vector source',message,span);};
export function guardRustBorrowedVectorObservation(parser,node,path,operation,span=node.span){
 const table=parser.origins;if(!table.borrowedVectorSourceEnabled)return;
 const pending=[table.origin(node,path)],seen=new Set();let visits=0;
 while(pending.length){const value=pending.pop();if(!value||seen.has(value))continue;seen.add(value);const budget=readRustBorrowedVectorPositionBudgetError(++visits,0);if(budget)fail(budget,span);
  const error=readRustBorrowedVectorObservationError(Boolean(value.arrayElementPosition),operation);if(error)fail(error,span);if(value.arrayElementPosition)checkRustOwnedOrigin(table,value,span);pending.push(...(value.fields??[]));
 }
}
export function readRustBorrowedVectorArgumentRegions(origin){
 let loans=readRustCopyClosureValueBorrows(origin);
 if(origin.wrappers[0]==='reference'){
  loans=mergeRustOwnedLoanBindings(loans,projectRustOwnedLoanBindings(origin.sourceBindingIdentity??'',origin.sourceLoanBindingIdentities??[]));
  loans=mergeRustOwnedLoanBindings(loans,projectRustOwnedLoanBindings(origin.sourcePlaceBindingIdentity??'',origin.sourcePlaceLoanBindingIdentities??[]));
  if(loans.length===0&&origin.sourceLexicalBindingIdentity)loans=[origin.sourceLexicalBindingIdentity];
 }
 return loans;
}
export function seedRustBorrowedVectorParameterRegions(parser,names){
 const table=parser.origins;if(!table.borrowedVectorSourceEnabled)return;
 for(const name of names){const value=table.scope.get(name);if(!value||!value.sourceLexicalBindingIdentity||!(value.wrappers.length===1&&value.wrappers[0]==='reference'||isRustBorrowedVectorSourceOrigin(table,value)))continue;
  // This is a formal region placeholder for this function body. Call results
  // substitute actual argument loans; this identity never enters a TYPE facet.
  table.scope.set(name,{...value,sourceValueBorrowBindings:mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(value),[value.sourceLexicalBindingIdentity])});
 }
}
export function captureRustBorrowedVectorConstructor(table,node,origin){
 if(!isRustBorrowedVectorSourceOrigin(table,origin))return false;
 table.borrowedVectorNeedsFinalAdmission=true;
 const count=readRustBorrowedVectorElementCountError(node.k==='array'?node.items.length:-1);if(count)fail(count,node.span);
 reserveRustOwnedContainerCopies(table,[origin]);const expected=rustBorrowedVectorPositionOrigin(origin.arrayElementPosition,origin.path),fields=[];let loans=[];
 for(const item of node.items){if(item.spread)fail('a borrowed vector needs individually checked immutable reference elements',item.value.span);
  const actual=table.origin(item.value,origin.path);checkRustOwnedOrigin(table,actual,item.value.span);validateRustCopyClosureValueBorrows(table,actual,item.value.span);reserveRustOwnedContainerCopies(table,[actual]);
  const child=readRustBorrowedVectorArgumentRegions(actual),error=readRustBorrowedVectorProductionError(table.same(actual,expected)&&JSON.stringify(actual.sourceOwner)===JSON.stringify(expected.sourceOwner),actual.wrappers.length===1&&actual.wrappers[0]==='reference',actual.proven&&child.length>0,table.collecting);if(error)fail(error,item.value.span);
  table.argument(actual,expected,item.value.span);loans=mergeRustOwnedLoanBindings(loans,child);fields.push(child);
 }
 const produced={...origin,copyEvidence:false,sourceLexicalBindingIdentity:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceBorrowedPlace:false,sourceValueBorrowBindings:loans,sourceValueFieldBorrowBindings:fields,sourceOwnedResultProof:true};
 if(node.span)table.freshVectorConstructors.add(`${node.span.start}:${node.span.end}`);table.mark(node,captureRustMutableRecordValueLoans(table,produced,node.span));return true;
}
export function readRustBorrowedVectorLiteral(parser,args,path,span){
 if(!parser.origins.borrowedVectorSourceEnabled)return null;
 const count=readRustBorrowedVectorElementCountError(args.length);if(count)fail(count,span);
 const expected=parser.standardSumExpectedOrigin,firstActual=args.length?parser.origins.origin(args[0],path):null,context=acceptRustBorrowedVectorLiteralContext(true,isRustBorrowedVectorSourceOrigin(parser.origins,expected),firstActual?.wrappers[0]==='reference',args.length);
 if(args.length===0&&!context)return null;
 if(context)reserveRustOwnedContainerCopies(parser.origins,[expected]);
 const target=context?rustBorrowedVectorPositionOrigin(expected.arrayElementPosition,expected.path):null;
 const first=args.length?projectRustBorrowedVectorElement(parser,args[0],path,target):target;if(readRustBorrowedVectorElementAdmission(parser,first)!=='admit')return null;
 if(target)for(const argument of args.slice(1))projectRustBorrowedVectorElement(parser,argument,path,target);
 reserveRustOwnedContainerCopies(parser.origins,[first]);const actual=qualifyRustStandardSumSourceOrigin(parser,first);if(!actual)fail('a borrowed vector referent needs its actual qualified source owner',span);
 const position=createRustBorrowedVectorElementPosition(actual),origin={type:{kind:'array',element:actual.type,reference:false},path:[...path],wrappers:[],fields:null,arraySourceModel:'owned vector',arrayElementCopyEvidence:true,arrayElementPosition:position,sourceOwner:{kind:'library',path:['std','vec','Vec'],arguments:[actual.sourceOwner]},sourceLifetimes:[],copyEvidence:false,proven:true};
 const node={k:'array',items:args.map(value=>({value,spread:false})),span};captureRustBorrowedVectorConstructor(parser.origins,node,origin);return node;
}
export function readRustBorrowedVectorEmptySourceConstructor(parser,segments,args,path,span){const resolved=parser.resolve(segments.slice(0,-1),path),disposition=readRustBorrowedVectorEmptyConstructor(isRustBorrowedVectorSourceOrigin(parser.origins,parser.standardSumExpectedOrigin),resolved.join('::'),segments.at(-1)??'',args.length);if(!disposition)return null;parser.origins.vector(segments.slice(0,-1),resolved,path,span);if(disposition==='arity')throw typeError('Vec::new expects 0 arguments',span);return readRustBorrowedVectorLiteral(parser,[],path,span);}
export function validateRustBorrowedVectorFunctionResultRegions(parser,body,result,names){
 const table=parser.origins;if(!isRustBorrowedVectorSourceOrigin(table,result))return null;
 const actual=table.origin(body,parser.modulePath),parameters=names.map(name=>table.scope.get(name)),plan=readRustBorrowedVectorResultParameters(result,parameters);let allowed=[];
 for(const index of plan)if(index>=0)allowed=mergeRustOwnedLoanBindings(allowed,readRustBorrowedVectorArgumentRegions(parameters[index]));
 const loans=readRustCopyClosureValueBorrows(actual),error=readRustBorrowedVectorResultError(sameRustBorrowedVectorType(table,actual,result)&&Boolean(actual.arrayElementPosition)&&JSON.stringify(actual.arrayElementPosition.owner)===JSON.stringify(result.arrayElementPosition.owner),actual.sourceOwnedResultProof===true||table.availableOwnedBinding(actual),!plan.includes(-1)&&loans.every(identity=>allowed.includes(identity)),table.collecting);if(error)fail(error,body.span);
 validateRustCopyClosureValueBorrows(table,actual,body.span);return {regions:loans};
}
export function validateRustBorrowedVectorFunctionBody(parser,body,result,names){return validateRustBorrowedVectorFunctionResultRegions(parser,body,result,names)!==null;}
export function markRustBorrowedVectorCallResult(table,node,path,entry,origin){
 if(!isRustBorrowedVectorSourceOrigin(table,origin)||node.k!=='app'||node.fn.k!=='name'||node.fn.path.length===1&&table.scope.has(node.fn.path[0]))return origin;
 const recorded=Boolean(node.span)&&table.freshCalls.has(`${node.span.start}:${node.span.end}`),parameters=entry.parameters??[],plan=readRustBorrowedVectorResultParameters(origin,parameters);if(!recorded)return origin;
 if(plan.includes(-1))fail('a borrowed vector call needs its actual source parameter region plan',node.span);
 let loans=[];const lifetimes=[];for(const index of plan){const arg=node.args[index];if(!arg)fail('a borrowed vector call needs every selected actual argument',node.span);const actual=table.origin(arg,path);validateRustCopyClosureValueBorrows(table,actual,arg.span);const regions=readRustBorrowedVectorArgumentRegions(actual),error=readRustBorrowedVectorCallRegionError(actual.wrappers[0]==='reference',regions.length,table.collecting);if(error)fail(error,arg.span);loans=mergeRustOwnedLoanBindings(loans,regions);lifetimes.push(actual.arrayElementPosition?.lifetimes?.[0]??actual.sourceLifetimes?.[0]??'');}
 const lifetime=readRustBorrowedVectorCallLifetime(origin.arrayElementPosition.lifetimes[0]??'',lifetimes),position={...origin.arrayElementPosition,lifetimes:[lifetime||null]};
 return {...origin,arrayElementPosition:position,freshArrayValue:false,sourceLexicalBindingIdentity:null,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceValueBorrowBindings:loans,sourceValueFieldBorrowBindings:null,sourceOwnedResultProof:true};
}
export function validateRustBorrowedVectorCallRegions(table,args,path,parameters,result,span){
 if(!isRustBorrowedVectorSourceOrigin(table,result)||table.collecting)return;
 const plan=readRustBorrowedVectorResultParameters(result,parameters);if(plan.includes(-1))fail('a borrowed vector call needs its actual source parameter region plan',span);
 for(const index of plan){const arg=args[index];if(!arg)fail('a borrowed vector call needs every selected actual argument',span);const actual=table.origin(arg,path),regions=readRustBorrowedVectorArgumentRegions(actual),error=readRustBorrowedVectorCallRegionError(actual.wrappers[0]==='reference',regions.length,false);if(error)fail(error,arg.span);checkRustOwnedOrigin(table,actual,arg.span);}
}
export function createRustBorrowedVectorElementPosition(origin){
 measureRustStandardSumSourceMetadata([origin]);
 let visits=0;
 function clone(position,depth){
  const error=readRustBorrowedVectorPositionBudgetError(++visits,depth);if(error)throw typeError(error);
  if(!position?.type||!position.owner)throw typeError('a retained array element position needs its actual type and owner');
  return {type:structuredClone(position.type),wrappers:[...position.wrappers],lifetimes:[...position.lifetimes],owner:structuredClone(position.owner),...(position.arrayElementPosition?{arrayElementPosition:clone(position.arrayElementPosition,depth+1)}:{})};
 }
 return clone({type:origin.type,wrappers:origin.wrappers,lifetimes:origin.sourceLifetimes??[],owner:origin.sourceOwner,arrayElementPosition:origin.arrayElementPosition},0);
}
