import {planRustFormattingValue,planRustFormattingOperandCapture} from './rust-formatting-rules.js';
import {acceptRustSourceIdentifierName,normalizeRustSourceIdentifier} from './unicode-identifiers.js';
import {readRustOrdinaryStatementDisposition,readRustImplicitBlockStatementDisposition,readRustUnitStatementDisposition,findRustTerminalReturnBoundary,readRustFunctionMacroDisposition,acceptRustOrdinaryCompletionProfiles,readRustFunctionCompletionPresence} from './rust-unit-value-rules.js';
import {readRustOrdinaryFunctionCompletion,installRustOrdinaryCompletionDeclarations} from './rust-ordinary-completions.js';
import {acceptRustSharedArrayHandleSourceInterpretation as acceptRustSharedArrayHandleSourceModel} from './rust-shared-array-handle-rules.js';
import {requireRustSharedArrayHandleSourceText,verifyRustSharedArrayHandleTokens,createRustSharedArrayHandleState,readRustSharedArrayHandleType,
  isRustSharedArrayHandleOrigin,refuseRustSharedArrayBareVector,noteRustSharedArrayHandleName,
  noteRustSharedArrayHandleBorrow,readRustSharedArrayHandleMethod,readRustSharedArrayHandleConstructor,
  noteRustSharedArrayLiteral,readRustSharedArrayHandleCall,finishRustSharedArrayHandleSource,refuseRustSharedArrayHandleValueComparison,refuseRustSharedArrayHandleObservation} from './rust-shared-array-handles.js';
import {acceptRustRuntimeShellModel} from './rust-runtime-shell-rules.js';
import { acceptRustPrimitiveBitwisePrecedence, readRustPrimitiveBitwiseOperation } from './rust-primitive-bitwise-rules.js';
import { readRustShiftSourceOperation } from './rust-shift-rules.js';
import { readRustRecordItem, readRustRecordConstruction, readRustRecordProjection, readRustRecordValuePath, readRustNamedVariantFields, readRustRecordPattern, readRustNominalBindingPattern, rustNominalBindingNames, lowerRustNominalBinding, expandRustConstructorRest, readRustLetElsePattern } from './rust-records.js';
import { planTuplePatternFields, readRustLetElseInitializerError } from './rust-record-rules.js';
import { readRustConditionalPattern } from './rust-pattern-conditions.js';
import {hasRustConditionalChain,readRustConditionalChain,lowerRustGuardedMatch} from './rust-conditional-chains.js';
import { readRustInherentItem, readRustInherentReceiver, readRustInherentSelfType, resolveRustInherentPath, readRustInherentMethod, splitRustReferenceToken } from './rust-inherent.js';
import {readRustRuntimeShellSource,refuseRustRuntimeShellComposition} from './rust-runtime-shell.js';
import {acceptRustPureUnitCallModel} from './rust-pure-unit-call-rules.js';
import {readRustPureUnitFunction,readRustPureUnitMain,finishRustPureUnitSource,readRustPureUnitItemContext} from './rust-pure-unit-calls.js';
import {acceptRustEmptyOptionSourceModel} from './rust-empty-option-rules.js';
import {readRustEmptyOptionResult,readRustEmptyOptionBody,createRustEmptyOptionProof,finishRustEmptyOptionSource} from './rust-empty-options.js';
import { acceptRustFloatPredicateMethod, acceptRustFloatOperationMethod, acceptRustFloatConstantName, acceptRustFloatConstantOwner, acceptRustFloatPredicateReceiver } from './rust-float-predicate-rules.js';
import { createRustFloatPredicateExpression, createRustFloatOperationExpression, createRustFloatConstantExpression } from './rust-float-predicates.js';
import { acceptRustCompletionModel } from './rust-completion-rules.js';
import { readRustFunctionCompletion } from './rust-completions.js';
import { readRustCompletionPanicMessage } from './rust-completion-panic.js';
import { acceptRustSourceSignatureModel } from './rust-source-signature-rules.js';
import { readRustSourceTypeHead, readRustSourceTypeOwner, createRustSourceSignature, RUST_SOURCE_SIGNATURE_POLICY } from './rust-source-signatures.js';
import { acceptRustTypeNamespaceModel, acceptRustTypeNamespaceDeclaration, acceptRustTypeNamespaceBodyConstructor, acceptRustTypeNamespaceBodyMethod, acceptRustTypeNamespaceMethodDiscovery, acceptRustTypeNamespaceDiscoveryType } from './rust-type-namespace-rules.js';
import { captureRustUnitCloneDerivation, acceptRustUnitCloneReceiver } from './rust-unit-clone.js';
import { createRustTypeNamespaceProof, requireRustTypeNamespaceWithoutImports, requireRustTypeNamespaceAnchor } from './rust-type-namespace.js';
import { readRustInclusiveRangeQuery, readRustStringEmptyQuery, readRustEmptyStringConstructor } from './rust-read-only-queries.js';
import { acceptRustReadOnlyQueryOwner } from './rust-read-only-query-rules.js';
import { readStringMapOperation, readStringTestOperation } from './frontend-rules.js';
// Rust frontend for the portable core. It reads the subset of Rust the core
// can represent faithfully: modules with `use` of crate items, enums with
// unit and tuple variants (`Box<T>` fields are the recursive occurrences of
// a value type), free functions over machine integers, `bool` and `String`,
// expression bodies with `let`, `if`, `match` and `format!`, and a `main`
// made of `println!`, `let` and `assert!`/`assert_eq!` statements.
// Everything else is rejected with a precise obligation.

import { rustMacroMessage } from './aborts.js';
import { TranslationError, unsupported } from './diagnostics.js';
import { TokenCursor, describe, tokenize } from './lexer.js';
import { BOOL, FLOAT, STRING, UNIT, array, rustFixedType, tuple } from './types.js';
import { RustSourceOrigins } from './rust-source-origins.js';
import { acceptRustSourceBoxOwner, acceptRustSourceLifetimeBinding, acceptRustSourceLifetimeReference, acceptRustSourceLifetimeResult } from './rust-indirection-rules.js';
import { acceptRustProductOrigin, acceptRustTupleIndexSpelling } from './rust-product-rules.js';
import { acceptRustSourceArrayOwner, acceptRustSourceArrayElement, acceptRustSourceArrayResultModel, acceptRustSourceArrayResult } from './rust-array-rules.js';
import { acceptRustUnwrittenScalarParameter } from './rust-scalar-parameter-rules.js';
import { readRustScalarParameterBody } from './rust-scalar-parameters.js';
import { acceptRustScalarLoopModel, readRustScalarLoop } from './rust-scalar-loops.js';
import { checkProgram } from './check.js';
import {acceptRustPrimitiveOwnerModel} from './rust-primitive-owner-rules.js';
import {readRustStandardPrimitivePosition,createRustPrimitiveOwnerProof,acceptRustPrimitiveOwnerImportName} from './rust-primitive-owners.js';
import {acceptRustOwnedLocalStateModel,acceptRustOwnedLocalSourceOperation} from './rust-owned-local-state-rules.js';
import {readRustOwnedLocalStateBody} from './rust-owned-local-state-reader.js';
import {acceptRustOwnedTupleCallModel} from './rust-owned-tuple-call-rules.js';
import {RUST_OWNED_LOCAL_STATE_POLICY} from './rust-owned-local-state.js';
import {acceptRustConstFunctionModel} from './rust-const-function-rules.js';
import {createRustConstFunctionProof} from './rust-const-functions.js';
import {planRustDiagnosticAttribute} from './rust-diagnostic-attribute-rules.js';
import {createRustDiagnosticAttributeFacet} from './rust-diagnostic-attributes.js';
import { acceptRustCastFollowingToken, readRustCastSyntaxMessage } from './rust-cast-syntax-rules.js';
import { readRustCharacterExpressionType } from './rust-character-rules.js';
import { readRustLiteralMatch } from './rust-literal-matches.js';
import { readRustAsciiTextPredicate } from './rust-ascii-text-predicates.js';

const ACCEPTED_DERIVES = new Set(['Clone', 'Copy', 'Debug', 'PartialEq', 'Eq', 'Hash', 'PartialOrd', 'Ord']);
const ACCEPTED_LINTS = new Set(['allow', 'warn', 'deny', 'expect', 'must_use', 'inline']);
const COMPARISON = { '==': 'eq', '!=': 'ne', '<': 'lt', '<=': 'le', '>': 'gt', '>=': 'ge' };
const ADDITIVE = { '+': 'add', '-': 'sub' };
const MULTIPLICATIVE = { '*': 'mul', '/': 'div', '%': 'rem' };
const RESERVED_ITEMS = new Set(['impl', 'trait', 'struct', 'union', 'static', 'type', 'extern', 'macro_rules', 'async', 'unsafe']);

export function parseRust(source, { arrayResultModel = '', completionModel = '', sourceSignatureModel = '', scalarLoopModel = '', primitiveOwnerModel = '', emptyOptionSourceModel = '', typeNamespaceModel = '',pureUnitCallModel='',localValueStateModel='',constFunctionModel='',runtimeShellModel='',sharedArrayHandleSourceModel='' } = {}) {
  if(!acceptRustSharedArrayHandleSourceModel(sharedArrayHandleSourceModel))throw unsupported('Rust shared array handle source model','select the legacy model or standard Rust shared array identity handles 1');
  if(sharedArrayHandleSourceModel&&[arrayResultModel,completionModel,sourceSignatureModel,scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,pureUnitCallModel,localValueStateModel,constFunctionModel,runtimeShellModel].some(Boolean))throw unsupported('Rust shared array handle source model','other source interpretations require independent composition proofs');
  if(!acceptRustConstFunctionModel(constFunctionModel))throw unsupported('Rust const function model','select the legacy model or closed pure Rust const predicates 1');
  if(constFunctionModel&&[arrayResultModel,completionModel,scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,localValueStateModel,pureUnitCallModel,runtimeShellModel].some(Boolean))throw unsupported('Rust const function model','other source interpretations require independent composition proofs');
  if(!acceptRustRuntimeShellModel(runtimeShellModel))throw unsupported('Rust runtime shell model','select the legacy model or closed pure Rust unit runtime shell 1');
  if(runtimeShellModel){if([arrayResultModel,completionModel,sourceSignatureModel,scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,pureUnitCallModel,localValueStateModel,constFunctionModel].some(Boolean))refuseRustRuntimeShellComposition('the whole runtime interpretation requires its independent source profile');return readRustRuntimeShellSource(source);}
  if (!acceptRustTypeNamespaceModel(typeNamespaceModel)) throw unsupported('Rust type namespace model', 'select the legacy model or resolved Rust type namespace 1');
  if(!acceptRustOwnedLocalStateModel(localValueStateModel)&&!acceptRustOwnedTupleCallModel(localValueStateModel))throw unsupported('Rust owned local state model','select the original move profile or canonical String copy profile');
  if(localValueStateModel&&(arrayResultModel||completionModel||scalarLoopModel||primitiveOwnerModel||emptyOptionSourceModel||typeNamespaceModel||pureUnitCallModel))throw unsupported('Rust owned local state model','other source state and completion profiles require independent composition proofs');
  if(!acceptRustPrimitiveOwnerModel(primitiveOwnerModel))throw unsupported('Rust primitive owner model','select the legacy model or standard primitive Rust binary64 owner 1');
  if (!acceptRustScalarLoopModel(scalarLoopModel)) throw unsupported('Rust scalar loop model', 'select the legacy model or closed primitive scalar loop 1');
  if (!acceptRustSourceArrayResultModel(arrayResultModel)) throw unsupported('Rust array result model', 'select the legacy model or immutable fresh scalar vector results 1');
  if (!acceptRustCompletionModel(completionModel)) throw unsupported('Rust completion model', 'select the legacy model or closed guarded function returns 1');
  if (!acceptRustSourceSignatureModel(sourceSignatureModel)) throw unsupported('Rust source signature model', 'select the legacy model or Rust source function signature 1');
  if (!acceptRustEmptyOptionSourceModel(emptyOptionSourceModel)) throw unsupported('Rust empty option source model', 'select the legacy model or closed empty primitive Rust option 1');
  if (emptyOptionSourceModel && [arrayResultModel,completionModel,sourceSignatureModel,scalarLoopModel,primitiveOwnerModel,typeNamespaceModel,localValueStateModel].some(Boolean)) throw unsupported('Rust empty option source model', 'the closed empty interpretation requires its independent source profile');
  if(!acceptRustPureUnitCallModel(pureUnitCallModel))throw unsupported('Rust pure Unit-call model','select the legacy model, closed pure Rust unit calls 1 or closed public pure Rust unit calls 1');
  if(pureUnitCallModel&&[arrayResultModel,completionModel,sourceSignatureModel,scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,localValueStateModel].some(Boolean))throw unsupported('Rust pure Unit-call model','the closed Unit-call interpretation requires its independent source profile');
  if(sharedArrayHandleSourceModel)requireRustSharedArrayHandleSourceText(source);
  const { tokens } = tokenize(source, 'Rust');
  return parseRustTokenStream(tokens, { arrayResultModel, completionModel, sourceSignatureModel, scalarLoopModel, primitiveOwnerModel, emptyOptionSourceModel, typeNamespaceModel, pureUnitCallModel, localValueStateModel, constFunctionModel, sharedArrayHandleSourceModel }, source);
}

// Internal complete-token entry; public source parsing validates options and lexes first.
export function parseRustTokenStream(tokens, { arrayResultModel = '', completionModel = '', sourceSignatureModel = '', scalarLoopModel = '', primitiveOwnerModel = '', emptyOptionSourceModel = '', typeNamespaceModel = '', pureUnitCallModel = '', localValueStateModel = '', constFunctionModel = '', sharedArrayHandleSourceModel = '' } = {}, source = null) {
  if(!acceptRustSharedArrayHandleSourceModel(sharedArrayHandleSourceModel))throw unsupported('Rust shared array handle source model','select the installed source model');
  if(sharedArrayHandleSourceModel){
    if([arrayResultModel,completionModel,sourceSignatureModel,scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,pureUnitCallModel,localValueStateModel,constFunctionModel].some(Boolean))throw unsupported('Rust shared array handle source model','source interpretations need independent composition proofs');
    verifyRustSharedArrayHandleTokens(tokens,source);
    // Caller getters or later reads cannot replace an authenticated occurrence.
    tokens=tokenize(source,'Rust').tokens;
  }
  const declarations = new RustParser(tokens, null, arrayResultModel, completionModel, sourceSignatureModel, scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,pureUnitCallModel,localValueStateModel,constFunctionModel,source,sharedArrayHandleSourceModel);
  declarations.file();
  declarations.origins.resolveTypes((segments, path) => declarations.resolve(segments, path));
  const syntax = new RustParser(tokens, declarations.origins.catalog, arrayResultModel, completionModel, sourceSignatureModel, scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,pureUnitCallModel,localValueStateModel,constFunctionModel,source,sharedArrayHandleSourceModel);
  syntax.origins.collecting = true;
  syntax.aliases = declarations.aliases;
  syntax.namedRecords = new Map(declarations.namedRecords);
  syntax.inherentMethods = new Map(declarations.inherentMethods);
  const surface = syntax.file();
  // Preserve syntax trees and the existing checker's diagnostic precedence
  // for programs that already cannot enter the canonical semantic core.
  try { checkProgram(surface); } catch (error) {
    if (!(error instanceof TranslationError)) throw error;
    if(!sharedArrayHandleSourceModel)return surface;
  }
  const parser = new RustParser(tokens, syntax.origins.catalog, arrayResultModel, completionModel, sourceSignatureModel, scalarLoopModel,primitiveOwnerModel,emptyOptionSourceModel,typeNamespaceModel,pureUnitCallModel,localValueStateModel,constFunctionModel,source,sharedArrayHandleSourceModel);
  parser.aliases = syntax.aliases;
  parser.namedRecords = new Map(syntax.namedRecords);
  parser.inherentMethods = new Map(syntax.inherentMethods);
  return parser.file();
}

class RustParser {
  constructor(tokens, catalog = null, arrayResultModel = '', completionModel = '', sourceSignatureModel = '', scalarLoopModel = '',primitiveOwnerModel='',emptyOptionSourceModel='',typeNamespaceModel='',pureUnitCallModel='',localValueStateModel='',constFunctionModel='',source=null,sharedArrayHandleSourceModel='') {
    this.cursor = new TokenCursor([...tokens], 'Rust');
    // `use` aliases, per module path.
    this.aliases = new Map();
    this.modulePath = [];
    this.crateDeclarations = new Set();
    this.origins = new RustSourceOrigins(catalog, arrayResultModel,localValueStateModel,sharedArrayHandleSourceModel);
    this.sharedArrayHandleSourceModel=sharedArrayHandleSourceModel;
    createRustSharedArrayHandleState(this,sharedArrayHandleSourceModel);
    this.localValueStateModel=localValueStateModel;
    this.ownedTupleCallDiscovery=Boolean(acceptRustOwnedTupleCallModel(localValueStateModel)&&catalog===null);
    this.constFunctionModel=constFunctionModel;
    this.source=source;
    this.diagnosticAttributeColumns=null;
    this.completionModel = completionModel;
    this.sourceSignatureModel = sourceSignatureModel || (typeNamespaceModel||constFunctionModel ? 'Rust source function signature 1' : '');
    this.typeNamespaceModel = typeNamespaceModel;
    this.typeNamespaceDiscovery = Boolean(typeNamespaceModel && catalog === null);
    this.typeNamespaceFrames = [];
    this.scalarLoopModel = scalarLoopModel;
    this.emptyOptionSourceModel=emptyOptionSourceModel;
    this.pureUnitCallModel=pureUnitCallModel;
    this.emptyOptionMainSeen=false;
    this.primitiveOwnerModel=primitiveOwnerModel;
    this.primitiveOwnerFrames=[];
    this.primitiveConstantSites=[];
    this.primitiveImportNames=new Set();
    this.sourceOwnerFrames = [];
    this.reservedNames = new Set(tokens.filter(token => token.kind === 'identifier').map(token => token.value));
    this.namedRecords = new Map();
    this.inherentOwner = null;
    this.inherentMethods = new Map();
    this.inherentDiscovery = catalog === null && tokens.some(token => token.kind === 'identifier' && token.value === 'impl');
    this.hasRecordDeclarations = tokens.some(token => token.kind === 'identifier' && token.value === 'struct');
    this.hasRecordConstructions = this.hasRecordDeclarations || tokens.some(token => token.kind === 'identifier' && token.value === 'enum');
    this.recordPatternDepth = 0;
    this.productBindingCount = 0;
    this.ordinaryCompletionDeclarations = [];
    this.lifetimeBindings = new Set();
    this.lifetimeHeader = false;
  }

  fail(message, token = this.cursor.peek()) {
    return new TranslationError('syntax', `${message} but found ${describe(token)}`, token);
  }

  file() {
    const root = { items: [], main: null };
    this.innerAttributes();
    this.items(root, root.items, [], () => this.cursor.atEnd());
    if(this.primitiveOwnerModel&&(root.items.some(item=>item.k!=='fn')||root.main?.effects.length))throw unsupported('Rust primitive owner profile','only flat primitive leaf functions and an empty main are admitted');
    if(this.localValueStateModel&&(!root.items.length||root.items.some(item=>item.k!=='fn'||!Object.hasOwn(item,'sourceLocalState')||RUST_OWNED_LOCAL_STATE_POLICY.reservedOwners.includes(item.name))||root.main?.effects.length))throw unsupported('Rust owned local state','only flat functions with an empty main and unshadowed standard owners are admitted');
    if(this.constFunctionModel&&(!root.items.length||root.items.some(item=>item.k!=='fn'||!Object.hasOwn(item,'sourceConstFunction'))||root.main?.effects.length))throw unsupported('Rust const function source','only closed flat const predicates and an empty ordinary main are admitted');
    if (!root.main&&!this.sharedArrayHandleSourceModel) throw new TranslationError('syntax', 'a Rust program needs fn main', this.cursor.peek());
    installRustOrdinaryCompletionDeclarations(this,root.items);
    finishRustEmptyOptionSource(this,root);
    finishRustPureUnitSource(this,root);
    finishRustSharedArrayHandleSource(this,root);
    return { language: 'Rust', items: root.items, main: root.main,...(this.sharedArrayHandleSourceModel?{referenceSemantics:true}:{}),...(this.pureUnitCallModel?{sourcePureUnitCalls:{model:this.pureUnitCallModel,language:'Rust'}}:{}) };
  }

  innerAttributes() {
    const c = this.cursor;
    if(this.pureUnitCallModel&&c.is('#'))throw unsupported('Rust pure Unit-call source','attributes require a separate compilation context proof',span(c.peek(),c.peek()));
    if(this.constFunctionModel&&c.is('#')&&c.is('!',1))throw unsupported('Rust const function context','crate attributes require an independent compilation interpretation');
    if (this.emptyOptionSourceModel && c.is('#')) throw unsupported('Rust empty option source', 'attributes require a separate compilation context proof', span(c.peek(),c.peek()));
    while (c.is('#') && c.is('!', 1)) {
      const start = c.next();
      c.next();
      this.attributeBody(start);
    }
  }

  attributes(diagnosticAttributes=null) {
    const c = this.cursor;
    const derivations=[];
    if(this.pureUnitCallModel&&c.is('#'))throw unsupported('Rust pure Unit-call source','attributes require a separate compilation context proof',span(c.peek(),c.peek()));
    if (this.emptyOptionSourceModel && c.is('#')) throw unsupported('Rust empty option source', 'attributes require a separate compilation context proof', span(c.peek(),c.peek()));
    while (c.is('#') && c.is('[', 1)) {
      const start = c.next();
      const derivation=this.attributeBody(start,diagnosticAttributes);
      if(derivation)derivations.push(derivation);
    }
    return derivations;
  }

  /** `#[derive(Clone, …)]` and lint attributes do not change behaviour. */
  attributeBody(start,diagnosticAttributes=null) {
    const c = this.cursor;
    if(this.constFunctionModel&&diagnosticAttributes!==null){
      this.diagnosticAttributeColumns??={kinds:c.tokens.map(token=>token.kind),values:c.tokens.map(token=>token.value)};
      const plan=planRustDiagnosticAttribute(this.diagnosticAttributeColumns.kinds,this.diagnosticAttributeColumns.values,c.index-1);
      if(plan[0]>=0){c.index=plan[0];diagnosticAttributes.push(start);return null;}
    }
    c.expect('[', 'attribute');
    const name = c.identifier('attribute').value;
    if(this.constFunctionModel&&(name!=='must_use'||!c.is(']')))throw unsupported('Rust const function attribute','only the actual zero-argument must_use function attribute is admitted');
    const traits=[];
    if (name === 'derive') {
      c.expect('(', 'derive');
      while (!c.is(')')) {
        const trait = c.identifier('derive');
        traits.push(trait.value);
        if (!ACCEPTED_DERIVES.has(trait.value)) throw unsupported(`derive(${trait.value})`, 'only derives that add no behaviour are portable', span(trait, trait));
        if (!c.eat(',')) break;
      }
      c.expect(')', 'derive');
    } else if (ACCEPTED_LINTS.has(name)) {
      let depth = 0;
      while (!c.atEnd() && !(depth === 0 && c.is(']'))) {
        if (c.is('(')) depth += 1;
        if (c.is(')')) depth -= 1;
        c.next();
      }
    } else {
      throw unsupported(`#[${name}]`, 'attributes that change compilation are outside the portable core', span(start, c.peek()));
    }
    const close=c.expect(']', 'attribute');
    return name==='derive'?{name,traits,span:{start:start.start,end:close.end}}:null;
  }

  items(root, items, path, done) {
    const c = this.cursor;
    while (!done()) {
      const firstAttributeIndex=c.index;
      const diagnosticAttributes=[];
      const derivations=this.attributes(diagnosticAttributes);
      const start = c.peek();
      this.modulePath = path;
      readRustPureUnitItemContext(this,path,start);
      if(this.constFunctionModel&&c.is('pub')&&c.is('(',1))throw unsupported('Rust const function visibility','restricted visibility requires an independent module proof');
      this.visibility();
      if(this.typeNamespaceModel&&derivations.some(attribute=>attribute.traits.includes('Clone'))&&!c.is('enum'))throw unsupported('Rust unit Clone derivation','a Clone derive must be attached to an actual datatype',span(start,c.peek()));
      if (this.emptyOptionSourceModel && (path.length || !c.is('fn'))) throw unsupported('Rust empty option source', 'only flat zero-argument empty leaf functions and an empty main are admitted', span(start,c.peek()));
      if(this.constFunctionModel&&!(c.is('const')&&c.is('fn',1)||!path.length&&c.is('fn')&&c.is('main',1)))throw unsupported('Rust const function context','only actual const predicate declarations and the ordinary main are admitted',span(start,c.peek()));
      if(diagnosticAttributes.length&&!(c.is('const')&&c.is('fn',1)))throw unsupported('Rust const diagnostic attribute attachment','the supported diagnostic prefix must belong to an actual const function',span(c.tokens[firstAttributeIndex],c.peek()));
      if(this.constFunctionModel&&c.is('const')&&c.is('fn',1)){
        const constKeywordIndex=c.index;
        const keyword=c.next();
        const declaration=this.fn(path,keyword);
        if(diagnosticAttributes.length)declaration.sourceDiagnosticAttributes=createRustDiagnosticAttributeFacet(this.source,c.tokens,firstAttributeIndex,constKeywordIndex,declaration,path);
        items.push(declaration);
        continue;
      }
      if (c.is('mod')) {
        c.next();
        const name = c.identifier('mod').value;
        if (c.is(';')) throw unsupported('out-of-line module', 'the program must be a single file', span(start, c.peek()));
        c.expect('{', 'mod');
        const module = { k: 'module', name, items: [], span: span(start, c.peek()) };
        this.origins.declaration(name, path, { k: 'module' });
        items.push(module);
        this.items(root, module.items, [...path, name], () => c.is('}'));
        c.expect('}', 'mod');
        continue;
      }
      if (path.length === 0 && c.is('extern') && c.is('crate', 1) && c.is('alloc', 2) && c.is(';', 3)) {
        if(this.typeNamespaceModel)throw unsupported('Rust type namespace imports','external crate declarations require a separate source binding proof',span(start,start));
        if(this.localValueStateModel)throw unsupported('Rust owned local state','external crate declarations require a separate namespace interpretation');
        const key = [...path, 'alloc'].join('.');
        if (this.crateDeclarations.has(key)) throw unsupported('extern crate alloc', 'each source crate declaration must be unique in its module', span(start, start));
        this.crateDeclarations.add(key);
        for (let index = 0; index < 4; index += 1) c.next();
        this.origins.declaration('alloc', path, { k: 'externalCrate' });
        continue;
      }
      if (c.is('use')) {
        this.use(path);
        continue;
      }
      if (c.is('enum')) {
        items.push(this.enumItem(path,derivations));
        continue;
      }
      if (c.is('struct')) {
        items.push(readRustRecordItem(this,path));
        continue;
      }
      if (c.is('impl')) {
        items.push(readRustInherentItem(this,path));
        continue;
      }
      if (c.is('fn') || c.is('const')) {
        if (c.is('fn') && c.is('main', 1) && path.length === 0) {
          if(this.pureUnitCallModel&&this.emptyOptionMainSeen)throw unsupported('Rust pure Unit-call source','the complete source must have one owned main declaration',span(start,c.peek()));
          if(this.constFunctionModel&&this.emptyOptionMainSeen)throw unsupported('Rust const function source','the complete source must have one ordinary main declaration',span(start,c.peek()));
          if(this.emptyOptionSourceModel && this.emptyOptionMainSeen) throw unsupported('Rust empty option source', 'the complete source must have one owned main declaration', span(start,c.peek()));
          this.emptyOptionMainSeen=true;
          root.main = this.main();
          continue;
        }
        items.push(c.is('fn') ? this.fn(path) : this.constItem(path));
        continue;
      }
      if (start.kind === 'identifier' && RESERVED_ITEMS.has(start.value)) {
        throw unsupported(`Rust ${start.value} item`, `${start.value} items are outside the portable core`, span(start, start));
      }
      if (c.atEnd()) throw new TranslationError('syntax', `module ${path.at(-1)} is not closed`, start);
      throw this.fail('expected a Rust item');
    }
  }

  visibility() {
    const c = this.cursor;
    if (!c.eat('pub')) return;
    if (c.is('(')) {
      c.next();
      c.eat('in');
      while (!c.is(')') && !c.atEnd()) c.next();
      c.expect(')', 'visibility');
    }
  }

  /**
   * `use super::Tree;`, `use crate::a::{b, c as d};`: each imported name is
   * an alias for its absolute path inside the importing module.
   */
  use(path) {
    requireRustTypeNamespaceWithoutImports(this.typeNamespaceModel);
    if(this.localValueStateModel)throw unsupported('Rust owned local state','imports require a separate actual owner and namespace interpretation');
    const c = this.cursor;
    const start = c.next();
    const key = path.join('.');
    if (!this.aliases.has(key)) this.aliases.set(key, new Map());
    const aliases = this.aliases.get(key);
    const tree = (prefix) => {
      if (c.eat('{')) {
        while (!c.is('}')) {
          tree(prefix);
          if (!c.eat(',')) break;
        }
        c.expect('}', 'use');
        return;
      }
      if (c.is('*')) throw unsupported('glob import', 'name each imported item explicitly', span(start, c.peek()));
      const segment = c.identifier('use').value;
      const full = [...prefix, segment];
      if (c.eat('::')) {
        tree(full);
        return;
      }
      const standardBox = full.length > 1 && acceptRustSourceBoxOwner(full.join('::'), false);
      const standardVector = full.length > 1 && acceptRustSourceArrayOwner(full.join('::'), false);
      const standardString = acceptRustReadOnlyQueryOwner('string empty', 'library', full.join('::'));
      const standardPrimitive=Boolean(readRustStandardPrimitivePosition(this,full,path));
      if(this.primitiveOwnerModel&&!standardPrimitive)throw unsupported('Rust primitive owner import','only the two actual standard primitive reexports can be imported',span(start,c.peek()));
      if (!['crate', 'super', 'self'].includes(full[0]) && !standardBox && !standardVector && !standardString&&!standardPrimitive) {
        throw unsupported(`use ${full.join('::')}`, 'only items of this crate can be imported; the standard library is outside the portable core', span(start, c.peek()));
      }
      if (standardBox) this.origins.box(full, full, path, span(start, c.peek()));
      if (standardVector) this.origins.vector(full, full, path, span(start, c.peek()));
      const alias = c.eat('as') ? c.identifier('use alias').value : segment;
      if(this.primitiveOwnerModel){
        if(!acceptRustPrimitiveOwnerImportName(alias)||this.primitiveImportNames.has(alias))throw unsupported('Rust primitive owner alias','each import needs one distinct ASCII identifier outside the standard crate namespace',span(start,c.peek()));
        this.primitiveImportNames.add(alias);
      }
      aliases.set(alias, standardBox || standardVector || standardString||standardPrimitive ? full : absolute(full, path, span(start, c.peek())));
    };
    tree([]);
    c.expect(';', 'use');
  }

  /** A `use` alias is visible only in the module that declares it. */
  resolve(segments, path) {
    const inherent=resolveRustInherentPath(this,segments,path);
    if(inherent)return inherent;
    const aliases = this.aliases.get(path.join('.'));
    return aliases?.has(segments[0]) ? [...aliases.get(segments[0]), ...segments.slice(1)] : segments;
  }

  enumItem(path,derivations=[]) {
    const c = this.cursor;
    const start = c.next();
    const name = c.identifier('enum').value;
    if (c.is('<')) throw unsupported('generic enum', 'type parameters are outside the portable core', span(start, c.peek()));
    c.expect('{', 'enum');
    const ctors = [];
    const owner = [...path, name].join('.');
    this.origins.declaration(name, path, { k: 'data' });
    while (!c.is('}')) {
      const variantAttributes=this.attributes();
      if(this.typeNamespaceModel&&variantAttributes.some(attribute=>attribute.traits.includes('Clone')))throw unsupported('Rust unit Clone derivation','a Clone derive must be attached to its datatype',span(c.peek(),c.peek()));
      const variant = c.identifier('variant');
      const fields = [];
      const sourceFields = [];
      if (c.eat('(')) {
        while (!c.is(')')) {
          fields.push({ type: this.type() });
          sourceFields.push(this.origins.type);
          if (!c.eat(',')) break;
        }
        c.expect(')', 'variant');
      } else if (c.is('{')) {
        const named=readRustNamedVariantFields(this,path,name,variant);
        fields.push(...named.fields);sourceFields.push(...named.origins);
      } else if (c.is('=')) {
        throw unsupported('explicit discriminant', 'discriminant values are outside the portable core', span(variant, c.peek()));
      }
      ctors.push({ name: variant.value, fields });
      this.origins.declaration(variant.value, [...path, name], { k: 'ctor', owner, fields: sourceFields, result: { wrappers: [], type: { kind: 'named', path: ['crate', ...path, name] }, path: [] } });
      if (!c.eat(',')) break;
    }
    c.expect('}', 'enum');
    const declaration={ k: 'data', name, ctors, span: span(start, c.peek()) };
    const proof=this.typeNamespaceModel?captureRustUnitCloneDerivation(derivations,declaration,path):null;
    if(proof)declaration.sourceUnitCloneDerivation=proof;
    this.origins.declaration(name,path,{k:'data',unitFields:ctors.every(constructor=>constructor.fields.length===0),...(proof?{sourceUnitCloneDerivation:proof}:{})});
    return declaration;
  }

  /**
   * Types: machine integers, `bool`, `String`, `&str`, `()`, crate types,
   * and `&T`/`Box<T>`, which denote the value `T` since every portable value
   * is immutable.
   */
  type() {
    splitRustReferenceToken(this);
    // Actual source owners also prove readonly method dispatch. Capturing a
    // public source-signature facet remains opt-in, independently of this fact.
    const head = readRustSourceTypeHead(this.cursor);
    const children = [];
    const primitiveChildren=[];
    const namespaceChildren = [];
    this.sourceOwnerFrames.push(children);
    this.primitiveOwnerFrames.push(primitiveChildren);
    this.typeNamespaceFrames.push(namespaceChildren);
    try {
      const result = this.typeValue();
      if(this.sharedArrayHandleSourceModel&&this.origins.type.wrappers.length)throw unsupported('Rust shared array handle source','borrowed or boxed source type wrappers need an independent composition proof',span(this.cursor.tokens[Math.max(0,this.cursor.index-1)],this.cursor.peek()));
      const position=readRustStandardPrimitivePosition(this,head)??(['&','('].includes(head[0])&&primitiveChildren.length===1?primitiveChildren[0]:null);
      this.origins.type.primitiveOwnerPosition=position;
      const owner = position?{kind:'primitive',path:['f64'],arguments:[]}:readRustSourceTypeOwner(this, head, children, result);
      this.origins.type.sourceOwner = owner;
      this.origins.type.namespaceSpelling = !['&', '('].includes(head[0]) ? [...head]
        : namespaceChildren.length === 1 && !head.includes('[') ? [...namespaceChildren[0]]
          : head[0] === '&' && !head.includes('[') ? ['str'] : [];
      this.typeNamespaceFrames.at(-2)?.push(this.origins.type.namespaceSpelling);
      this.sourceOwnerFrames.at(-2)?.push(owner);
      this.primitiveOwnerFrames.at(-2)?.push(position);
      return result;
    } finally { this.sourceOwnerFrames.pop();this.primitiveOwnerFrames.pop();this.typeNamespaceFrames.pop(); }
  }

  typeValue() {
    const c = this.cursor;
    const token = c.peek();
    const inherentSelf=readRustInherentSelfType(this);
    if(inherentSelf)return inherentSelf;
    if (c.eat('&')) {
      let sourceLifetime = null;
      if (c.is('\'')) {
        c.next();
        const lifetime = c.identifier('lifetime');
        if (!acceptRustSourceLifetimeReference(lifetime.value, this.lifetimeBindings.has(lifetime.value))) throw unsupported('lifetime parameter', 'a reference needs a declared function lifetime, an elided placeholder, or \'static', span(token, lifetime));
        if (this.lifetimeBindings.has(lifetime.value) && !this.lifetimeHeader) throw unsupported('local lifetime annotation', 'named lifetimes in body annotations require an explicit region proof', span(token, lifetime));
        sourceLifetime = lifetime.value;
      }
      if (c.is('mut')) throw unsupported('mutable reference', 'mutation is outside the portable core', span(token, c.peek()));
      if (c.eat('[')) {
        const elementToken = c.peek();
        const inner = this.type();
        this.arrayElement(token, elementToken);
        c.expect(']', 'immutable slice');
        const result = this.origins.typed(array(inner), this.modulePath, ['reference']);
        this.origins.type.arraySourceModel = 'immutable slice';
        this.origins.type.sourceLifetimes = [sourceLifetime];
        return result;
      }
      if (c.is('str') && !(this.typeNamespaceModel && this.origins.lookup(['str'], this.modulePath, RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds))) {
        c.next();
        const result = this.origins.typed(STRING, this.modulePath, ['reference'], null, 'string slice');
        this.origins.type.sourceLifetimes = [sourceLifetime];
        return result;
      }
      const inner = this.type();
      const arraySourceModel = this.origins.type.arraySourceModel;
      const lifetimes = [sourceLifetime, ...(this.origins.type.sourceLifetimes ?? [])];
      const result = this.origins.typed(inner, this.origins.type.path, ['reference', ...this.origins.type.wrappers], this.origins.type.fields, this.origins.type.textSourceModel);
      this.origins.type.sourceLifetimes = lifetimes;
      if (arraySourceModel) this.origins.type.arraySourceModel = arraySourceModel;
      return result;
    }
    if (c.eat('(')) {
      if (c.eat(')')) return this.origins.typed(UNIT, this.modulePath);
      const first = this.type();
      const fields = [this.origins.type];
      if (!c.eat(',')) {
        c.expect(')', 'parenthesised type');
        return first;
      }
      const elements = [first];
      while (!c.is(')')) {
        elements.push(this.type());
        fields.push(this.origins.type);
        if (!c.eat(',')) break;
      }
      c.expect(')', 'tuple type');
      const result = this.origins.typed(tuple(elements), this.modulePath, [], fields);
      if (!acceptRustProductOrigin(this.origins.immutableProduct(this.origins.type), this.origins.ownedLocalProduct(this.origins.type))) {
        throw unsupported('immutable Rust tuple product', 'every field requires a direct recursively immutable scalar Copy source value', span(token, c.peek()));
      }
      return result;
    }
    const segments = this.pathSegments('type');
    const name = segments.join('::');
    const resolved = this.resolve(segments, this.modulePath);
    const sharedHandle=readRustSharedArrayHandleType(this,segments,token);
    if(sharedHandle)return sharedHandle;
    const emptyOption=readRustEmptyOptionResult(this,segments,token);
    if(emptyOption)return emptyOption;
    if (this.typeNamespaceModel) {
      requireRustTypeNamespaceAnchor(segments, this.modulePath);
      const declared = this.origins.lookup(segments, this.modulePath, RUST_SOURCE_SIGNATURE_POLICY.typeOwnerKinds);
      if (declared) {
        if (!acceptRustTypeNamespaceDeclaration(declared.k) || c.is('<')) throw unsupported('Rust type namespace binding', 'the selected source position needs an actual datatype without generic arguments', span(token, c.peek()));
        return this.origins.typed({ kind: 'named', path: ['crate', ...declared.fullName.split('.')], span: span(token, token) }, this.modulePath);
      }
    }
    if(readRustStandardPrimitivePosition(this,segments))return this.origins.typed(FLOAT,this.modulePath);
    if (acceptRustSourceArrayOwner(name, false) || acceptRustSourceArrayOwner(resolved.join('::'), false)) {
      refuseRustSharedArrayBareVector(this,span(token,c.peek()));
      this.origins.vector(segments, resolved, this.modulePath, span(token, c.peek()));
      c.expect('<', 'Vec');
      const elementToken = c.peek();
      const inner = this.type();
      this.arrayElement(token, elementToken);
      c.expect('>', 'Vec');
      return this.origins.typed(array(inner), this.modulePath);
    }
    if (acceptRustSourceBoxOwner(name, false) || acceptRustSourceBoxOwner(resolved.join('::'), false)) {
      this.origins.box(segments, resolved, this.modulePath, span(token, c.peek()));
      c.expect('<', 'Box');
      const inner = this.type();
      c.expect('>', 'Box');
      const lifetimes = this.origins.type.sourceLifetimes;
      const arraySourceModel = this.origins.type.arraySourceModel;
      const result = this.origins.typed(inner, this.origins.type.path, ['box', ...this.origins.type.wrappers], this.origins.type.fields, this.origins.type.textSourceModel);
      if (lifetimes) this.origins.type.sourceLifetimes = lifetimes;
      if (arraySourceModel) this.origins.type.arraySourceModel = arraySourceModel;
      return result;
    }
    if (c.is('<')) throw unsupported(`type ${name}<…>`, 'generic types are outside the portable core', span(token, c.peek()));
    const fixed = segments.length === 1 ? rustFixedType(name) : undefined;
    if (fixed) return this.origins.typed(fixed, this.modulePath);
    if (name === 'bool') return this.origins.typed(BOOL, this.modulePath);
    if (name === 'String' || resolved.join('::') === 'std::string::String') return this.origins.typed(STRING, this.modulePath);
    if (name === 'f64') return this.origins.typed(FLOAT, this.modulePath);
    if (['str', 'char', 'f32', 'Vec', 'Option', 'Result', 'Rc', 'Arc', 'HashMap', 'Self'].includes(name)) {
      if (acceptRustTypeNamespaceDiscoveryType(this.typeNamespaceDiscovery, name)) return this.origins.typed({kind:'named',path:resolved,span:span(token,token)},this.modulePath);
      throw unsupported(`Rust type ${name}`, 'outside the portable core', span(token, token));
    }
    return this.origins.typed({ kind: 'named', path: this.resolve(segments, this.modulePath), span: span(token, token) }, this.modulePath);
  }

  pathSegments(context) {
    const c = this.cursor;
    const segments = [c.identifier(context).value];
    while (c.is('::') && c.peek(1).kind === 'identifier') {
      c.next();
      segments.push(c.next().value);
    }
    return segments;
  }

  arrayElement(token, elementToken) {
    const shadowed = !this.origins.collecting && elementToken?.kind === 'identifier' && this.origins.lookup([elementToken.value], this.modulePath);
    if (shadowed || !acceptRustSourceArrayElement(this.origins.type.type?.kind ?? '', this.origins.type.wrappers.length > 0)) throw unsupported('Rust array element', 'only direct immutable scalar and owned string elements are admitted', span(token, this.cursor.peek()));
  }

  fn(path,constKeyword=null) {
    const c = this.cursor;
    const functionKeyword = c.next();
    const start = constKeyword??functionKeyword;
    const name = c.identifier('fn').value;
    if(this.pureUnitCallModel)return readRustPureUnitFunction(this,path,start,name);
    const outerLifetimes = this.lifetimeBindings;
    const outerHeader = this.lifetimeHeader;
    this.lifetimeBindings = new Set();
    this.lifetimeHeader = true;
    try {
      if(this.emptyOptionSourceModel && c.is('<')) throw unsupported('Rust empty option source', 'generic functions and lifetime binders require a separate interpretation', span(start,c.peek()));
      if (c.is('<')) {
        const generic = c.next();
        while (!c.is('>')) {
          if (!c.eat("'")) throw unsupported('generic function', 'type parameters are outside the portable core', span(start, generic));
          const lifetime = c.identifier('lifetime');
          if (!acceptRustSourceLifetimeBinding(lifetime.value, this.lifetimeBindings.has(lifetime.value), c.is(':'))) throw unsupported('function lifetime binding', 'lifetime names must be unique and unconstrained; bounds need their own linked obligations', span(lifetime, c.peek()));
          this.lifetimeBindings.add(lifetime.value);
          if (!c.eat(',')) break;
        }
        c.expect('>', 'function lifetimes');
        if (!this.lifetimeBindings.size) throw unsupported('function lifetime binding', 'a lifetime parameter list must contain a named binding', span(generic, c.peek()));
      }
      c.expect('(', 'fn');
      if(this.emptyOptionSourceModel && !c.is(')')) throw unsupported('Rust empty option source', 'the closed empty interpretation admits zero parameters', span(start,c.peek()));
      const params = [];
      const sourceParams = [];
      const mutableParams = [];
      while (!c.is(')')) {
        const paramToken = c.peek();
        const receiver=readRustInherentReceiver(this,name,path,params.length);
        if(receiver){
          params.push(receiver.parameter);sourceParams.push(receiver.origin);
          if(!c.eat(','))break;
          continue;
        }
        const mutable = Boolean(c.eat('mut'));
        if (c.is('self') || c.is('&')) throw unsupported('method', 'methods are outside the Rust frontend; declare a free function', span(paramToken, paramToken));
        const paramName = c.identifier('parameter').value;
        c.expect(':', 'parameter');
        const typeToken = c.peek();
        params.push({ name: paramName, type: this.type(), span: span(paramToken, c.peek()) });
        sourceParams.push(this.origins.type);
        if (mutable) mutableParams.push({ name: paramName, token: paramToken, typeToken, origin: this.origins.type });
        if (!c.eat(',')) break;
      }
      c.expect(')', 'fn');
      const explicitResult=Boolean(c.eat('->'));
      if(!explicitResult&&this.emptyOptionSourceModel)throw unsupported('function without a result', `${name} returns (); only value-returning functions are portable`, span(start,c.peek()));
      const resultToken=c.peek();
      const ret = explicitResult?this.type():this.origins.typed(UNIT,path);
      const sourceResult = this.origins.type;
      if(this.sharedArrayHandleSourceModel&&(mutableParams.length||this.lifetimeBindings.size))throw unsupported('Rust shared array handle source','mutable parameters and named lifetime binders need another source profile',span(start,c.peek()));
      if(this.constFunctionModel&&(mutableParams.length||this.lifetimeBindings.size))throw unsupported('Rust const function interface','mutable parameters and lifetime binders require an independent interpretation');
      if(this.localValueStateModel&&(mutableParams.length||this.lifetimeBindings.size))throw unsupported('Rust owned local state','mutable parameters and lifetimes require a separate interface proof');
      if(this.emptyOptionSourceModel && (!sourceResult.emptyOptionResult || sourceResult.wrappers.length || sourceResult.emptyOptionResult.span.start!==resultToken.start)) throw unsupported('Rust empty option source', 'the result needs an exact direct standard Option<f64> owner', span(start,c.peek()));
      if(this.primitiveOwnerModel&&(mutableParams.length||this.lifetimeBindings.size))throw unsupported('Rust primitive owner context','mutable parameters and named lifetime binders require a separate owned context proof');
      const scalarLoop = Boolean(this.scalarLoopModel && c.is('{') && c.is('loop', 1));
      if (scalarLoop && path.length) throw unsupported('Rust scalar loop', 'the selected scalar state model requires a flat function', span(start, c.peek()));
      if (mutableParams.length && !scalarLoop) {
        const bodyEvidence = readRustScalarParameterBody(c);
        for (const { token, typeToken, origin } of mutableParams) {
          const shadowed = typeToken.kind !== 'identifier' || Boolean(this.origins.lookup(this.resolve([typeToken.value], path), path));
          if (!acceptRustUnwrittenScalarParameter(origin.type?.kind ?? '', this.origins.immutableProduct(origin), shadowed, bodyEvidence.closed, bodyEvidence.possibleWrite)) {
            throw unsupported('mutable parameter', 'mutation is outside the portable core', span(token, token));
          }
        }
      }
      if (ret.kind === 'array' && !isRustSharedArrayHandleOrigin(this,sourceResult) && !acceptRustSourceArrayResult(this.origins.arrayResultModel, ret.element.kind, sourceResult.arraySourceModel === 'owned vector' && sourceResult.wrappers.length === 0, true)) throw unsupported('Rust array result', 'returning vectors or borrowed slices needs a separate consumption and outlives proof', span(start, c.peek()));
      if (!acceptRustSourceLifetimeResult(this.lifetimeBindings.size > 0, sourceResult.wrappers.includes('reference'))) throw unsupported('function lifetime result', 'reference results with lifetime binders require an explicit return-lifetime instantiation and outlives proof', span(start, c.peek()));
      this.origins.declaration(name, path, { k: 'fn', lifetimeBindings: [...this.lifetimeBindings], parameters: sourceParams, result: sourceResult });
      if (c.is('where')) throw unsupported('where clause', 'trait bounds are outside the portable core', span(start, c.peek()));
      const constHeaderEnd=constKeyword?c.peek().start:null;
      this.lifetimeHeader = false;
      const previous = this.origins.enter(new Map(params.map((param, index) => [param.name, sourceParams[index]])));
      const signature = scalarLoop||this.localValueStateModel ? createRustSourceSignature('Rust source function signature 1', sourceParams, sourceResult, this.lifetimeBindings) : null;
      let lowered = null;
      const previousLoop = this.scalarLoopActive;
      this.scalarLoopActive = scalarLoop;
      try {
        if (scalarLoop) lowered = readRustScalarLoop(this, path, name, params, ret, signature, mutableParams.map(param => param.name));
        else if(this.localValueStateModel)lowered=readRustOwnedLocalStateBody(this,path,name,params,ret,signature);
      } finally { this.scalarLoopActive = previousLoop; }
      const emptyOptionBody=this.emptyOptionSourceModel?readRustEmptyOptionBody(this,path):null;
      const body = emptyOptionBody?.body ?? lowered?.body ?? (this.completionModel ? readRustFunctionCompletion(this, path) : this.ordinaryCompletionSelected()?readRustOrdinaryFunctionCompletion(this,path,ret,sourceResult):this.block(path,true));
      if (ret.kind === 'array'&&!isRustSharedArrayHandleOrigin(this,sourceResult)) {
        if (!acceptRustSourceArrayResult(this.origins.arrayResultModel, ret.element.kind, sourceResult.arraySourceModel === 'owned vector' && sourceResult.wrappers.length === 0, this.origins.freshArrayResult(body, path))) throw unsupported('fresh Rust array result', 'every returning tail must construct a new vector of direct scalar Copy values; vector names, calls, clones and borrowed results need separate consumption proofs', body.span);
        sourceResult.freshArrayValue = true;
        this.origins.mark(body, sourceResult);
      }
      this.origins.argument(this.origins.origin(body, path), sourceResult, body.span);
      this.origins.scope = previous;
      return { k: 'fn', name, params, ret, body, span: emptyOptionBody?{start:start.start,end:c.tokens[c.index-1].end}:span(start, c.peek()),
        ...(constKeyword?{sourceConstFunction:createRustConstFunctionProof(name,constKeyword,functionKeyword,constHeaderEnd,c.tokens[c.index-1].end,this.constFunctionModel)}:{}),
        ...(emptyOptionBody?{sourceEmptyOption:createRustEmptyOptionProof(this,name,sourceResult.emptyOptionResult,emptyOptionBody.construction)}:{}),
        ...(this.typeNamespaceModel ? {sourceTypeNamespace: createRustTypeNamespaceProof(sourceParams, sourceResult)} : {}),
        ...(this.localValueStateModel?{sourceSignature:signature,sourceLocalState:lowered.proof}:{}),
        ...(this.primitiveOwnerModel?{sourcePrimitiveOwners:createRustPrimitiveOwnerProof(this,sourceParams,sourceResult,{span:span(start,c.peek())})}:{}),
        ...(scalarLoop ? {sourceSignature: signature, sourceLoop: lowered.proof} : this.sourceSignatureModel ? { sourceSignature: createRustSourceSignature(this.sourceSignatureModel, sourceParams, sourceResult, this.lifetimeBindings) } : {}),
      };
    } finally {
      this.lifetimeBindings = outerLifetimes;
      this.lifetimeHeader = outerHeader;
    }
  }

  /** `const N: T = e;` is a function without parameters. */
  constItem(path) {
    if (this.typeNamespaceModel) throw unsupported('Rust type namespace constant', 'constant-as-function projection needs a separate owned source binding proof');
    const c = this.cursor;
    const start = c.next();
    const name = c.identifier('const').value;
    c.expect(':', 'const');
    const ret = this.type();
    const sourceResult = this.origins.type;
    if (ret.kind === 'array') throw unsupported('Rust array constant', 'vector allocation and slice constants require a separate constant evaluation model', span(start, c.peek()));
    this.origins.declaration(name, path, { k: 'fn', parameters: [], result: sourceResult });
    c.expect('=', 'const');
    const body = this.expr(path);
    this.origins.argument(this.origins.origin(body, path), sourceResult, body.span);
    c.expect(';', 'const');
    return { k: 'fn', name, params: [], ret, body, span: span(start, c.peek()) };
  }

  /**
   * A block `{ let x = e; …; tail }` is a chain of lets ending in its tail
   * expression. Statements with effects are not values of the portable core.
   */
  isTerminalReturn() {
    const c=this.cursor;
    let count=64;
    for(;;){
      const values=c.tokens.slice(c.index,c.index+count).map(token=>token.kind==='punct'?token.value:'');
      const boundary=findRustTerminalReturnBoundary(values,0);
      if(boundary!==-2)return boundary>=0;
      if(count>=c.tokens.length-c.index)return false;
      count*=2;
    }
  }

  ordinaryCompletionContext() {
    return acceptRustOrdinaryCompletionProfiles([this.completionModel,this.sourceSignatureModel,this.scalarLoopModel,this.primitiveOwnerModel,this.emptyOptionSourceModel,this.typeNamespaceModel,this.pureUnitCallModel,this.localValueStateModel,this.constFunctionModel,this.sharedArrayHandleSourceModel,this.origins.arrayResultModel]);
  }

  ordinaryCompletionSelected() {
    if(!this.ordinaryCompletionContext())return false;
    const c=this.cursor;
    let count=64;
    for(;;){
      const values=c.tokens.slice(c.index,c.index+count).map((token,index)=>token.kind==='punct'&&(token.value==='{'||token.value==='}')?token.value:token.kind==='macro'&&token.value==='panic'?'abort':c.is('return',index)?'return':'');
      const presence=readRustFunctionCompletionPresence(values);
      if(presence!==-1)return presence===1;
      if(count>=c.tokens.length-c.index)return false;
      count*=2;
    }
  }

  functionPrintValue(path) {
    const effect=this.mainMacro(path),unit={k:'unit',span:effect.span};
    return this.origins.mark({k:'print',expr:effect.expr,style:effect.style,body:unit,span:effect.span},this.origins.origin(unit,path));
  }

  block(path,functionBody=false) {
    const c = this.cursor;
    const open = c.expect('{', 'block');
    const previous = this.origins.enter();
    const lets = [];
    const ordinary = this.ordinaryCompletionContext();
    let tail = null;
    while (!c.is('}')) {
      if(c.eat(';'))continue;
      const token = c.peek();
      if(tail?.k==='abort')throw unsupported('unreachable continuation','statements after an abort are not removed by this model',span(token,token));
      if (c.is('let')) {
        lets.push(...this.letStatement(path));
        continue;
      }
      if(token.kind==='macro'&&readRustFunctionMacroDisposition(token.value)==='print'){
        const printed=this.functionPrintValue(path);
        if(c.eat(';')){lets.push({name:this.freshProductBinding(),value:printed,span:printed.span});continue;}
        if(!c.is('}'))throw this.fail('expected ; or }');
        tail=printed;break;
      }
      if(functionBody&&c.is('return')&&(c.is(';',1)||c.is('}',1)||this.isTerminalReturn())){
        c.next();
        tail=c.is(';')||c.is('}')?{k:'unit',span:span(token,token)}:this.expr(path);
        while(c.eat(';')){}
        if(!c.is('}'))throw unsupported('unreachable continuation','statements after a completed return are not removed by this model',span(token,c.peek()));
        break;
      }
      if (c.is('return')) throw unsupported('early return', 'write the result as the tail expression of the block', span(token, token));
      if (['while', 'loop', 'for'].includes(token.value) && token.kind === 'identifier') {
        throw unsupported(`${token.value} loop`, 'loops over mutable state are outside the portable core; use recursion', span(token, token));
      }
      const expr = this.expr(path, { statement: true });
      if (c.eat(';')) {
        const origin=this.origins.origin(expr,path);
        const disposition=(ordinary?readRustOrdinaryStatementDisposition:readRustUnitStatementDisposition)(expr.k,this.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0);
        if(disposition==='abort'){tail=expr;continue;}
        if(disposition==='bind'){lets.push({name:this.freshProductBinding(),value:expr,span:expr.span});continue;}
        throw unsupported('expression statement', 'statements with effects are outside the portable core in function bodies', expr.span);
      }
      if(ordinary){
        const origin=this.origins.origin(expr,path);
        const disposition=readRustImplicitBlockStatementDisposition(expr.k,expr.block===true,!c.is('}'),this.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0);
        if(disposition==='bind'){lets.push({name:this.freshProductBinding(),type:UNIT,value:expr,span:expr.span});continue;}
      }
      tail = expr;
      if (!c.is('}')) {
        if ((expr.block===true||['if', 'match', 'block'].includes(expr.k))) throw unsupported('expression statement', 'a block value must be its last expression', expr.span);
        throw this.fail('expected }');
      }
    }
    const close = c.expect('}', 'block');
    if(!tail)tail={k:'unit',span:span(open,close)};
    const origin = this.origins.origin(tail, path);
    const result = lets.reduceRight((body, binding) => ({ k: 'let', ...binding, body, span: binding.span }), tail);
    this.origins.scope = previous;
    return this.origins.mark(result, origin);
  }

  letStatement(path) {
    const c = this.cursor;
    const start = c.next();
    if (c.is('mut')) throw unsupported('mutable binding', 'mutation is outside the portable core', span(start, c.peek()));
    const pattern = readRustLetElsePattern(this,path)??this.productBindingPattern();
    const names = new Set();
    const validateNames = binding => {
      if(binding.k==='nominal'){
        for(const item of rustNominalBindingNames(this,binding.pattern))validateNames({k:'name',...item});
        return;
      }
      if (binding.k === 'name') {
        if (names.has(binding.name)) throw unsupported('duplicate tuple binding', 'each name occurs only once in a destructuring pattern', binding.span);
        names.add(binding.name);
      }
      for (const item of binding.items ?? []) validateNames(item);
    };
    validateNames(pattern);
    const type = c.eat(':') ? this.type() : undefined;
    const sourceType = type ? this.origins.type : null;
    if (!c.eat('=')) throw unsupported('uninitialised binding', 'every binding needs its value', span(start, c.peek()));
    const initializerStart=c.index;
    const value = this.expr(path);
    let fallback=null;
    if(c.is('else')){
      const tokens=c.tokens.slice(initializerStart,c.index),reason=readRustLetElseInitializerError(tokens.map(token=>token.kind==='punct'?'punct':'atom'),tokens.map(token=>token.value));
      if(reason)throw unsupported('Rust let-else initializer',reason,span(start,c.peek()));
    }
    if(c.eat('else')){
      fallback=this.block(path);
      if(!this.origins.origin(fallback,path).diverging)throw unsupported('Rust let-else fallback','the fallback needs a represented diverging completion',fallback.span);
    }
    c.expect(';', 'let');
    const range = span(start, c.peek());
    if (pattern.k === 'name') {
      this.origins.bind(pattern.name, value, path, sourceType);
      return [{ name: pattern.name, ...(type === undefined && this.typeNamespaceModel ? {} : {type}), value, span: range }];
    }
    const bindings = [];
    const name = this.freshProductBinding();
    this.origins.bind(name, value, path, sourceType);
    bindings.push({ name, ...(type === undefined && this.typeNamespaceModel ? {} : {type}), value, span: range });
    if(fallback!==null){
      if(pattern.k!=='nominal')throw unsupported('let-else','refutable bindings are outside the portable core',range);
      lowerRustNominalBinding(this,pattern.pattern,{k:'name',path:[name],span:range},path,bindings,fallback);
      return bindings;
    }
    this.lowerProductBinding(pattern, { k: 'name', path: [name], span: range }, path, bindings);
    return bindings;
  }

  freshProductBinding() {
    let name;
    do { name = `ml_tuple_value${this.productBindingCount++}`; } while (this.reservedNames.has(name));
    this.reservedNames.add(name);
    return name;
  }

  productBindingPattern(allowMutable=false) {
    const c = this.cursor;
    const token = c.peek();
    const mutable=Boolean(allowMutable&&this.localValueStateModel&&c.eat('mut'));
    if (c.is('mut')) throw unsupported('mutable tuple binding', 'mutation is outside the immutable product model', span(token, token));
    if (c.is('ref') || c.is('&')) throw unsupported('reference tuple binding', 'product bindings must retain direct Copy values', span(token, token));
    const nominal=readRustNominalBindingPattern(this);
    if(nominal)return nominal;
    if (c.eat('_')) return { k: 'wild', span: { start: token.start, end: token.end } };
    if (c.eat('(')) {
      if (c.eat(')')) return { k: 'unit', span: { start: token.start, end: c.tokens[c.index - 1].end } };
      const items=[];let rest=null,comma=false;
      while (!c.is(')')) {
        if(c.eat('..')){
          if(rest!==null)throw unsupported('tuple rest binding','a tuple binding has at most one rest position',span(token,c.peek()));
          rest=items.length;
        }else items.push(this.productBindingPattern(allowMutable));
        if (!c.eat(',')) break;
        comma=true;
      }
      const close = c.expect(')', 'tuple binding');
      if(items.length===1&&rest===null&&!comma)return items[0];
      return { k: 'tuple', items,...(rest===null?{}:{rest}), span: { start: token.start, end: close.end } };
    }
    const name = c.identifier('let');
    if (!name.raw.startsWith('r#')&&(name.value === 'true' || name.value === 'false')) throw unsupported('refutable product binding', 'only immutable names, ignored fields and anonymous tuple products are admitted', { start: name.start, end: name.end });
    if (c.is('::') || c.is('(') || c.is('{') || c.is('@')) throw unsupported('refutable product binding', 'only immutable names, ignored fields and anonymous tuple products are admitted', span(token, c.peek()));
    return { k: 'name', name: name.value, ...(mutable?{mutable:true}:{}),span: { start: name.start, end: name.end } };
  }

  lowerProductBinding(pattern, value, path, bindings) {
    if(pattern.k==='nominal'){lowerRustNominalBinding(this,pattern.pattern,value,path,bindings);return;}
    if (pattern.k === 'wild') return;
    if (pattern.k === 'name') {
      this.origins.bind(pattern.name, value, path);
      bindings.push({ name: pattern.name, value, span: pattern.span });
      return;
    }
    const origin = this.origins.origin(value, path);
    if (pattern.k === 'unit') {
      if (!this.origins.collecting && (origin.type?.kind !== 'unit' || !this.origins.immutableProduct(origin))) {
        throw unsupported('unit product binding', 'the unit pattern requires a direct unit source value', pattern.span);
      }
      return;
    }
    this.origins.requireProduct(origin, pattern.span);
    const plan=planTuplePatternFields(origin.fields?.length??pattern.items.length,pattern.items.length,pattern.rest??-1);
    if (!this.origins.collecting && plan[0]<0) {
      throw new TranslationError('type', 'tuple binding has the wrong number of fields', pattern.span);
    }
    const positions=new Map(this.origins.collecting?pattern.items.map((_,index)=>[index,index]):plan.slice(1).map((index,position)=>[index,position]));
    for (const [source, item] of pattern.items.entries()) {
      const index=positions.get(source);
      const projection = this.origins.project({ k: 'tupleIndex', object: value, index, span: item.span }, path, index, item.span);
      this.lowerProductBinding(item, projection, path, bindings);
    }
  }

  main() {
    if(this.pureUnitCallModel)return readRustPureUnitMain(this);
    const c = this.cursor;
    const start = c.next();
    c.expect('main', 'fn main');
    c.expect('(', 'fn main');
    c.expect(')', 'fn main');
    if (c.is('->')) throw unsupported('main with a result', 'main must return ()', span(start, c.peek()));
    this.modulePath = [];
    const previous = this.origins.enter(new Map());
    c.expect('{', 'fn main');
    const effects = [];
    while (!c.is('}')) {
      if(c.eat(';'))continue;
      const token = c.peek();
      if (c.is('let')) {
        for (const binding of this.letStatement([])) effects.push({ k: 'let', name: binding.name, ...(binding.type === undefined && this.typeNamespaceModel ? {} : {type: binding.type}), value: binding.value, span: binding.span });
        continue;
      }
      if (token.kind === 'macro') {
        effects.push(this.mainMacro());
        c.expect(';', token.value);
        continue;
      }
      const value=this.expr([],{statement:true}),origin=this.origins.origin(value,[]);
      if(readRustUnitStatementDisposition(value.k,this.origins.collecting,origin.proven,origin.type?.kind??'',origin.wrappers.length>0)!=='bind')throw unsupported('main statement','main may only print, bind, assert and evaluate direct Unit calls',span(token,token));
      if(!c.eat(';')&&!c.is('}'))throw this.fail('expected ; or }');effects.push({k:'let',name:this.freshProductBinding(),value,span:value.span});
    }
    c.expect('}', 'fn main');
    this.origins.scope = previous;
    return { effects, span: this.emptyOptionSourceModel?{start:start.start,end:c.tokens[c.index-1].end}:span(start, c.peek()) };
  }

  mainMacro(path=[]) {
    const c = this.cursor;
    const token = c.next();
    const args = this.macroArguments(token, path);
    switch (token.value) {
      case 'println':
        if (!args.length) return { k: 'print', expr: { k: 'str', value: '' }, style: 'rust', span: span(token, c.peek()) };
        return { k: 'print', expr: this.format(args, token,path), style: 'rust', span: span(token, c.peek()) };
      case 'assert':
        if (args.length !== 1) throw unsupported('assert! message', 'custom assertion messages are not kept', span(token, c.peek()));
        return { k: 'assert', prop: propOf(args[0]), span: span(token, c.peek()) };
      case 'assert_eq':
      case 'assert_ne':
        if (args.length !== 2) throw unsupported(`${token.value}! message`, 'custom assertion messages are not kept', span(token, c.peek()));
        return { k: 'assert', prop: { p: token.value === 'assert_eq' ? 'eq' : 'ne', left: args[0], right: args[1] }, span: span(token, c.peek()) };
      default:
        throw unsupported(`${token.value}!`, 'main may only use println!, assert!, assert_eq! and assert_ne!', span(token, token));
    }
  }

  /** Macro arguments, parsed as expressions; the first one of a format macro is its literal. */
  macroArguments(token, path) {
    const c = this.cursor;
    const close = { '(': ')', '[': ']', '{': '}' }[c.peek().value];
    if (!close) throw this.fail(`expected ( after ${token.value}!`);
    c.next();
    const args = [];
    while (!c.is(close)) {
      args.push(this.expr(path));
      if (!c.eat(',')) break;
    }
    c.expect(close, `${token.value}!`);
    return args;
  }

  /**
   * `format!("{} and {name}", a)`: `{}` takes the next argument, `{name}` a
   * variable in scope, `{{`/`}}` are braces; every value is rendered with
   * Display, which for integers is its decimal text.
   */
  format(args, token, path=this.modulePath) {
    const [template,...values]=args;
    if(template?.k!=='str')throw unsupported(`${token.value}!`,'the format string must be a literal',span(token,token));
    const units=Array.from({length:template.value.length},(_,index)=>template.value.charCodeAt(index));
    const plan=planRustFormattingValue(units,values.length);
    if(plan[0]!==0){
      const spec=template.value.slice(plan[1],plan[2]);
      if(plan[0]===-1)throw new TranslationError('syntax','unmatched } in format string',template.span);
      if(plan[0]===-2)throw new TranslationError('syntax','unclosed { in format string',template.span);
      if(plan[0]===-3)throw unsupported(`format spec {${spec}}`,'only plain {} Display formatting is portable',template.span);
      if(plan[0]===-5)throw new TranslationError('syntax','format string has more {} than arguments',template.span);
      if(plan[0]===-6)throw new TranslationError('syntax',`format argument ${spec} is missing`,template.span);
      if(plan[0]===-7)throw new TranslationError('syntax','format string has fewer {} than arguments',template.span);
      throw unsupported('Rust formatting facts','the format requires actual literal units and operand positions',template.span);
    }
    const captures=planRustFormattingOperandCapture(values.length).map(index=>{
      const value=values[index],name=this.freshProductBinding();
      refuseRustSharedArrayHandleObservation(this,value,path,value.span);
      const reference=this.origins.mark({k:'name',path:[name],span:value.span},this.origins.origin(value,path));
      return {name,value,reference,span:value.span};
    });
    const pieces=[];
    for(let at=1;at<plan.length;at+=3){
      const code=plan[at],first=plan[at+1],last=plan[at+2];
      if(code===0){pieces.push({k:'str',value:template.value.slice(first,last),span:template.span});continue;}
      let value;
      if(code===1)value=captures[first].reference;
      else if(code===2){
        const sourceName=template.value.slice(first,last);
        if(sourceName==='_'||!acceptRustSourceIdentifierName(sourceName))throw unsupported(`format argument {${sourceName}}`,'outside the portable core',template.span);
        const name=normalizeRustSourceIdentifier(sourceName,template.span);
        value={k:'name',path:[name],span:template.span};
        refuseRustSharedArrayHandleObservation(this,value,path,value.span);
      }else throw new TypeError('unknown Rust formatting term instruction');
      pieces.push({k:'show',arg:value,style:'rust',span:value.span});
    }
    let body=pieces.reduce((left,right)=>({k:'binary',op:'concat',left,right,span:template.span}));
    for(let at=captures.length-1;at>=0;at-=1){const binding=captures[at];body={k:'let',name:binding.name,value:binding.value,body,span:binding.span};}
    return this.origins.mark(body,this.origins.typeOrigin(STRING,path));
  }

  /** Expressions, by Rust precedence. */
  expr(path, options = {}) {
    return this.or(path, options);
  }

  or(path, options) {
    let left = this.and(path, options);
    while (this.cursor.is('||')) {
      const token = this.cursor.next();
      const right = this.and(path, {});
      left = { k: 'binary', op: 'or', left, right, span: joined(left, right, token) };
    }
    return left;
  }

  and(path, options) {
    let left = this.comparison(path, options);
    while (this.cursor.is('&&')) {
      const token = this.cursor.next();
      const right = this.comparison(path, {});
      left = { k: 'binary', op: 'and', left, right, span: joined(left, right, token) };
    }
    return left;
  }

  comparison(path, options) {
    const c = this.cursor;
    const left = this.primitiveBitwise(path, options, 0);
    const token = c.peek();
    const shift = (token.value === '<' || token.value === '>') && c.is(token.value, 1) && c.peek(1).start === token.end;
    const blockStatement = options.statement && (left.block===true||['if', 'match'].includes(left.k));
    if (token.kind === 'punct' && (shift || ['|', '^', '&'].includes(token.value)) && !blockStatement) {
      throw unsupported(`operator ${shift ? token.value.repeat(2) : token.value}`, 'bitwise operators are outside the portable core', span(token, c.peek(shift ? 1 : 0)));
    }
    if (token.kind === 'punct' && COMPARISON[token.value]) {
      c.next();
      const right = this.primitiveBitwise(path, {}, 0);
      refuseRustSharedArrayHandleValueComparison(this,left,right,path,joined(left,right,token));
      const after = c.peek();
      if (after.kind === 'punct' && COMPARISON[after.value]) throw this.fail('comparison operators cannot be chained', after);
      return { k: 'binary', op: COMPARISON[token.value], left, right, span: joined(left, right, token) };
    }
    return left;
  }

  primitiveBitwise(path, options, level) {
    if (level === 3) return this.shift(path, options);
    let left = this.primitiveBitwise(path, options, level + 1);
    while (this.cursor.peek().kind === 'punct' && acceptRustPrimitiveBitwisePrecedence(this.cursor.peek().value, level)) {
      if (options.statement && (left.block===true||['if', 'match'].includes(left.k))) return left;
      const token = this.cursor.next(), right = this.primitiveBitwise(path, {}, level + 1);
      left = { k: 'math', op: readRustPrimitiveBitwiseOperation(token.value), name: 'Rust primitive bitwise operation',
        args: [{ value: left, spread: false }, { value: right, spread: false }], span: joined(left, right, token) };
    }
    return left;
  }

  shift(path, options) {
    const c = this.cursor;
    let left = this.additive(path, options);
    while ((c.is('<') || c.is('>')) && c.is(c.peek().value,1) && c.peek(1).start===c.peek().end) {
      if (options.statement && (left.block===true||['if','match'].includes(left.k))) return left;
      const token=c.next();c.next();
      const right=this.additive(path,{});
      left={k:'math',op:readRustShiftSourceOperation(token.value.repeat(2)),name:'Rust primitive shift',
        args:[{value:left,spread:false},{value:right,spread:false}],span:joined(left,right,token)};
    }
    return left;
  }

  additive(path, options) {
    let left = this.multiplicative(path, options);
    for (;;) {
      const token = this.cursor.peek();
      if (token.kind !== 'punct' || !ADDITIVE[token.value]) return left;
      if (options.statement && (left.block===true||['if', 'match', 'block'].includes(left.k))) return left;
      this.cursor.next();
      const right = this.multiplicative(path, {});
      left = this.scalarLoopArithmetic({ k: 'binary', op: ADDITIVE[token.value], left, right, span: joined(left, right, token) }, path);
    }
  }

  multiplicative(path, options) {
    let left = this.cast(path, options);
    for (;;) {
      const token = this.cursor.peek();
      if (token.kind !== 'punct' || !MULTIPLICATIVE[token.value]) return left;
      if (options.statement && (left.block===true||['if', 'match', 'block'].includes(left.k))) return left;
      this.cursor.next();
      const right = this.cast(path, {});
      left = this.scalarLoopArithmetic({ k: 'binary', op: MULTIPLICATIVE[token.value], left, right, span: joined(left, right, token) }, path);
    }
  }

  /** `e as T` converts between integer types when every value fits. */
  cast(path, options) {
    let value = this.unary(path, options);
    while (this.cursor.is('as')) {
      const token = this.cursor.next();
      const parenthesizedType = this.cursor.is('(');
      const to = this.type();
      if (!this.origins.collecting && this.origins.type.wrappers.length) throw unsupported('source cast', 'numeric casts require a direct source and target value type', span(token, this.cursor.peek()));
      this.requireCastFollowingToken(parenthesizedType);
      value = { k: 'cast', arg: value, to, flavor: 'exact', span: joined(value, { span: span(token, this.cursor.peek()) }, token) };
    }
    return value;
  }

  requireCastFollowingToken(parenthesizedType) {
    const token = this.cursor.peek();
    if (!acceptRustCastFollowingToken(token.value, parenthesizedType)) {
      throw new TranslationError('syntax', readRustCastSyntaxMessage(), token);
    }
  }

  unary(path, options) {
    const c = this.cursor;
    const token = c.peek();
    if(this.localValueStateModel&&token.value==='*'&&!acceptRustOwnedLocalSourceOperation('dereference'))throw unsupported('Rust owned local state','source indirection requires a separate ownership interpretation',span(token,token));
    if(this.localValueStateModel===RUST_OWNED_LOCAL_STATE_POLICY.models.copies&&['&','&&'].includes(token.value)&&!acceptRustOwnedLocalSourceOperation('borrow'))throw unsupported('Rust owned local state','source indirection requires a separate ownership interpretation',span(token,token));
    if (this.scalarLoopActive && ['&', '&&', '*'].includes(token.value)) throw unsupported('Rust scalar loop', 'references and dereferences require a separate loop state model', span(token, token));
    if (c.eat('-')) {
      const arg = this.unary(path, {});
      const result = { k: 'unary', op: 'neg', arg, span: span(token, c.peek()) };
      const origin = this.origins.origin(arg, path);
      return this.acceptFloatReceiverOrigin(origin, 0) ? this.origins.mark(result, origin) : result;
    }
    if (c.eat('!')) {
      const arg = this.unary(path, {});
      return { k: 'unary', op: 'not', arg, span: span(token, c.peek()) };
    }
    // Source evidence proves immutable indirection before projecting its value.
    if (c.is('&') || c.is('&&')) {
      c.next();
      if (c.is('mut')) throw unsupported('mutable reference', 'mutation is outside the portable core', span(token, c.peek()));
      const arg = this.unary(path, {});
      if (token.value === '&&') this.origins.wrap(arg, path, 'reference');
      return noteRustSharedArrayHandleBorrow(this,this.origins.wrap(arg, path, 'reference'),path);
    }
    if (c.eat('*')) return this.origins.dereference(this.unary(path, {}), path, span(token, c.peek()));
    return this.postfix(path, options);
  }

  postfix(path, options) {
    const c = this.cursor;
    let value = this.primary(path, options);
    for (;;) {
      const token = c.peek();
      if (c.is('.')) {
        c.next();
        if (c.isKind('number')) {
          const number = this.tupleIndexToken();
          const index = Number(number.raw);
          if (!Number.isSafeInteger(index)) throw unsupported('tuple index', 'the positional index exceeds the exact supported integer range', { start: number.start, end: number.end });
          const recordProjection=readRustRecordProjection(this,value,number.raw,path,{start:value.span?.start??token.start,end:number.end});
          if(recordProjection){value=recordProjection;continue;}
          value = this.origins.project({ k: 'tupleIndex', object: value, index, span: { start: value.span?.start ?? token.start, end: number.end } }, path, index);
          continue;
        }
        const method = c.identifier('method');
        const ascii = readRustAsciiTextPredicate(this, value, method, path);
        if (ascii) { value = ascii; continue; }
        if (!c.is('(')) {
          const recordProjection=readRustRecordProjection(this,value,method.value,path,span(token,method));
          if(recordProjection){value=recordProjection;continue;}
          throw unsupported('field access', `field ${method.value} is outside the portable core`, span(token, method));
        }
        c.next();
        const args = [];
        while (!c.is(')')) {
          args.push(this.expr(path));
          if (!c.eat(',')) break;
        }
        c.expect(')', method.value);
        value = this.method(value, method, args, span(token, c.peek()),path);
        continue;
      }
      if (c.is('?')) throw unsupported('? operator', 'Result and Option are outside the portable core', span(token, token));
      if (c.is('[')) throw unsupported('indexing', 'collections are outside the portable core', span(token, token));
      return value;
    }
  }

  tupleIndexToken() {
    const c = this.cursor;
    const token = c.peek();
    const parts = token.raw.split('.');
    if (parts.length === 2 && parts.every(acceptRustTupleIndexSpelling)) {
      const middle = token.start + parts[0].length;
      c.tokens.splice(c.index, 1,
        { kind: 'number', value: parts[0], suffix: '', raw: parts[0], start: token.start, end: middle },
        { kind: 'punct', value: '.', raw: '.', start: middle, end: middle + 1 },
        { kind: 'number', value: parts[1], suffix: '', raw: parts[1], start: middle + 1, end: token.end });
    }
    const number = c.next();
    if (!acceptRustTupleIndexSpelling(number.raw)) {
      throw unsupported('tuple index spelling', 'Rust tuple indices are decimal digits without leading zeros, separators or suffixes', { start: number.start, end: number.end });
    }
    return number;
  }

  acceptFloatReceiverOrigin(origin, argumentsCount) {
    if(this.primitiveOwnerModel&&origin.primitiveOwnerPosition?.resolvedPath.length>1){
      const actual=readRustStandardPrimitivePosition(this,origin.primitiveOwnerPosition.sourcePath,origin.path);
      return Boolean(actual)&&acceptRustFloatPredicateReceiver(origin.type?.kind??'',origin.floatSourceModel??'',origin.proven,origin.wrappers.length>0,false,String(argumentsCount));
    }
    const owner = this.resolve(['f64'], origin.path);
    const shadowed = Boolean(this.origins.lookup(['f64'], origin.path) || this.origins.lookup(owner, origin.path));
    return acceptRustFloatPredicateReceiver(origin.type?.kind ?? '', origin.floatSourceModel ?? '', origin.proven, origin.wrappers.length > 0, shadowed, String(argumentsCount));
  }

  scalarLoopArithmetic(node, path) {
    if (!this.scalarLoopActive) return node;
    const left = this.origins.origin(node.left, path), right = this.origins.origin(node.right, path);
    return this.acceptFloatReceiverOrigin(left, 0) && this.acceptFloatReceiverOrigin(right, 0)
      ? this.origins.mark(node, {...left, type: FLOAT}) : node;
  }

  method(receiver, method, args, range, path=this.modulePath) {
    const sharedMethod=readRustSharedArrayHandleMethod(this,receiver,method,args,range);
    if(sharedMethod)return sharedMethod;
    const inherent=readRustInherentMethod(this,receiver,method,args,range,path);
    if(inherent)return inherent;
    const discovery=acceptRustTypeNamespaceMethodDiscovery(this.typeNamespaceDiscovery,method.value);
    if (!acceptRustTypeNamespaceBodyMethod(Boolean(this.typeNamespaceModel)&&!discovery, method.value, acceptRustUnitCloneReceiver(this,receiver))) throw unsupported('Rust type namespace body method', `${method.value} requires a separate actual method owner proof`, range);
    const expect = (count) => {
      if (args.length !== count) throw new TranslationError('type', `${method.value} expects ${count} arguments but got ${args.length}`, range);
    };
    const floatOperation = acceptRustFloatOperationMethod(method.value);
    if (floatOperation || acceptRustFloatPredicateMethod(method.value)) {
      expect(0);
      const origin = this.origins.origin(receiver, this.modulePath);
      if (!this.origins.collecting && !this.acceptFloatReceiverOrigin(origin, args.length)) {
        throw unsupported(floatOperation ? 'Rust binary64 operation' : 'Rust binary64 predicate', 'the receiver needs a proven direct unshadowed primitive f64 source type', range);
      }
      const expression = floatOperation ? createRustFloatOperationExpression(method.value, receiver, range)
        : createRustFloatPredicateExpression(method.value, receiver, range);
      // The operation result retains the admitted primitive owner's evidence;
      // a generic inferred FLOAT never establishes a source receiver proof.
      return this.origins.mark(expression, floatOperation ? { ...origin, type: FLOAT }
        : this.origins.typeOrigin(BOOL, this.modulePath));
    }
    const mapOperation = readStringMapOperation('Rust', method.value);
    if (method.value === 'is_empty') {
      const query = readRustStringEmptyQuery(this, receiver, args, range);
      if (query) return query;
    }
    if (['len', 'is_empty'].includes(method.value)) {
      const origin = this.origins.origin(receiver, this.modulePath);
      if (!origin.arraySourceModel) throw unsupported(`method ${method.value}`, 'outside the portable core', range);
      expect(0);
      if (!origin.proven || origin.type?.kind !== 'array' || !origin.arraySourceModel) throw unsupported('Rust array read', 'read-only length needs a proven vector or immutable slice source', range);
      const length = { k: 'length', object: receiver, integer: false, span: range };
      let node = length;
      if (method.value === 'len') {
        const c = this.cursor;
        if (!c.is('as') || !c.is('f64', 1)) throw unsupported('Rust array length', 'the selected Number length model requires an explicit as f64 conversion', range);
        c.next(); c.next();
        this.requireCastFollowingToken(false);
      } else node = { k: 'binary', op: 'eq', left: length, right: { k: 'num', value: '0', type: FLOAT, span: range }, span: range };
      return this.origins.mark(node, { type: method.value === 'len' ? FLOAT : BOOL, floatSourceModel: method.value === 'len' ? 'binary64 scalar' : null, path: [...this.modulePath], wrappers: [], textSourceModel: null, fields: null, proven: true, copyEvidence: true });
    }
    if (mapOperation) {
      expect(0);
      return { k: 'stringMap', op: mapOperation, object: receiver, span: range };
    }
    switch (method.value) {
      case 'starts_with':
      case 'ends_with':
      case 'contains':
        expect(1);
        return { k: 'stringTest', op: readStringTestOperation('Rust', method.value), object: receiver, search: args[0], span: range };
      case 'to_lowercase':
      case 'to_uppercase':
        expect(0);
        return { k: 'stringMap', op: readStringMapOperation('Rust', method.value), object: receiver, span: range };
      case 'clone':
      case 'to_owned': {
        expect(0);
        const origin = this.origins.origin(receiver, this.modulePath);
        if(this.localValueStateModel){
          if(this.localValueStateModel!==RUST_OWNED_LOCAL_STATE_POLICY.models.copies||method.value!=='clone'||!['name','tupleIndex'].includes(receiver.k)||origin.type?.kind!=='string'||origin.wrappers.length||origin.textSourceModel!=='owned string')throw unsupported('Rust owned local state','only actual standard String clone is admitted by the canonical copy interpretation',range);
          return this.origins.mark({...receiver,span:{start:receiver.span?.start??range.start,end:range.end}},{...origin,localValueCopy:true});
        }
        if (origin.arraySourceModel) throw unsupported('Rust array clone', 'array cloning needs a separate source allocation and ownership proof', range);
        if (origin.wrappers[0] === 'reference') origin.wrappers.shift();
        if (method.value === 'to_owned' && origin.type?.kind === 'string') origin.textSourceModel = 'owned string';
        return this.origins.mark(receiver, origin);
      }
      case 'as_str':
        expect(0);
        if(this.localValueStateModel&&!acceptRustOwnedLocalSourceOperation('borrow'))throw unsupported('Rust owned local state','source indirection requires a separate ownership interpretation',range);
        if (this.origins.origin(receiver, this.modulePath).arraySourceModel) throw unsupported('method as_str', 'outside the portable core', range);
        return this.origins.mark(receiver, { ...this.origins.origin(receiver, this.modulePath), type: STRING, textSourceModel: 'string slice', path: [...this.modulePath], wrappers: ['reference'] });
      case 'to_string':
        expect(0);
        return { k: 'toString', arg: receiver, span: range };
      case 'div_euclid':
      case 'rem_euclid':
        expect(1);
        this.origins.direct(args[0], this.modulePath, args[0].span);
        return this.origins.mark({ k: 'binary', op: method.value === 'div_euclid' ? 'div' : 'rem', rounding: 'euclid', left: receiver, right: args[0], span: range }, { ...this.origins.origin(receiver, this.modulePath), wrappers: [] });
      default:
        throw unsupported(`method ${method.value}`, 'outside the portable core', range);
    }
  }

  primary(path, options) {
    const c = this.cursor;
    const token = c.peek();
    if (token.kind === 'number') {
      c.next();
      const node = { k: 'num', value: token.value, span: span(token, token) };
      if (token.suffix) {
        const type = token.suffix === 'f64' ? FLOAT : rustFixedType(token.suffix);
        if (!type) throw unsupported(`literal suffix ${token.suffix}`, 'portable numeric literals are integers and IEEE-754 binary64 values', span(token, token));
        node.type = type;
      } else if (token.floating) node.type = FLOAT;
      if (token.suffix === 'f64') this.origins.mark(node, { ...this.origins.origin(node, path), floatSourceModel: 'binary64 scalar' });
      return node;
    }
    if (token.kind === 'string') {
      c.next();
      return this.origins.mark({ k: 'str', value: token.value, span: span(token, token) }, { type: STRING, textSourceModel: 'string slice', sourceOwner: { kind: 'primitive', path: ['str'], arguments: [] }, path: [...path], wrappers: ['reference'] });
    }
    if (token.kind === 'macro') {
      c.next();
      return this.expressionMacro(token, path);
    }
    const characterTypeName = readRustCharacterExpressionType(token.kind);
    if (characterTypeName) {
      const type = rustFixedType(characterTypeName);
      if (!type) throw unsupported(`character literal type ${characterTypeName}`, 'a character expression needs its represented fixed type', span(token, token));
      c.next();
      return { k: 'num', value: token.value, type, span: span(token, token) };
    }
    if(token.kind==='byteString')throw unsupported('Rust byte string literal','the actual static byte-array reference and its ownership must be represented',span(token,token));
    if (token.kind === 'char' || token.kind === 'byteChar' || c.is('\'')) throw unsupported('character literal', 'characters are outside the portable core', span(token, token));
    if (c.is('(')) {
      c.next();
      if (c.eat(')')) return { k: 'unit', span: span(token, token) };
      const inner = this.expr(path);
      if (c.is('..=')) return readRustInclusiveRangeQuery(this, token, path, inner);
      if (!c.eat(',')) {
        c.expect(')', 'parenthesised expression');
        return inner;
      }
      const items = [inner];
      while (!c.is(')')) {
        items.push(this.expr(path));
        if (!c.eat(',')) break;
      }
      const close = c.expect(')', 'tuple expression');
      return this.origins.product({ k: 'tuple', items, ...(this.localValueStateModel?{}:{frozen:true,productModel:'immutable value product'}),span: { start: token.start, end: close.end } }, path);
    }
    if (c.is('{')) {
      const block = this.block(path);
      return this.origins.mark({ ...block, block: true }, this.origins.origin(block, path));
    }
    if (c.is('if')) return this.ifExpr(path);
    if (c.is('match')) return this.match(path);
    if (c.is('|') || c.is('||') || c.is('move')) throw unsupported('closure', 'higher-order values are outside the portable core', span(token, token));
    if (c.is('true') || c.is('false')) {
      c.next();
      return { k: 'bool', value: token.value === 'true', span: span(token, token) };
    }
    if (c.is('unsafe') || c.is('loop') || c.is('while') || c.is('for') || c.is('return') || c.is('break')) {
      throw unsupported(`${token.value} expression`, 'outside the portable core', span(token, token));
    }
    if (token.kind === 'identifier') {
      const segments = this.pathSegments('expression');
      const name = segments.join('::');
      const resolved = this.resolve(segments, path);
      if(this.primitiveOwnerModel&&segments.length>1&&acceptRustFloatConstantName(segments.at(-1))){
        const owner=readRustStandardPrimitivePosition(this,segments.slice(0,-1),path);
        if(owner){
          const range={start:token.start,end:c.tokens[c.index-1].end};
          if(c.is('(')||this.origins.scope.has(segments[0]))throw unsupported('Rust primitive constant owner','an associated constant needs its actual unshadowed type path and value syntax',range);
          const node=createRustFloatConstantExpression(segments.at(-1),range);
          this.primitiveConstantSites.push({owner,name:segments.at(-1),span:range});
          return this.origins.mark(node,{...this.origins.typeOrigin(FLOAT,path),sourceOwner:{kind:'primitive',path:['f64'],arguments:[]},primitiveOwnerPosition:owner});
        }
      }
      if (segments.length === 2 && segments[0] === 'f64' && acceptRustFloatConstantName(segments[1])) {
        const range = { start: token.start, end: c.tokens[c.index - 1].end };
        const owner = this.resolve(['f64'], path);
        const shadowed = Boolean(this.origins.lookup(['f64'], path) || this.origins.lookup(owner, path));
        if (!acceptRustFloatConstantOwner(owner.join('::'), shadowed) || c.is('(')) throw unsupported('Rust binary64 constant', 'associated constants require the exact unshadowed primitive f64 owner and value syntax', range);
        return this.origins.mark(createRustFloatConstantExpression(segments[1], range), this.origins.typeOrigin(FLOAT, path));
      }
      if (c.is('::') && c.is('<', 1) && (acceptRustSourceArrayOwner(name, false) || acceptRustSourceArrayOwner(resolved.join('::'), false))) {
        refuseRustSharedArrayBareVector(this,span(token,c.peek()));
        this.origins.vector(segments, resolved, path, span(token, c.peek()));
        c.next(); c.next();
        const elementToken = c.peek();
        const element = this.type();
        this.arrayElement(token, elementToken);
        c.expect('>', 'Vec constructor'); c.expect('::', 'Vec constructor'); c.expect('new', 'Vec constructor');
        c.expect('(', 'Vec constructor'); c.expect(')', 'Vec constructor');
        return this.origins.mark({ k: 'array', items: [], element, span: span(token, c.peek()) }, { type: array(element), path: [...path], wrappers: [], fields: null, textSourceModel: null, arraySourceModel: 'owned vector', proven: true });
      }
      if (c.is('::') && c.is('<', 1)) throw unsupported('turbofish', 'generic functions are outside the portable core', span(token, c.peek()));
      if (c.is('{') && (/^[A-Z]/u.test(segments.at(-1)) || this.hasRecordConstructions) && !options.noStruct) {
        return readRustRecordConstruction(this,segments,path,token);
      }
      const sharedConstruction=readRustSharedArrayHandleConstructor(this,segments,token,path);
      if(sharedConstruction)return sharedConstruction;
      const recordValuePath=readRustRecordValuePath(this,segments,path,c.is('('));
      if (!c.is('(')) return noteRustSharedArrayHandleName(this,{ k: 'name', path: recordValuePath??this.resolve(segments, path), span: span(token, token) },path);
      c.next();
      const args = [];
      while (!c.is(')')) {
        args.push(this.expr(path));
        if (!c.eat(',')) break;
      }
      c.expect(')', 'call');
      const range = span(token, c.peek());
      if(recordValuePath){
        this.origins.call(recordValuePath,args,path,range);
        return args.length?{k:'app',fn:{k:'name',path:recordValuePath,span:span(token,token)},args,span:range}:{k:'name',path:recordValuePath,span:range};
      }
      const sharedCall=readRustSharedArrayHandleCall(this,segments,args,path,range);
      if(sharedCall)return sharedCall;
      const emptyString = readRustEmptyStringConstructor(this, segments, args, path, range);
      if (emptyString) return emptyString;
      const resolvedOwner = segments.length > 1 ? this.resolve(segments.slice(0, -1), path) : [];
      if (segments.at(-1) === 'new' && (acceptRustSourceBoxOwner(segments.slice(0, -1).join('::'), false) || acceptRustSourceBoxOwner(resolvedOwner.join('::'), false))) {
        if(this.localValueStateModel&&!acceptRustOwnedLocalSourceOperation('box construction'))throw unsupported('Rust owned local state','Box allocation requires a separate source ownership interpretation',range);
        if (args.length !== 1) throw new TranslationError('type', 'Box::new expects 1 argument', range);
        this.origins.box(segments.slice(0, -1), resolvedOwner, path, range);
        return this.origins.wrap(args[0], path, 'box');
      }
      if (name === 'String::from') {
        if (!acceptRustTypeNamespaceBodyConstructor(Boolean(this.typeNamespaceModel), name)) throw unsupported('Rust type namespace body constructor', 'String::from requires a separate actual owner and argument proof', range);
        if (args.length !== 1) throw new TranslationError('type', 'String::from expects 1 argument', range);
        const expression = this.origins.conversion({ k: 'toString', arg: args[0], span: range }, path);
        if (this.localValueStateModel) this.origins.mark(expression, {
          ...this.origins.origin(expression, path), localValueConstructor: 'String::from',
          localValueConstructorSpan: { start: token.start, end: c.peek(-1).end },
        });
        return expression;
      }
      if (segments.length === 2 && segments[1] === 'from' && rustFixedType(segments[0])) {
        if (!acceptRustTypeNamespaceBodyConstructor(Boolean(this.typeNamespaceModel), name)) throw unsupported('Rust type namespace body constructor', `${name} requires a separate actual owner and argument proof`, range);
        if (args.length !== 1) throw new TranslationError('type', `${name} expects 1 argument`, range);
        return { k: 'cast', arg: args[0], to: rustFixedType(segments[0]), flavor: 'exact', span: range };
      }
      if (segments.length === 2 && rustFixedType(segments[0])) {
        throw unsupported(`${name}`, 'outside the portable core', range);
      }
      if (['std', 'core', 'alloc'].includes(segments[0])) throw unsupported(name, 'the standard library is outside the portable core', range);
      this.origins.call(this.resolve(segments, path), args, path, range);
      if (!args.length) return { k: 'name', path: this.resolve(segments, path), span: range };
      return { k: 'app', fn: { k: 'name', path: this.resolve(segments, path), span: span(token, token) }, args, span: range };
    }
    throw this.fail('expected an expression');
  }

  expressionMacro(token, path) {
    if (token.value === 'matches') return readRustLiteralMatch(this, token, path);
    const args = this.macroArguments(token, path);
    const range = span(token, this.cursor.peek());
    switch (token.value) {
      case 'vec': {
        for (const arg of args) {
          const origin = this.origins.origin(arg, path);
          if (origin.wrappers.length || (origin.type && !acceptRustSourceArrayElement(origin.type.kind, false))) throw unsupported('Rust array element', 'only direct immutable scalar and owned string elements are admitted', range);
        }
        const element = args.length ? this.origins.origin(args[0], path).type : null;
        return noteRustSharedArrayLiteral(this,this.origins.mark({ k: 'array', items: args.map(value => ({ value, spread: false })), span: range }, { type: element ? array(element) : null, path: [...path], wrappers: [], fields: null, textSourceModel: null, arraySourceModel: 'owned vector', proven: true }),path);
      }
      case 'format': {
        const formatted=this.format(args,token,path);
        return this.origins.mark({...formatted,span:range},this.origins.origin(formatted,path));
      }
      case 'panic':
      case 'unreachable':
      case 'unimplemented':
      case 'todo': {
        const message = readRustCompletionPanicMessage(this.completionModel, token.value, args);
        if (message !== null) return { k: 'abort', message, span: range };
        if (args.length > 1) throw unsupported(`${token.value}! arguments`, 'only a literal message is portable', range);
        if (args.length && args[0].k !== 'str') throw unsupported(`${token.value}! message`, 'the message must be a literal', range);
        return { k: 'abort', message: rustMacroMessage(token.value, args.length ? panicText(args[0]) : undefined), span: range };
      }
      default:
        throw unsupported(`${token.value}!`, 'outside the portable core', range);
    }
  }

  ifExpr(path) {
    const c = this.cursor;
    const token = c.next();
    if(hasRustConditionalChain(this))return readRustConditionalChain(this,path,token);
    if (c.is('let')) return readRustConditionalPattern(this,path,token);
    const cond = this.expr(path, { noStruct: true });
    this.origins.direct(cond, path, cond.span);
    const then = this.block(path);
    if (!c.eat('else')) throw unsupported('if without else', 'the value of an if must be defined on both branches', span(token, c.peek()));
    const otherwise = c.is('if') ? this.ifExpr(path) : this.block(path);
    return { k: 'if', cond, then, else: otherwise, span: span(token, c.peek()) };
  }

  match(path) {
    const c = this.cursor;
    const token = c.next();
    const scrutinee = this.expr(path, { noStruct: true });
    c.expect('{', 'match');
    const rows = [],guardedArms=[];
    while (!c.is('}')) {
      const armStart = c.peek();
      const aliases = [];
      c.eat('|');
      const alternatives = [this.pattern(path, aliases)];
      while (c.eat('|')) alternatives.push(this.pattern(path, aliases));
      const previous = this.origins.enter(this.origins.cases(alternatives, scrutinee, path));
      let body,bodyOrigin,guard;
      try{
        for (const alias of aliases) this.origins.bind(alias.name, alias.value, path);
        guard=c.eat('if')?this.expr(path):null;
        if(guard)this.origins.direct(guard,path,guard.span);
        c.expect('=>','match arm');body=this.expr(path);bodyOrigin=this.origins.origin(body,path);
      }finally{this.origins.scope=previous;}
      if (body.block || body.k === 'if' || body.k === 'match') c.eat(',');
      else if (!c.is('}')) c.expect(',', 'match arm');
      if (alternatives.length > 1 && aliases.length) {
        throw unsupported('binding in an or-pattern', 'bind the value in each alternative separately', span(armStart, c.peek()));
      }
      guardedArms.push({patterns:alternatives,body,aliases:[...aliases],guard,origin:bodyOrigin,span:span(armStart,c.peek())});
      for (const alias of [...aliases].reverse()) body = { k: 'let', name: alias.name, value: alias.value, body, span: body.span };
      this.origins.mark(body, bodyOrigin);
      // `A | B => e` is one row per alternative with the same body.
      for (const pattern of alternatives) rows.push({ patterns: [pattern], body, span: span(armStart, c.peek()) });
    }
    c.expect('}', 'match');
    if (!rows.length) throw unsupported('empty match', 'uninhabited types are outside the portable core', span(token, c.peek()));
    if(guardedArms.some(arm=>arm.guard))return lowerRustGuardedMatch(this,scrutinee,guardedArms,path,span(token,c.peek()));
    const origins = rows.map(row => this.origins.origin(row.body, path));
    const result = { k: 'match', scrutinees: [scrutinee], rows, span: span(token, c.peek()) };
    return this.origins.mark(result, this.origins.join(origins));
  }

  /** Patterns: `_`, bindings, literals, `Enum::Variant(p, …)`, `&p` and `name @ p`. */
  pattern(path, aliases) {
    const c = this.cursor;
    const token = c.peek();
    if (c.eat('&')) return this.origins.referencePattern(this.pattern(path, aliases));
    if (c.eat('_')) return this.recordPatternDepth?{k:'wild',span:{start:token.start,end:token.end}}:{k:'wild'};
    if (c.is('ref') || c.is('mut')) throw unsupported(`${token.value} binding`, 'bindings are by value in the portable core', span(token, token));
    if (c.is('-') && c.isKind('number', 1)) {
      c.next();
      const number = c.next();
      return { k: 'numLit', value: number.value, negative: true, span: span(token, number) };
    }
    if (token.kind === 'number') {
      c.next();
      if (c.is('..') || c.is('..=')) throw unsupported('range pattern', 'ranges are outside the portable pattern language', span(token, c.peek()));
      return { k: 'numLit', value: token.value, span: span(token, token) };
    }
    if (token.kind === 'char' || token.kind === 'byteChar') throw unsupported('character pattern', 'characters are outside the portable core', span(token, token));
    if (token.kind === 'string') throw unsupported('string pattern', 'match on &str is outside the Rust frontend', span(token, token));
    if (c.is('true') || c.is('false')) {
      c.next();
      return { k: 'boolLit', value: token.value === 'true', span: span(token, token) };
    }
    if (c.eat('(')) {
      const args=[];let rest=null,comma=false;
      this.recordPatternDepth+=1;
      try{
        while(!c.is(')')){
          if(c.eat('..')){
            if(rest!==null)throw unsupported('tuple rest pattern','a tuple pattern has at most one rest position',span(token,c.peek()));
            rest=args.length;
          }else args.push(this.pattern(path,aliases));
          if(!c.eat(','))break;
          comma=true;
        }
        c.expect(')','tuple pattern');
      }finally{this.recordPatternDepth-=1;}
      return args.length===1&&rest===null&&!comma?args[0]:{k:'tuple',args,...(rest===null?{}:{rest}),span:span(token,c.peek())};
    }
    if (token.kind !== 'identifier') throw unsupported('pattern', `${describe(token)} is outside the portable pattern language`, span(token, token));
    const segments = this.pathSegments('pattern');
    if (c.eat('@')) {
      if (segments.length !== 1) throw this.fail('expected a binding before @', token);
      const inner = this.pattern(path, aliases);
      aliases.push({ name: segments[0], value: patternValue(inner, span(token, c.peek())) });
      return inner;
    }
    if (c.is('{')) return readRustRecordPattern(this,segments,path,aliases,token);
    const resolved = readRustRecordValuePath(this,segments,path,c.is('('))??this.resolve(segments, path);
    if (c.eat('(')) {
      const args = [];
      let rest=null;
      while (!c.is(')')) {
        if(c.is('..')){
          if(rest)throw unsupported('constructor rest pattern','a constructor pattern has at most one rest position',span(token,c.peek()));
          rest={index:args.length,token:c.next()};
        }else args.push(this.pattern(path, aliases));
        if (!c.eat(',')) break;
      }
      c.expect(')', 'pattern');
      return { k: 'ctor', path: resolved, args:expandRustConstructorRest(this,resolved,args,rest,path), span: span(token, c.peek()) };
    }
    if (segments.length === 1 && resolved.length === 1) return { k: token.raw.startsWith('r#')?'bind':'bindOrCtor', name: segments[0], span: span(token, token) };
    return { k: 'ctor', path: resolved, args: [], span: span(token, token) };
  }
}

/** An imported path from the crate root, so it means the same wherever it is used. */
function absolute(segments, path, range) {
  if (segments[0] === 'crate') return segments;
  if (segments[0] === 'self') return ['crate', ...path, ...segments.slice(1)];
  let up = 0;
  while (segments[up] === 'super') up += 1;
  if (up > path.length) throw new TranslationError('type', 'super beyond crate root', range);
  return ['crate', ...path.slice(0, path.length - up), ...segments.slice(up)];
}

function propOf(expr) {
  if (expr.k === 'binary' && ['eq', 'ne', 'lt', 'le', 'gt', 'ge'].includes(expr.op)) {
    return { p: expr.op, left: expr.left, right: expr.right };
  }
  if (expr.k === 'binary' && (expr.op === 'and' || expr.op === 'or')) return { p: expr.op, left: propOf(expr.left), right: propOf(expr.right) };
  if (expr.k === 'unary' && expr.op === 'not') return { p: 'not', arg: propOf(expr.arg) };
  return { p: 'bool', expr };
}

/** The value a pattern matched, for `name @ p`. */
function patternValue(pattern, range) {
  switch (pattern.k) {
    case 'bindOrCtor':
      return { k: 'name', path: [pattern.name], span: range };
    case 'numLit':
      return { k: 'num', value: pattern.value, negative: pattern.negative, span: range };
    case 'boolLit':
      return { k: 'bool', value: pattern.value, span: range };
    case 'ctor':
      if (!pattern.args.length) return { k: 'name', path: pattern.path, span: range };
      return { k: 'app', fn: { k: 'name', path: pattern.path, span: range }, args: pattern.args.map((arg) => patternValue(arg, range)), span: range };
    default:
      throw unsupported('@ binding', 'the aliased pattern must bind every field', range);
  }
}

function joined(left, right, token) {
  return { start: left.span?.start ?? token.start, end: right.span?.end ?? token.end };
}

function span(from, to) {
  return { start: from.start, end: Math.max(from.end, to && to.kind !== 'eof' ? to.start : from.end) };
}

/** The text a `panic!` message literal prints: `{{` and `}}` are braces; a lone brace captures a variable. */
function panicText(template) {
  let text = '';
  for (let index = 0; index < template.value.length; index += 1) {
    const char = template.value[index];
    if (char === '{' || char === '}') {
      if (template.value[index + 1] !== char) throw unsupported('formatted panic message', 'only a literal message is portable', template.span);
      index += 1;
    }
    text += char;
  }
  return text;
}
