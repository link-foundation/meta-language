import {checkJavaScriptOptionalChain} from './javascript-optional-chains.js';
import {checkJavaScriptOwnProperty} from './javascript-own-properties.js';
import {prepareJavaScriptRecordOrderProfile} from './javascript-record-order-profile.js';
import {checkJavaScriptObjectEnumeration} from './javascript-object-enumeration.js';
import {checkRecordOrderOperation,checkOrderedRecordOrderRead} from './record-order-operations.js';
import {constructRecordPropertyNameRows} from './ordered-record-emission.js';
import {checkJavaScriptNullishField,checkJavaScriptNullishAccess,checkJavaScriptNullishIndex} from './javascript-nullish-fields.js';
import {checkJavaScriptNamedProperty,readJavaScriptLiteralIndexSource,checkJavaScriptStringIndex} from './javascript-indexed-properties.js';
import {requireJavaScriptSliceReferenceCopy} from './javascript-nullish-field-rules.js';
import {readJavaScriptOptionalChainOperation} from './javascript-optional-chain-rules.js';
import {forgetJavaScriptOwnPropertyGuard,readJavaScriptOwnPropertyBranchEnvironments,checkJavaScriptGuardedRecordIndex,createJavaScriptOwnKeyCallbackEnvironment,installJavaScriptOwnKeyCallbackBindings} from './javascript-guarded-record-indices.js';
import {checkJavaScriptOptionalCall} from './javascript-optional-calls.js';
import {readJavaScriptOptionalCallOperation} from './javascript-optional-call-rules.js';
import {captureRustRuntimeShell,checkRustRuntimeShellContainers} from './rust-runtime-shell.js';
import { acceptOptionalAnnotationTypes } from './optional-inference.js';
import {checkRustPureUnitCalls} from './rust-pure-unit-calls.js';
import {checkRustEmptyOptionProof} from './rust-empty-options.js';
// Resolves names, checks types, and fixes the exact operator semantics of a
// surface program produced by one of the language frontends. The result is
// the portable-core program the emitters consume: every expression carries
// its type and every operator carries the source language's semantics, so a
// target can never silently substitute its own (for example, Lean's Euclidean
// `Int` division for JavaScript's truncating BigInt division).

import { typeError, unsupported } from './diagnostics.js';
import {acceptLinkedJavaScriptSourceException,readJavaScriptExceptionConstructorClass} from './source-exception-rules.js';
import {checkJavaScriptSourceException,checkJavaScriptSourceExceptionValue} from './source-exceptions.js';
import {acceptLinkedJavaScriptBigIntegerResult} from './big-integer-conversion-rules.js';
import {checkJavaScriptBigIntegerBuiltin,checkJavaScriptBigIntegerResult,materializeJavaScriptBigIntegerDivision,checkJavaScriptExponentiation} from './javascript-big-integer-conversion.js';
import {materializeJavaScriptNumericFailure} from './javascript-numeric-failures.js';
import {materializeJavaScriptPrimitiveComparison,checkJavaScriptLooseComparison,checkJavaScriptSameValue} from './javascript-comparisons.js';
import {acceptJavaScriptSameValueOperation,acceptJavaScriptLooseEqualityOperation} from './javascript-comparison-rules.js';
import {acceptLinkedJavaScriptStringUnitConstruction,acceptJavaScriptStringUnits} from './javascript-string-unit-rules.js';
import {acceptLinkedJavaScriptStringConstruction} from './string-construction-rules.js';
import {checkJavaScriptStringConstruction,checkJavaScriptStringConstructionValue} from './javascript-string-construction.js';
import {readJavaScriptNumberConversionResult,readJavaScriptPrimitiveNumberOperand,readJavaScriptMathematicalConversionForm} from './string-number-rules.js';
import {checkJavaScriptNumberBuiltin,materializeJavaScriptNumberConversion,checkJavaScriptMathematicalArguments} from './javascript-number-conversion.js';
import { captureArraySourceOperands, wrapArraySourceOperands } from './array-source-operands.js';
import { readSourceIdentifierSupply, collectSourceIdentifierTexts } from './source-identifiers.js';
import { renderSourceGeneratedIdentifier } from './source-identifier-rules.js';
import { planTuplePatternFields } from './rust-record-rules.js';
import { checkJavaScriptSourcePropertyCall } from './javascript-property-calls.js';
import {readJavaScriptNumberBitwiseArity} from './number-bitwise-rules.js';
import { readBigIntegerBitwiseOperation, readBigIntegerBitwiseArity, readPrimitiveBitwiseDomain } from './big-integer-bitwise-rules.js';
import { readRustPrimitiveBitwiseArity, readRustPrimitiveBitwiseOperation } from './rust-primitive-bitwise-rules.js';
import { checkRustPrimitiveBitwise } from './rust-primitive-bitwise.js';
import { readRustShiftCheckedOperation } from './rust-shift-rules.js';
import { checkRustShift } from './rust-shifts.js';
import { readFixedIntegerCastFlavor } from './fixed-integer-cast-rules.js';
import { acceptSemanticBranchContextReordering } from './semantic-input-rules.js';
import { checkReferenceMethod } from './reference-methods.js';
import { checkRecordFieldWrite, checkSourceRecordFieldWrite } from './record-writes.js';
import { checkArrayStateMethod } from './array-state-methods.js';
import { readCollectionAccessArgumentCount } from './collection-access-rules.js';
import {acceptReferenceIdentityComparison} from './reference-identity-rules.js';
import {readRustSharedArrayHandleComparisonProjection} from './rust-shared-array-handle-rules.js';
import { checkCollectionConstruction } from './collection-construction.js';
import { readJavaScriptArrayClassificationPlan } from './array-classification-rules.js';
import { selectOptionalArrayIndexNumber, selectOptionalArrayRead } from './array-value-rules.js';
import { readArrayFromArgumentError, readArrayFromMapCallbackError, readArrayFromReceiverError } from './array-from-rules.js';
import { readArrayFromExpectedElement, readArrayFromIterableType, readArrayFromMapCallback } from './array-from.js';
import { readArrayCallbackResultForm, readArrayCallbackSignatureError, readArrayCallbackParameterRole,
  readArrayCallbackArgumentCountError, readArrayCallbackCheckedResultContext,
  readArrayCallbackResultError, readArrayCallbackCheckedArgumentPosition } from './array-callback-rules.js';
import { acceptJavaScriptArrayRestReceiver } from './array-binding-rules.js';
import { checkJavaScriptArrayBindingWrapper } from './array-binding-validation.js';
import {checkJavaScriptBindingDefault} from './javascript-binding-defaults.js';
import {checkJavaScriptBooleanBuiltin,checkJavaScriptArrayBuiltin} from './javascript-boolean-conversion.js';
import {checkJavaScriptPrimitiveTextBuiltin} from './javascript-string-search.js';
import { createStringCodeUnitTemplateExpression } from './string-code-unit-template.js';
import { requirePartialCallableResultInference } from './partial-callable-rules.js';
import { constructCallableFunctionType, readCallableCellElement, readCallableParameterTypes, readCallableLambdaInference } from './callable-source.js';
import { acceptCallableFunctionReference, readCallableApplicationError, readCallableCellError, requireCallableComputedApplication } from './callable-source-rules.js';
import { checkRustSourceSignature } from './rust-source-signatures.js';
import { validateRustTypeNamespace, checkRustTypeNamespaceRecordShapes } from './rust-type-namespace.js';
import {prepareRustPrimitiveOwnerProof,checkRustPrimitiveOwnerProof} from './rust-primitive-owners.js';
import {prepareRustOwnedLocalStateProof,checkRustOwnedLocalStateProof,checkRustOwnedLocalStateContainers} from './rust-owned-local-state.js';
import {checkRustConstFunctionProof,checkRustConstFunctionContainers} from './rust-const-functions.js';
import {checkRustDiagnosticAttributePlacement} from './rust-diagnostic-attributes.js';
import { acceptRustNumericLiteralNegation } from './rust-character-rules.js';
import { acceptArgumentCount, acceptCheckedTypeQueryOperand, acceptTypeQueryOperand, readTypeQueryResult, acceptJavaScriptOptionalTupleContext } from './frontend-rules.js';
import { readParameterDefaultValueStep, requireParameterDefaultArgumentBindings } from './parameter-default-rules.js';
import { normaliseProof } from './proof.js';
import { referenceOperation } from './reference-model.js';
import { readSurfaceReferenceResultField, selectSurfaceReferenceAllocation, selectSurfaceReferenceRead, readSurfaceReferencePayloadOperand, acceptSurfaceReferencePayloadField, readSurfaceReferencePayloadError } from './surface-reference-analysis-rules.js';
import { acceptTupleIndex, readNumberDisplayRefusal, readScalarCellOperation, readTuplePosition } from './primitive-value-rules.js';
import { checkRustScalarLoopProof } from './rust-scalar-loops.js';
import { acceptOptionalAbsence, mergeOptionalAbsence } from './structured-type-rules.js';
import { acceptOptionalLiteralAbsence, acceptOptionalPrimitiveKind, readOptionalLogicalOperation, readOptionalComparisonOperation, readOptionalBooleanOperation, readOptionalCoercionOperation, readOptionalFieldOperation, readOptionalNumberOperation, requireOptionalCoalescing, requireOptionalFlattening } from './optional-value-rules.js';
import { readRecordValueOrder, readRecordProjectionError, readRecordFieldPosition, readRecordFieldOrder, readRecordFieldInsertionOrder, readRecordMethodError } from './record-value-rules.js';
import { readPrimitiveCopySliceForm, readPrimitiveMethodMinimumArguments, readPrimitiveMethodMaximumArguments,
  readPrimitiveMethodArgumentCount, readPrimitiveMethodCheckedArgumentKind, readPrimitiveMethodDefaultForm,
  readPrimitiveMethodDefaultValue, readPrimitiveMethodCheckedResultForm, readPrimitiveMethodArgumentObligation,
  readPrimitiveMethodLoweringForm, readPrimitiveMethodArgumentObligationFeature } from './primitive-method-rules.js';
import { acceptProgramTupleProductModel, acceptProgramExpressionOperation } from '../typed-program-link-rules.js';
import {readStringOrderOperation} from './string-order-rules.js';
import {readNumberPredicateArgumentForm} from './number-predicate-rules.js';
import {
  BOOL, FLOAT, INT, NAT, STRING, UNIT, array, data, fixedBounds, immutableProductType, isFloat, isNatural, isNumeric, sameType, tuple, typeKey,
} from './types.js';

const COMPARISONS = new Set(['eq', 'ne', 'lt', 'le', 'gt', 'ge']);
const ARITHMETIC = new Set(['add', 'sub', 'mul', 'div', 'rem']);
// The styles of the values console.log shows: its argument, or those util.format reads.
const CONSOLE_STYLES = new Set(['js-console', 'js-format-first', 'js-format-number', 'js-format-integer']);
// Environment entry recording, for a local, the constructor it is known to
// have matched and the locals naming its fields.
const KNOWN = Symbol('known constructors');
const PRODUCT_VALUES = Symbol('known immutable product values');
/** Natural numeral patterns above this unfold to equality tests rather than successor chains. */
const NAT_PATTERN_UNFOLD = 16n;

// Conservative structural bottom classification. Unknown forms use the
// existing checker order; this predicate never invents an operand type.
function needsBranchResultContext(node, depth = 0) {
  if (!node || depth > 256) return false;
  if (node.k === 'abort') return true;
  if (node.k === 'let' || node.k === 'print' || node.k === 'cellWrite') return needsBranchResultContext(node.body, depth + 1);
  if (node.k === 'if') return needsBranchResultContext(node.then, depth + 1) && needsBranchResultContext(node.else, depth + 1);
  return false;
}

// The Math and Number functions whose value is a boolean.
const PREDICATES = new Set(['isInteger', 'isSafeInteger', 'isFinite', 'isNaN']);

/** Binds a local, forgetting constructor facts the new binding shadows. */
export function bindLocal(env, name, type, immutableProduct = false) {
  forgetJavaScriptOwnPropertyGuard(env,name);
  env.set(name, type);
  const products = env.get(PRODUCT_VALUES);
  if (immutableProduct || products?.has(name)) {
    const updated = new Map(products);
    if (immutableProduct) updated.set(name, true);
    else updated.delete(name);
    env.set(PRODUCT_VALUES, updated);
  }
  const known = env.get(KNOWN);
  if (known && [...known].some(([variable, fact]) => variable === name || fact.binds.includes(name))) {
    env.set(KNOWN, new Map([...known].filter(([variable, fact]) => variable !== name && !fact.binds.includes(name))));
  }
}

// A nested product needs a proof about the value returned, not only its type.
// Unknown parameter/call origins remain unsupported by this local proof.
function immutableProductValue(value, bindings = new Map()) {
  if (value.type.kind !== 'tuple') return true;
  switch (value.k) {
    case 'tuple':
      return value.frozen === true && value.items.every(item => immutableProductValue(item, bindings));
    case 'let':
      return immutableProductValue(value.body, new Map(bindings).set(value.name, immutableProductValue(value.value, bindings)));
    case 'if':
      return immutableProductValue(value.then, bindings) && immutableProductValue(value.else, bindings);
    case 'var':
      return bindings.get(value.name) === true;
    default:
      return false;
  }
}

export function checkProgram(surface) { return checkSurfaceProgram(surface, false); }

// Internal graph boundary: terms already carry normalized primitive operations.
// All core type/arity/ownership/proof checks still run; source-call desugaring
// must not run again and change the represented operation or fresh bindings.
export function checkProjectedProgram(surface) { return checkSurfaceProgram(surface, true); }

function checkSurfaceProgram(surface, checkedProjection) {
  const orderProfile=prepareJavaScriptRecordOrderProfile(surface);
  surface=orderProfile.surface;
  const sourceProofSelected=checkRustConstFunctionContainers(surface);
  if(sourceProofSelected)checkRustDiagnosticAttributePlacement(surface);
  checkRustRuntimeShellContainers(surface);
  checkRustTypeNamespaceRecordShapes(surface);
  checkRustOwnedLocalStateContainers(surface);
  const checker = new Checker(surface.language, surface.externals ?? [], Boolean(surface.referenceSemantics), checkedProjection);
  checker.orderedRecordSource=orderProfile.selected;
  return checker.program(surface);
}

// The linked checked-IR boundary reuses the source checker's pattern rules.
// Graph adapters restore lexical spellings before calling this operation.
export function readCheckedMatchPatternBindings(pattern, type, declarations) {
  if (!pattern || typeof pattern !== 'object' || !['wild', 'bind', 'natZero', 'natSucc', 'ctor'].includes(pattern.k)) throw typeError('invalid checked match pattern');
  if ((pattern.k === 'bind' || pattern.k === 'natSucc') && (typeof pattern.name !== 'string' || !pattern.name)) throw typeError('a checked pattern binding has no name');
  if (pattern.k === 'ctor') {
    if (pattern.data !== type?.name || type?.kind !== 'data') throw typeError('a checked constructor pattern belongs to another data type');
    if (typeof pattern.ctor !== 'string' || !pattern.ctor || !Array.isArray(pattern.binds)
      || pattern.binds.some(name => name !== null && (typeof name !== 'string' || !name || name === '_'))) throw typeError('a checked constructor pattern has invalid binding slots');
  }
  const checker = new Checker('JavaScript', []);
  checker.items = declarations;
  const environment = new Map();
  const surface = pattern.k === 'ctor' ? { ...pattern, path: [pattern.ctor] } : pattern;
  const checked = checker.pattern(surface, type, environment, [], undefined);
  const names = checked.k === 'ctor' ? checked.binds.filter(name => name !== null)
    : checked.k === 'bind' || checked.k === 'natSucc' ? [checked.name] : [];
  if (new Set(names).size !== names.length) throw typeError('a checked pattern binds the same name twice');
  return { pattern: checked, bindings: [...environment].map(([name, type]) => ({ name, type })) };
}

export function checkLinkedMatchCoverage(type, patterns, declarations, span) {
  checkExhaustive(type, patterns, declarations, span);
}

/**
 * The checker's entry for another item of the module, which the program
 * calls or reads by name but does not declare: a function with its
 * signature, or a constant with its type. A call of either is a `call` of
 * its name, which no declaration of the program has.
 */
function externalEntry(external) {
  const entry = { k: external.k, name: external.name, fullName: external.name, modulePath: [] };
  if (external.k === 'const') return { ...entry, type: external.type };
  return { ...entry, params: external.params.map(({ name, type }) => ({ name, type, guard: null, default: null })), ret: external.ret };
}

class Checker {
  constructor(language, externals, referenceSemantics = false, checkedProjection = false) {
    this.checkedProjection = checkedProjection;
    this.language = language;
    this.referenceSemantics = referenceSemantics;
    this.items = new Map();
    this.externals = new Map(externals.map((external) => [external.name, externalEntry(external)]));
    this.fresh = 0;
    this.generatedPrefix = 'ml_';
    this.modules = new Map([['', { path: [], names: new Map(), modules: new Map() }]]);
  }

  program(surface) {
    this.generatedPrefix = readSourceIdentifierSupply(collectSourceIdentifierTexts(surface));
    this.declare(surface.items, []);
    const items = this.checkItems(surface.items, []);
    const main = surface.main ? this.checkMain(surface.main) : null;
    const program = {
      schemaVersion: 1,
      sourceLanguage: this.language,
      items,
      main,
      declarations: this.items,
    };
    if (surface.imports) program.imports = surface.imports;
    if (surface.externals) program.externals = surface.externals;
    if(Object.hasOwn(surface,'sourcePureUnitCalls'))program.sourcePureUnitCalls=surface.sourcePureUnitCalls;
    if (this.referenceSemantics) program.referenceSemantics = true;
    for (const entry of this.items.values()) {
      if (entry.k === 'fn') entry.body = reuseSuccessors(entry.body, new Map());
    }
    analyseRecursion(program);
    checkRustPrimitiveOwnerProof(program);
    checkRustConstFunctionProof(program);
    checkRustEmptyOptionProof(program);
    validateRustTypeNamespace(program);
    checkRustPureUnitCalls(program);
    checkRustOwnedLocalStateProof(program);
    if(Object.hasOwn(surface,'sourceRuntimeShell'))captureRustRuntimeShell(program,surface.sourceRuntimeShell);
    return program;
  }

  declare(items, path) {
    const scope = this.modules.get(path.join('.'));
    for (const item of items) {
      const fullName = [...path, item.name].join('.');
      if (item.k !== 'module' && scope.names.has(item.name)) {
        throw typeError(`duplicate declaration ${fullName}`, item.span);
      }
      if (item.sourceLoop !== undefined) checkRustScalarLoopProof(item, this.language, fullName);
      if (this.language !== 'JavaScript' && /^ml_/u.test(item.name) && !item.generated && item.sourceLoop === undefined) {
        throw unsupported('reserved identifier', `${item.name} uses the translator's reserved ml_ prefix`, item.span);
      }
      if (item.k === 'module') {
        // Modules live in their own namespace, so Lean's `namespace Tree` may accompany `inductive Tree`.
        if (!scope.modules.has(item.name)) {
          scope.modules.set(item.name, { k: 'module', fullName });
          this.modules.set(fullName, { path: [...path, item.name], names: new Map(), modules: new Map() });
        }
        this.declare(item.items, [...path, item.name]);
        continue;
      }
      const entry = { ...item, fullName, modulePath: path };
      scope.names.set(item.name, entry);
      this.items.set(fullName, entry);
      if (item.k === 'data') {
        for (const ctor of item.ctors) {
          if (scope.names.has(`${item.name}.${ctor.name}`)) throw typeError(`duplicate constructor ${ctor.name}`, item.span);
          const ctorEntry = { k: 'ctor', data: fullName, name: ctor.name, ctor };
          scope.names.set(`${item.name}.${ctor.name}`, ctorEntry);
          if (this.language === 'Rocq') {
            if (scope.names.has(ctor.name)) throw typeError(`duplicate declaration ${ctor.name}`, item.span);
            scope.names.set(ctor.name, ctorEntry);
          }
        }
      }
    }
  }

  checkItems(items, path) {
    // Data layouts and signatures are resolved program-wide before any body,
    // so declarations may refer to each other in any order.
    this.walk(items, path, (item, itemPath, entry) => {
      if (item.k !== 'data') return;
      entry.ctors = item.ctors.map((ctor) => ({
        name: ctor.name,
        fields: ctor.fields.map((field, index) => ({
          name: field.name ?? `field${index}`,
          type: this.resolveType(field.type, itemPath, item.span),
        })),
      }));
    });
    this.walk(items, path, (item, itemPath, entry) => {
      if (item.k !== 'fn') return;
      entry.params = item.params.map((param) => ({
        name: param.name,
        type: this.resolveType(param.type, itemPath, item.span),
        guard: param.guard ?? null,
        default: param.default ?? null,
        ...(param.optional && { optional: true }),
      }));
      entry.ret = this.resolveType(item.semanticRet ?? item.ret, itemPath, item.span);
      if (item.semanticRet) {
        entry.annotatedRet = this.resolveType(item.ret, itemPath, item.span);
        if (!acceptOptionalAnnotationTypes(entry.ret, entry.annotatedRet)) throw typeError('the semantic return type must retain the annotated value types', item.span);
      }
      if (item.sourceSignature !== undefined) entry.sourceSignature = checkRustSourceSignature(item.sourceSignature, entry.params.map(parameter => parameter.type), entry.ret, this.language, value => this.resolveType(value, itemPath, item.span));
    });
    this.walk(items, path, (item, itemPath, entry) => {
      if (item.k === 'fn') {
        const env = new Map(entry.params.map((param) => [param.name, param.type]));
        entry.body = coerce(this.expr(item.body, env, itemPath, entry.ret), entry.ret, this.language, item.span);
        if(item.sourcePrimitiveOwners!==undefined)entry.sourcePrimitiveOwners=prepareRustPrimitiveOwnerProof(item.sourcePrimitiveOwners,entry.body);
        if(item.sourceLocalState!==undefined)entry.sourceLocalState=prepareRustOwnedLocalStateProof(item.sourceLocalState,entry.body);
        if (entry.sourceLoop !== undefined) checkRustScalarLoopProof(entry, this.language);
      } else if (item.k === 'theorem') {
        entry.binders = item.binders.map((binder) => ({ name: binder.name, type: this.resolveType(binder.type, itemPath, item.span) }));
        let prop = this.prop(item.prop, new Map(entry.binders.map((binder) => [binder.name, binder.type])), itemPath);
        // `theorem t : ∀ n, P n` and `theorem t (n) : P n` state the same
        // proposition; hoisting leading binders gives every target one shape.
        while (prop.p === 'forall') {
          entry.binders.push(...prop.binders);
          prop = prop.body;
        }
        entry.prop = prop;
        entry.proof = normaliseProof(item.proof, entry, {
          lookup: (name) => this.lookup(name.split('.'), itemPath, item.span),
          dataEntry: (name) => this.items.get(name),
        });
      } else if (item.k !== 'data') {
        throw typeError(`unknown item ${item.k}`, item.span);
      }
    });
    const build = (list, listPath) => list.map((item) => (item.k === 'module'
      ? { k: 'module', name: item.name, fullName: [...listPath, item.name].join('.'), items: build(item.items, [...listPath, item.name]), span: item.span }
      : this.items.get([...listPath, item.name].join('.'))));
    return build(items, path);
  }

  walk(items, path, visit) {
    for (const item of items) {
      if (item.k === 'module') this.walk(item.items, [...path, item.name], visit);
      else visit(item, path, this.items.get([...path, item.name].join('.')));
    }
  }

  resolveType(type, path, span) {
    if (!type) throw typeError('missing type annotation', span);
    if (type.kind === 'array') return array(this.resolveType(type.element, path, span), Boolean(type.reference));
    if (type.kind === 'option' || type.kind === 'set' || type.kind === 'cell') return { ...type, element: this.resolveType(type.element, path, span) };
    if (type.kind === 'tuple') return { ...type, elements: type.elements.map((element) => this.resolveType(element, path, span)) };
    if (type.kind === 'union') return { ...type, alternatives: type.alternatives.map(alternative => this.resolveType(alternative, path, span)) };
    if (type.kind === 'record') {
      if(type.ordered!==undefined&&typeof type.ordered!=='boolean')throw typeError('record ordered facet must be a boolean',span);
      return { ...type, fields: type.fields.map((field) => ({ ...field, type: this.resolveType(field.type, path, span) })).sort((a, b) => a.name < b.name ? -1 : a.name > b.name ? 1 : 0) };
    }
    if (type.kind === 'map') return { ...type, key: this.resolveType(type.key, path, span), value: this.resolveType(type.value, path, span) };
    if (type.kind === 'function') return { ...type, params: type.params.map((param) => this.resolveType(param.type ?? param, path, span)), ret: this.resolveType(type.ret, path, span) };
    if (type.kind !== 'named') return type;
    const entry = this.lookup(type.path, path, span);
    if (!entry || entry.k !== 'data') throw typeError(`unknown type ${type.path.join('.')}`, type.span ?? span);
    return data(entry.fullName);
  }

  lookup(segments, path, span) {
    let names = [...segments];
    let start = path;
    if (names[0] === 'crate') {
      names = names.slice(1);
      start = [];
    } else if (names[0] === 'self') {
      names = names.slice(1);
    } else {
      let up = 0;
      while (names[0] === 'super') {
        names = names.slice(1);
        up += 1;
      }
      if (up > path.length) throw typeError('super beyond crate root', span);
      if (up > 0) start = path.slice(0, path.length - up);
    }
    for (let depth = start.length; depth >= 0; depth -= 1) {
      const found = this.lookupFrom(start.slice(0, depth), names);
      if (found) return found;
      if (segments[0] === 'crate' || segments[0] === 'self' || segments[0] === 'super') break;
    }
    // The other items of the module are in scope after the program's own.
    return names.length === 1 ? this.externals.get(names[0]) : undefined;
  }

  lookupFrom(modulePath, names) {
    let scope = this.modules.get(modulePath.join('.'));
    for (let index = 0; index < names.length - 1; index += 1) {
      const rest = names.slice(index).join('.');
      if (index === names.length - 2 && scope.names.has(rest)) return scope.names.get(rest);
      const module = scope.modules.get(names[index]);
      if (!module) return undefined;
      scope = this.modules.get(module.fullName);
    }
    const last = names.at(-1);
    if (scope.names.has(last)) return scope.names.get(last);
    // Inside `namespace T`, the constructors of a sibling `inductive T` are in scope.
    if (scope.path.length > 0) {
      const parent = this.modules.get(scope.path.slice(0, -1).join('.'));
      const sibling = parent.names.get(scope.path.at(-1));
      if (sibling?.k === 'data') return parent.names.get(`${sibling.name}.${last}`);
    }
    return undefined;
  }

  checkMain(main) {
    const env = new Map();
    const effects = [];
    for (const effect of main.effects) {
      if (effect.k === 'print') {
        effects.push({ k: 'print', expr: this.show(effect.expr, env, [], effect.style), span: effect.span });
        continue;
      }
      if (effect.k === 'let') {
        const expected = effect.type ? this.resolveType(effect.type, [], effect.span) : undefined;
        let value = this.expr(effect.value, env, [], expected);
        if (expected) value = coerce(value, expected, this.language, effect.span);
        bindLocal(env, effect.name, value.type, value.type.kind === 'tuple' && immutableProductValue(value, env.get(PRODUCT_VALUES)));
        effects.push({ k: 'let', name: effect.name, value, ...(effect.constant ? { constant: true } : {}), span: effect.span,...(Object.hasOwn(effect,'sourcePureUnitCall')?{sourcePureUnitCall:effect.sourcePureUnitCall}:{}) });
        continue;
      }
      if (effect.k === 'assert') {
        effects.push({ k: 'assert', prop: this.prop(effect.prop, env, []), span: effect.span });
        continue;
      }
      throw typeError(`unknown effect ${effect.k}`, effect.span);
    }
    return { effects, span: main.span, ...(main.sequentialAsync ? { sequentialAsync: true } : {}) };
  }

  show(expr, env, path, style) {
    const value = this.expr(expr, env, path, undefined);
    const displayRefusal = readNumberDisplayRefusal(this.language, value.type.kind);
    if (displayRefusal) throw unsupported('Rust Float display', displayRefusal, expr.span);
    const bigint = value.type.kind === 'int' || value.type.kind === 'nat';
    if (value.type.kind === 'option') {
      if (!primitiveKey(value.type.element)) throw unsupported('output of nullable structured values', 'this output needs its source structured-value formatter', expr.span);
      if (style === 'js-format-first' && value.type.element.kind === 'string') throw unsupported('console.log with a computed first string', 'util.format reads the % directives of a first string argument, which only the run knows; pass a literal format string, or print one template literal', expr.span);
      if (style === 'js-format-number' || style === 'js-format-integer') throw unsupported('numeric format of a nullable value', 'the format directive converts each absent value before printing; this conversion needs its own operational formatter', expr.span);
      return { k: 'optionalText', arg: value, console: CONSOLE_STYLES.has(style), type: STRING };
    }
    if (style === 'js-format-first' && value.type.kind === 'string') {
      throw unsupported('console.log with a computed first string', 'util.format reads the % directives of a first string argument, which only the run knows; pass a literal format string, or print one template literal', expr.span);
    }
    if (style === 'js-format-number' && !bigint && !isFloat(value.type)) {
      throw unsupported(`%d of a ${typeKey(value.type)}`, '%d prints a BigInt or a Number; the conversion Number(value) of other values is not kept', expr.span);
    }
    if (style === 'js-format-integer' && !bigint) {
      throw unsupported(`%i of a ${typeKey(value.type)}`, '%i prints a BigInt; the conversion parseInt(value) of other values is not kept', expr.span);
    }
    if (value.type.kind === 'string') return value;
    if (value.type.kind === 'data' || value.type.kind === 'unit') {
      throw unsupported('output of structured values', `printing a ${typeKey(value.type)} value has no portable textual form`, expr.span);
    }
    if (value.type.kind === 'array') throw arrayText(expr.span);
    const text = { k: 'toString', arg: value, type: STRING };
    const consoleStyle = CONSOLE_STYLES.has(style);
    // console.log prints -0 as "-0", where String(-0) is "0".
    if (consoleStyle && isFloat(value.type)) return { ...text, console: true };
    if (consoleStyle && bigint) {
      // Node's console.log prints BigInt values with their `n` suffix.
      return { k: 'binary', op: 'concat', left: text, right: literal(STRING, 'n'), type: STRING };
    }
    return text;
  }

  prop(prop, env, path) {
    switch (prop.p) {
      case 'forall': {
        const binders = prop.binders.map((binder) => ({ name: binder.name, type: this.resolveType(binder.type, path, prop.span) }));
        if (binders.some((binder) => isFloat(binder.type))) {
          throw unsupported('quantification over Numbers', 'a theorem quantifies over integers, naturals, booleans, strings or data', prop.span);
        }
        const inner = new Map(env);
        for (const binder of binders) bindLocal(inner, binder.name, binder.type);
        return { p: 'forall', binders, body: this.prop(prop.body, inner, path) };
      }
      case 'and':
      case 'or':
      case 'implies':
        return { p: prop.p, left: this.prop(prop.left, env, path), right: this.prop(prop.right, env, path) };
      case 'not':
        return { p: 'not', arg: this.prop(prop.arg, env, path) };
      case 'bool':
        return { p: 'bool', expr: coerce(this.expr(prop.expr, env, path, BOOL), BOOL, this.language, prop.span) };
      default: {
        if (!COMPARISONS.has(prop.p)) throw typeError(`unknown proposition ${prop.p}`, prop.span);
        const [left, right] = this.operands(prop.left, prop.right, env, path, prop.span);
        if (left.type.kind === 'tuple') throw unsupported('comparison of tuples', 'tuple value equality and JavaScript reference identity need an explicit operational comparison model; compare positional elements', prop.span);
        if (prop.p !== 'eq' && prop.p !== 'ne' && !isOrdered(left.type)) {
          throw typeError(`ordering on ${typeKey(left.type)}`, prop.span);
        }
        if (prop.reference && (left.type.kind === 'data' || left.type.kind === 'unit')) {
          throw unsupported('identity comparison of objects', 'JavaScript compares objects by identity; use assert.deepStrictEqual', prop.span);
        }
        // Equality propositions over data are structural; executable targets get a generated equality.
        // Over Numbers, assert.strictEqual's SameValue differs from ===: NaN equals NaN, and 0 differs from -0.
        if (prop.sameValue && isFloat(left.type) && (prop.p === 'eq' || prop.p === 'ne')) {
          return { p: prop.p, left, right, domain: left.type, sameValue: true };
        }
        return { p: prop.p, left, right, domain: left.type };
      }
    }
  }

  operands(leftSurface, rightSurface, env, path, span) {
    let left = this.expr(leftSurface, env, path, undefined, true);
    let right = this.expr(rightSurface, env, path, left.type.kind === 'literal' ? undefined : left.type, true);
    if (left.type.kind === 'literal' && right.type.kind !== 'literal') left = this.expr(leftSurface, env, path, right.type);
    if (left.type.kind === 'literal') {
      left = this.expr(leftSurface, env, path, this.defaultNumber());
      right = this.expr(rightSurface, env, path, left.type);
    }
    return unify(left, right, this.language, span);
  }

  defaultNumber() {
    return { JavaScript: INT, Rust: { kind: 'fixed', bits: 32, signed: true }, Lean: NAT, Rocq: NAT }[this.language];
  }

  expr(node, env, path, expected, allowLiteral = false) {
    const payload = readSurfaceReferencePayloadOperand(node.k);
    if (payload) {
      const descriptors = Object.getOwnPropertyDescriptors(node);
      const prototype = Object.getPrototypeOf(node);
      if ((prototype !== Object.prototype && prototype !== null) || !Object.hasOwn(descriptors, 'k')
        || Reflect.ownKeys(descriptors).some(field => typeof field !== 'string' || !acceptSurfaceReferencePayloadField(field)
          || !descriptors[field].enumerable || !Object.hasOwn(descriptors[field], 'value'))
        || !Object.hasOwn(descriptors, payload) || !descriptors[payload].value || typeof descriptors[payload].value !== 'object'
        || Array.isArray(descriptors[payload].value) || readSurfaceReferencePayloadOperand(descriptors[payload].value.k)) {
        throw typeError('a reference payload needs its closed owned expression');
      }
    }
    const expression = payload ? node[payload] : node;
    if (expression.prototypeDefinition) throw unsupported('object prototype definition',
      'an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the portable core does not represent object prototypes', expression.prototypeDefinition.span);
    // Control flow passes its expected result type to every branch. Literal
    // builders already construct value payloads before the allocation below.
    let result = this.exprInner(expression, env, path, expected, allowLiteral);
    if (payload) {
      const refusal = readSurfaceReferencePayloadError(result.type.kind, result.type.reference === true);
      if (refusal) throw typeError(refusal, node.span);
    }
    if (selectSurfaceReferenceAllocation(this.referenceSemantics && !payload, result.type.kind, result.type.reference)) {
      result = { k: 'referenceAllocate', value: result, type: { ...result.type, reference: true } };
    }
    if ((node.span || (payload && expression.span)) && !result.span) result.span = node.span || expression.span;
    return result;
  }

  exprInner(node, env, path, expected, allowLiteral) {
    switch (node.k) {
      case 'arrayBindingRest': {
        checkJavaScriptArrayBindingWrapper(node);
        const object = this.referenceValue(this.expr(node.object, env, path, undefined));
        if (this.language !== 'JavaScript' || !acceptJavaScriptArrayRestReceiver(object.type.kind)) throw unsupported('array rest binding', 'the selected rest binding needs an actual homogeneous array receiver', node.span);
        // Pattern positions are JavaScript Numbers, independently of the
        // portable core's default type for an unannotated integer literal.
        const position = Number(node.start.value) * (node.start.negative ? -1 : 1);
        const start = { ...node.start, value: Object.is(position, -0) ? '-0' : String(position), negative: false, type: FLOAT };
        return this.primitiveMethod({ k: 'method', op: 'slice', args: [start], span: node.span }, object, env, path);
      }
      case 'arrayBindingDefault': {
        checkJavaScriptArrayBindingWrapper(node);
        return checkJavaScriptBindingDefault(this,node,env,path);
      }
      case 'num': {
        // A JavaScript Number literal: its canonical text, `String(value)`, with the sign applied.
        if (node.type && isFloat(node.type)) return literal(FLOAT, node.negative ? negateNumber(node.value) : node.value);
        // The 1 of `x++` takes the type of `x`, which may be a Number.
        if (node.unit && expected && isFloat(expected)) return literal(FLOAT, node.value);
        const type = node.type ?? (expected && isNumeric(expected) ? expected : undefined);
        if (!type) {
          if (allowLiteral) return { k: 'lit', type: { kind: 'literal' }, value: node.value };
          return this.expr(node, env, path, this.defaultNumber());
        }
        if (!isNumeric(type)) throw typeError(`numeric literal where ${typeKey(type)} is expected`, node.span);
        const value = BigInt(node.value) * (node.negative ? -1n : 1n);
        if (isNatural(type) && value < 0n) throw typeError('negative literal for a natural type', node.span);
        if (type.kind === 'fixed') {
          const { min, max } = fixedBounds(type);
          if (value < min || value > max) throw typeError(`literal out of range for ${typeKey(type)}`, node.span);
        }
        return literal(type, value.toString());
      }
      case 'bool':
        return literal(BOOL, node.value);
      case 'str':
        return literal(STRING, node.value);
      case 'unit':
        return expected?.kind === 'option' && this.language === 'JavaScript' ? { k: 'none', absence: 'undefined', type: expected } : { k: 'unit', type: UNIT };
      case 'name': {
        if (node.path.length === 1 && env.has(node.path[0])) {
          return { k: 'var', name: node.path[0], type: env.get(node.path[0]) };
        }
        const declaration = this.lookup(node.path, path, node.span);
        if (acceptCallableFunctionReference(node.k, expected?.kind ?? '', declaration?.k ?? '')) {
          return { k: 'functionRef', fn: declaration.fullName, type: constructCallableFunctionType(declaration.params, declaration.ret) };
        }
        return this.application(node, [], env, path, expected, node.span);
      }
      case 'functionValue': {
        const declaration = this.lookup(node.path, path, node.span);
        if (!acceptCallableFunctionReference(node.k, '', declaration?.k ?? '')) throw typeError(`unknown function ${node.path.join('.')}`, node.span);
        return { k: 'functionRef', fn: declaration.fullName, type: constructCallableFunctionType(declaration.params, declaration.ret) };
      }
      case 'dotCtor':
      case 'app':
        return this.application(node.k === 'app' ? node.fn : node, node.args ?? [], env, path, expected, node.span);
      case 'field': {
        const value = this.referenceValue(this.expr(node.object, env, path, undefined));
        const guarded=checkJavaScriptNullishField(this,node,value,env,path,expected);
        if(guarded)return guarded;
        return checkJavaScriptNamedProperty(value,node.field,node.span);
      }
      case 'length': {
        const object = this.expr(node.object, env, path, undefined);
        if (object.type.kind !== 'array') throw typeError(`length of ${typeKey(object.type)}`, node.span);
        return { k: 'length', array: object, type: node.integer ? INT : FLOAT };
      }
      case 'array': {
        const context = acceptJavaScriptOptionalTupleContext(this.language, node.k, expected?.kind ?? '', expected?.element?.kind ?? '') ? expected.element : expected;
        if (context?.kind === 'tuple') {
          if (node.items.length !== context.elements.length || node.items.some((item) => item.spread)) throw typeError('tuple literal has the wrong number of elements', node.span);
          return { k: 'tuple', items: node.items.map((item, index) => coerce(this.expr(item.value, env, path, context.elements[index]), context.elements[index], this.language, node.span)), type: context };
        }
        return this.arrayLiteral(node, env, path, expected);
      }
      case 'tuple': {
        const context = acceptJavaScriptOptionalTupleContext(this.language, node.k, expected?.kind ?? '', expected?.element?.kind ?? '') ? expected.element : expected;
        if (node.frozen !== undefined && typeof node.frozen !== 'boolean') throw typeError('tuple frozen facet must be a boolean', node.span);
        if (node.productModel !== undefined && !acceptProgramTupleProductModel(node.productModel, node.frozen === true)) throw typeError('unknown or contradictory tuple product model', node.span);
        if (node.frozen && node.items.length === 0) throw unsupported('empty immutable tuple product', 'an empty positional product is distinct from a unit value and needs its own target encoding', node.span);
        if (context && context.kind !== 'tuple') throw typeError(`tuple literal where ${typeKey(context)} is expected`, node.span);
        if (context && node.items.length !== context.elements.length) throw typeError('tuple literal has the wrong number of elements', node.span);
        const items = node.items.map((item, index) => {
          const want = context?.elements[index];
          const value = this.expr(item, env, path, want);
          return want ? coerce(value, want, this.language, item.span ?? node.span) : value;
        });
        const type = context ?? tuple(items.map(item => item.type));
        if (node.frozen && (!immutableProductType(type) || items.some(item => !immutableProductValue(item, env.get(PRODUCT_VALUES))))) {
          throw unsupported('immutable tuple product', 'every element needs the recursive scalar Copy value model; a mutable inner tuple cannot be frozen by its outer product', node.span);
        }
        return { k: 'tuple', items, ...(node.frozen && { frozen: true }), ...(node.productModel !== undefined && { productModel: node.productModel }), type };
      }
      case 'tupleIndex': {
        const object = this.referenceValue(this.expr(node.object, env, path, undefined));
        if (object.type.kind !== 'tuple') throw typeError('tuple projection requires a tuple value', node.span);
        if (!acceptTupleIndex(node.index, object.type.elements.length)) throw typeError('tuple index out of range', node.span);
        const index = readTuplePosition(node.index);
        return { k: 'tupleIndex', object, index, type: object.type.elements[index] };
      }
      case 'copySlice': {
        const object = this.expr(node.object, env, path, undefined);
        const source={...node,k:'method',op:'slice',args:[]};
        const nullable=this.checkedProjection?null:checkJavaScriptNullishAccess(this,source,object,env,path,expected);
        if(nullable)return nullable;
        if(requireJavaScriptSliceReferenceCopy(this.language,object.type.kind,object.type.reference===true))return this.method(source,env,path,expected,object);
        const form = readPrimitiveCopySliceForm(object.type.kind);
        if (form === 'identity') return object;
        if (form === 'method') return this.method({ ...node, k: 'method', op: 'slice', args: [] }, env, path, expected, object);
        throw unsupported('method call .slice()', 'this value has no string or array slice method', node.span);
      }
      case 'record':
        return this.recordLiteral(node, env, path, expected);
      case 'cellAllocate': {
        const selected = readCallableCellElement(node, expected);
        const want = selected.declared ? this.resolveType(selected.type, path, node.span) : selected.type;
        const raw = this.expr(node.value, env, path, want);
        const value = want ? coerce(raw, want, this.language, node.span) : raw;
        return { k: 'cellAllocate', value, referenceOperation: referenceOperation(readScalarCellOperation(node.k)).id, type: { kind: 'cell', element: want ?? value.type } };
      }
      case 'cellRead': {
        const cell = this.expr(node.cell, env, path, undefined);
        const error = readCallableCellError(cell.type.kind, node.k);
        if (error) throw typeError(error, node.span);
        return { k: 'cellRead', cell, referenceOperation: referenceOperation(readScalarCellOperation(node.k)).id, type: cell.type.element };
      }
      case 'cellWrite': {
        const cell = this.expr(node.cell, env, path, undefined);
        const error = readCallableCellError(cell.type.kind, node.k);
        if (error) throw typeError(error, node.span);
        const value = coerce(this.expr(node.value, env, path, cell.type.element), cell.type.element, this.language, node.span);
        const body = this.expr(node.body, env, path, expected);
        return { k: 'cellWrite', cell, value, body, referenceOperation: referenceOperation(readScalarCellOperation(node.k)).id, type: body.type };
      }
      case 'referenceWrite':
        return checkSourceRecordFieldWrite(this, node, env, path);
      case 'set':
      case 'map':
        return checkCollectionConstruction(this, node, env, path, expected);
      case 'arrayFrom': {
        const argumentError = readArrayFromArgumentError(node.args.length);
        if (argumentError) throw typeError(argumentError, node.span);
        const object = this.expr(node.args[0], env, path, undefined);
        const selection = readArrayFromIterableType(object.type);
        if (!selection) throw unsupported('Array.from', readArrayFromReceiverError(object.type.kind), node.span);
        let result;
        if (selection.form === 0) result = object;
        else if (selection.form === 1) result = { k: 'stringPoints', object, type: array(selection.element) };
        else result = { k: 'collectionItems', object, op: selection.form === 2 ? 'values' : 'entries', type: array(selection.element) };
        if (!node.args[1]) return result;
        const callbackEnv=this.language==='JavaScript'&&!this.checkedProjection?createJavaScriptOwnKeyCallbackEnvironment(node.args[0],node.args[1],'map',env):env;
        const callback = this.expr(node.args[1], callbackEnv, path, { kind: 'function', params: [result.type.element, FLOAT], ret: readArrayFromExpectedElement(expected).element });
        return { k: 'method', object: result, op: 'map', args: [callback], type: array(callback.type.ret) };
      }
      case 'forOfIterable': {
        const object = this.expr(node.object, env, path, undefined);
        return checkJavaScriptIterableValue(object,node.span);
      }
      case 'arrayIsArray': {
        const value = this.expr(node.arg, env, path, undefined);
        const plan = readJavaScriptArrayClassificationPlan(value.type.kind, value.type.element?.kind ?? '');
        if (plan.length !== 2 || !plan[0] && plan[1]) throw unsupported('Array.isArray operand',
          'the checked value needs a closed represented kind or a flat optional array/tuple; unknown union/proxy domains need an explicit operational model', node.span);
        const name = this.freshName('ml_array_test');
        const body = plan[1] ? this.booleanValue({ k: 'var', name, type: value.type }, node.span) : literal(BOOL, plan[0]);
        return { k: 'let', name, value, body, type: BOOL };
      }
      case 'lambda': {
        const inner = new Map(env);
        const callable = readCallableLambdaInference(node, expected);
        const params = callable.params.map(({ parameter: param, type: selected, declared }) => {
          const type = declared ? this.resolveType(selected, path, param.span) : selected;
          if (!type) throw typeError(`cannot infer callback parameter ${param.name}`, param.span ?? node.span);
          bindLocal(inner, param.name, type);
          return { ...param, type };
        });
        installJavaScriptOwnKeyCallbackBindings(node,params,inner);
        const result = callable.declaredRet ? this.resolveType(callable.ret, path, node.span) : callable.ret;
        let body = this.expr(node.body, inner, path, result);
        if (result) body = coerce(body, result, this.language, node.span);
        return { k: 'lambda', params, body, type: constructCallableFunctionType(params, body.type) };
      }
      case 'method': {
        const callbackEnv=this.language==='JavaScript'&&!this.checkedProjection?createJavaScriptOwnKeyCallbackEnvironment(node.object,node.args[0],node.op,env):env;
        return (this.checkedProjection ? null : checkJavaScriptSourcePropertyCall(this,node,callbackEnv,path,expected))??this.method(node, callbackEnv, path, expected);
      }
      case 'booleanValue':
        return this.booleanValue(this.expr(node.arg, env, path, undefined), node.span);
      case 'numberValue':
        return coerce(this.numberValue(this.expr(node.arg, env, path, undefined), node.span), FLOAT, this.language, node.span);
      case 'none':
        if (!acceptOptionalLiteralAbsence(node.absence)) throw typeError('an absent optional value requires its concrete null or undefined tag', node.span);
        if (expected?.kind === 'option' && !acceptOptionalAbsence(expected.absence ?? 'both', node.absence)) throw typeError('an absent optional value disagrees with its retained absence policy', node.span);
        return { k: 'none', absence: node.absence, type: expected?.kind === 'option' ? expected : { kind: 'option', element: UNIT, absence: node.absence } };
      case 'some': {
        const type = this.resolveType(node.type, path, node.span);
        if (type.kind !== 'option') throw typeError('a present optional value requires its actual optional type', node.span);
        const value = coerce(this.expr(node.value, env, path, type.element), type.element, this.language, node.span);
        return { k: 'some', value, type };
      }
      case 'flattenOption': {
        const value = this.expr(node.value, env, path, undefined);
        if (!requireOptionalFlattening(value.type.kind, value.type.element?.kind ?? '')) throw typeError('optional flattening requires two actual optional layers', node.span);
        const inner = value.type.element;
        return { k: 'flattenOption', value, type: { kind: 'option', element: inner.element, absence: mergeOptionalAbsence(inner.absence, value.type.absence) } };
      }
      case 'coalesce': {
        const left = node.left.k === 'index' ? this.expr({ ...node.left, optional: true }, env, path, undefined) : this.expr(node.left, env, path, undefined);
        if (!requireOptionalCoalescing(left.type.kind)) return left;
        const right = coerce(this.expr(node.right, env, path, left.type.element), left.type.element, this.language, node.span);
        return { k: 'coalesce', left, right, type: left.type.element };
      }
      case 'optionalField': {
        const object = this.expr(node.object, env, path, undefined);
        const operation = readOptionalFieldOperation(object.type.kind, object.type.element?.kind ?? '', node.field);
        if (operation === 'field') return this.expr({ k: 'field', object: node.object, field: node.field, span: node.span }, env, path, expected);
        if (operation === 'project') return { k: 'optionalProject', object, field: node.field, type: { kind: 'option', element: FLOAT, absence: 'undefined' } };
        if (operation === 'unsupported') throw unsupported('optional field access', 'this property needs a known structural type', node.span);
        const field = object.type.element.fields.find((field) => field.name === node.field);
        if (!field) throw typeError(`record has no field ${node.field}`, node.span);
        return flattenOption({ k: 'optionalField', object, field: node.field, index: object.type.element.fields.indexOf(field), type: { kind: 'option', element: field.type, absence: 'undefined' } });
      }
      case 'stringPoints': {
        const object = coerce(this.expr(node.object, env, path, STRING), STRING, this.language, node.span);
        return { k: 'stringPoints', object, type: array(STRING) };
      }
      case 'math':
        return this.math(node, env, path, expected);
      case 'index': {
        const object = this.referenceValue(this.expr(node.object, env, path, undefined));
        const nullable=this.checkedProjection?null:checkJavaScriptNullishIndex(this,node,object,env,path,expected);
        if(nullable)return nullable;
        if(this.language==='JavaScript'&&!this.checkedProjection){
          const literal=readJavaScriptLiteralIndexSource(node,object.type.kind);
          if(literal?.k==='field')return checkJavaScriptNamedProperty(object,literal.field,node.span);
          if(literal)node=literal;
          if(object.type.kind==='string')return checkJavaScriptStringIndex(this,node,object,env,path);
        }
        if(this.language==='JavaScript'&&!this.checkedProjection){const guarded=checkJavaScriptGuardedRecordIndex(this,node,object,env,path,expected);if(guarded)return guarded;}
        if (object.type.kind === 'tuple' && node.index.k === 'num') {
          const index = readTuplePosition(Number(node.index.value) * (node.index.negative ? -1 : 1));
          if (!acceptTupleIndex(index, object.type.elements.length)) throw typeError('tuple index out of range', node.span);
          return { k: 'tupleIndex', object, index, type: object.type.elements[index] };
        }
        if (object.type.kind !== 'array') {
          throw unsupported(`indexing of ${typeKey(object.type)}`, 'only arrays are indexed in the portable core', node.span);
        }
        let index = this.expr(node.index, env, path, undefined);
        if (selectOptionalArrayIndexNumber(this.language, index.type.kind, index.type.element?.kind ?? '')) index = { k: 'optionalIndexNumber', arg: index, type: FLOAT };
        if (!isFloat(index.type) && index.type.kind !== 'int' && index.type.kind !== 'nat') {
          throw typeError(`array index of type ${typeKey(index.type)}; an index is a Number or a BigInt`, node.index.span ?? node.span);
        }
        const captured = captureArraySourceOperands(this, object, index, env);
        const result = selectOptionalArrayRead(this.language, node.optional === true, node.provenInBounds === true, expected?.kind === 'option' && sameType(expected.element, object.type.element))
          ? flattenOption({ k: 'optionalIndex', array: captured.array, index: captured.index, type: { kind: 'option', element: object.type.element, absence: 'undefined' } })
          : { k: 'index', array: captured.array, index: captured.index, type: object.type.element, ...(node.provenInBounds && { provenInBounds: true }) };
        return wrapArraySourceOperands(result, captured.bindings);
      }
      case 'typeOf': {
        const arg = this.expr(node.arg, env, path, undefined);
        if (arg.type.kind === 'option') {
          const present = arg.type.element.kind === 'unit' && this.language === 'JavaScript' ? 'undefined' : readTypeQueryResult(arg.type.element.kind);
          if (!present) throw unsupported(`typeof ${typeKey(arg.type)}`, 'type queries require a known JavaScript value type', node.span);
          return { k: 'optionalTypeOf', arg, present, type: STRING };
        }
        if (!acceptTypeQueryOperand(node.arg.k)) throw unsupported('typeof operand', 'type queries currently require a bound value or literal; evaluating other operands must retain their effects and exceptions', node.span);
        const constantReference = arg.k === 'call' && this.externals.get(arg.fn)?.k === 'const';
        if (!acceptCheckedTypeQueryOperand(arg.k, constantReference)) throw unsupported('typeof operand', 'type queries currently require a bound value or literal; evaluating other operands must retain their effects and exceptions', node.span);
        const value = readTypeQueryResult(arg.type.kind);
        if (!value) throw unsupported(`typeof ${typeKey(arg.type)}`, 'type queries require a known JavaScript value type', node.span);
        return literal(STRING, value);
      }
      case 'unary': {
        if (node.op === 'not') {
          const arg = this.expr(node.arg, env, path, this.language === 'Rust' ? expected : undefined);
          if (this.language === 'Rust' && arg.type.kind === 'fixed') return {
            k: 'math', op: readRustPrimitiveBitwiseOperation('!'), args: [arg], type: arg.type };
          return { k: 'unary', op: 'not', arg: this.booleanValue(arg, node.span), type: BOOL };
        }
        if (node.op === 'neg') {
          if (node.arg.k === 'num') {
            if (!acceptRustNumericLiteralNegation(this.language, node.arg.type?.kind ?? '', node.arg.type?.signed === true)) {
              throw typeError(`negation of ${typeKey(node.arg.type)}`, node.span);
            }
            const literal = this.expr({ ...node.arg, negative: !node.arg.negative, span: node.span }, env, path, expected, allowLiteral);
            if (!acceptRustNumericLiteralNegation(this.language, literal.type.kind, literal.type.signed === true)) {
              throw typeError(`negation of ${typeKey(literal.type)}`, node.span);
            }
            return literal;
          }
          const arg = this.numberValue(this.expr(node.arg, env, path, expected && isNumeric(expected) ? expected : undefined), node.span);
          if (isFloat(arg.type)) return { k: 'unary', op: 'neg', arg, type: FLOAT, semantics: 'ieee' };
          if (arg.type.kind === 'nat' && this.language === 'JavaScript') {
            // A guarded natural is still a BigInt, and its negation an integer.
            return { k: 'unary', op: 'neg', arg: coerce(arg, INT, this.language, node.span), type: INT, semantics: 'exact' };
          }
          if (arg.type.kind === 'nat' || (arg.type.kind === 'fixed' && !arg.type.signed)) {
            throw typeError(`negation of ${typeKey(arg.type)}`, node.span);
          }
          if (arg.type.kind !== 'int' && arg.type.kind !== 'fixed') throw typeError(`negation of ${typeKey(arg.type)}`, node.span);
          return { k: 'unary', op: 'neg', arg, type: arg.type, semantics: arg.type.kind === 'fixed' ? 'checked' : 'exact' };
        }
        throw typeError(`unknown unary ${node.op}`, node.span);
      }
      case 'referenceIdentity': {
        const descriptors = Object.getOwnPropertyDescriptors(node);
        const prototype = Object.getPrototypeOf(node);
        if ((prototype !== Object.prototype && prototype !== null)
          || ['k', 'op', 'left', 'right'].some(field => !Object.hasOwn(descriptors, field))
          || Reflect.ownKeys(descriptors).some(field => typeof field !== 'string'
            || !['k', 'op', 'left', 'right', 'span'].includes(field)
            || !descriptors[field].enumerable || !Object.hasOwn(descriptors[field], 'value'))) {
          throw typeError('explicit reference identity needs its closed owned operands');
        }
        const [left, right] = this.operands(node.left, node.right, env, path, node.span);
        if (!this.referenceSemantics
          || readRustSharedArrayHandleComparisonProjection(this.language, node.op, left.type.kind, left.type.reference === true) !== 'referenceIdentity'
          || !acceptReferenceIdentityComparison(node.op, left.type.kind, right.type.kind,
            left.type.reference === true, right.type.reference === true, sameType(left.type, right.type))) {
          throw unsupported('explicit Rust array identity', 'only the owned array reference domain admits this operation', node.span);
        }
        return { k: 'binary', op: node.op, left, right, type: BOOL, domain: left.type };
      }
      case 'binary':
        return this.binary(node, env, path, expected, allowLiteral);
      case 'if': {
        const cond = this.booleanValue(this.expr(node.cond, env, path, undefined), node.span);
        const branchesEnv=this.language==='JavaScript'?readJavaScriptOwnPropertyBranchEnvironments(node.cond,env):{then:env,else:env};
        if (acceptSemanticBranchContextReordering(expected != null, needsBranchResultContext(node.then), needsBranchResultContext(node.else))) {
          let otherwise = this.expr(node.else, branchesEnv.else, path, undefined, allowLiteral);
          if (otherwise.type.kind === 'literal') otherwise = this.expr(node.else, branchesEnv.else, path, this.defaultNumber());
          const then = this.expr(node.then, branchesEnv.then, path, otherwise.type, allowLiteral);
          const branches = unifyBranches(then, otherwise, expected, this.language, node.span);
          return { k: 'if', cond, then: branches[0], else: branches[1], type: branches[0].type };
        }
        let then = this.expr(node.then, branchesEnv.then, path, expected, allowLiteral);
        let otherwise = this.expr(node.else, branchesEnv.else, path, expected ?? (then.type.kind === 'literal' ? undefined : then.type), allowLiteral);
        if (then.type.kind === 'literal' && otherwise.type.kind !== 'literal') then = this.expr(node.then, branchesEnv.then, path, otherwise.type);
        if (then.type.kind === 'literal') {
          then = this.expr(node.then, branchesEnv.then, path, this.defaultNumber());
          otherwise = this.expr(node.else, branchesEnv.else, path, then.type);
        }
        [then, otherwise] = unifyBranches(then, otherwise, expected, this.language, node.span);
        return { k: 'if', cond, then, else: otherwise, type: then.type };
      }
      case 'let': {
        const declared = node.type ? this.resolveType(node.type, path, node.span) : undefined;
        let value = this.expr(node.value, env, path, declared);
        if (declared) value = coerce(value, declared, this.language, node.span);
        const inner = new Map(env);
        bindLocal(inner, node.name, value.type, value.type.kind === 'tuple' && immutableProductValue(value, env.get(PRODUCT_VALUES)));
        const body = this.expr(node.body, inner, path, expected, allowLiteral);
        return { k: 'let', name: node.name, value, body, type: body.type };
      }
      case 'match':
        return this.expr(this.compileMatch(node, env, path), env, path, expected, allowLiteral);
      case 'match1':
        return this.match(node, env, path, expected, allowLiteral);
      case 'stringTest': {
        if (!acceptProgramExpressionOperation('stringTest', node.op)) throw unsupported('string test operation', 'the declared string operation has no checked model', node.span);
        const string = this.expr(node.object, env, path, undefined);
        if (string.type.kind !== 'string') {
          throw unsupported(`method call .${node.op}()`, `.${node.op}() is portable on strings, not on ${typeKey(string.type)}`, node.span);
        }
        const search = coerce(this.expr(node.search, env, path, STRING), STRING, this.language, node.search.span);
        if (search.type.kind !== 'string') throw typeError(`.${node.op}() of ${typeKey(search.type)}`, node.search.span);
        return { k: 'stringTest', op: node.op, string, search, type: BOOL, span: node.span };
      }
      case 'stringMap': {
        const string = this.expr(node.object, env, path, undefined);
        if (string.type.kind !== 'string') {
          throw unsupported(`method call .${node.op}()`, `.${node.op}() is portable on strings, not on ${typeKey(string.type)}`, node.span);
        }
        return { k: 'stringMap', op: node.op, string, type: STRING, span: node.span };
      }
      case 'toString': {
        const arg = this.expr(node.arg, env, path, undefined);
        if (arg.type.kind === 'string') return arg;
        if (arg.type.kind === 'array') throw arrayText(node.span);
        if (arg.type.kind === 'option' && primitiveKey(arg.type.element)) return { k: 'optionalText', arg, console: false, type: STRING };
        const displayRefusal = readNumberDisplayRefusal(this.language, arg.type.kind);
        if (displayRefusal) throw unsupported('Rust Float display', displayRefusal, node.span);
        if (!isOrdered(arg.type) && arg.type.kind !== 'bool') throw typeError(`toString of ${typeKey(arg.type)}`, node.span);
        return { k: 'toString', arg, type: STRING };
      }
      case 'show':
        return this.show(node.arg, env, path, node.style);
      case 'cast': {
        const to = this.resolveType(node.to, path, node.span);
        const arg = this.expr(node.arg, env, path, node.from);
        return castTo(arg, to, node.flavor, node.span, this.language);
      }
      case 'print': {
        const text = this.show(node.expr, env, path, node.style);
        const body = this.expr(node.body, env, path, expected, allowLiteral);
        return { k: 'print', text, body, type: body.type };
      }
      case 'abort': {
        if (!expected) throw typeError('abort needs a known result type', node.span);
        return { k: 'abort', message: node.message, type: expected };
      }
      case 'ctorObject':
        return this.ctorObject(node, env, path, expected);
      default:
        throw typeError(`unknown expression ${node.k}`, node.span);
    }
  }

  /**
   * `[a, ...xs, b]` is the array of the runs of elements between the spread
   * arrays, appended in order; `[...xs]` is `xs` itself, since no array is
   * ever mutated.
   */
  recordLiteral(node, env, path, expected) {
    if(!expected&&node.type?.kind==='record'&&node.type.ordered===true)expected=this.resolveType(node.type,path,node.span);
    const entries = [];
    const bindings = [];
    const orderChunks=[];
    const units = name => Array.from({ length: name.length }, (_, index) => name.charCodeAt(index));
    const expectedNames = expected?.kind === 'record' ? expected.fields.map(field => units(field.name)) : [];
    if (Object.hasOwn(node, 'recordProjection')) {
      if (!Array.isArray(node.entries)) throw typeError('checked record projection needs an ordered field array', node.span);
      const names = node.entries.map(entry => typeof entry?.name === 'string' ? units(entry.name) : []);
      const admitted = node.entries.map(entry => entry !== null && typeof entry === 'object' && !Array.isArray(entry)
        && Object.hasOwn(entry, 'name') && typeof entry.name === 'string'
        && Object.hasOwn(entry, 'spread') && entry.spread === false && Object.hasOwn(entry, 'value')
        && entry.value !== null && typeof entry.value === 'object' && !Array.isArray(entry.value)
        && Object.keys(entry).every(field => field === 'name' || field === 'spread' || field === 'value'));
      const error = readRecordProjectionError(node.recordProjection, names, admitted);
      if (error) throw typeError(error, node.span);
      for (const entry of node.entries) {
        const position = readRecordFieldPosition(expectedNames, units(entry.name));
        const want = position < 0 ? undefined : expected.fields[position].type;
        const value = this.expr(entry.value, env, path, want);
        entries.push({ name: entry.name, value });
      }
      const ordered=node.propertyOrder!==undefined;
      if(node.type?.kind==='record'&&Boolean(node.type.ordered)!==ordered)throw typeError(node.type.ordered?'an ordered record projection needs its actual own-field order':'a structural record projection cannot contain an own-field order',node.span);
      if(expected?.kind==='record'&&Boolean(expected.ordered)!==ordered)throw typeError(expected.ordered?'an ordered record projection needs its actual own-field order':'a structural record projection cannot contain an own-field order',node.span);
      const order=ordered?coerce(this.referenceValue(this.expr(node.propertyOrder,env,path,array(FLOAT))),array(FLOAT),this.language,node.span):null;
      return { k: 'record', entries, ...(ordered?{order}:{}), type: { kind: 'record', fields: entries.map(entry => ({ name: entry.name, type: entry.value.type })),...(ordered?{ordered:true}:{}) } };
    }
    const evaluationOrder = readRecordValueOrder(node.entries.length, false);
    if (evaluationOrder[0] < 0) throw typeError('record entries need an exact ordered input length', node.span);
    for (const index of evaluationOrder) {
      const entry = node.entries[index];
      if (entry.spread) {
        const value = this.referenceValue(this.expr(entry.value, env, path, undefined));
        if (value.type.kind !== 'record') throw typeError('record spread needs a record value', node.span);
        const name = this.freshName('ml_spread');
        bindings.push({ name, value });
        const object = { k: 'var', name, type: value.type };
        orderChunks.push({object});
        for (const [index, field] of value.type.fields.entries()) {
          const name=this.freshName('ml_spread_field');
          bindings.push({name,value:{k:'recordField',object,field:field.name,index,type:field.type}});
          entries.push({name:field.name,value:{k:'var',name,type:field.type}});
        }
      } else {
        orderChunks.push({name:entry.name});
        const position = readRecordFieldPosition(expectedNames, units(entry.name));
        const want = position < 0 ? undefined : expected.fields[position].type;
        const value = this.expr(entry.value, env, path, want);
        const name = this.freshName('ml_field');
        bindings.push({ name, value });
        entries.push({ name: entry.name, value: { k: 'var', name, type: value.type } });
      }
    }
    // Overwritten fields still evaluate: bind every entry before selecting the final values.
    const fieldOrder = readRecordFieldOrder(entries.map(entry => units(entry.name)));
    if (fieldOrder[0] < 0) throw typeError('record fields contain an invalid UTF16 name', node.span);
    const selected = fieldOrder.map(index => entries[index]);
    const ordered=node.propertyOrder!==undefined||expected?.ordered===true||this.orderedRecordSource===true;
    let order;
    if(node.propertyOrder!==undefined)order=coerce(this.referenceValue(this.expr(node.propertyOrder,env,path,array(FLOAT))),array(FLOAT),this.language,node.span);
    else if(ordered){
      if(node.entries.some(entry=>entry.spread)){
        const target=constructRecordPropertyNameRows(selected);
        const chunks=orderChunks.map(chunk=>{
          if(chunk.object){
            if(chunk.object.type.open||chunk.object.type.ordered!==true)throw unsupported('ordered record spread','the actual source own-field order must be retained and rebased',node.span);
            const source=constructRecordPropertyNameRows(chunk.object.type.fields);
            const actual={k:'method',op:'readJavaScriptRecordPropertyOrder',object:chunk.object,args:[],type:array(FLOAT)};
            return {k:'math',op:'rebaseJavaScriptRecordPropertyOrder',args:[target,source,actual],type:array(FLOAT)};
          }
          const index=selected.findIndex(field=>field.name===chunk.name);
          if(index<0)throw typeError('an ordered record needs every actual own field',node.span);
          return {k:'array',items:[{k:'lit',value:String(index),type:FLOAT}],type:array(FLOAT)};
        });
        order={k:'math',op:'mergeJavaScriptRecordPropertyOrder',args:[target,{k:'array',items:chunks,type:array(array(FLOAT))}],type:array(FLOAT)};
      }else{
        const insertion=readRecordFieldInsertionOrder(entries.map(entry=>units(entry.name)));
        const positions=insertion.map(index=>selected.findIndex(field=>field.name===entries[index].name));
        if(positions.some(index=>index<0))throw typeError('an ordered record needs every actual own field',node.span);
        order={k:'array',items:positions.map(index=>({k:'lit',value:String(index),type:FLOAT})),type:array(FLOAT)};
      }
    }
    const type = { kind: 'record', fields: selected.map((entry) => ({ name: entry.name, type: entry.value.type })),...(ordered?{ordered:true}:{}) };
    let body = { k: 'record', entries: selected, ...(ordered?{order}:{}), type };
    for (const index of readRecordValueOrder(bindings.length, true)) {
      const binding = bindings[index];
      body = { k: 'let', name: binding.name, value: binding.value, body, type };
    }
    return body;
  }

  primitiveMethod(node, object, env, path) {
    const receiver = object.type.kind;
    const minimum = readPrimitiveMethodMinimumArguments(receiver, node.op);
    const maximum = readPrimitiveMethodMaximumArguments(receiver, node.op);
    if (minimum < 0) throw unsupported(`method call .${node.op}()`, `.${node.op}() has no checked string operation`, node.span);
    if (node.args.length < minimum || node.args.length > maximum) throw unsupported(`method call .${node.op}()`, `.${node.op}() expects ${minimum === maximum ? minimum : `${minimum} to ${maximum}`} arguments`, node.span);
    const element = object.type.element;
    const obligation = readPrimitiveMethodArgumentObligation(receiver, node.op,
      element?.kind === 'option' ? element.element.kind : element?.kind ?? '');
    if (obligation) throw unsupported(readPrimitiveMethodArgumentObligationFeature(node.op), obligation, node.span);
    const values = [];
    for (let index = 0; index < readPrimitiveMethodArgumentCount(receiver, node.op); index += 1) {
      const kind = readPrimitiveMethodCheckedArgumentKind(receiver, node.op, index);
      const type = kind === 'element' ? object.type.element : kind === 'float' || kind === 'number-value' ? FLOAT : STRING;
      const form = readPrimitiveMethodDefaultForm(receiver, node.op, index);
      const fallback = form === 'absent' ? null : form === 'required' ? undefined
        : literal(form === 'float' ? FLOAT : STRING, readPrimitiveMethodDefaultValue(receiver, node.op, index));
      const argument = node.args[index];
      if (!argument) values.push(fallback);
      else {
        const value = this.expr(argument, env, path, type);
        values.push(coerce(kind === 'number-value' ? this.numberValue(value, argument.span ?? node.span) : value,
          type, this.language, argument.span ?? node.span));
      }
    }
    return this.materializePrimitiveMethod(node, object, values, path);
  }

  materializePrimitiveMethod(node, object, values, path) {
    const receiver = object.type.kind;
    const lowering = readPrimitiveMethodLoweringForm(receiver, node.op);
    if (lowering === 'relative-element-template') {
      const result = { kind: 'option', element: object.type.element, absence: 'undefined' };
      const signature = { kind: 'function', params: [object.type, FLOAT], ret: result };
      const fn = coerce(this.expr(createArrayRelativeElementTemplateExpression(object.type, node.span), new Map(), path, signature),
        signature, this.language, node.span);
      return flattenOption({ k: 'apply', fn, args: [object, ...values], type: fn.type.ret, span: node.span });
    }
    if (lowering === 'relative-character-template') {
      const signature = { kind: 'function', params: [STRING, FLOAT], ret: { kind: 'option', element: STRING, absence: 'undefined' } };
      const fn = coerce(this.expr(createStringRelativeCharacterTemplateExpression(node.span), new Map(), path, signature),
        signature, this.language, node.span);
      return { k: 'apply', fn, args: [object, ...values], type: fn.type.ret, span: node.span };
    }
    if (lowering === 'code-unit-template' || lowering === 'character-template') {
      const character = lowering === 'character-template';
      const signature = { kind: 'function', params: [STRING, FLOAT], ret: character ? STRING : FLOAT };
      const template = character ? createStringCharacterTemplateExpression(node.span) : createStringCodeUnitTemplateExpression(node.span);
      const fn = coerce(this.expr(template, new Map(), path, signature),
        signature, this.language, node.span);
      return { k: 'apply', fn, args: [object, ...values], type: fn.type.ret, span: node.span };
    }
    const form = readPrimitiveMethodCheckedResultForm(receiver, node.op);
    const type = form === 'receiver' ? object.type : form === 'bool' ? BOOL : form === 'float' ? FLOAT
      : form === 'array-string' ? array(STRING)
      : form === 'optional-float' ? { kind: 'option', element: FLOAT, absence: 'undefined' } : STRING;
    return { k: 'method', object, op: node.op, args: values, type, span: node.span };
  }

  method(node, env, path, expected, preparedReceiver) {
    const receiver = preparedReceiver??this.expr(node.object, env, path, undefined);
    const nullable=this.checkedProjection?null:checkJavaScriptNullishAccess(this,node,receiver,env,path,expected);
    if(nullable)return nullable;
    const recordWrite = checkRecordFieldWrite(this, node, receiver, env, path);
    if (recordWrite) return recordWrite;
    const arrayStateMethod = checkArrayStateMethod(this, node, receiver, env, path);
    if (arrayStateMethod) return arrayStateMethod;
    const referenceMethod = checkReferenceMethod(this, node, receiver, env, path);
    if (referenceMethod) return flattenOption(referenceMethod);
    const object = this.referenceValue(receiver);
    const ownOrder=checkOrderedRecordOrderRead(node,object);
    if(ownOrder)return ownOrder;
    const args = node.args;
    const count = (minimum, maximum = minimum) => {
      if (args.length < minimum || args.length > maximum) throw unsupported(`method call .${node.op}()`, `.${node.op}() expects ${minimum === maximum ? minimum : `${minimum} to ${maximum}`} arguments`, node.span);
    };
    const checked = (index, type, fallback) => args[index] ? coerce(this.expr(args[index], env, path, type), type, this.language, args[index].span ?? node.span) : fallback;
    const number = (index, fallback) => checked(index, FLOAT, literal(FLOAT, fallback));
    if (object.type.kind === 'record') {
      const field = object.type.fields.find((field) => field.name === node.op);
      const error = readRecordMethodError(field?.type.kind ?? '', node.op, field?.type.params?.length ?? 0, args.length);
      if (error) throw typeError(error, node.span);
      return { k: 'apply', fn: { k: 'recordField', object, field: node.op, index: object.type.fields.indexOf(field), type: field.type }, args: args.map((arg, index) => checked(index, field.type.params[index])), type: field.type.ret };
    }
    if (object.type.kind === 'set' || object.type.kind === 'map') {
      const key = object.type.kind === 'set' ? object.type.element : object.type.key;
      if (node.op === 'has' || (node.op === 'get' && object.type.kind === 'map')) {
        count(readCollectionAccessArgumentCount(node.op));
        return flattenOption({ k: 'collectionRead', object, op: node.op, key: checked(0, key), type: node.op === 'has' ? BOOL : { kind: 'option', element: object.type.value, absence: 'undefined' } });
      }
      if (['keys', 'values', 'entries'].includes(node.op)) {
        count(readCollectionAccessArgumentCount(node.op));
        const value = object.type.kind === 'map' ? object.type.value : key;
        const element = node.op === 'keys' ? key : node.op === 'values' ? value : { kind: 'tuple', elements: [key, value] };
        return { k: 'collectionItems', object, op: node.op, type: array(element) };
      }
      throw unsupported(`mutable collection .${node.op}()`, 'mutation requires shared reference identities and cells so aliases observe every change', node.span);
    }
    if (object.type.kind === 'string') {
      return this.primitiveMethod(node, object, env, path);
    }
    if (object.type.kind === 'array') {
      const element = object.type.element;
      const callback = (index, params, ret) => {
        const value = this.expr(args[index], env, path, { kind: 'function', params, ret });
        const kindError = readArrayCallbackSignatureError(node.op, value.type.kind, 0, params.length, true, true, '');
        if (kindError) throw typeError(kindError, node.span);
        const parametersMatch = value.type.params.length > params.length
          || value.type.params.every((type, index) => sameType(type, params[index]));
        const parameterError = readArrayCallbackSignatureError(node.op, value.type.kind,
          value.type.params.length, params.length, parametersMatch, true, '');
        if (parameterError) throw typeError(parameterError, node.span);
        if (ret) {
          const resultMatches = sameType(value.type.ret, ret);
          const resultError = readArrayCallbackSignatureError(node.op, value.type.kind,
            value.type.params.length, params.length, true, resultMatches, resultMatches ? '' : typeKey(ret));
          if (resultError) throw typeError(resultError, node.span);
        }
        return value;
      };
      if (node.op === 'map') {
        count(1);
        const params = [element, FLOAT, object.type];
        const fn = this.expr(args[0], env, path, { kind: 'function', params, ret: undefined });
        const selection = readArrayFromMapCallback(fn.type, params);
        const error = readArrayFromMapCallbackError(selection.status, selection.pairs.every(([left, right]) => sameType(left, right)));
        if (error) throw typeError(error, node.span);
        return { k: 'method', object, op: node.op, args: [fn], type: array(selection.ret), span: node.span };
      }
      const callbackForm = readArrayCallbackResultForm(node.op);
      if (callbackForm) {
        const countError = readArrayCallbackArgumentCountError(node.op, args.length);
        if (countError) throw unsupported(`method call .${node.op}()`, countError, node.span);
        const initialPosition = readArrayCallbackCheckedArgumentPosition(node.op, Boolean(args[1]), 0);
        const initial = initialPosition === 1 ? this.expr(args[1], env, path, expected) : null;
        const accumulator = initial ? initial.type : element;
        const params = [];
        for (let position = 0; ; position += 1) {
          const role = readArrayCallbackParameterRole(node.op, position);
          if (!role) break;
          params.push(role === 'accumulator' ? accumulator : role === 'element' ? element : role === 'index' ? FLOAT : object.type);
        }
        const context = readArrayCallbackCheckedResultContext(node.op);
        const fn = callback(0, params, context === 'bool' ? BOOL : context === 'accumulator' ? accumulator : undefined);
        const resultError = readArrayCallbackResultError(node.op, fn.type.ret.kind);
        if (resultError) throw typeError(resultError, node.span);
        const type = callbackForm === 'receiver' ? object.type
          : callbackForm === 'optional element' ? { kind: 'option', element, absence: 'undefined' }
          : callbackForm === 'float' ? FLOAT : callbackForm === 'bool' ? BOOL
          : callbackForm === 'unit' ? UNIT : callbackForm === 'accumulator' ? accumulator
          : callbackForm === 'mapped array' ? array(fn.type.ret) : fn.type.ret;
        return { k: 'method', object, op: node.op, args: initial ? [fn, initial] : [fn], type, span: node.span };
      }
      if (node.op === 'slice') {
        return this.primitiveMethod(node, object, env, path);
      }
      if (node.op === 'indexOf' || node.op === 'lastIndexOf' || node.op === 'includes') {
        return this.primitiveMethod(node, object, env, path);
      }
      if (node.op === 'join' || node.op === 'at') return this.primitiveMethod(node, object, env, path);
    }
    throw unsupported(`method call .${node.op}()`, `.${node.op}() has no checked operation on ${typeKey(object.type)}`, node.span);
  }

  arrayLiteral(node, env, path, expected) {
    let element = expected?.kind === 'array' ? expected.element : undefined;
    // Inference fixes the element type of an array literal with no element of its own.
    if (!element && node.element) element = this.resolveType(node.element, path, node.span);
    const parts = [];
    let run = null;
    for (const item of node.items) {
      const want = element && item.spread ? array(element) : element;
      let value = this.expr(item.value, env, path, want);
      if (item.spread) value = this.referenceValue(value);
      if (item.spread && value.type.kind !== 'array') {
        throw unsupported(`spread of ${typeKey(value.type)}`, 'only arrays are spread into an array literal', item.value.span ?? node.span);
      }
      let type = item.spread ? value.type.element : value.type;
      // Guarded parameters are naturals, and every BigInt an integer.
      if (!element && this.language === 'JavaScript' && type.kind === 'nat') type = INT;
      element ??= type;
      value = coerce(value, item.spread ? array(element) : element, this.language, item.value.span ?? node.span);
      if (item.spread) {
        parts.push(value);
        run = null;
      } else if (run) {
        run.items.push(value);
      } else {
        run = { k: 'array', items: [value], type: array(element) };
        parts.push(run);
      }
    }
    if (!element) throw unsupported('empty array of an unknown type', 'nothing fixes the type of the elements of this empty array; declare it with JSDoc', node.span);
    if (!parts.length) return { k: 'array', items: [], type: array(element) };
    if (parts.length === 1 && node.items.length === 1 && node.items[0].spread) {
      return { k: 'append', left: { k: 'array', items: [], type: array(element) }, right: parts[0], type: array(element) };
    }
    return parts.reduce((left, right) => ({ k: 'append', left, right, type: array(element) }));
  }

  /**
   * `Math.floor(x)`, `Math.max(a, ...xs)`, `Number.isInteger(x)`: the
   * arguments are Numbers, which none of them converts, and a spread
   * argument makes `max` and `min` a fold over one array.
   */
  math(node, env, path, expected) {
    if(node.op===readJavaScriptRecordFormalDefaultOperation())return checkJavaScriptBindingDefault(this,readJavaScriptRecordFormalDefaultOperands(node,this.language),env,path);
    if(node.op===readJavaScriptArrayFormalIterableOperation())return checkJavaScriptArrayFormalIterable(this,node,env,path);
    const enumeration=checkJavaScriptObjectEnumeration(this,node,env,path,expected);
    if(enumeration)return enumeration;
    const order=checkRecordOrderOperation(this,node,env,path,expected);
    if(order)return order;
    if(node.op==='testJavaScriptOwnedProperty')return checkJavaScriptOwnProperty(this,node,env,path);
    if(node.op===readJavaScriptOptionalChainOperation())return checkJavaScriptOptionalChain(this,node,env,path);
    if(node.op===readJavaScriptOptionalCallOperation())return checkJavaScriptOptionalCall(this,node,env,path);
    if(node.op==='testJavaScriptArrayValue')return checkJavaScriptArrayBuiltin(this,node,env,path);
    if(node.op==='convertJavaScriptBooleanValue')return checkJavaScriptBooleanBuiltin(this,node,env,path);
    if(node.op==='convertJavaScriptPrimitiveText')return checkJavaScriptPrimitiveTextBuiltin(this,node,env,path);
    if(node.op==='constructJavaScriptStringFromCharacterCodes'||node.op==='constructJavaScriptStringFromCodePoints')return checkJavaScriptStringConstruction(this,node,env,path);
    if(acceptLinkedJavaScriptStringConstruction('math',node.op))return checkJavaScriptStringConstructionValue(this,node,env,path);
    if(acceptLinkedJavaScriptStringUnitConstruction('math',node.op)){
      if(this.language!=='JavaScript'||node.args.length!==1||node.args[0].spread)throw typeError('a JavaScript UTF16 literal needs one owned unit column',node.span);
      const units=this.expr(node.args[0].value,env,path,array(FLOAT));
      if(units.k!=='array'||units.items.some(item=>item.k!=='lit'||item.type.kind!=='float')||!acceptJavaScriptStringUnits(units.items.map(item=>Number(item.value))))throw typeError('a JavaScript UTF16 literal needs explicit checked integral code units',node.span);
      return{k:'math',op:node.op,args:[units],type:STRING};
    }
    if(node.op==='convertJavaScriptBigInteger')return checkJavaScriptBigIntegerBuiltin(this,node,env,path);
    if(acceptJavaScriptSameValueOperation(node.op))return checkJavaScriptSameValue(this,node,env,path);
    if(acceptJavaScriptLooseEqualityOperation(node.op))return checkJavaScriptLooseComparison(this,node,env,path);
    if(node.op==='exponentiateJavaScriptNumericValue')return checkJavaScriptExponentiation(this,node,env,path,expected);
    if(acceptLinkedJavaScriptBigIntegerResult('math',node.op))return checkJavaScriptBigIntegerResult(this,node,env,path);
    if(readJavaScriptExceptionConstructorClass(node.op))return checkJavaScriptSourceException(this,node,env,path,expected);
    if(acceptLinkedJavaScriptSourceException('math',node.op))return checkJavaScriptSourceExceptionValue(this,node,env,path,expected);
    if(readJavaScriptNumberConversionResult(node.op))return checkJavaScriptNumberBuiltin(this,node,env,path);
    if (readRustShiftCheckedOperation(node.op)) return checkRustShift(this,node,env,path,expected);
    if (readRustPrimitiveBitwiseArity(node.op)) return checkRustPrimitiveBitwise(this, node, env, path, expected);
    const numberArity=readJavaScriptNumberBitwiseArity(node.op);
    const bitwiseArity=numberArity || readBigIntegerBitwiseArity(node.op);
    if(bitwiseArity){
      if(node.args.length!==bitwiseArity||node.args.some(item=>item.spread))throw typeError('a Number bitwise operation requires its exact unspread operands',node.span);
      const args=node.args.map(({value})=>this.numberValue(this.expr(value,env,path,undefined),value.span??node.span));
      const failure=materializeJavaScriptNumericFailure(this,node.op,args,expected);
      if(failure)return failure;
      const domain=readPrimitiveBitwiseDomain(args.map(argument=>argument.type.kind));
      const operation=domain==='BigInt'?readBigIntegerBitwiseOperation(node.op):'';
      if(operation) return{k:'math',op:operation,args:args.map(argument=>coerce(argument,INT,this.language,node.span)),type:INT};
      if(domain==='Number'&&numberArity) return{k:'math',op:node.op,args,type:FLOAT};
      throw unsupported('primitive bitwise operand','the actual operands need one complete Number or BigInt domain; implicit coercions and unsigned BigInt shifts retain their own obligations',node.span);
    }
    const type = PREDICATES.has(node.op) ? BOOL : FLOAT;
    const variadic = node.op === 'max' || node.op === 'min';
    if (variadic && node.args.some((item) => item.spread)) {
      const values = this.arrayLiteral({ k: 'array', items: node.args, span: node.span }, env, path, array(FLOAT));
      return { k: 'math', op: `${node.op}Of`, args: [values], type };
    }
    const spread = node.args.find((item) => item.spread);
    if (spread) throw unsupported('spread argument', `pass ${node.name} its argument`, spread.value.span ?? node.span);
    if (readNumberPredicateArgumentForm(this.language, node.op, '', '')) {
      const args = node.args.map(({ value }) => this.expr(value, env, path, undefined));
      if (args.length === 0) return { k: 'lit', value: false, type: BOOL };
      const first = args[0];
      const form = readNumberPredicateArgumentForm(this.language, node.op, first.type.kind, first.type.kind === 'option' ? first.type.element.kind : '');
      if (form === 'unsupported') throw typeError(`${node.name} of ${typeKey(first.type)}; it takes Numbers, and converts or rejects anything else`, node.args[0].value.span ?? node.span);
      if (args.length === 1 && form === 'number') return { k: 'math', op: node.op, args, type: BOOL };
      if (args.length === 1 && form === 'optional-number') return { k: 'optionalPredicate', op: node.op, arg: first, type: BOOL };
      const name = this.freshName('ml_number_predicate');
      const bound = { k: 'var', name, type: first.type };
      let body = form === 'number' ? { k: 'math', op: node.op, args: [bound], type: BOOL }
        : form === 'optional-number' ? { k: 'optionalPredicate', op: node.op, arg: bound, type: BOOL }
        : { k: 'lit', value: false, type: BOOL };
      for (let index = args.length - 1; index >= 0; index--) {
        body = { k: 'let', name: index === 0 ? name : this.freshName('ml_number_predicate_unused'), value: args[index], body, type: BOOL };
      }
      return body;
    }
    const mathematical=readJavaScriptMathematicalConversionForm(this.language,node.op);
    if(mathematical)return checkJavaScriptMathematicalArguments(this,node,env,path,mathematical);
    if (!variadic && node.args.length !== 1) {
      throw unsupported(`${node.name} with ${node.args.length} arguments`, `${node.name} takes one Number`, node.span);
    }
    const args = node.args.map(({ value }) => {
      let arg = this.expr(value, env, path, FLOAT);
      if (arg.type.kind === 'option') {
        if (PREDICATES.has(node.op)) return { k: 'optionalPredicate', op: node.op, arg, type: BOOL };
        arg = this.numberValue(arg, value.span ?? node.span);
      }
      if (!isFloat(arg.type)) {
        throw typeError(`${node.name} of ${typeKey(arg.type)}; it takes Numbers, and converts or rejects anything else`, value.span ?? node.span);
      }
      return arg;
    });
    if (PREDICATES.has(node.op) && args[0]?.k === 'optionalPredicate') return args[0];
    return { k: 'math', op: node.op, args, type };
  }

  binary(node, env, path, expected, allowLiteral) {
    const { op } = node;
    if (op === 'and' || op === 'or') {
      const left = this.expr(node.left, env, path, undefined);
      const branchesEnv=this.language==='JavaScript'?readJavaScriptOwnPropertyBranchEnvironments(node.left,env):{then:env,else:env};
      const right = this.expr(node.right, op==='and'?branchesEnv.then:branchesEnv.else, path, undefined);
      const operation = readOptionalLogicalOperation(this.language, left.type.kind, right.type.kind,
        left.type.kind === 'option' && sameType(left.type.element, right.type), right.type.kind === 'option' && sameType(right.type.element, left.type), sameType(left.type, right.type), op);
      if (operation === 'boolean') return { k: 'binary', op, left, right, type: BOOL };
      if (operation === 'language-error') throw typeError('logical operands must be booleans', node.span);
      const name = this.freshName('ml_logical');
      const bound = { k: 'var', name, type: left.type };
      let retained = bound;
      let other = right;
      let type = left.type;
      if (operation === 'unwrap-left') { retained = { k: 'unwrapOption', arg: bound, type: right.type }; type = right.type; }
      else if (operation === 'wrap-right') other = coerce(right, type, this.language, node.span);
      else if (operation === 'wrap-left') { type = right.type; retained = coerce(bound, type, this.language, node.span); }
      else if (operation === 'union-error') throw unsupported('logical value union', 'these logical operands need a tagged union of their distinct value types', node.span);
      const cond = this.booleanValue(bound, node.span);
      const body = { k: 'if', cond, then: op === 'or' ? retained : other, else: op === 'or' ? other : retained, type };
      return { k: 'let', name, value: left, body, type };
    }
    if (op === 'concat') {
      const left = coerce(this.expr(node.left, env, path, STRING), STRING, this.language, node.span);
      const right = coerce(this.expr(node.right, env, path, STRING), STRING, this.language, node.span);
      return { k: 'binary', op, left, right, type: STRING };
    }
    if (COMPARISONS.has(op)) {
      if (this.language === 'JavaScript') {
        let left = this.expr(node.left, env, path, undefined, true);
        let right = this.expr(node.right, env, path, undefined, true);
        if (left.type.kind === 'literal') left = this.expr(node.left, env, path, this.defaultNumber());
        if (right.type.kind === 'literal') right = this.expr(node.right, env, path, this.defaultNumber());
        const comparison=materializeJavaScriptPrimitiveComparison(this,op,left,right);
        if(comparison)return comparison;
        if (left.type.kind === 'option' || right.type.kind === 'option') {
          const a = left.type.kind === 'option' ? left.type.element : left.type;
          const b = right.type.kind === 'option' ? right.type.element : right.type;
          const operation = readOptionalComparisonOperation(this.language, left.type.kind, right.type.kind, op, sameType(a, b), primitiveKey(a));
          if (operation === 'object-error') throw unsupported('nullable object equality', 'equality of these values requires shared reference identities', node.span);
          if (operation === 'strict') return { k: 'strictEqual', left, right, negate: op === 'ne', type: BOOL };
          if([left,right].some(value=>value.type.kind==='option'&&value.type.element.kind==='string'))throw unsupported('nullable arithmetic operand',
            'nullable string ordering needs its lexical present-value branch before numeric conversion',node.span);
          left = this.numberValue(left, node.span); right = this.numberValue(right, node.span);
          [left, right] = unify(left, right, this.language, node.span);
          return { k: 'binary', op, left, right, type: BOOL, domain: left.type };
        }
      }
      const [left, right] = this.operands(node.left, node.right, env, path, node.span);
      if (this.language === 'JavaScript' && acceptReferenceIdentityComparison(op, left.type.kind, right.type.kind, left.type.reference === true, right.type.reference === true, sameType(left.type, right.type))) {
        return {k: 'binary', op, left, right, type: BOOL, domain: left.type};
      }
      if (left.type.kind === 'tuple') throw unsupported('comparison of tuples', 'tuple value equality and JavaScript reference identity need an explicit operational comparison model; compare positional elements', node.span);
      if (left.type.kind === 'data' || left.type.kind === 'unit') {
        throw unsupported('structural equality of data values', 'comparisons of data values are not in the portable core', node.span);
      }
      if (left.type.kind === 'array') {
        throw unsupported('comparison of arrays', '=== of two arrays compares which array each is, which a value translation does not keep; compare their elements', node.span);
      }
      const stringOrder = readStringOrderOperation(this.language, op, left.type.kind);
      if (stringOrder) return { k: 'stringTest', op: stringOrder, string: left, search: right, type: BOOL, span: node.span };
      if (op !== 'eq' && op !== 'ne' && !isOrdered(left.type)) throw typeError(`ordering on ${typeKey(left.type)}`, node.span);
      return { k: 'binary', op, left, right, type: BOOL, domain: left.type };
    }
    if (op === 'plus') return this.plus(node, env, path, expected, allowLiteral);
    return this.arithmetic(node, env, path, expected, allowLiteral);
  }

  /**
   * JavaScript's `+` concatenates when either operand is a string, reading
   * the other as `String(value)`, and adds numbers otherwise.
   */
  plus(node, env, path, expected, allowLiteral) {
    const left = this.expr(node.left, env, path, undefined, true);
    const right = this.expr(node.right, env, path, undefined, true);
    if (left.type.kind !== 'string' && right.type.kind !== 'string') {
      return this.arithmetic({ ...node, op: 'add' }, env, path, expected, allowLiteral, [left, right]);
    }
    const text = (value, surface) => {
      if (value.type.kind === 'string') return value;
      const checked = value.type.kind === 'literal' ? this.expr(surface, env, path, this.defaultNumber()) : value;
      if (checked.type.kind === 'data' || checked.type.kind === 'unit') {
        throw unsupported('string conversion of structured values', `String(${typeKey(checked.type)}) has no portable textual form`, surface.span);
      }
      if (checked.type.kind === 'array') throw arrayText(surface.span);
      if (checked.type.kind === 'option') {
        if (!primitiveKey(checked.type.element)) throw unsupported('string conversion of nullable structured values', 'this conversion needs its source structured-value formatter', surface.span);
        return { k: 'optionalText', arg: checked, console: false, type: STRING };
      }
      return { k: 'toString', arg: checked, type: STRING };
    };
    return { k: 'binary', op: 'concat', left: text(left, node.left), right: text(right, node.right), type: STRING };
  }

  /** `checked` holds operands already checked without an expected type, as `plus` does. */
  arithmetic(node, env, path, expected, allowLiteral, checked = undefined) {
    const { op } = node;
    if (!ARITHMETIC.has(op)) throw typeError(`unknown operator ${op}`, node.span);
    const numericExpected = expected && isNumeric(expected) ? expected : undefined;
    let left = checked && checked[0].type.kind !== 'literal' ? checked[0] : this.expr(node.left, env, path, numericExpected, true);
    let right = checked && checked[1].type.kind !== 'literal'
      ? checked[1]
      : this.expr(node.right, env, path, left.type.kind === 'literal' ? numericExpected : left.type, true);
    if (left.type.kind === 'literal') left = this.expr(node.left, env, path, right.type.kind === 'literal' ? numericExpected : right.type, true);
    if (left.type.kind === 'literal' && right.type.kind === 'literal') {
      if (allowLiteral) return { k: 'lit', type: { kind: 'literal' }, value: '0', pending: node };
      left = this.expr(node.left, env, path, this.defaultNumber());
      right = this.expr(node.right, env, path, left.type);
    }
    if (right.type.kind === 'literal') right = this.expr(node.right, env, path, left.type);
    if (left.type.kind === 'literal') left = this.expr(node.left, env, path, right.type);
    if (this.language === 'JavaScript') {
      left = this.numberValue(left, node.span);
      right = this.numberValue(right, node.span);
    }
    if (this.language === 'JavaScript' && (left.type.kind === 'nat' || right.type.kind === 'nat')) {
      // BigInt arithmetic is integer arithmetic even on guarded naturals: `n - 1n` is -1n for n = 0n.
      const widen = (value) => (value.k === 'lit' ? literal(INT, value.value) : coerce(value, INT, this.language, node.span));
      left = widen(left);
      right = widen(right);
    }
    const failure=materializeJavaScriptNumericFailure(this,op,[left,right],expected);
    if(failure)return failure;
    [left, right] = unify(left, right, this.language, node.span);
    if (!isOrdered(left.type)) throw typeError(`arithmetic on ${typeKey(left.type)}`, node.span);
    const division=materializeJavaScriptBigIntegerDivision(this,op,left,right);
    if(division)return division;
    const semantics = arithmeticSemantics(this.language, op, left.type, node);
    return { k: 'binary', op, left, right, type: left.type, domain: left.type, ...semantics };
  }

  numberValue(value, span) {
    const primitive=readJavaScriptPrimitiveNumberOperand(this.language,value.type.kind,value.type.element?.kind??'');
    if(primitive)return materializeJavaScriptNumberConversion(this,value,primitive);
    const operation = readOptionalNumberOperation(this.language, value.type.kind, value.type.element?.kind ?? '');
    if (operation === 'identity') return value;
    if (operation === 'unsupported') throw unsupported('nullable arithmetic operand', 'this operand needs its source numeric coercion and error behavior represented', span);
    return { k: 'nullishNumber', arg: value, type: FLOAT };
  }

  booleanValue(value, span) {
    const operation = readOptionalBooleanOperation(this.language, value.type.kind);
    if (operation === 'identity') return value;
    if (operation === 'coerce') return coerce(value, BOOL, this.language, span);
    return { k: 'truthy', arg: value, type: BOOL };
  }

  referenceValue(value) {
    return selectSurfaceReferenceRead(value.type.reference) ? { k: 'referenceRead', reference: value, type: { ...value.type, [readSurfaceReferenceResultField('referenceRead')]: false } } : value;
  }

  application(head, args, env, path, expected, span) {
    if (requireCallableComputedApplication(head.k, head.path?.length ?? 0, head.path?.length === 1 ? env.get(head.path[0])?.kind ?? '' : '')) {
      const fn = this.expr(head, env, path, undefined);
      const error = readCallableApplicationError(fn.type.kind, fn.type.params?.length ?? 0, args.length);
      if (error) throw typeError(error, span);
      return { k: 'apply', fn, args: args.map((arg, index) => coerce(this.expr(arg, env, path, fn.type.params[index]), fn.type.params[index], this.language, span)), type: fn.type.ret };
    }
    if (head.k === 'dotCtor') {
      if (!expected || expected.kind !== 'data') throw typeError(`cannot infer the type of .${head.name}`, span);
      const dataEntry = this.items.get(expected.name);
      const ctor = dataEntry.ctors.find((candidate) => candidate.name === head.name);
      if (!ctor) throw typeError(`${expected.name} has no constructor ${head.name}`, span);
      return this.construct(dataEntry, ctor, args, env, path, span);
    }
    if (head.k !== 'name') {
      throw unsupported('higher-order application', 'only named functions and constructors can be applied', span);
    }
    if (head.path.length === 1 && env.has(head.path[0])) {
      throw unsupported('higher-order application', `local ${head.path[0]} is not a function in the portable core`, span);
    }
    const entry = this.lookup(head.path, path, span);
    if (!entry) throw typeError(`unknown name ${head.path.join('.')}`, span);
    if (entry.k === 'const') {
      if (args.length) throw typeError(`${head.path.join('.')} is not a function`, span);
      return { k: 'call', fn: entry.fullName, args: [], type: entry.type };
    }
    if (entry.k === 'ctor') {
      const dataEntry = this.items.get(entry.data);
      const ctor = dataEntry.ctors.find((candidate) => candidate.name === entry.name);
      return this.construct(dataEntry, ctor, args, env, path, span);
    }
    if (entry.k !== 'fn') throw typeError(`${head.path.join('.')} is not a function`, span);
    if (!entry.params) throw typeError(`${entry.fullName} is used before its signature is known`, span);
    const provided = args.length;
    // A call that leaves out trailing parameters with defaults passes the defaults.
    if (args.length < entry.params.length && acceptArgumentCount(entry.params.map((param) => Boolean(param.default || param.optional)), args.length)) {
      args = [...args, ...entry.params.slice(args.length).map((param) => ({ ...(param.default ?? { k: 'none', absence: 'undefined' }), closedDefault: true }))];
    }
    if (args.length !== entry.params.length) {
      if (args.length < entry.params.length) {
        throw unsupported('partial application', `${entry.fullName} expects ${entry.params.length} arguments`, span);
      }
      throw typeError(`${entry.fullName} expects ${entry.params.length} arguments but got ${args.length}`, span);
    }
    const plans = args.map((arg, index) => {
      const param = entry.params[index];
      const replaced = Boolean(param.default && arg.k === 'none' && arg.absence === 'undefined');
      if (replaced) arg = { ...param.default, closedDefault: true };
      // A guarded argument is an integer the guard checks, wherever in it a negative value arises.
      let value = this.expr(arg, arg.closedDefault ? new Map() : env, arg.closedDefault ? entry.modulePath : path, param.guard && arg.k !== 'num' ? INT : param.type);
      const input = index < provided && !replaced ? value : null;
      let fallback = replaced ? value : null;
      let coalesced = false;
      if (param.default && value.type.kind === 'option' && value.type.absence === 'undefined' && sameType(value.type.element, param.type)) {
        fallback = coerce(this.expr(param.default, new Map(), entry.modulePath, param.type), param.type, this.language, span);
        value = { k: 'coalesce', left: value, right: fallback, type: param.type };
        coalesced = true;
      }
      return { param, value, input, fallback, coalesced, span: arg.span ?? span };
    });
    const pureDefault = (value) => {
      for (;;) {
        const step = readParameterDefaultValueStep(value.k, value.flavor ?? '');
        if (step === 'arg') value = value.arg;
        else if (step === 'value') value = value.value;
        else return step === 'pure';
      }
    };
    const bindArguments = plans.some((plan, index) => plan.fallback
      && requireParameterDefaultArgumentBindings(index, provided, pureDefault(plan.fallback)));
    const bindings = [];
    if (bindArguments) {
      // Literal undefined has no effects. Every other actual input is bound
      // once, before any initializer; defaults remain in their callee scope.
      for (const plan of plans) {
        if (!plan.input) continue;
        const name = this.freshName('ml_a');
        bindings.push({ name, value: plan.input });
        const value = { k: 'var', name, type: plan.input.type };
        plan.value = plan.coalesced ? { ...plan.value, left: value } : value;
      }
    }
    const checkedArgs = plans.map(({ param, value, span: where }) => {
      const checked = coerce(value, param.type, this.language, where, param.guard ? 'checked' : undefined);
      return checked.k === 'cast' && checked.flavor === 'checked' ? { ...checked, message: param.guard.message, order: param.guard.order } : checked;
    });
    let call = this.guardOrder({ k: 'call', fn: entry.fullName, args: checkedArgs, type: entry.ret });
    for (const binding of bindings.reverse()) call = { k: 'let', name: binding.name, value: binding.value, body: call, type: call.type };
    return call;
  }

  /**
   * A JavaScript call evaluates every argument, then the callee's guards run
   * in their order. Checking each argument where it is passed is the same
   * unless a later argument could print or abort, or the guards run in
   * another order; then the arguments are bound first and checked after.
   */
  guardOrder(call) {
    const guarded = call.args.map((arg, index) => ({ arg, index })).filter(({ arg }) => arg.k === 'cast' && arg.order !== undefined);
    if (!guarded.length) return call;
    const simple = (e) => e.k === 'var' || e.k === 'lit' || (e.k === 'cast' && e.flavor === 'exact' && simple(e.arg));
    const first = guarded[0].index;
    const inOrder = guarded.every(({ arg }, at) => !at || guarded[at - 1].arg.order < arg.order);
    const later = call.args.slice(first + 1).every((arg) => simple(arg.order === undefined ? arg : arg.arg));
    if (inOrder && later) return call;
    const bound = call.args.map((arg) => {
      const value = arg.order === undefined ? arg : arg.arg;
      return { name: this.freshName('ml_a'), value };
    });
    const checks = guarded
      .toSorted((left, right) => left.arg.order - right.arg.order)
      .map(({ arg, index }) => ({ index, name: this.freshName('ml_a'), value: { ...arg, arg: { k: 'var', name: bound[index].name, type: arg.arg.type } } }));
    const args = call.args.map((arg, index) => {
      const check = checks.find((candidate) => candidate.index === index);
      return check ? { k: 'var', name: check.name, type: arg.type } : { k: 'var', name: bound[index].name, type: arg.type };
    });
    let body = { ...call, args };
    for (const binding of [...bound, ...checks].reverse()) body = { k: 'let', name: binding.name, value: binding.value, body, type: body.type };
    return body;
  }

  construct(dataEntry, ctor, args, env, path, span) {
    if (args.length !== ctor.fields.length) {
      throw typeError(`${dataEntry.fullName}.${ctor.name} expects ${ctor.fields.length} fields but got ${args.length}`, span);
    }
    const checkedArgs = args.map((arg, index) => coerce(
      this.expr(arg, env, path, ctor.fields[index].type),
      ctor.fields[index].type,
      this.language,
      arg.span ?? span,
    ));
    return { k: 'ctor', data: dataEntry.fullName, ctor: ctor.name, args: checkedArgs, type: data(dataEntry.fullName) };
  }

  ctorObject(node, env, path, expected) {
    const candidates = [];
    for (const entry of this.items.values()) {
      if (entry.k !== 'data' || !entry.ctors) continue;
      if (expected && expected.kind === 'data' && entry.fullName !== expected.name) continue;
      const ctor = entry.ctors.find((candidate) => candidate.name === node.tag);
      if (ctor) candidates.push([entry, ctor]);
    }
    if (candidates.length !== 1) {
      throw typeError(candidates.length === 0 ? `no data type has tag ${node.tag}` : `tag ${node.tag} is ambiguous`, node.span);
    }
    const [entry, ctor] = candidates[0];
    const names = Object.keys(node.fields);
    const expectedNames = ctor.fields.map((field) => field.name);
    if (names.length !== expectedNames.length || names.some((name) => !expectedNames.includes(name))) {
      throw typeError(`${node.tag} expects fields ${expectedNames.join(', ')}`, node.span);
    }
    const value = this.construct(entry, ctor, expectedNames.map((name) => node.fields[name]), env, path, node.span);
    if (Object.hasOwn(node, 'frozen') && typeof node.frozen !== 'boolean') throw typeError('constructor immutability must be Boolean', node.span);
    return node.frozen ? { ...value, frozen: true } : value;
  }

  /**
   * Compiles a surface match (several scrutinees, nested and literal
   * patterns) into single-level constructor matches and literal if-chains.
   * Bind patterns name the constructor fields directly so structural
   * recursion stays visible to the recursion analysis.
   */
  compileMatch(node, env, path) {
    const lets = [];
    const columns = node.scrutinees.map((scrutinee) => {
      const checked = this.expr(scrutinee, env, path, undefined);
      if (scrutinee.k === 'name' && scrutinee.path.length === 1 && env.has(scrutinee.path[0])) {
        return { name: scrutinee.path[0], type: checked.type };
      }
      const name = this.freshName('ml_s');
      lets.push({ name, value: scrutinee, type: checked.type });
      return { name, type: checked.type };
    });
    const rows = node.rows.map((row) => ({ patterns: row.patterns, body: row.body, binds: [], span: row.span }));
    let body = this.compileRows(columns, rows, node.span);
    for (const binding of lets.reverse()) {
      body = { k: 'let', name: binding.name, value: binding.value, body, span: node.span };
    }
    return body;
  }

  freshName(prefix) {
    this.fresh += 1;
    return renderSourceGeneratedIdentifier(this.generatedPrefix, prefix, this.fresh);
  }

  compileRows(columns, rows, span) {
    if (!rows.length) throw typeError('non-exhaustive match', span);
    const normalised = rows.map((row) => ({
      ...row,
      patterns: row.patterns.map((pattern, index) => this.normalisePattern(pattern, columns[index].type, span)),
    }));
    const [first] = normalised;
    const refutable = first.patterns.findIndex((pattern) => pattern.k !== 'wild' && pattern.k !== 'bind');
    if (refutable < 0) {
      const binds = [...first.binds];
      first.patterns.forEach((pattern, index) => {
        if (pattern.k === 'bind') binds.push([pattern.name, columns[index].name]);
      });
      return binds.reduceRight((body, [name, variable]) => (name === variable
        ? body
        : { k: 'let', name, value: { k: 'name', path: [variable] }, body, span }), first.body);
    }
    const column = columns[refutable];
    const pivot = first.patterns[refutable];
    const rest = (patterns) => patterns.filter((_, index) => index !== refutable);
    const bindColumn = (row) => {
      const pattern = row.patterns[refutable];
      return pattern.k === 'bind' ? [...row.binds, [pattern.name, column.name]] : row.binds;
    };
    if(pivot.k==='tuple'){
      const reserved=new Set(columns.map(item=>item.name));
      const collect=value=>{
        if(typeof value==='string'){if(value.startsWith('ml_tuple_field'))reserved.add(value);return;}
        if(value&&typeof value==='object')for(const item of Object.values(value))collect(item);
      };
      for(const row of normalised)collect(row);
      const fields=column.type.elements.map(type=>{
        let name;do{name=this.freshName('ml_tuple_field');}while(reserved.has(name));
        reserved.add(name);return{name,type};
      });
      const expanded=normalised.map(row=>{
        const pattern=row.patterns[refutable];
        return{...row,patterns:[...(pattern.k==='tuple'?pattern.args:fields.map(()=>({k:'wild'}))),...rest(row.patterns)],binds:bindColumn(row)};
      });
      let body=this.compileRows([...fields,...columns.filter((_,index)=>index!==refutable)],expanded,span);
      for(let index=fields.length-1;index>=0;index-=1)body={k:'let',name:fields[index].name,value:{k:'tupleIndex',object:{k:'name',path:[column.name]},index},body,span};
      return body;
    }
    // A natural column that also holds successor patterns splits on zero and
    // successor, which decrements its literal patterns.
    const natural = column.type.kind === 'nat' && normalised.some((row) => row.patterns[refutable].k === 'nat');
    if (pivot.k === 'lit' && !natural) {
      // Literal patterns on integers, booleans and strings become equality tests.
      const matching = normalised.filter((row) => ['wild', 'bind'].includes(row.patterns[refutable].k)
        || (row.patterns[refutable].k === 'lit' && row.patterns[refutable].key === pivot.key));
      // Where a boolean column is not the pivot it is the other value, so its
      // remaining literal patterns always match there.
      const bool = column.type.kind === 'bool';
      const others = normalised
        .filter((row) => row.patterns[refutable].k !== 'lit' || row.patterns[refutable].key !== pivot.key)
        .map((row) => (bool && row.patterns[refutable].k === 'lit'
          ? { ...row, patterns: row.patterns.map((pattern, index) => (index === refutable ? { k: 'wild' } : pattern)) }
          : row));
      const remaining = columns.filter((_, index) => index !== refutable);
      const then = this.compileRows(remaining, matching.map((row) => ({ ...row, patterns: rest(row.patterns), binds: bindColumn(row) })), span);
      if (bool && others.length === 0) throw typeError('non-exhaustive match on bool', span);
      const otherwise = this.compileRows(columns, others, span);
      return {
        k: 'if',
        cond: { k: 'binary', op: 'eq', left: { k: 'name', path: [column.name] }, right: pivot.value },
        then,
        else: otherwise,
        span,
      };
    }
    const ctors = natural
      ? [{ name: 'zero', fields: [] }, { name: 'succ', fields: [{ type: column.type }] }]
      : this.items.get(column.type.name).ctors;
    const cases = ctors.map((ctor) => {
      const fieldNames = ctor.fields.map((_, fieldIndex) => {
        for (const row of normalised) {
          const pattern = row.patterns[refutable];
          if (pattern.ctor === ctor.name && pattern.args[fieldIndex].k === 'bind') return pattern.args[fieldIndex].name;
        }
        return this.freshName('ml_f');
      });
      const fieldColumns = ctor.fields.map((field, fieldIndex) => ({ name: fieldNames[fieldIndex], type: field.type }));
      const specialised = [];
      for (const row of normalised) {
        const pattern = row.patterns[refutable];
        if (pattern.k === 'wild' || pattern.k === 'bind') {
          specialised.push({
            ...row,
            patterns: [...fieldColumns.map(() => ({ k: 'wild' })), ...rest(row.patterns)],
            binds: bindColumn(row),
          });
        } else if (pattern.ctor === ctor.name) {
          specialised.push({ ...row, patterns: [...pattern.args, ...rest(row.patterns)] });
        } else if (pattern.k === 'lit') {
          const value = BigInt(pattern.value.value);
          if (ctor.name === 'zero' && value === 0n) specialised.push({ ...row, patterns: rest(row.patterns) });
          if (ctor.name === 'succ' && value > 0n) {
            const predecessor = this.literalPattern({ k: 'num', value: String(value - 1n), negative: false }, column.type, span);
            specialised.push({ ...row, patterns: [predecessor, ...rest(row.patterns)] });
          }
        }
      }
      const body = this.compileRows([...fieldColumns, ...columns.filter((_, index) => index !== refutable)], specialised, span);
      const casePattern = natural
        ? (ctor.name === 'zero' ? { k: 'natZero' } : { k: 'natSucc', name: fieldNames[0] })
        : { k: 'ctor', path: [ctor.name], binds: fieldNames };
      return { pattern: casePattern, body };
    });
    return { k: 'match1', scrutinee: { k: 'name', path: [column.name] }, cases, span };
  }

  /** Resolves a surface pattern against the column type: constructors, naturals, literals, binders. */
  normalisePattern(pattern, type, span) {
    const natural = type.kind === 'nat';
    switch (pattern.k) {
      case 'wild':
      case 'bind':
      case 'nat':
      case 'data':
      case 'lit':
        return pattern;
      case 'bindOrCtor': {
        if (type.kind === 'data') {
          const ctor = this.items.get(type.name).ctors.find((candidate) => candidate.name === pattern.name);
          if (ctor) {
            if (ctor.fields.length) throw typeError(`${pattern.name} pattern needs ${ctor.fields.length} fields`, pattern.span ?? span);
            return { k: 'data', ctor: ctor.name, args: [] };
          }
        }
        if (natural && this.language === 'Rocq' && pattern.name === 'O') return { k: 'nat', ctor: 'zero', args: [] };
        if (type.kind === 'bool' && (pattern.name === 'true' || pattern.name === 'false')) {
          return this.literalPattern({ k: 'bool', value: pattern.name === 'true' }, type, span);
        }
        return { k: 'bind', name: pattern.name };
      }
      case 'numLit': {
        // A JavaScript `case 0n:` is an `===` test, which the recursion analysis reads as a zero test.
        // Numerals up to NAT_PATTERN_UNFOLD unfold to successor chains; larger
        // ones are equality tests, so a pattern never costs a node per unit.
        if (natural && this.language !== 'JavaScript' && BigInt(pattern.value) <= NAT_PATTERN_UNFOLD) {
          let result = { k: 'nat', ctor: 'zero', args: [] };
          for (let count = 0n; count < BigInt(pattern.value); count += 1n) result = { k: 'nat', ctor: 'succ', args: [result] };
          return result;
        }
        return this.literalPattern({ k: 'num', value: pattern.value, negative: pattern.negative }, type, span);
      }
      case 'boolLit':
        return this.literalPattern({ k: 'bool', value: pattern.value }, type, span);
      case 'tuple':{
        const width=type.kind==='tuple'?type.elements.length:type.kind==='unit'?0:-1;
        const plan=planTuplePatternFields(width,pattern.args.length,pattern.rest??-1);
        if(plan[0]<0)throw typeError('tuple pattern has the wrong type or number of fields',pattern.span??span);
        if(type.kind==='unit')return{k:'wild'};
        return{k:'tuple',args:plan.slice(1).map((index,position)=>index<0?{k:'wild'}:this.normalisePattern(pattern.args[index],type.elements[position],span))};
      }
      case 'strLit':
        return this.literalPattern({ k: 'str', value: pattern.value }, type, span);
      case 'natAdd': {
        if (!natural) throw unsupported('n + k pattern', `only natural-number scrutinees support n + k patterns, not ${typeKey(type)}`, pattern.span ?? span);
        if (BigInt(pattern.add) > NAT_PATTERN_UNFOLD) {
          throw unsupported('n + k pattern', `n + k patterns take offsets of at most ${NAT_PATTERN_UNFOLD}`, pattern.span ?? span);
        }
        let result = this.normalisePattern(pattern.inner, type, span);
        for (let count = 0; count < Number(pattern.add); count += 1) result = { k: 'nat', ctor: 'succ', args: [result] };
        return result;
      }
      case 'ctor': {
        const name = pattern.path.at(-1);
        if (natural && pattern.args.length === 1 && (name === 'succ' || name === 'S')) {
          return { k: 'nat', ctor: 'succ', args: [this.normalisePattern(pattern.args[0], type, span)] };
        }
        if (natural && pattern.args.length === 0 && (name === 'zero' || name === 'O')) return { k: 'nat', ctor: 'zero', args: [] };
        if (type.kind !== 'data') throw typeError(`constructor pattern ${name} on ${typeKey(type)}`, pattern.span ?? span);
        const dataEntry = this.items.get(type.name);
        if (pattern.path.length > 1) {
          const owner = this.lookup(pattern.path, [], pattern.span ?? span) ?? this.lookup(pattern.path, dataEntry.modulePath, pattern.span ?? span);
          if (!owner || owner.k !== 'ctor' || owner.data !== type.name) throw typeError(`${pattern.path.join('.')} is not a constructor of ${type.name}`, pattern.span ?? span);
        }
        const ctor = dataEntry.ctors.find((candidate) => candidate.name === name);
        if (!ctor) throw typeError(`${type.name} has no constructor ${name}`, pattern.span ?? span);
        if (pattern.args.length !== ctor.fields.length) {
          throw typeError(`${name} pattern has ${pattern.args.length} of ${ctor.fields.length} fields`, pattern.span ?? span);
        }
        return {
          k: 'data',
          ctor: name,
          args: pattern.args.map((arg, index) => this.normalisePattern(arg, ctor.fields[index].type, span)),
        };
      }
      default:
        throw unsupported('pattern', `${pattern.k} patterns are outside the portable core`, pattern.span ?? span);
    }
  }

  literalPattern(value, type, span) {
    const checked = this.expr(value, new Map(), [], type);
    if (!sameType(checked.type, type)) throw typeError(`literal pattern of type ${typeKey(checked.type)} on ${typeKey(type)}`, span);
    return { k: 'lit', value, key: `${typeKey(type)}:${checked.value}` };
  }

  match(node, env, path, expected, allowLiteral = false) {
    const variable = node.scrutinee.k === 'name' && node.scrutinee.path.length === 1 && env.has(node.scrutinee.path[0])
      ? node.scrutinee.path[0]
      : null;
    const fact = variable && env.get(KNOWN)?.get(variable);
    if (fact) return this.knownMatch(node, fact, variable, env, path, expected, allowLiteral);
    const scrutinee = this.expr(node.scrutinee, env, path, undefined);
    const type = scrutinee.type;
    const cases = [];
    // An Array.from/map callback supplies contextual parameters before its
    // return is known. Coercing branches to that incomplete object used to
    // reject valid callbacks or carry a missing return into checked IR.
    const partialCallable = requirePartialCallableResultInference(expected?.kind ?? '', expected?.ret !== undefined);
    let resultType = partialCallable ? undefined : expected;
    const pending = [];
    for (const kase of node.cases) {
      const inner = new Map(env);
      const pattern = this.pattern(kase.pattern, type, inner, path, kase.span ?? node.span);
      if (variable && pattern.k === 'ctor' && pattern.binds.every(Boolean) && !pattern.binds.includes(variable)) {
        inner.set(KNOWN, new Map([...(inner.get(KNOWN) ?? []), [variable, { ctor: pattern.ctor, binds: pattern.binds }]]));
      }
      pending.push({ pattern, body: kase.body, env: inner, span: kase.span });
    }
    for (const kase of pending) {
      const body = this.expr(kase.body, kase.env, path, partialCallable ? expected : resultType, true);
      if (body.type.kind !== 'literal' && !resultType) resultType = body.type;
      cases.push({ pattern: kase.pattern, body, surface: kase.body, env: kase.env });
    }
    if (!resultType) resultType = this.defaultNumber();
    const finalCases = cases.map((kase) => {
      let body = kase.body.type.kind === 'literal' ? this.expr(kase.surface, kase.env, path, resultType) : kase.body;
      body = coerce(body, resultType, this.language, kase.surface.span ?? node.span);
      return { pattern: kase.pattern, body };
    });
    checkExhaustive(type, finalCases.map((kase) => kase.pattern), this.items, node.span);
    return { k: 'match', scrutinee, cases: finalCases, type: resultType };
  }

  /**
   * Inside a case that matched `variable` against a constructor, a nested
   * match on the same variable can only take that constructor's case: it is
   * the case's body with the fields bound to the outer case's locals. The
   * value is unchanged, and structural recursion stays visible to Lean and
   * Rocq, which do not relate the inner match to the outer one.
   */
  knownMatch(node, fact, variable, env, path, expected, allowLiteral) {
    const kase = node.cases.find((candidate) => (candidate.pattern.k === 'ctor' && candidate.pattern.path.at(-1) === fact.ctor)
      || candidate.pattern.k === 'wild' || candidate.pattern.k === 'bind');
    if (!kase) throw typeError(`non-exhaustive match: no case for ${fact.ctor}`, node.span);
    const alias = (name, value, body) => (name === null || name === '_' || name === value
      ? body
      : { k: 'let', name, value: { k: 'name', path: [value] }, body, span: kase.span ?? node.span });
    let body = kase.body;
    if (kase.pattern.k === 'bind') body = alias(kase.pattern.name, variable, body);
    if (kase.pattern.k === 'ctor') {
      if (kase.pattern.binds.length !== fact.binds.length) {
        throw typeError(`${fact.ctor} pattern binds ${kase.pattern.binds.length} of ${fact.binds.length} fields`, kase.span ?? node.span);
      }
      // Simultaneous binding: every alias reads an outer local before any is shadowed.
      const inner = kase.pattern.binds.map((name, index) => [name, fact.binds[index]]).filter(([name, value]) => name && name !== '_' && name !== value);
      if (inner.some(([, value]) => inner.some(([name]) => name === value))) {
        const temporaries = inner.map(([, value]) => [this.freshName('ml_k'), value]);
        body = inner.reduceRight((rest, [name], index) => alias(name, temporaries[index][0], rest), body);
        body = temporaries.reduceRight((rest, [name, value]) => alias(name, value, rest), body);
      } else {
        body = inner.reduceRight((rest, [name, value]) => alias(name, value, rest), body);
      }
    }
    return this.expr(body, env, path, expected, allowLiteral);
  }

  pattern(pattern, type, env, path, span) {
    if (pattern.k === 'wild') return { k: 'wild' };
    if (pattern.k === 'bind') {
      bindLocal(env, pattern.name, type);
      return { k: 'bind', name: pattern.name };
    }
    if (pattern.k === 'natZero' || pattern.k === 'natSucc') {
      if (!isNatural(type) || type.kind === 'fixed') throw typeError(`natural-number pattern on ${typeKey(type)}`, span);
      if (pattern.k === 'natSucc') {
        bindLocal(env, pattern.name, type);
        return { k: 'natSucc', name: pattern.name };
      }
      return { k: 'natZero' };
    }
    if (pattern.k === 'ctor') {
      if (type.kind !== 'data') throw typeError(`constructor pattern on ${typeKey(type)}`, span);
      const dataEntry = this.items.get(type.name);
      const ctorName = pattern.path.at(-1);
      const ctor = dataEntry.ctors.find((candidate) => candidate.name === ctorName);
      if (!ctor) throw typeError(`${type.name} has no constructor ${ctorName}`, span);
      if (pattern.binds.length !== ctor.fields.length) {
        throw typeError(`${ctorName} pattern binds ${pattern.binds.length} of ${ctor.fields.length} fields`, span);
      }
      const binds = pattern.binds.map((bind, index) => {
        if (bind === '_' || bind === null) return null;
        bindLocal(env, bind, ctor.fields[index].type);
        return bind;
      });
      return { k: 'ctor', data: type.name, ctor: ctorName, binds };
    }
    throw unsupported('pattern', `${pattern.k} patterns are not in the portable core`, span);
  }
}

import {createStringCharacterTemplateExpression} from './string-character-template.js';
import {createStringRelativeCharacterTemplateExpression} from './string-relative-character-template.js';

/** Arrays have a text, but not one the translation reproduces yet. */
function arrayText(span) {
  return unsupported('text of an array', 'console.log lays an array out with util.inspect and String joins its elements with commas, which the translation does not reproduce yet; print the elements one by one', span);
}

function literal(type, value) {
  return { k: 'lit', type, value };
}

/** Integers and Numbers: the types with an order and arithmetic. */
function isOrdered(type) {
  return isNumeric(type) || isFloat(type);
}

function primitiveKey(type) {
  return acceptOptionalPrimitiveKind(type.kind);
}

function flattenOption(value) {
  if (!requireOptionalFlattening(value.type.kind, value.type.element?.kind ?? '')) return value;
  const inner = value.type.element;
  const type = { kind: 'option', element: inner.element, absence: mergeOptionalAbsence(inner.absence, value.type.absence) };
  return flattenOption({ k: 'flattenOption', value, type });
}

/** The canonical text of a negated Number: `-x`, and NaN for NaN. */
function negateNumber(text) {
  if (text === 'NaN') return text;
  return text.startsWith('-') ? text.slice(1) : `-${text}`;
}

/**
 * Inside `match v with | succ k => …`, the value `k + 1` is `v` itself.
 * Rewriting it to `v` keeps recursion such as `fib (n + 1) + fib n`
 * structural for Lean and Rocq without changing any value.
 */
function reuseSuccessors(expr, predecessors) {
  if (!expr || typeof expr !== 'object') return expr;
  switch (expr.k) {
    case 'binary': {
      const { left, right } = expr;
      if (expr.op === 'add' && expr.type.kind === 'nat' && left.k === 'var' && predecessors.has(left.name)
        && right.k === 'lit' && right.value === '1') {
        return { k: 'var', name: predecessors.get(left.name), type: expr.type, span: expr.span };
      }
      return { ...expr, left: reuseSuccessors(left, predecessors), right: reuseSuccessors(right, predecessors) };
    }
    case 'let': {
      const inner = without(predecessors, [expr.name]);
      return { ...expr, value: reuseSuccessors(expr.value, predecessors), body: reuseSuccessors(expr.body, inner) };
    }
    case 'match':
      return {
        ...expr,
        scrutinee: reuseSuccessors(expr.scrutinee, predecessors),
        cases: expr.cases.map((kase) => {
          const bound = kase.pattern.k === 'natSucc' ? [kase.pattern.name] : (kase.pattern.binds ?? []).filter(Boolean);
          const inner = without(predecessors, bound);
          if (kase.pattern.k === 'natSucc' && expr.scrutinee.k === 'var' && !bound.includes(expr.scrutinee.name)) {
            inner.set(kase.pattern.name, expr.scrutinee.name);
          }
          return { ...kase, body: reuseSuccessors(kase.body, inner) };
        }),
      };
    default: {
      const copy = { ...expr };
      for (const [key, value] of Object.entries(expr)) {
        if (key === 'type' || key === 'domain' || key === 'from' || key === 'to') continue;
        if (Array.isArray(value)) copy[key] = value.map((item) => reuseSuccessors(item, predecessors));
        else if (value && typeof value === 'object' && value.k) copy[key] = reuseSuccessors(value, predecessors);
      }
      return copy;
    }
  }
}

function without(map, names) {
  const copy = new Map(map);
  for (const [key, value] of map) if (names.includes(key) || names.includes(value)) copy.delete(key);
  return copy;
}

function arithmeticSemantics(language, op, type, node) {
  // IEEE-754 binary64 arithmetic rounds to nearest, even on a tie; `%` is the
  // exact truncated remainder (C fmod); nothing aborts.
  if (isFloat(type)) return { semantics: 'ieee' };
  const division = op === 'div' || op === 'rem';
  if (type.kind === 'fixed') {
    // Rust integer arithmetic with overflow checks: `/` and `%` truncate,
    // `div_euclid` and `rem_euclid` round Euclidean; all panic on zero.
    return division ? { semantics: 'checked', rounding: node.rounding ?? 'trunc', byZero: 'abort' } : { semantics: 'checked' };
  }
  if (op === 'sub') return { semantics: type.kind === 'nat' ? 'truncated' : 'exact' };
  if (!division) return { semantics: 'exact' };
  if (language === 'JavaScript') return { semantics: 'exact', rounding: 'trunc', byZero: 'abort' };
  // Lean and Rocq division is total: x / 0 = 0 and x % 0 = x.
  if (type.kind === 'nat') return { semantics: 'exact', rounding: 'trunc', byZero: 'total' };
  if (language === 'Lean') return { semantics: 'exact', rounding: node.rounding ?? 'euclid', byZero: 'total' };
  return { semantics: 'exact', rounding: node.rounding ?? 'floor', byZero: 'total' };
}

function unify(left, right, language, span) {
  if (sameType(left.type, right.type)) return [left, right];
  if (isFloat(left.type) || isFloat(right.type)) {
    if (language === 'JavaScript' && (isNumeric(left.type) || isNumeric(right.type))) {
      throw typeError('mixing BigInt and Number: JavaScript throws "TypeError: Cannot mix BigInt and other types, use explicit conversions"', span);
    }
    throw typeError(`operand types differ: ${typeKey(left.type)} and ${typeKey(right.type)}`, span);
  }
  if (language === 'JavaScript') {
    // Guarded parameters are naturals, but every BigInt operation is an integer operation.
    return [coerce(left, INT, language, span), coerce(right, INT, language, span)];
  }
  throw typeError(`operand types differ: ${typeKey(left.type)} and ${typeKey(right.type)}`, span);
}

function unifyBranches(then, otherwise, expected, language, span) {
  if (sameType(then.type, otherwise.type)) return [then, otherwise];
  if (then.k === 'abort') return [{ ...then, type: otherwise.type }, otherwise];
  if (otherwise.k === 'abort') return [then, { ...otherwise, type: then.type }];
  if (expected) return [coerce(then, expected, language, span), coerce(otherwise, expected, language, span)];
  return unify(then, otherwise, language, span);
}

/** Inserts the source language's implicit conversions, which only JavaScript has. */
export function coerce(value, type, language, span, flavor = undefined) {
  if (value.type.kind === 'literal') throw typeError('untyped literal', span);
  if (sameType(value.type, type)) return value;
  if (value.k === 'abort') return { ...value, type };
  const optional = readOptionalCoercionOperation(language, type.kind, value.type.kind, value.k,
    type.kind === 'option' && acceptOptionalAbsence(type.absence, value.absence),
    type.kind === 'option' && acceptOptionalAbsence(type.absence, 'undefined'),
    type.kind === 'option' && sameType(value.type, type.element));
  if (optional === 'absence-error') throw typeError(`expected ${type.absence} absence but found ${value.absence}`, span);
  if (optional === 'none') return { ...value, type };
  if (optional === 'unit' || optional === 'discard-unit') {
    const absent = { k: 'none', absence: 'undefined', type };
    return optional === 'unit' ? absent : { k: 'let', name: 'ml_discarded_unit', value, body: absent, type };
  }
  if (optional === 'some') return { k: 'some', value, type };
  if (language === 'JavaScript' && value.type.kind === 'nat' && type.kind === 'int') {
    return castTo(value, INT, 'exact', span);
  }
  if (language === 'JavaScript' && value.type.kind === 'int' && type.kind === 'nat') {
    return castTo(value, NAT, flavor ?? 'checked', span);
  }
  throw typeError(`expected ${typeKey(type)} but found ${typeKey(value.type)}`, span);
}

function castTo(arg, to, flavor, span, language = '') {
  if (sameType(arg.type, to)) return arg;
  const from = arg.type;
  if (from.kind === 'nat' && to.kind === 'int') return { k: 'cast', arg, from, to, flavor: 'exact', type: to };
  if (from.kind === 'int' && to.kind === 'nat') {
    if (flavor !== 'checked' && flavor !== 'clamp') throw typeError('int to nat conversion needs checked or clamp semantics', span);
    return { k: 'cast', arg, from, to, flavor, type: to };
  }
  if (from.kind === 'fixed' && to.kind === 'fixed') {
    const source = fixedBounds(from);
    const target = fixedBounds(to);
    const selected = readFixedIntegerCastFlavor(language, source.min >= target.min && source.max <= target.max, flavor ?? '');
    if (selected) return { k: 'cast', arg, from, to, flavor: selected, type: to };
    throw unsupported('narrowing integer cast', `${typeKey(from)} as ${typeKey(to)} wraps in Rust and has no portable encoding yet`, span);
  }
  throw typeError(`cannot convert ${typeKey(from)} to ${typeKey(to)}`, span);
}

function checkExhaustive(type, patterns, items, span) {
  if (patterns.some((pattern) => pattern.k === 'wild' || pattern.k === 'bind')) return;
  if (type.kind === 'data') {
    const ctors = items.get(type.name).ctors.map((ctor) => ctor.name);
    const covered = new Set(patterns.map((pattern) => pattern.ctor));
    const missing = ctors.filter((ctor) => !covered.has(ctor));
    if (missing.length) throw typeError(`match on ${type.name} misses ${missing.join(', ')}`, span);
    return;
  }
  if (isNatural(type)) {
    const kinds = new Set(patterns.map((pattern) => pattern.k));
    if (kinds.has('natZero') && kinds.has('natSucc')) return;
  }
  throw typeError(`non-exhaustive match on ${typeKey(type)}`, span);
}

/**
 * Marks each function recursive or not and, for recursive functions, finds a
 * structurally decreasing parameter. Without one, Lean writes a partial def and
 * Rocq a Definition over `ml_fix`; the other targets accept general recursion.
 */
function analyseRecursion(program) {
  for (const entry of program.declarations.values()) {
    if (entry.k !== 'fn') continue;
    const calls = [];
    collectCalls(entry.body, calls);
    const selfCalls = calls.filter((call) => call.fn === entry.fullName);
    entry.recursive = selfCalls.length > 0;
    entry.decreasing = entry.recursive ? structuralParameter(entry) : null;
    if (entry.recursive && entry.decreasing === null) exposeNaturalCases(entry);
  }
  const graph = new Map();
  for (const entry of program.declarations.values()) {
    if (entry.k !== 'fn') continue;
    const calls = [];
    collectCalls(entry.body, calls);
    graph.set(entry.fullName, new Set(calls.map((call) => call.fn).filter((name) => name !== entry.fullName)));
  }
  for (const [name, targets] of graph) {
    for (const target of targets) {
      if (reaches(graph, target, name, new Set())) {
        const entry = program.declarations.get(name);
        entry.mutual = true;
      }
    }
  }
}

/**
 * `if n == 0 then a else b` on a natural or unsigned machine-integer
 * parameter is the case split `match n with 0 => a | succ p => b'`, where
 * `b'` reads `n - k` as `p - (k - 1)` and `n == k` as `p == k - 1`: in `b`,
 * `n` is at least 1, so both agree with truncated subtraction, with checked
 * machine subtraction, and with a checked conversion of `n - k` back to a
 * natural. A `let m := n` in `b` is an alias read the same way. Writing the
 * split explicitly makes recursion such as `n * fact (n - 1)` structural,
 * which Lean and Rocq require.
 */
function exposeNaturalCases(entry) {
  for (let index = 0; index < entry.params.length; index += 1) {
    const param = entry.params[index];
    if (!isNatural(param.type)) continue;
    const prefix = readSourceIdentifierSupply(collectSourceIdentifierTexts(entry));
    let fresh = 0;
    const freshName = () => {
      fresh += 1;
      return renderSourceGeneratedIdentifier(prefix, 'p', fresh);
    };
    const body = splitNatural(entry.body, param.name, freshName);
    if (body !== entry.body && decreasesOn(body, entry.fullName, index, param.name, new Set())) {
      entry.body = body;
      entry.decreasing = index;
      return;
    }
  }
}

function isVar(node, name) {
  return node?.k === 'var' && node.name === name;
}

function natLiteral(node) {
  if (node?.k !== 'lit' || !/^\d+$/u.test(String(node.value))) return null;
  return BigInt(node.value);
}

/** `n == 0`, `0 == n`: the zero test that selects the `then` branch. */
function zeroTest(cond, name) {
  if (cond.k !== 'binary' || (cond.op !== 'eq' && cond.op !== 'ne') || !cond.domain || !isNatural(cond.domain)) return null;
  const zeroOn = (a, b) => isVar(a, name) && natLiteral(b) === 0n;
  if (zeroOn(cond.left, cond.right) || zeroOn(cond.right, cond.left)) return cond.op === 'eq' ? 'then' : 'else';
  return null;
}

function splitNatural(expr, name, freshName) {
  if (!expr || typeof expr !== 'object') return expr;
  if (expr.k === 'if') {
    const zeroBranch = zeroTest(expr.cond, name);
    if (zeroBranch) {
      const zero = zeroBranch === 'then' ? expr.then : expr.else;
      const positive = zeroBranch === 'then' ? expr.else : expr.then;
      const pred = freshName();
      const type = expr.cond.domain;
      let succBody = predecessorOf(positive, name, pred, type);
      succBody = splitNatural(splitNatural(succBody, pred, freshName), name, freshName);
      return {
        k: 'match',
        scrutinee: { k: 'var', name, type, span: expr.cond.span },
        cases: [
          { pattern: { k: 'natZero' }, body: splitNatural(zero, name, freshName) },
          { pattern: { k: 'natSucc', name: pred }, body: succBody },
        ],
        type: expr.type,
        span: expr.span,
      };
    }
  }
  if ((expr.k === 'let' && expr.name === name) || (expr.k === 'match' && binds(expr, name))) return expr;
  let changed = false;
  const copy = { ...expr };
  for (const [key, value] of Object.entries(expr)) {
    if (key === 'type' || key === 'domain' || key === 'from' || key === 'to' || key === 'span') continue;
    if (Array.isArray(value)) {
      const next = value.map((item) => (item && typeof item === 'object' && !item.k && item.body
        ? { ...item, body: splitNatural(item.body, name, freshName) }
        : splitNatural(item, name, freshName)));
      if (next.some((item, position) => item !== value[position] && (item.body ?? item) !== (value[position].body ?? value[position]))) {
        copy[key] = next;
        changed = true;
      }
    } else if (value && typeof value === 'object' && value.k) {
      const next = splitNatural(value, name, freshName);
      if (next !== value) {
        copy[key] = next;
        changed = true;
      }
    }
  }
  return changed ? copy : expr;
}

function binds(match, name) {
  return match.cases.some((kase) => kase.pattern.name === name || (kase.pattern.binds ?? []).includes(name));
}

/**
 * Rewrites uses of `name - k` and `name == k` (k ≥ 1) in terms of
 * `pred = name - 1`, including through `let` aliases of `name`.
 */
function predecessorOf(expr, name, pred, type) {
  const predVar = () => ({ k: 'var', name: pred, type });
  const minus = (k, like) => (k === 1n
    ? predVar()
    : { ...like, left: predVar(), right: { k: 'lit', type, value: String(k - 1n) }, type, domain: type });
  const visit = (node, names) => {
    if (!node || typeof node !== 'object') return node;
    if (Array.isArray(node)) return node.map((item) => visit(item, names));
    const named = (candidate) => candidate?.k === 'var' && names.has(candidate.name);
    if (node.k === 'binary' && node.domain && isNatural(node.domain) && named(node.left)) {
      const k = natLiteral(node.right);
      if (k !== null && k >= 1n && node.op === 'sub') return minus(k, node);
      if (k !== null && k >= 1n && (node.op === 'eq' || node.op === 'ne')) {
        return { ...node, left: predVar(), right: { k: 'lit', type, value: String(k - 1n) } };
      }
    }
    // JavaScript's `n - 1n` on a natural `n` is the checked conversion of
    // an integer difference; with n ≥ 1 it is the natural predecessor.
    if (node.k === 'cast' && node.to.kind === 'nat' && node.arg.k === 'binary' && node.arg.op === 'sub'
      && node.arg.left.k === 'cast' && node.arg.left.from?.kind === 'nat' && named(node.arg.left.arg)) {
      const k = natLiteral(node.arg.right);
      if (k !== null && k >= 1n) {
        return minus(k, { k: 'binary', op: 'sub', semantics: 'truncated', span: node.span });
      }
    }
    if (node.k === 'let') {
      const inner = new Set(names);
      inner.delete(node.name);
      if (node.name === pred) return { ...node, value: visit(node.value, names) };
      if (named(node.value)) inner.add(node.name);
      return { ...node, value: visit(node.value, names), body: visit(node.body, inner) };
    }
    if (node.k === 'match') {
      const scrutinee = visit(node.scrutinee, names);
      if (binds(node, pred)) return { ...node, scrutinee };
      const cases = node.cases.map((kase) => {
        const inner = new Set([...names].filter((candidate) => !binds({ cases: [kase] }, candidate)));
        return { ...kase, body: visit(kase.body, inner) };
      });
      return { ...node, scrutinee, cases };
    }
    const copy = { ...node };
    for (const [key, value] of Object.entries(node)) {
      if (key === 'type' || key === 'domain' || key === 'from' || key === 'to' || key === 'span' || key === 'pattern') continue;
      if (value && typeof value === 'object') copy[key] = visit(value, names);
    }
    return copy;
  };
  return visit(expr, new Set([name]));
}

function reaches(graph, from, to, seen) {
  if (from === to) return true;
  if (seen.has(from)) return false;
  seen.add(from);
  for (const next of graph.get(from) ?? []) {
    if (reaches(graph, next, to, seen)) return true;
  }
  return false;
}

export function collectCalls(expr, calls) {
  if (!expr || typeof expr !== 'object') return;
  if (expr.k === 'call') calls.push(expr);
  for (const value of Object.values(expr)) {
    if (Array.isArray(value)) value.forEach((item) => collectCalls(item, calls));
    else if (value && typeof value === 'object' && value !== expr.type) collectCalls(value, calls);
  }
}

function structuralParameter(entry) {
  for (let index = 0; index < entry.params.length; index += 1) {
    const param = entry.params[index];
    if (decreasesOn(entry.body, entry.fullName, index, param.name, new Set())) return index;
  }
  return null;
}

/** Every recursive call's argument at `index` must be a strict subterm of `name`. */
function decreasesOn(expr, fn, index, name, subterms) {
  if (!expr || typeof expr !== 'object') return true;
  switch (expr.k) {
    case 'call':
      if (expr.fn === fn) {
        const arg = expr.args[index];
        if (!(arg.k === 'var' && subterms.has(arg.name))) return false;
      }
      return expr.args.every((arg) => decreasesOn(arg, fn, index, name, subterms));
    case 'match': {
      if (!decreasesOn(expr.scrutinee, fn, index, name, subterms)) return false;
      const isParam = expr.scrutinee.k === 'var' && (expr.scrutinee.name === name || subterms.has(expr.scrutinee.name));
      return expr.cases.every((kase) => {
        const inner = new Set(subterms);
        if (isParam) {
          if (kase.pattern.k === 'natSucc') inner.add(kase.pattern.name);
          if (kase.pattern.k === 'ctor') {
            for (const bind of kase.pattern.binds) if (bind) inner.add(bind);
          }
        }
        return decreasesOn(kase.body, fn, index, name, inner);
      });
    }
    case 'let': {
      if (!decreasesOn(expr.value, fn, index, name, subterms)) return false;
      const inner = new Set(subterms);
      inner.delete(expr.name);
      // `let k := subterm` (introduced by pattern compilation) is itself a subterm.
      if (expr.value.k === 'var' && subterms.has(expr.value.name)) inner.add(expr.name);
      return decreasesOn(expr.body, fn, index, name, inner);
    }
    default:
      for (const [key, value] of Object.entries(expr)) {
        if (key === 'type' || key === 'domain' || key === 'from' || key === 'to') continue;
        if (Array.isArray(value)) {
          if (!value.every((item) => decreasesOn(item, fn, index, name, subterms))) return false;
        } else if (value && typeof value === 'object' && !decreasesOn(value, fn, index, name, subterms)) {
          return false;
        }
      }
      return true;
  }
}

import {createArrayRelativeElementTemplateExpression} from './array-relative-element-template.js';
import {checkJavaScriptIterableValue,checkJavaScriptArrayFormalIterable} from './array-formal-iterables.js';
import {readJavaScriptArrayFormalIterableOperation} from './array-formal-parameter-rules.js';

import {readJavaScriptRecordFormalDefaultOperation} from './javascript-record-parameter-rules.js';
import {readJavaScriptRecordFormalDefaultOperands} from './record-formal-defaults.js';
