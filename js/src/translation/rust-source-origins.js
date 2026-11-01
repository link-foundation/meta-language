import {retainRustNominalCopyOrigin} from './rust-nominal-copies.js';
import {recordRustSourceExpressionType,readRustSourceExpressionType,reuseRustSourceExpressionType} from './rust-source-expression-types.js';
import {resolveRustStandardSumSourcePattern,readRustStandardSumSourceIdentity} from './rust-standard-sum-source.js';
import {projectRustStandardSumPatternField,markRustStandardSumOwnedCallResult,requireRustStandardSumPatternTransfer,finishRustStandardSumFieldDereference} from './rust-standard-sum-ownership.js';
import {assignRustCopyClosureLexicalBinding,readRustCopyClosureValueBorrows,carryRustCopyClosureValueBorrows,projectRustCopyClosureValueField,validateRustCopyClosureValueBorrows} from './rust-copy-closure-lifetimes.js';
import {readRustNominalOwnershipFacts,projectRustNominalPatternField,prepareRustNominalPatternTransfers,acceptRustNominalVectorElement} from './rust-nominal-ownership.js';
import {mergeRustOwnedLoanBindings,projectRustOwnedLoanBindings,readRustOwnedStringConstructionError,acceptRustOwnedBindingValue} from './rust-owned-binding-rules.js';
import {isRustOwnedBindingValue,captureRustOwnedBorrow,finishRustOwnedExpression,expireRustOwnedBindings,transferRustOwnedResult,checkRustOwnedOrigin,registerRustOwnedBinding,consumeRustOwnedOrigin,availableRustOwnedOrigin,joinRustOwnedBindings,checkRustOwnedGuard} from './rust-owned-bindings.js';
import {finishRustOwnedBoxDereference,reserveRustOwnedContainerCopies} from './rust-owned-containers.js';
import {captureRustOwnedVectorConstructor,isRustOwnedVectorProducedValue} from './rust-owned-vectors.js';
import {validateRustBorrowedVectorArgument,captureRustBorrowedVectorConstructor,isRustBorrowedVectorSourceOrigin,isRustBorrowedVectorProducedValue,markRustBorrowedVectorCallResult,validateRustBorrowedVectorCallRegions} from './rust-borrowed-vectors.js';
import {acceptRustOwnedArgumentReadCheck} from './rust-owned-recursive-datatype-rules.js';
import {testRustSourceReferenceLifetimeLayer} from './rust-copy-closure-parameter-rules.js';
import {readRustStandardSumGuardBindingDisposition,readRustStandardSumGuardAlternativeBindingError,readRustStandardSumGuardAliasTransferError} from './rust-standard-sum-guard-rules.js';
import {countRustStandardSumGuardAliases,sameRustStandardSumGuardLineage} from './rust-standard-sum-guards.js';
import {acceptRustStaticByteArrayCoercion,acceptRustStaticByteArrayOwnerEquality} from './rust-byte-string-expression-rules.js';
import {readRustSourceNumericPatternOwnerError,acceptRustSourceNumericPatternAlias} from './rust-numeric-pattern-rules.js';
import {acceptRustSourceLiteralPatternAlias} from './rust-byte-pattern-rules.js';
import {readRustSourceStringPatternError} from './rust-string-pattern-rules.js';
import {projectRustCallableOwnerDescriptor} from './rust-callable-owner-origins.js';
import {readRustCallableOwnerError,readRustCallableOwnerJoinError,acceptRustCallableOwnerDescriptorsEqual} from './rust-callable-owner-rules.js';
import {selectRustPrimitiveExpressionOrigin} from './rust-primitive-expression-origins.js';
import {acceptRustPrimitiveBooleanResult} from './rust-primitive-expression-rules.js';
// Source indirection evidence for the Rust value projection. The existing
// parser collects declarations before validating bodies, retaining its own
// type, path and pattern records rather than inferring a second typed program.
import { unsupported } from './diagnostics.js';
import { planTuplePatternFields } from './rust-record-rules.js';
import {acceptRustSharedArrayHandleTransfer} from './rust-shared-array-handle-rules.js';
import { rustFixedType } from './types.js';
import { acceptRustImmutableIndirectionLayer, acceptRustSourceIndirectionArgument, acceptRustSourceBoxCoercionLayer, acceptRustSourceBoxOwner, acceptRustSourceBoxAvailability, acceptRustSourceTextEquality, readRustSourceExpressionOrigin } from './rust-indirection-rules.js';
import { acceptRustImmutableProductType, acceptRustProductOrigin } from './rust-product-rules.js';
import { acceptRustSourceArrayOwner, acceptRustSourceArrayElement, acceptRustSourceArrayCoercion, acceptRustFreshArrayResultTail } from './rust-array-rules.js';
import {ownedLocalPair} from './rust-owned-local-state.js';
import {readRustCopyClosureJoinError} from './rust-copy-closure-rules.js';
import {readRustFunctionPointerCoercionError} from './rust-function-pointer-rules.js';
import {readRustFreshVectorArgumentForm,acceptRustFreshOwnedVectorArgument} from './rust-fresh-vector-argument-rules.js';
import {acceptRustKnownFreshVectorCall} from './rust-fresh-vector-result-rules.js';
import {acceptRustFreshVectorResultCall} from './rust-fresh-vector-call-result-rules.js';
import {acceptRustScalarVectorCopyOccurrence} from './rust-scalar-vector-copy-rules.js';
import {acceptRustScalarArrayIndexElement} from './rust-array-index-rules.js';
import {readRustPointerIntegerOwnerError,readRustPointerIntegerOwnerJoinError,selectRustPointerIntegerResultOwner} from './rust-pointer-integer-owner-rules.js';
import {readRustPointerIntegerContractOwner} from './rust-pointer-integer-owners.js';
import {readRustSourceNamespaceRole,readRustSourceNamespaceLookupRole,readRustSourceNamespaceCandidate,acceptRustSourceNamespaceDeclarationPair,acceptRustSourceNamespaceOuterCandidate} from './rust-source-namespace-rules.js';

// Parser passes reuse the actual catalog Map. Preserve both namespace entries
// even when that historical one-entry map overwrites the other namespace.
const sourceNamespaceCatalogs=new WeakMap();

const unknown = (uncertain = false) => ({ floatSourceModel: null, wrappers: [], type: null, path: [], fields: null, textSourceModel: null, copyEvidence: false, proven: false, diverging: false, uncertain });
const value = (copyEvidence = false, type = null) => ({ ...unknown(), proven: true, copyEvidence, type, textSourceModel: type?.kind === 'string' ? 'owned string' : null });
const clone = origin => origin && ({ proven: true, diverging: false, uncertain: false, copyEvidence: false, fields: null, ...origin, ...(origin.arrayElementPosition?{arrayElementPosition:structuredClone(origin.arrayElementPosition)}:{}), ...(origin.sourceLifetimes?{sourceLifetimes:[...origin.sourceLifetimes]}:{}), wrappers: [...origin.wrappers], path: [...origin.path], fields: origin.fields?.map(clone) ?? null,callableParameters:origin.callableParameters?.map(clone)??null,callableResult:clone(origin.callableResult)??null });

export class RustSourceOrigins {
  constructor(catalog = null, arrayResultModel = '',localValueStateModel='',sharedArrayHandleSourceModel='') {
    this.localValueStateModel=localValueStateModel;
    this.sharedArrayHandleSourceModel=sharedArrayHandleSourceModel;
    this.arrayResultModel = arrayResultModel;
    this.freshCalls = new Set();
    this.freshVectorConstructors = new Set();
    this.collecting = catalog === null;
    this.catalog = catalog ?? new Map();
    let namespaces=sourceNamespaceCatalogs.get(this.catalog);
    if(!namespaces){namespaces={type:new Map(),value:new Map()};for(const [name,entry]of this.catalog){const role=readRustSourceNamespaceRole(entry.k);if(role)namespaces[role].set(name,entry);}sourceNamespaceCatalogs.set(this.catalog,namespaces);}
    this.typeCatalog=namespaces.type;this.valueCatalog=namespaces.value;
    this.nominalCopyDerivations=new Map();this.nominalCopyOwners=new Set();
    this.nominalOwnershipDeclarations=new Map();this.nominalOwnershipScopeComplete=false;
    this.ownedContainerSourceEnabled=false;
    this.borrowedVectorSourceEnabled=false;
    this.borrowedVectorNeedsFinalAdmission=false;
    this.ownedContainerMaterializedNodes=0;this.ownedContainerMaterializedUnits=0;
    this.scope = new Map();
    this.sourceTemporaryBindings=new Set();this.ownedArgumentTransfers=new WeakSet();this.sourceBindingStates=new Map();this.nextSourceBindingIdentity=0;this.ownedBindingExcluded=false;
    this.expressions = new WeakMap();
    this.patternReferences = new WeakMap();
    this.literalPatternAliases = new WeakMap();
    this.type = unknown();
  }

  copy(origin){return clone(origin);}
  unknownOrigin(path){return {...unknown(true),path:[...path]};}

  enter(scope = this.scope) {
    const previous = this.scope;
    this.scope = new Map([...scope].map(([name,origin])=>{let value=origin.sourceBindingIdentity==null?registerRustOwnedBinding(this,origin):clone(origin);if(value.sourceLexicalBindingIdentity==null)value=assignRustCopyClosureLexicalBinding(this,value);return [name,value];}));
    return previous;
  }

  ownedTemporaryFrame(){return new Set(this.sourceTemporaryBindings);}
  captureOwnedBorrow(origin){return captureRustOwnedBorrow(this,origin);}
  finishOwnedExpression(node,path,frame,extending){finishRustOwnedExpression(this,node,path,frame,extending);}
  transferOwnedResult(node,path){return transferRustOwnedResult(this,node,path);}
  prepareOwnedArgument(node,parameter,path){if(!parameter||!isRustOwnedBindingValue(this,parameter))return;const actual=this.origin(node,path);if(consumeRustOwnedOrigin(this,actual,node.span))this.ownedArgumentTransfers.add(node);}
  expireOwnedBindings(before,result,retainedRegions=[]){expireRustOwnedBindings(this,before,result,retainedRegions);}
  snapshotOwnedBindings(){return new Map(this.sourceBindingStates);}
  restoreOwnedBindings(snapshot){this.sourceBindingStates=new Map(snapshot);}
  joinOwnedBindings(left,right,leftReturns,rightReturns){joinRustOwnedBindings(this,left,right,leftReturns,rightReturns);}
  checkOwnedGuard(snapshot,span){checkRustOwnedGuard(this,snapshot,span);}
  availableOwnedBinding(origin){return availableRustOwnedOrigin(this,origin);}
  consumeOwnedBinding(origin,span){return consumeRustOwnedOrigin(this,origin,span);}
  typed(type, path, wrappers = [], fields = null, textSourceModel = type.kind === 'string' ? 'owned string' : null) {
    const standardSum=type.kind==='named'?this.lookupTypeOwner(type.path,path)?.standardSumSourceOrigin:null;
    if(standardSum){this.type={...clone(standardSum),path:[...path],wrappers:[...wrappers]};return type;}
    fields ??= type.kind === 'tuple' ? type.elements.map(element => this.typeOrigin(element, path)) : null;
    this.type = { floatSourceModel: type.kind === 'float' ? 'binary64 scalar' : null, type, path: [...path], wrappers, fields, textSourceModel, ...(type.kind === 'array' && { arraySourceModel: 'owned vector',arrayElementCopyEvidence:acceptRustScalarArrayIndexElement(type.element.kind) }), copyEvidence: acceptRustImmutableProductType(type.kind, fields?.every(field => this.immutableProduct(field)) ?? false, false), proven: true, diverging: false, uncertain: false };
    retainRustNominalCopyOrigin(this,this.type);
    return type;
  }

  typeOrigin(type, path) {
    const standardSum=type.kind==='named'?this.lookupTypeOwner(type.path,path)?.standardSumSourceOrigin:null;
    if(standardSum)return clone(standardSum);
    const fields = type.kind === 'tuple' ? type.elements.map(element => this.typeOrigin(element, path)) : null;
    return retainRustNominalCopyOrigin(this,{ ...value(acceptRustImmutableProductType(type.kind, fields?.every(field => this.immutableProduct(field)) ?? false, false), type), floatSourceModel: type.kind === 'float' ? 'binary64 scalar' : null, path: [...path], fields });
  }

  immutableProduct(origin) {
    return Boolean(origin?.proven && origin.copyEvidence && origin.wrappers.length === 0
      && (!origin.fields || origin.fields.every(field => this.immutableProduct(field))));
  }

  requireProduct(origin, span) {
    if (!this.collecting && (!origin.fields || !acceptRustProductOrigin(this.immutableProduct(origin), this.ownedLocalProduct(origin)))) {
      throw unsupported('immutable Rust tuple product', 'every field requires a direct recursively immutable scalar Copy source value', span);
    }
  }

  product(node, path, span = node.span) {
    const fields = node.items.map(item => this.origin(item, path));
    const type=this.localValueStateModel&&fields.every(field=>field.type)?{kind:'tuple',elements:fields.map(field=>field.type)}:null;
    const origin = { ...value(fields.every(field => this.immutableProduct(field)),type), fields };
    this.requireProduct(origin, span);
    return this.mark(node, origin);
  }

  ownedLocalProduct(origin){return Boolean(this.localValueStateModel&&ownedLocalPair(origin?.type)&&origin.proven&&!origin.wrappers.length&&origin.fields?.every(field=>field.proven&&!field.wrappers.length)&&(origin.fields[0].textSourceModel==='owned string'));}

  project(node, path, index, span = node.span) {
    const origin = this.origin(node.object, path);
    this.requireProduct(origin, span);
    return this.mark(node, projectRustCopyClosureValueField(origin,clone(origin.fields?.[index]??unknown()),index));
  }

  declaration(name, path, entry) {
    if (this.collecting) {
      const key = [...path, name].join('.');
      const existing = this.lookupNamespace(['crate',...path,name],[],'type');
      if (existing && !acceptRustSourceNamespaceDeclarationPair(existing.k,entry.k)) {
        throw unsupported('extern crate alloc', 'a source crate declaration cannot share its module name with another item');
      }
      const values=entry.k==='fn'?[...entry.parameters,entry.result]:entry.k==='ctor'?[...entry.fields,entry.result]:entry.k==='const'?[entry.result]:[];
      values.forEach(origin=>retainRustNominalCopyOrigin(this,origin));
      this.registerCatalogEntry(key, entry);
    }
  }

  resolveTypes(resolve) {
    const resolveOrigin=origin=>{
      if(origin.type?.kind==='named')origin.type={...origin.type,path:resolve(origin.type.path,origin.path)};
      origin.fields?.forEach(resolveOrigin);origin.callableParameters?.forEach(resolveOrigin);if(origin.callableResult)resolveOrigin(origin.callableResult);
    };
    for(const entry of new Set([...this.catalog.values(),...this.typeCatalog.values(),...this.valueCatalog.values()])){
      const origins=entry.k==='fn'?[...entry.parameters,entry.result]:entry.k==='const'?[entry.result]:entry.k==='ctor'?[...entry.fields,entry.result]:[];
      origins.forEach(resolveOrigin);
    }
  }

  registerCatalogEntry(key,entry){
    const role=readRustSourceNamespaceRole(entry.k);
    if(role==='type')this.typeCatalog.set(key,entry);
    if(role==='value')this.valueCatalog.set(key,entry);
    this.catalog.set(key,entry);
  }

  removeCatalogEntry(key){
    const entry=this.catalog.get(key);if(!entry)return;
    const role=readRustSourceNamespaceRole(entry.k);this.catalog.delete(key);
    if(role==='type')this.typeCatalog.delete(key);
    if(role==='value')this.valueCatalog.delete(key);
    const peer=role==='type'?this.valueCatalog.get(key):this.typeCatalog.get(key);
    if(peer)this.catalog.set(key,peer);
  }
  lookupTypeOwner(segments,path){return this.lookupNamespace(segments,path,'type',[]);}
  lookupValueOwner(segments,path){return this.lookupNamespace(segments,path,'value',[]);}
  namespaceEntry(name,role){
    const primary=this.catalog.get(name),indexed=(role==='type'?this.typeCatalog:role==='value'?this.valueCatalog:this.catalog).get(name);
    return primary&&(role==='any'||readRustSourceNamespaceRole(primary.k)===role)?primary:indexed;
  }
  lookup(segments, path, kinds = null) {
    return this.lookupNamespace(segments,path,readRustSourceNamespaceLookupRole(kinds??[]),kinds??[]);
  }

  lookupNamespace(segments,path,role,kinds=[]){
    let names = [...segments];
    let start = [...path];
    let absolute = false;
    if (names[0] === 'crate') { names.shift(); start = []; absolute = true; }
    else if (names[0] === 'self') { names.shift(); absolute = true; }
    else if (names[0] === 'super') {
      while (names[0] === 'super') { names.shift(); start.pop(); }
      absolute = true;
    }
    for (let depth = start.length; depth >= 0; depth -= 1) {
      const fullName = [...start.slice(0, depth), ...names].join('.');
      // Direct legacy catalog writes remain visible; indexed entries preserve
      // an independent namespace when the primary entry belongs to its peer.
      const found=this.namespaceEntry(fullName,role);
      const action=found?readRustSourceNamespaceCandidate(found.k,role,kinds):'continue';
      if(action==='select')return {...found,fullName};
      if(action==='stop')return null;
      const prefix=this.namespaceEntry([...start.slice(0,depth),names[0]].join('.'),'type');
      if(!acceptRustSourceNamespaceOuterCandidate(names.length,prefix?.k??''))return null;
      if (absolute) break;
    }
    return null;
  }

  same(left, right) {
    if(!acceptRustCallableOwnerDescriptorsEqual(projectRustCallableOwnerDescriptor(left),projectRustCallableOwnerDescriptor(right)))return false;
    return this.compareStructure(left,right);
  }

  compareStructure(left,right){
    if(left&&right&&readRustCopyClosureJoinError(Boolean(left.copyClosureSpan),Boolean(right.copyClosureSpan),left.copyClosureSpan?.start===right.copyClosureSpan?.start&&left.copyClosureSpan?.end===right.copyClosureSpan?.end))return false;
    if(left&&right&&!acceptRustStaticByteArrayOwnerEquality(left.arraySourceModel??'',left.sourceOwner?.kind??'',left.sourceOwner?.path.join('::')??'',right.sourceOwner?.kind??'',right.sourceOwner?.path.join('::')??''))return false;
    if (!left || !right || left.proven !== right.proven || left.diverging !== right.diverging || left.uncertain !== right.uncertain || left.copyEvidence !== right.copyEvidence || (left.floatSourceModel ?? null) !== (right.floatSourceModel ?? null) || left.textSourceModel !== right.textSourceModel || left.arraySourceModel !== right.arraySourceModel || JSON.stringify(left.wrappers) !== JSON.stringify(right.wrappers)) return false;
    if (Boolean(left.fields) !== Boolean(right.fields) || (left.fields && (left.fields.length !== right.fields.length || left.fields.some((field, index) => !this.compareStructure(field, right.fields[index]))))) return false;
    if(Boolean(left.callableParameters)!==Boolean(right.callableParameters)||left.callableParameters&&(left.callableParameters.length!==right.callableParameters.length||left.callableParameters.some((parameter,index)=>!this.compareStructure(parameter,right.callableParameters[index]))))return false;
    if(Boolean(left.callableResult)!==Boolean(right.callableResult)||left.callableResult&&!this.compareStructure(left.callableResult,right.callableResult))return false;
    if (left.type?.kind === 'named' && right.type?.kind === 'named') {
      const leftOwner = this.lookupTypeOwner(left.type.path, left.path);
      const rightOwner = this.lookupTypeOwner(right.type.path, right.path);
      if (leftOwner && rightOwner) return leftOwner.fullName === rightOwner.fullName;
      return JSON.stringify(left.type.path) === JSON.stringify(right.type.path) && JSON.stringify(left.path) === JSON.stringify(right.path);
    }
    return JSON.stringify(left.type) === JSON.stringify(right.type);
  }

  box(segments, resolved, path, span) {
    if (this.collecting) return;
    const owner = resolved.join('::');
    const source = this.lookupTypeOwner([segments[0]], path);
    const target = this.lookupTypeOwner([resolved[0]], path);
    const shadowed = Boolean(source && source.k !== 'externalCrate') || Boolean(target && target.k !== 'externalCrate');
    if (!acceptRustSourceBoxOwner(owner, shadowed)) {
      throw unsupported('Box owner', 'only the unshadowed standard immutable Box is portable', span);
    }
    if (!acceptRustSourceBoxAvailability(owner, target?.k === 'externalCrate')) {
      throw unsupported('Box source crate', 'alloc requires an actual extern crate alloc declaration', span);
    }
  }

  vector(segments, resolved, path, span) {
    if (this.collecting) return;
    const shadowed = [segments[0], resolved[0]].some(name => this.lookupTypeOwner([name], path));
    if (!acceptRustSourceArrayOwner(resolved.join('::'), shadowed)) throw unsupported('Vec owner', 'only the unshadowed standard vector is admitted by the immutable array model', span);
  }

  origin(node, path) {
    if (!node) return unknown();
    if (this.expressions.has(node)) return clone(this.expressions.get(node));
    if (node.k === 'name' && node.path.length === 1 && this.scope.has(node.path[0])) {
      return clone(this.scope.get(node.path[0]));
    }
    if (node.k === 'name' || (node.k === 'app' && node.fn.k === 'name')) {
      const entry = this.lookupValueOwner(node.k === 'name' ? node.path : node.fn.path, path);
      if (entry?.k === 'const') return clone(entry.result);
      if (entry?.k === 'fn') return markRustBorrowedVectorCallResult(this,node,path,entry,markRustStandardSumOwnedCallResult(this,node,path,{ ...clone(entry.result), freshArrayValue: acceptRustFreshVectorResultCall(node.k,entry.result.arraySourceModel??'',entry.result.wrappers.length,Boolean(entry.result.freshArrayValue),entry.result.freshVectorResultContract===true,this.freshCalls.has(`${node.span?.start}:${node.span?.end}`)) }));
      if (entry?.k === 'ctor') return clone(entry.result);
      if (node.k === 'name' && node.path.length === 2 && !this.lookupTypeOwner([node.path[0]], path)
        && rustFixedType(node.path[0]) && ['MIN', 'MAX'].includes(node.path[1])) return {...value(true, rustFixedType(node.path[0])),sourceOwner:{kind:'primitive',path:[node.path[0]],arguments:[]}};
    }
    if (node.k === 'if') {
      return this.join([this.origin(node.then, path), this.origin(node.else, path)]);
    }
    if(node.k==='unit')return {...value(true,{kind:'unit'}),sourceOwner:{kind:'unit',path:[],arguments:[]}};
    const form = readRustSourceExpressionOrigin(node.k, node.op ?? '');
    if (form === 'value') return value(['num', 'bool', 'unit'].includes(node.k), node.type ?? (node.k === 'bool' ? { kind: 'bool' } : node.k === 'unit' ? { kind: 'unit' } : null));
    if (form === 'diverging') return { ...value(), diverging: true };
    if (form === 'derived-value') {
      const operands = [node.arg ?? node.object, ...(node.search ? [node.search] : [])].map(operand => this.origin(operand, path));
      if (operands.some(operand => !operand.proven)) return unknown(operands.some(operand => operand.uncertain));
      return node.k==='stringTest'?{...value(true,{kind:'bool'}),sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}}:{...value(false,{kind:'string'}),sourceOwnedResultProof:true};
    }
    if (['numeric-cast', 'primitive-operation', 'comparison', 'boolean-operation'].includes(form)) {
      const operands = (node.arg ? [node.arg] : [node.left, node.right]).map(operand => this.origin(operand, path));
      if(operands.some(operand=>operand.integerSourceOwnerConflict))return {...unknown(true),integerSourceOwnerConflict:true};
      if (!this.collecting) {
        if (operands.some(operand => !operand.proven)) return unknown(operands.some(operand => operand.uncertain || operand.wrappers.length));
        if (form === 'comparison' && JSON.stringify(operands[0].wrappers) !== JSON.stringify(operands[1].wrappers)) {
          const directString = operand => operand.textSourceModel === 'owned string' && operand.wrappers.length === 0;
          const borrowedSlice = operand => operand.textSourceModel === 'string slice' && operand.wrappers.length === 1 && operand.wrappers[0] === 'reference';
          if (!acceptRustSourceTextEquality(node.op, directString(operands[0]), directString(operands[1]), borrowedSlice(operands[0]), borrowedSlice(operands[1]))) return unknown(true);
        }
        if (form === 'primitive-operation' && operands.some(operand => operand.wrappers.includes('box'))) return unknown(true);
        if (['numeric-cast', 'boolean-operation'].includes(form) && operands.some(operand => operand.wrappers.length > 0)) return unknown(true);
      }
      const copyEvidence = form !== 'primitive-operation' || (node.op !== 'concat' && operands.every(operand => operand.copyEvidence));
      const owners=operands.map(operand=>readRustPointerIntegerContractOwner(operand.sourceOwner,operand.integerSourceOwnerConstraint??''));
      if(['primitive-operation','comparison'].includes(form)&&readRustPointerIntegerOwnerJoinError(owners))return {...unknown(true),integerSourceOwnerConflict:true};
      const owner=form==='primitive-operation'?selectRustPointerIntegerResultOwner(owners):'';
      const primitive=form==='primitive-operation'?selectRustPrimitiveExpressionOrigin(node,operands):null;
      const boolean=acceptRustPrimitiveBooleanResult(form,node.op??'',operands.length===1&&operands[0].type?.kind==='bool'&&operands[0].proven&&operands[0].wrappers.length===0);
      const result=value(copyEvidence,node.to??(boolean?{kind:'bool'}:node.op==='concat'?{kind:'string'}:primitive?.type??(owner?operands.find((_,index)=>owners[index]===owner)?.type??null:null)));
      if(boolean)return {...result,sourceOwner:{kind:'primitive',path:['bool'],arguments:[]}};
      if(primitive)return {...result,sourceOwner:primitive.sourceOwner,floatSourceModel:primitive.type.kind==='float'?'binary64 scalar':null};
      return owner?{...result,sourceOwner:{kind:'primitive',path:[owner],arguments:[]}}:result;
    }
    return unknown();
  }

  join(origins) {
    const returning = origins.filter(origin => !origin.diverging);
    if (!returning.length) return { ...value(), diverging: true };
    const owners=returning.map(origin=>readRustPointerIntegerContractOwner(origin.sourceOwner,origin.integerSourceOwnerConstraint??''));
    if(returning.some(origin=>origin.integerSourceOwnerConflict)||readRustPointerIntegerOwnerJoinError(owners))return {...unknown(true),integerSourceOwnerConflict:true};
    const inheritedCallableError=returning.find(origin=>origin.callableSourceOwnerError)?.callableSourceOwnerError;if(inheritedCallableError)return {...unknown(true),callableSourceOwnerError:inheritedCallableError};
    const callableError=readRustCallableOwnerJoinError(returning.flatMap(projectRustCallableOwnerDescriptor));if(callableError)return {...unknown(true),callableSourceOwnerError:callableError};
    const first = returning[0];
    const sourceStandardSumBorrowedField=returning.some(origin=>origin.sourceStandardSumBorrowedField===true);
    const sourceStandardSumPatternMove=returning.some(origin=>origin.sourceStandardSumPatternMove===true);
    const sourceNominalBorrowedField=returning.some(origin=>origin.sourceNominalBorrowedField===true),sourceNominalPatternMove=returning.some(origin=>origin.sourceNominalPatternMove===true);
    const sourceValueBorrowBindings=returning.reduce((all,origin)=>mergeRustOwnedLoanBindings(all,readRustCopyClosureValueBorrows(origin)),[]),sourceValueFieldBorrowBindings=returning.every(origin=>JSON.stringify(origin.sourceValueFieldBorrowBindings??null)===JSON.stringify(first.sourceValueFieldBorrowBindings??null))?first.sourceValueFieldBorrowBindings??null:null;
    for(const origin of returning){const error=readRustCopyClosureJoinError(Boolean(first.copyClosureSpan),Boolean(origin.copyClosureSpan),first.copyClosureSpan?.start===origin.copyClosureSpan?.start&&first.copyClosureSpan?.end===origin.copyClosureSpan?.end);if(error)throw unsupported('Rust closure source join',error);}
    const sourceExtendingTemporaryBindings=returning.reduce((ids,origin)=>mergeRustOwnedLoanBindings(ids,origin.sourceExtendingTemporaryBindings??[]),[]);
    const sourceLoanBindingIdentities=first.wrappers.length?returning.reduce((loans,origin)=>mergeRustOwnedLoanBindings(loans,projectRustOwnedLoanBindings(origin.sourceBindingIdentity??'',origin.sourceLoanBindingIdentities??[])),[]):[];
    if (returning.every(origin => this.same(origin, first))) return { ...clone(first),sourceLexicalBindingIdentity:null,sourceValueBorrowBindings,sourceValueFieldBorrowBindings,sourceStandardSumBorrowedField,sourceStandardSumPatternMove,sourceNominalBorrowedField,sourceNominalPatternMove,sourceBorrowedPlace:returning.some(origin=>origin.sourceBorrowedPlace===true),sourceExtendingTemporaryBindings,sourceLoanBindingIdentities,sourcePlaceLoanBindingIdentities:[],sourceBindingIdentity:returning.every(origin=>origin.sourceBindingIdentity===first.sourceBindingIdentity)?first.sourceBindingIdentity:null,sourceOwnedResultProof:returning.every(origin=>origin.sourceOwnedResultProof===true), integerSourceOwnerConstraint:selectRustPointerIntegerResultOwner(owners),freshArrayValue: returning.every(origin => origin.freshArrayValue),arrayElementCopyEvidence:returning.every(origin=>origin.arrayElementCopyEvidence===true) };
    if (returning.some(origin => origin.arraySourceModel)) return { ...unknown(true), arraySourceModel: 'unproved array',integerSourceOwnerConstraint:selectRustPointerIntegerResultOwner(owners) };
    if (returning.every(origin => origin.proven && JSON.stringify(origin.wrappers) === JSON.stringify(first.wrappers))) {
      const fields = returning.every(origin => origin.fields?.length === first.fields?.length) && first.fields
        ? first.fields.map((_, index) => this.join(returning.map(origin => origin.fields[index]))) : null;
      const textSourceModel = returning.every(origin => origin.textSourceModel === first.textSourceModel) ? first.textSourceModel : null;
      const integerSourceOwnerConstraint=selectRustPointerIntegerResultOwner(owners);
      return { ...value(returning.every(origin => origin.copyEvidence), textSourceModel ? { kind: 'string' } : integerSourceOwnerConstraint?rustFixedType(integerSourceOwnerConstraint):null),sourceValueBorrowBindings,sourceValueFieldBorrowBindings, wrappers: [...first.wrappers], fields, textSourceModel,integerSourceOwnerConstraint,sourceStandardSumBorrowedField,sourceStandardSumPatternMove,sourceNominalBorrowedField,sourceNominalPatternMove,sourceExtendingTemporaryBindings,sourceLoanBindingIdentities,sourcePlaceLoanBindingIdentities:[] };
    }
    return unknown(returning.some(origin => origin.uncertain || origin.wrappers.length > 0));
  }

  mark(node, origin) {
    if (this.arrayResultModel && node.k === 'array') origin = { ...origin,
      freshArrayValue: origin.arraySourceModel === 'owned vector' && origin.wrappers.length === 0
        && node.items.every(item => !item.spread && this.immutableProduct(this.origin(item.value, origin.path))) };
    if(origin.wrappers.length===0&&origin.type?.kind!=='array'&&!isRustOwnedBindingValue(this,origin))origin={...origin,sourcePlaceBindingIdentity:origin.sourcePlaceBindingIdentity??origin.sourceBindingIdentity??null,sourceBindingIdentity:null,sourcePlaceLoanBindingIdentities:mergeRustOwnedLoanBindings(origin.sourcePlaceLoanBindingIdentities??[],origin.sourceLoanBindingIdentities??[]),sourceLoanBindingIdentities:[]};
    this.expressions.set(node, clone(origin));
    return node;
  }
  // Pure source alpha-renaming/instance copies preserve the same expression
  // witness. Cache entries are immutable; origin() returns an independent
  // clone before any caller can alter facts. This allocates no type tree and
  // never manufactures a producer or binding identity from a type contract.
  reuseExpressionOrigin(from,to){const origin=this.expressions.get(from);if(origin)this.expressions.set(to,origin);reuseRustSourceExpressionType(this,from,to);return to;}
  // Read-only host inspection for type selection. Callers never alter this
  // retained witness; public origin() remains the independent mutable view.
  recordedExpressionOrigin(node){return this.expressions.get(node);}
  recordExpressionType(node,path){return recordRustSourceExpressionType(this,node,path);}
  recordedExpressionType(node){return readRustSourceExpressionType(this,node);}

  // The parser retains constructor evidence while each lexical scope is live.
  // A named vector is never a constructor, even if its binding was fresh.
  freshArrayResult(node, path) {
    const children = node.k === 'let' ? this.freshArrayResult(node.body, path)
      : node.k === 'if' ? this.freshArrayResult(node.then, path) && this.freshArrayResult(node.else, path) : false;
    return acceptRustFreshArrayResultTail(node.k, Boolean(this.origin(node, path).freshArrayValue), children);
  }

  // Independent of the explicit array-result profile and its binding evidence.
  // Source intervals survive the parser's block and completion reconstruction.
  markOwnedStringConstructor(node,origin){return this.mark(node,{...origin,sourceLexicalBindingIdentity:null,sourceValueBorrowBindings:[],sourceValueFieldBorrowBindings:null,sourceStandardSumBorrowedField:false,sourceStandardSumPatternMove:false,sourceNominalBorrowedField:false,sourceNominalPatternMove:false,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false,sourceOwnedResultProof:acceptRustOwnedBindingValue(origin.type?.kind??'',origin.textSourceModel??'',origin.wrappers.length,origin.proven===true)});}

  markVectorConstructor(node,origin) {
    if(isRustBorrowedVectorSourceOrigin(this,origin)){captureRustBorrowedVectorConstructor(this,node,origin);return node;}
    if(this.ownedContainerSourceEnabled)return captureRustOwnedVectorConstructor(this,node,origin);
    const values=node.k==='array'?node.items.map(item=>this.origin(item.value,origin.path)):[];
    const owner=origin.expandedSourceOwner??origin.sourceOwner,element=origin.type?.kind==='array'?this.typeOrigin(origin.type.element,origin.path):null;if(element&&owner?.kind==='library'&&owner.path.join('::')==='std::vec::Vec'&&owner.arguments.length===1)element.sourceOwner=structuredClone(owner.arguments[0]);
    if(element&&!acceptRustSourceArrayElement(element.type.kind,false)&&!acceptRustNominalVectorElement(this,element)||values.some(value=>(value.type||value.fields||value.wrappers.length)&&!acceptRustSourceArrayElement(value.type?.kind??'tuple',value.wrappers.length>0)&&!acceptRustNominalVectorElement(this,value)))throw unsupported('Rust array element','only direct immutable scalar, owned string or verified nominal Copy vector elements are admitted',node.span);
    this.freshVectorConstructors.add(`${node.span?.start}:${node.span?.end}`);
    return this.mark(node,{...origin,sourceLexicalBindingIdentity:null,sourceValueBorrowBindings:[],sourceValueFieldBorrowBindings:null,sourceStandardSumBorrowedField:false,sourceStandardSumPatternMove:false,sourceNominalBorrowedField:false,sourceNominalPatternMove:false,sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLoanBindingIdentities:[],sourcePlaceLoanBindingIdentities:[],sourceExtendingTemporaryBindings:[],sourceBorrowedPlace:false});
  }

  freshVectorArgument(node,path,allowCalls=true) {
    const origin=this.origin(node,path);
    if(isRustBorrowedVectorProducedValue(this,origin))return true;
    if(isRustOwnedVectorProducedValue(this,origin))return true;
    if(origin.type?.kind==='array'&&origin.sourceOwnedResultProof===true&&this.availableOwnedBinding(origin))return true;
    if(acceptRustScalarVectorCopyOccurrence(node.k,origin.arraySourceModel??'',origin.wrappers.length,this.freshVectorConstructors.has(`${node.span?.start}:${node.span?.end}`)))return true;
    if(allowCalls&&acceptRustKnownFreshVectorCall(node.k,origin.arraySourceModel??'',origin.wrappers.length,Boolean(origin.freshArrayValue)))return true;
    const form=readRustFreshVectorArgumentForm(node.k,origin.diverging===true,origin.arraySourceModel??'',origin.wrappers.length,this.freshVectorConstructors.has(`${node.span?.start}:${node.span?.end}`));
    if(form==='diverging'||form==='constructor')return true;
    if(form==='body')return this.freshVectorArgument(node.body,path,allowCalls);
    if(form==='alternatives')return this.freshVectorArgument(node.then,path,allowCalls)&&this.freshVectorArgument(node.else,path,allowCalls);
    if(form==='rows')return node.rows.length>0&&node.rows.every(row=>this.freshVectorArgument(row.body,path,allowCalls));
    return false;
  }

  wrap(node, path, layer) {
    let origin = this.origin(node, path);
    if(testRustSourceReferenceLifetimeLayer(layer)&&origin.sourceLexicalBindingIdentity!=null)origin.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(origin),[origin.sourceLexicalBindingIdentity]);
    if(testRustSourceReferenceLifetimeLayer(layer)&&isRustOwnedBindingValue(this,origin)){origin=captureRustOwnedBorrow(this,origin);if(!((origin.sourceStandardSumBorrowedField||origin.sourceNominalBorrowedField)&&origin.sourceBorrowedPlace)&&this.sourceTemporaryBindings.has(origin.sourceBindingIdentity))origin.sourceExtendingTemporaryBindings=mergeRustOwnedLoanBindings(origin.sourceExtendingTemporaryBindings??[],[origin.sourceBindingIdentity]);}
    if(testRustSourceReferenceLifetimeLayer(layer)){if(origin.sourcePlaceBindingIdentity!=null)origin.sourceBindingIdentity=origin.sourcePlaceBindingIdentity;origin.sourceLoanBindingIdentities=mergeRustOwnedLoanBindings(origin.sourceLoanBindingIdentities??[],origin.sourcePlaceLoanBindingIdentities??[]);}
    if(testRustSourceReferenceLifetimeLayer(layer))origin.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(origin),mergeRustOwnedLoanBindings(projectRustOwnedLoanBindings(origin.sourceBindingIdentity??'',origin.sourceLoanBindingIdentities??[]),projectRustOwnedLoanBindings(origin.sourcePlaceBindingIdentity??'',origin.sourcePlaceLoanBindingIdentities??[])));
    origin.wrappers.unshift(layer);if(testRustSourceReferenceLifetimeLayer(layer))origin.sourceLifetimes=[null,...(origin.sourceLifetimes??[])];
    return this.mark(node, origin);
  }

  dereference(node, path, span) {
    const boxed=finishRustOwnedBoxDereference(this,node,path,span);if(boxed)return boxed;
    const origin = this.origin(node, path);
    if (!this.collecting && origin.arraySourceModel && origin.wrappers.length === 1) throw unsupported('array dereference', 'moving an owned vector or an unsized slice needs a separate source ownership proof', span);
    if (!this.collecting && (!origin.proven || !acceptRustImmutableIndirectionLayer(origin.wrappers[0] ?? ''))) {
      throw unsupported('dereference', 'the operand must have a proven immutable reference or standard Box source type', span);
    }
    const layer=origin.wrappers.shift();if(testRustSourceReferenceLifetimeLayer(layer??'')){origin.sourceLifetimes?.shift();origin.sourceBorrowedPlace=true;}
    validateRustCopyClosureValueBorrows(this,origin,span);const projected=finishRustStandardSumFieldDereference(this,origin,span);
    return this.mark(node, projected.wrappers.length===0&&this.immutableProduct(projected)?carryRustCopyClosureValueBorrows(projected,projected):projected);
  }

  bind(name, node, path, declared = null) {
    const actual = this.origin(node, path);
    if(actual.sourceBoxLayers?.length)reserveRustOwnedContainerCopies(this,[actual]);
    this.pointerIntegerArgument(actual,value(),node.span);
    if(this.sharedArrayHandleSourceModel&&!this.collecting&&actual.type?.kind==='array'&&!acceptRustSharedArrayHandleTransfer(actual.arraySourceModel,actual.arraySourceModel,actual.wrappers.length,actual.wrappers.length))throw unsupported('Rust shared array handle source','a binding requires an actual owned shared handle, not its vector payload',node.span);
    if (!this.collecting && actual.arraySourceModel && actual.uncertain) throw unsupported('source array join', 'every array alternative needs the same proven source type and ownership', node.span);
    if (!this.collecting && actual.arraySourceModel === 'owned vector' && actual.wrappers.length === 0 && node.k !== 'array' && !actual.freshArrayValue&&!this.availableOwnedBinding(actual)&&!isRustOwnedVectorProducedValue(this,actual)&&!isRustBorrowedVectorProducedValue(this,actual)) throw unsupported('owned vector binding', 'moving a local vector needs an explicit consumption proof', node.span);
    if (declared) this.argument(actual, declared, node.span);
    if(isRustOwnedBindingValue(this,actual))this.consumeOwnedBinding(actual,node.span);
    const binding={...clone(declared??actual),sourceBoxLayers:structuredClone(actual.sourceBoxLayers??[]),...(actual.wrappers.length?{sourceBindingIdentity:actual.sourceBindingIdentity,sourceLoanBindingIdentities:actual.sourceLoanBindingIdentities??[]}:{}),...(this.localValueStateModel?{localValueCopy:false}:{}),freshArrayValue:false,freshVectorResultContract:false,sourceExtendingTemporaryBindings:[],sourcePlaceBindingIdentity:null,sourcePlaceLoanBindingIdentities:[]};
    delete binding.sourceRecordStateCollectingSite;
    this.scope.set(name,assignRustCopyClosureLexicalBinding(this,carryRustCopyClosureValueBorrows(registerRustOwnedBinding(this,binding),actual)));
  }

  pointerIntegerArgument(actualOrigin,expectedOrigin,span){
    if(this.collecting||actualOrigin.diverging)return;
    if(actualOrigin.callableSourceOwnerError)throw unsupported('Rust callable source owner',actualOrigin.callableSourceOwnerError,span);
    const actualOwner=readRustPointerIntegerContractOwner(actualOrigin.sourceOwner,actualOrigin.integerSourceOwnerConstraint??'');
    const expectedOwner=readRustPointerIntegerContractOwner(expectedOrigin.sourceOwner,expectedOrigin.integerSourceOwnerConstraint??'');
    const ownerError=readRustPointerIntegerOwnerError(actualOrigin.integerSourceOwnerConflict?'usize':actualOwner,actualOrigin.integerSourceOwnerConflict?'u64':expectedOwner);if(ownerError)throw unsupported('Rust integer source owner',ownerError,span);
  }
  pointerIntegerExpression(node,path){
    const origin=this.origin(node,path);this.pointerIntegerArgument(origin,value(),node.span);
    return this.mark(node,origin);
  }
  argument(actualOrigin, expectedOrigin, span,transferred=false) {
    if (this.collecting) return;
    if (actualOrigin.diverging) return;
    validateRustBorrowedVectorArgument(this,actualOrigin,expectedOrigin,span);
    if(acceptRustOwnedArgumentReadCheck(actualOrigin.wrappers.length,transferred))checkRustOwnedOrigin(this,actualOrigin,span);
    this.pointerIntegerArgument(actualOrigin,expectedOrigin,span);
    const callableError=readRustCallableOwnerError(projectRustCallableOwnerDescriptor(actualOrigin),projectRustCallableOwnerDescriptor(expectedOrigin));if(callableError)throw unsupported('Rust callable source owner',callableError,span);
    const pointerError=readRustFunctionPointerCoercionError(expectedOrigin.type?.kind==='function'&&expectedOrigin.copyEvidence===true,actualOrigin.type?.kind??'',Boolean(actualOrigin.copyClosureSpan),actualOrigin.copyClosureCaptureCount??-1,actualOrigin.proven);if(pointerError)throw unsupported('Rust function pointer coercion',pointerError,span);
    if(this.sharedArrayHandleSourceModel&&(actualOrigin.type?.kind==='array'||expectedOrigin.type?.kind==='array')&&!acceptRustSharedArrayHandleTransfer(actualOrigin.arraySourceModel,expectedOrigin.arraySourceModel,actualOrigin.wrappers.length,expectedOrigin.wrappers.length))throw unsupported('Rust shared array handle source','call and return ownership must preserve actual shared handles without borrowed payload coercion',span);
    if (!actualOrigin.proven && !actualOrigin.uncertain && !actualOrigin.wrappers.length && !expectedOrigin.wrappers.length) return;
    if (!actualOrigin.proven || !expectedOrigin.proven) throw unsupported('source indirection', 'the source ownership of this value cannot be proved', span);
    const actual = actualOrigin.wrappers;
    const want = expectedOrigin.wrappers;
    const indirected = actual.length > 0 || want.length > 0;
    const exact = JSON.stringify(actual) === JSON.stringify(want);
    const borrowed = actual[0] === 'reference' && want[0] === 'reference';
    if (actualOrigin.arraySourceModel && expectedOrigin.arraySourceModel && !acceptRustSourceArrayCoercion(actualOrigin.arraySourceModel, expectedOrigin.arraySourceModel, borrowed)&&!acceptRustStaticByteArrayCoercion(actualOrigin.arraySourceModel,expectedOrigin.arraySourceModel,borrowed)) throw unsupported('source array coercion', 'an immutable slice cannot become a reference to an owned vector', span);
    const removed = actual.length - want.length;
    let initial = true;
    for (const layer of actual.slice(1, removed + 1)) {
      if (!acceptRustSourceBoxCoercionLayer(layer, initial)) initial = false;
    }
    const projected = actual.length ? [actual[0], ...actual.slice(removed + 1)] : [];
    const coerced = removed > 0 && initial && JSON.stringify(projected) === JSON.stringify(want);
    if (!acceptRustSourceIndirectionArgument(indirected, exact, borrowed, coerced)) {
      throw unsupported('source indirection', 'source ownership requires an explicit dereference or matching immutable reference or Box type', span);
    }
  }

  direct(node, path, span) {
    const expected = value();
    this.argument(this.origin(node, path), expected, span);
  }

  conversion(node, path) {
    const operand = this.origin(node.arg, path);
    checkRustOwnedOrigin(this,operand,node.span);if(operand.type?.kind==='string')this.consumeOwnedBinding(operand,node.span);
    const form = readRustSourceExpressionOrigin(node.k, 'from');
    if (!this.collecting && form === 'unboxed-conversion' && operand.wrappers.includes('box')) return this.mark(node, unknown(true));
    if(!this.collecting){const error=readRustOwnedStringConstructionError(operand.type?.kind??'',operand.textSourceModel??'',operand.wrappers.length,operand.proven,operand.wrappers.every(layer=>layer==='reference'));if(error)throw unsupported('Rust string construction',error,node.span);}
    return this.markOwnedStringConstructor(node, operand.proven ? value(false, { kind: 'string' }) : unknown(operand.uncertain));
  }

  // Keep Box ownership contracts until source admission has checked a call;
  // ordinary value type unification remains exclusively in the checker.
  call(segments, args, path, span,ordinary=false) {
    this.freshCalls.add(`${span?.start}:${span?.end}`);
    if (this.collecting) return;
    const entry = this.lookupValueOwner(segments, path);
    const expected = entry?.k === 'fn' ? entry.parameters : entry?.k === 'ctor' ? entry.fields : null;
    if (!expected || args.length !== expected.length) return;
    if(entry.k==='fn')validateRustBorrowedVectorCallRegions(this,args,path,entry.parameters,entry.result,span);
    for (let index = 0; index < args.length; index += 1) {
      const parameter=expected[index];
      const actual=this.origin(args[index],path),transferred=this.ownedArgumentTransfers.has(args[index]);if(!transferred)checkRustOwnedOrigin(this,actual,span);const consumable=transferred||this.availableOwnedBinding(actual)||isRustOwnedVectorProducedValue(this,actual)||isRustBorrowedVectorProducedValue(this,actual);
      const fresh=!parameter.arrayElementPosition&&parameter.arraySourceModel==='owned vector'&&parameter.wrappers.length===0&&acceptRustFreshOwnedVectorArgument(ordinary,parameter.arraySourceModel,parameter.wrappers.length,parameter.type?.kind==='array'?parameter.type.element.kind:'',this.freshVectorArgument(args[index],path));
      if (parameter.arraySourceModel === 'owned vector' && parameter.wrappers.length === 0
        && !fresh&&!consumable) throw unsupported('owned vector argument', 'passing a vector by value needs an explicit consumption proof', span);
      // Immutable reference coercions may dereference Box behind a reference;
      // an owned Box never coerces into its owned inner value.
      this.argument(fresh?{...actual,proven:true,uncertain:false,wrappers:[],arraySourceModel:'owned vector'}:actual, expected[index], span,transferred);
      if(!transferred&&isRustOwnedBindingValue(this,parameter))this.consumeOwnedBinding(actual,span);
    }
  }

  aliasLiteralPattern(pattern,node){if(!acceptRustSourceLiteralPatternAlias(pattern.k)&&!acceptRustSourceNumericPatternAlias(pattern.k))return;const aliases=this.literalPatternAliases.get(pattern)??[];aliases.push(node);this.literalPatternAliases.set(pattern,aliases);}

  referencePattern(pattern) {
    this.patternReferences.set(pattern, (this.patternReferences.get(pattern) ?? 0) + 1);
    return pattern;
  }

  pattern(pattern, origin, path, result = new Map(), sourceSpan = pattern.span) {
    this.pointerIntegerArgument(origin,value(),sourceSpan);
    origin = clone(origin);
    for (let count = this.patternReferences.get(pattern) ?? 0; count > 0; count -= 1) {
      if (origin.wrappers[0] !== 'reference') {
        if (!this.collecting && origin.proven) throw unsupported('source pattern', 'a reference pattern requires a proven reference source type', sourceSpan);
        origin = unknown(true);
      }
      else {origin.wrappers.shift();origin.sourceLifetimes?.shift();origin.sourceBorrowedPlace=true;}
    }
    if (!this.collecting && !['wild', 'bind', 'bindOrCtor'].includes(pattern.k) && origin.wrappers.includes('box')) {
      throw unsupported('source pattern', 'an owned Box requires an explicit dereference before a constructor or literal pattern', sourceSpan);
    }
    for(const alias of this.literalPatternAliases.get(pattern)??[])this.mark(alias,origin);
    if(pattern.k==='rustStandardSumSourcePattern'&&!resolveRustStandardSumSourcePattern(this,pattern,origin)){
      if(!this.collecting)throw unsupported('Rust standard sum source pattern','the constructor pattern requires its actual complete scrutinee source owner',sourceSpan);
      // Initial declaration collection can encounter a call before its actual
      // signature. Provisional bindings retain unknown origins; later complete
      // passes must resolve the original marker before semantic admission.
      this.standardSumSourceNeedsFinalAdmission=true;
      for(const argument of pattern.args)this.pattern(argument,unknown(),path,result,argument.span??sourceSpan);
      return result;
    }
    if(pattern.k==='typedNumLit'){const actual=readRustPointerIntegerContractOwner(origin.expandedSourceOwner??origin.sourceOwner,origin.integerSourceOwnerConstraint??''),error=readRustSourceNumericPatternOwnerError(actual,pattern.owner,Boolean(origin.type),this.collecting);if(error)throw unsupported('Rust numeric pattern',error,sourceSpan);}
    if(pattern.k==='strLit'){const owner=origin.sourceOwner,error=readRustSourceStringPatternError(origin.type?.kind??'',origin.textSourceModel??'',origin.proven,origin.wrappers.length,origin.wrappers.every(layer=>layer==='reference'),owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,this.collecting);if(error)throw unsupported('Rust string pattern',error,sourceSpan);}
    if(pattern.k==='integerRange'){
      for(const endpoint of [pattern.lower,pattern.upper])if(endpoint)this.pattern(endpoint,origin,path,result,endpoint.span??sourceSpan);
    } else if(pattern.k==='alias'){
      if(result.has(pattern.name))throw unsupported('duplicate source pattern binding','each pattern binds a source name at most once',sourceSpan);
      result.set(pattern.name,origin);this.pattern(pattern.pattern,origin,path,result,pattern.pattern.span??sourceSpan);
    } else if (pattern.k === 'bindOrCtor' || pattern.k === 'bind') {
      if (!this.lookupValueOwner([pattern.name], path)?.fields) {
        if(result.has(pattern.name))throw unsupported('duplicate source pattern binding','each pattern binds a source name at most once',sourceSpan);
        result.set(pattern.name, origin);
      }
    } else if (pattern.k === 'tuple') {
      const fields=origin.fields??[],plan=planTuplePatternFields(fields.length,pattern.args.length,pattern.rest??-1);
      const borrowed=origin.wrappers.length>0&&origin.wrappers.every(layer=>layer==='reference');
      const actual=new Map(plan[0]===0?plan.slice(1).map((index,position)=>[index,position]):[]);
      for(const[index,item]of pattern.args.entries()){
        const position=actual.get(index),field=position===undefined?unknown():projectRustCopyClosureValueField(origin,clone(fields[position]),position);
        if(borrowed&&position!==undefined){field.wrappers.unshift('reference');field.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(field),readRustCopyClosureValueBorrows(origin));}
        this.pattern(item,field,path,result,item.span??sourceSpan);
      }
    } else if (pattern.k === 'ctor') {
      const entry = this.lookupValueOwner(pattern.path, path);
      const typeEntry = origin.type?.kind === 'named' ? this.lookupTypeOwner(origin.type.path, origin.path) : null;
      const borrowed = origin.wrappers.length > 0 && origin.wrappers.every(layer => layer === 'reference');
      const admissible = entry?.k === 'ctor' && typeEntry?.k === 'data' && entry.owner === typeEntry.fullName
        && (origin.wrappers.length === 0 || borrowed);
      for (let index = 0; index < pattern.args.length; index += 1) {
        let field = admissible && entry.fields[index] ? clone(entry.fields[index]) : unknown();
        if(admissible&&typeEntry.standardSumSourceOrigin)field=projectRustStandardSumPatternField(this,origin,field,borrowed);
        else if(admissible&&['copy','owned'].includes(readRustNominalOwnershipFacts(this,{...origin,wrappers:[]}).profile))field=projectRustNominalPatternField(this,origin,field,index,borrowed);
        else if(admissible){field=projectRustCopyClosureValueField(origin,field,index);if(borrowed){field.wrappers.unshift('reference');field.sourceValueBorrowBindings=mergeRustOwnedLoanBindings(readRustCopyClosureValueBorrows(field),readRustCopyClosureValueBorrows(origin));}}
        this.pattern(pattern.args[index], field, path, result, pattern.args[index].span ?? sourceSpan);
      }
    }
    return result;
  }

  cases(patterns, scrutinee, path, deferredGuard=false) {
    const maps = patterns.map(pattern => this.pattern(pattern, this.origin(scrutinee, path), path, new Map(), pattern.span ?? scrutinee.span));
    const scope = new Map(this.scope);
    const span = patterns.find(pattern => pattern.span)?.span ?? scrutinee.span;
    if(deferredGuard){
      const aliases=countRustStandardSumGuardAliases(patterns);
      for(const map of maps){const error=readRustStandardSumGuardAliasTransferError(aliases,[...map.values()].filter(origin=>isRustOwnedBindingValue(this,origin)).length);if(error)throw unsupported('Rust owned standard sum guard',error,span);}
    }
    if (!this.collecting && maps.length > 1 && maps.some(map => map.size !== maps[0].size || [...maps[0].keys()].some(name => !map.has(name)))) {
      throw unsupported('or-pattern ownership', 'every alternative must bind the same names with the same source ownership', span);
    }
    const nominalPrepared=prepareRustNominalPatternTransfers(this,maps,span,deferredGuard);
    for (const [name, origin] of maps[0]) {
      const candidates = maps.map(map => map.get(name));
      if (!this.collecting && maps.length > 1 && (maps.some(map => map.size !== maps[0].size)
        || candidates.some(candidate => !candidate || !candidate.proven || JSON.stringify(candidate.wrappers) !== JSON.stringify(origin.wrappers)))) {
        throw unsupported('or-pattern ownership', 'every alternative must bind the same names with the same source ownership', span);
      }
      if(deferredGuard){
        const identities=candidates.map(candidate=>candidate?readRustStandardSumSourceIdentity({origins:this},candidate):'');
        const error=readRustStandardSumGuardAlternativeBindingError(identities.every(Boolean),identities.every(identity=>identity===identities[0]),candidates.every(candidate=>candidate&&sameRustStandardSumGuardLineage(origin,candidate)));
        if(error)throw unsupported('Rust owned standard sum guard',error,span);
      }
      const binding=this.join(candidates.filter(Boolean));binding.sourceExtendingTemporaryBindings=[];delete binding.sourceRecordStateCollectingSite;
      const disposition=readRustStandardSumGuardBindingDisposition(deferredGuard,isRustOwnedBindingValue(this,binding));
      if(disposition==='readonly'){binding.sourceBorrowedPlace=true;scope.set(name,assignRustCopyClosureLexicalBinding(this,binding));}
      else if(disposition==='transfer'){if(!nominalPrepared.has(name)){requireRustStandardSumPatternTransfer(this,binding,span);consumeRustOwnedOrigin(this,binding,span);}scope.set(name,assignRustCopyClosureLexicalBinding(this,registerRustOwnedBinding(this,binding)));}
      else scope.set(name,assignRustCopyClosureLexicalBinding(this,binding));
    }
    return scope;
  }
}
