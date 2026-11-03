// Irrefutable source bindings use the existing checked declaration projections.
// Every selected value is held before user names enter scope, including shadows.
import {unsupported} from './diagnostics.js';
import {hasRustSourceRegionComposition} from './rust-source-region-composition.js';
import {readRustExclusiveSourcePlaceTokens} from './rust-exclusive-source-places.js';
import {readRustOwnedRecordMutableName,registerRustOwnedRecordMutableBinding,readRustOwnedRecordStateDeclarations} from './rust-owned-record-state.js';
import {checkRustOwnedRecordRoot,projectRustOwnedRecordField} from './rust-owned-record-updates.js';
import {readRustOwnedBoxPlaceProjection} from './rust-owned-containers.js';
import {readRustNominalOwnershipFacts} from './rust-nominal-ownership.js';
import {readRustRecordPlaceOwner} from './rust-record-place-owner-rules.js';
import {readRustStandardSumSourceIdentity} from './rust-standard-sum-source.js';
import {attachRustRecordPlace} from './rust-record-place-regions.js';
import {attachRustSourceStateTransition} from './rust-source-state-transitions.js';
import {noteRustBorrowedExpressionSourcePosition} from './rust-borrowed-expression-positions.js';
import {readRustRecordProjection} from './rust-records.js';
import {readRustOwnedRecordStateStorageError} from './rust-owned-record-state-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust irrefutable place binding',reason,span);};
export function testRustModifiedPlaceBinding(pattern){const pending=[pattern];let work=0;while(pending.length){const current=pending.pop();if(++work>65536)fail('a complete pattern exceeds its source inspection budget',current?.span);if(current.sourceBindingMode||current.sourceBindingMutable)return true;pending.push(...(current.args??[]));}return false;}
export function readRustSourcePatternPlace(parser,pattern,path){
 if(pattern.k!=='nominal'||!hasRustSourceRegionComposition(parser))return null;const syntax=readRustExclusiveSourcePlaceTokens(parser,';');if(!syntax)return null;
 const {tokens,plan}=syntax,first=tokens[plan[1]],origin=parser.origins.scope.get(first.value);if(!origin)return null;
 checkRustOwnedRecordRoot(parser.origins,origin,first);let node=readRustOwnedRecordMutableName(parser,{k:'name',path:[first.value],span:{start:first.start,end:first.end}},path);
 for(const step of plan.slice(2)){
  if(step===-1){const actual=parser.origins.origin(node,path),payload=actual.wrappers[0]==='reference'?{...actual,wrappers:actual.wrappers.slice(1),sourceLifetimes:(actual.sourceLifetimes??[]).slice(1),sourceBorrowedPlace:true}:readRustOwnedBoxPlaceProjection(parser.origins,actual,1);if(!payload)fail('each initializer dereference retains its actual reference or canonical owning Box place',first);node=parser.origins.mark({...node},payload);noteRustBorrowedExpressionSourcePosition(parser,node,path);}
  else{const field=tokens[step],selected=readRustRecordProjection(parser,node,field.kind==='number'?field.raw:field.value,path,{start:tokens[0].start,end:field.end});if(!selected)fail('each initializer place component retains its actual nominal declaration',field);node=selected;}
 }
 for(let index=0;index<plan[0];index++)parser.cursor.next();return {node,place:true};
}
export function lowerRustSourcePlaceBinding(parser,pattern,value,path,range,sourceType=null,place=false){
 if(pattern.k!=='nominal'||!hasRustSourceRegionComposition(parser))return null;
 const root=pattern.pattern,table=parser.origins,original=table.origin(value,path),modified=testRustModifiedPlaceBinding(root);
 const pending=[root];let inspected=0;while(pending.length){const current=pending.pop();if(++inspected>65536)fail('a complete reference pattern inventory exceeds its source budget',current.span);if(table.patternReferences.get(current)){if(modified&&!table.collecting)fail('explicit reference patterns with modified bindings retain their separate reference-mode protocol',current.span);return null;}pending.push(...(current.args??[]));}
 // A refutable pattern retains the ordinary match/let-else interpretation.
 const constructorCount=owner=>{const declared=table.nominalOwnershipDeclarations.get(owner);if(Array.isArray(declared?.constructors))return declared.constructors.length;let count=0,work=0;for(const entry of table.valueCatalog.values()){if(++work>65536)fail('the actual constructor inventory exceeds its source inspection budget',root.span);if(entry.k==='ctor'&&entry.owner===owner)count++;}return count;};
 const entry=root.k==='ctor'?table.lookupValueOwner(root.path,path):root.k==='bindOrCtor'?table.lookupValueOwner([root.name],path):null,owner=original.type?.kind==='named'?table.lookupTypeOwner(original.type.path,original.path):null,leaf=root.k==='bind'||root.k==='bindOrCtor'&&entry?.k!=='ctor';
 if(!leaf&&(entry?.k!=='ctor'||owner?.k!=='data'||entry.owner!==owner.fullName||constructorCount(owner.fullName)!==1)){
  if(modified&&!table.collecting)fail('modified bindings need their actual irrefutable constructor inventory; refutable bindings retain a separate match protocol',root.span);if(modified)table.borrowedVectorNeedsFinalAdmission=true;return null;
 }
 if(sourceType&&!table.collecting){const actual=readRustStandardSumSourceIdentity(parser,original),expected=readRustStandardSumSourceIdentity(parser,sourceType);if(!actual||actual!==expected)fail('an annotated pattern initializer retains its exact complete source type and owner',range);}
 const bindings=[],members=[];let object=value;
 if(!place){const name=parser.freshProductBinding();table.bind(name,value,path,sourceType);bindings.push({name,value,span:range});object=table.mark({k:'name',path:[name],span:value.span},table.scope.get(name));}
 let visits=0;
 const visit=(current,selected,borrowed,depth)=>{
  const budget=readRustOwnedRecordStateStorageError(++visits,depth);if(budget)fail(budget,current.span);if(current.k==='wild')return;
  const actual=table.origin(selected,path);
  if(current.k==='bind'||current.k==='bindOrCtor'){
   if(current.k==='bindOrCtor'){const constructor=table.lookupValueOwner([current.name],path);if(constructor?.k==='ctor')return visit({...current,k:'ctor',path:['crate',...constructor.fullName.split('.')],args:[]},selected,borrowed,depth+1);}
   if(borrowed&&(current.sourceBindingMode||current.sourceBindingMutable))fail('the Rust 2024 binding profile allows modifiers only under the default move binding mode',current.span);
   if(current.sourceBindingMode&&!['shared reference'].includes(current.sourceBindingMode))fail('a binding retains its actual supported source modifier',current.span);
   const reference=borrowed||current.sourceBindingMode==='shared reference',name=parser.freshProductBinding();let expression=selected;
   if(reference)expression=table.wrap(expression,path,'reference');noteRustBorrowedExpressionSourcePosition(parser,expression,path);table.bind(name,expression,path);
   bindings.push({name,value:expression,span:current.span});members.push({pattern:current,name});return;
  }
  if(current.k!=='ctor')fail('a place binding retains an irrefutable nominal constructor or actual leaf binding',current.span);
  const constructor=table.lookupValueOwner(current.path,path),type=actual.type?.kind==='named'?table.lookupTypeOwner(actual.type.path,actual.path):null,inventory=type?constructorCount(type.fullName):0,borrowedParent=actual.wrappers.length>0&&actual.wrappers.every(layer=>layer==='reference');
  if(borrowedParent&&actual.sourceExclusiveMode==='exclusive'&&!table.collecting)fail('implicit exclusive-reference pattern bindings retain their complete mutable lease protocol',current.span);
  if(!table.collecting&&(constructor?.k!=='ctor'||type?.k!=='data'||constructor.owner!==type.fullName||inventory!==1||constructor.fields.length!==current.args.length||actual.wrappers.length&&!borrowedParent||!['owned','copy'].includes(readRustNominalOwnershipFacts(table,{...actual,wrappers:[]}).profile)))fail('every pattern constructor retains its actual single-variant destructor-free owner and complete field order',current.span);
  if(!constructor||!Array.isArray(constructor.fields)){if(table.collecting){table.borrowedVectorNeedsFinalAdmission=true;return;}fail('a constructor binding needs its actual complete field declaration',current.span);}
  for(let index=0;index<current.args.length;index++){
   const child=current.args[index];if(child.k==='wild')continue;const field=constructor.fields[index];if(!field)fail('every pattern field retains its actual declaration position',child.span);
   if(!table.collecting&&(field.arrayElementPosition||!readRustRecordPlaceOwner({type:field.type,owner:field.expandedSourceOwner??field.sourceOwner,wrappers:field.wrappers,lifetimes:field.sourceLifetimes??[],arrayElementPosition:field.arrayElementPosition},false)))fail('stored reference fields retain their separate source place protocol',child.span);
   const local=parser.freshProductBinding(),projection={k:'match',scrutinees:[selected],rows:[{patterns:[{k:'ctor',path:['crate',...constructor.fullName.split('.')],args:constructor.fields.map((_,at)=>at===index?{k:'bind',name:local}:{k:'wild'})}],body:{k:'name',path:[local],span:child.span}}],span:child.span};
   table.mark(projection,projectRustOwnedRecordField(table,actual,field,index,borrowedParent,child.span));const checked=attachRustRecordPlace(parser,projection,actual,field,index);visit(child,checked,borrowed||borrowedParent,depth+1);
  }
 };
 visit(root,object,false,0);
 for(const member of members){const item=member.pattern,reference=table.mark({k:'name',path:[member.name],span:item.span},table.scope.get(member.name));noteRustBorrowedExpressionSourcePosition(parser,reference,path);table.bind(item.name,reference,path);const name=item.sourceBindingMutable?registerRustOwnedRecordMutableBinding(parser,item.name,item.span):item.name,initialized=item.sourceBindingMutable?attachRustSourceStateTransition(parser,reference,readRustOwnedRecordStateDeclarations(parser).get(name),'initialize'):reference;bindings.push({name,value:initialized,span:item.span});}
 return bindings;
}
