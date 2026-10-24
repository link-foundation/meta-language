// Host JSON boundary adapter. Projection instructions and admission decisions
// are shared data; the existing checker still owns types and semantics.
import { TYPED_PROGRAM_LINKS_SCHEMA } from './typed-program-links.js';
import {acceptLinkedNumberBitwiseOperation} from './translation/number-bitwise-rules.js';
import {readRustSharedArrayHandleComparisonProjection} from './translation/rust-shared-array-handle-rules.js';
import { acceptProgramExpressionField, acceptProgramExpressionOperation, readProgramExpressionProjection } from './typed-program-link-rules.js';
import { checkRustScalarLoopProof } from './translation/rust-scalar-loops.js';
import { readRecordProjection, readRecordProjectionError } from './translation/record-value-rules.js';
import { readLinkedProgramNumberOperand } from './typed-program-number-projection.js';
import { readSurfaceReferenceProjectionForm, readSurfaceReferencePayloadOperand } from './translation/surface-reference-analysis-rules.js';
import { acceptLinkedReferenceMethodOperation } from './translation/reference-method-rules.js';
import { acceptLinkedRecordFieldWrite } from './translation/record-write-rules.js';
import { acceptLinkedBigIntegerBitwiseOperation } from './translation/big-integer-bitwise-rules.js';
import {acceptLinkedJavaScriptNumberConversion} from './translation/string-number-rules.js';
import {acceptLinkedJavaScriptSourceException} from './translation/source-exception-rules.js';
import {acceptLinkedJavaScriptBigIntegerResult} from './translation/big-integer-conversion-rules.js';
import {acceptLinkedJavaScriptStringUnitConstruction} from './translation/javascript-string-unit-rules.js';
import {acceptLinkedJavaScriptStringConstruction} from './translation/string-construction-rules.js';
import { acceptLinkedRustPrimitiveBitwiseOperation } from './translation/rust-primitive-bitwise-rules.js';
import { acceptLinkedArrayStateMethod } from './translation/array-state-method-rules.js';
import { acceptLinkedRustCheckedShift } from './translation/rust-shift-rules.js';

/** Restore identities from actual declarations after independent namespace validation. */
export function projectLinkedProgramDeclarationPath(name, declarations, rooted = false) {
  if (!rooted) return name.split('.');
  const declaration = declarations.get(name);
  if (!declaration) throw new TypeError('an owned namespace projection has no actual declaration target');
  const actual = [...declaration.modulePath, declaration.name];
  if (actual.join('.') !== name) throw new TypeError('an owned namespace projection disagrees with its actual declaration binding');
  return ['crate', ...actual];
}

export function projectLinkedProgramExpression(value, declarations = new Map(), {rootedNamespace = false, referenceSemantics = false, sourceLanguage = ''} = {}) {
  const plans = TYPED_PROGRAM_LINKS_SCHEMA.expressionProjections;
  if (['tuple', 'tupleIndex'].includes(value?.k)
    && Object.keys(value).some(field => !acceptProgramExpressionField(value.k, field))) throw new TypeError('a linked tuple expression has an unrecognized projection field');
  if (value?.k === 'record'
    && Object.keys(value).some(field => !acceptProgramExpressionField(value.k, field))) throw new TypeError('a linked record expression has an unrecognized projection field');
  if (Object.hasOwn(TYPED_PROGRAM_LINKS_SCHEMA.expressionOperations, value?.k)
    && !acceptProgramExpressionOperation(value.k, value.op) && !acceptLinkedNumberBitwiseOperation(value.k,value.op)
    && !acceptLinkedReferenceMethodOperation(value.k, value.op, value.object?.type?.kind ?? '', value.object?.type?.reference === true)
    && !acceptLinkedRecordFieldWrite(value.k, value.op, value.object?.type?.kind ?? '', value.object?.type?.reference === true)
    && !acceptLinkedBigIntegerBitwiseOperation(value.k, value.op)
    && !acceptLinkedJavaScriptNumberConversion(value.k,value.op)
    && !acceptLinkedJavaScriptSourceException(value.k,value.op)
    && !acceptLinkedJavaScriptBigIntegerResult(value.k,value.op)
    && !acceptLinkedJavaScriptStringUnitConstruction(value.k,value.op)
    && !acceptLinkedJavaScriptStringConstruction(value.k,value.op)
    && !acceptLinkedRustPrimitiveBitwiseOperation(value.k, value.op)
    && !acceptLinkedArrayStateMethod(value.k, value.op, value.object?.type?.kind ?? '', value.object?.type?.reference === true)
    && !acceptLinkedRustCheckedShift(value.k,value.op)) throw new TypeError('the linked program contains an unrecognized expression operation');
  let key = value?.k;
  const alternatives = TYPED_PROGRAM_LINKS_SCHEMA.expressionProjectionAlternatives;
  for (const alternative of alternatives && Object.hasOwn(alternatives, key) ? alternatives[key] : []) {
    if (alternative.path.reduce((current, field) => current?.[field], value) === alternative.value) {
      key = alternative.projection;
      break;
    }
  }
  const base = plans[key];
  if (base?.variant) {
    const discriminator = base.variant.reduce((current, field) => current?.[field], value);
    const variant = `${key}:${discriminator}`;
    if (readProgramExpressionProjection(variant)) key = variant;
  }
  if (!readProgramExpressionProjection(key ?? '') || !plans[key].construct) throw new TypeError(`the linked expression ${value?.k} has no surface projection`);
  const plan = plans[key];
  const project = child => projectLinkedProgramExpression(child, declarations, {rootedNamespace, referenceSemantics, sourceLanguage});
  // This domain already states identity semantics in checked IR. Reconstruct
  // its explicit operation, never ordinary Rust value-equality syntax.
  const identity = value.k === 'binary' ? readRustSharedArrayHandleComparisonProjection(sourceLanguage,
    value.op, value.domain?.kind ?? '', value.domain?.reference === true) : '';
  if (identity) return { k: identity, op: value.op, left: project(value.left), right: project(value.right),
    ...(value.span !== undefined ? { span: value.span } : {}) };
  const referenceForm = readSurfaceReferenceProjectionForm(value.k, value.type?.kind ?? '', value.type?.reference === true, referenceSemantics);
  if (plan.construct === 'operand') {
    let projected = project(value[plan.operand]);
    const payload = readSurfaceReferencePayloadOperand(projected.k);
    if (referenceForm === 'allocation' && payload) projected = projected[payload];
    return value.span !== undefined && projected.span === undefined ? { ...projected, span: value.span } : projected;
  }
  const result = { k: plan.construct, ...(value.span !== undefined ? { span: value.span } : {}) };
  const operation = plan.renames?.[value.op] ?? value.op;
  for (const [target, mode, source, right] of plan.fields) {
    const child = value[source];
    if (mode === 'copy') { if (child !== undefined) result[target] = child; }
    else if (mode === 'expression') result[target] = project(readLinkedProgramNumberOperand(value, source, child));
    else if (mode === 'expressions') result[target] = child.map(project);
    else if (mode === 'nullable-expressions') result[target] = child.map(value => value === null ? null : project(value));
    else if (mode === 'name-path') result[target] = [child];
    else if (mode === 'declaration-path') result[target] = projectLinkedProgramDeclarationPath(child, declarations, rootedNamespace);
    else if (mode === 'function-path') {
      const declaration=declarations.get(child);
      let path=[child];
      if(declaration?.sourceLoop?.continuation!==undefined){
        if(declaration.k!=='fn'||declaration.fullName!==child)throw new TypeError('a projected continuation must target its actual declaration identity');
        const proof=checkRustScalarLoopProof(declaration,declaration.sourceLoop.language,child);
        path=[proof.continuation.root,...proof.continuation.path];
      }else if(declaration?.k==='fn'){
        if(declaration.fullName!==child)throw new TypeError('a projected function must target its actual declaration identity');
        path=projectLinkedProgramDeclarationPath(child,declarations,true);
      }else if(rootedNamespace){
        path=projectLinkedProgramDeclarationPath(child,declarations,true);
      }
      result[target]={k:'name',path};
    }
    else if (mode === 'array-items') result[target] = child.map((item) => ({ value: project(item), spread: plan.spreadOperations?.includes(value.op) ?? false }));
    else if (mode === 'record-entries') {
      if (!Array.isArray(child)) throw new TypeError('checked record projection needs an ordered field array');
      const names = child.map(entry => typeof entry?.name === 'string'
        ? Array.from({ length: entry.name.length }, (_, index) => entry.name.charCodeAt(index)) : []);
      const entries = child.map(entry => entry !== null && typeof entry === 'object' && !Array.isArray(entry)
        && Object.hasOwn(entry, 'name') && typeof entry.name === 'string' && Object.hasOwn(entry, 'value')
        && entry.value !== null && typeof entry.value === 'object' && !Array.isArray(entry.value)
        && Object.keys(entry).every(field => field === 'name' || field === 'value'));
      const error = readRecordProjectionError(readRecordProjection(), names, entries);
      if (error) throw new TypeError(error);
      result.recordProjection = readRecordProjection();
      result[target] = child.map(entry => ({ name: entry.name, spread: false, value: project(entry.value) }));
    }
    else if (mode === 'spread-pair') result[target] = [child, value[right]].map((item) => ({ value: project(item), spread: true }));
    else if (mode === 'constructor-path') result[target] = { k: 'name', path: [...projectLinkedProgramDeclarationPath(value.data, declarations, rootedNamespace), child] };
    else if (mode === 'constructor-fields') {
      const declaration = declarations.get(value.data);
      const constructor = declaration?.k === 'data' && declaration.ctors.find(constructor => constructor.name === value.ctor);
      if (!constructor || child.length !== constructor.fields.length) throw new TypeError('the linked constructor projection differs from its declaration');
      result[target] = Object.fromEntries(constructor.fields.map((field, index) => [field.name, project(child[index])]));
    } else if (mode === 'match-cases') result[target] = child.map(kase => ({
      pattern: kase.pattern.k === 'ctor'
        ? { k: 'ctor', path: [...projectLinkedProgramDeclarationPath(kase.pattern.data, declarations, rootedNamespace), kase.pattern.ctor], binds: kase.pattern.binds }
        : { ...kase.pattern },
      body: project(kase.body), ...(kase.span !== undefined ? { span: kase.span } : {}),
    }));
    else if (mode === 'element-type') result[target] = child.element;
    else if (mode === 'return-type') result[target] = child.ret;
    else if (mode === 'binding-type') result[target] = child.type;
    else if (mode === 'integer-type') result[target] = child.kind !== 'float';
    else if (mode === 'operation') result[target] = operation;
    else if (mode === 'operation-name') result[target] = `Math.${operation}`;
    else if (mode === 'constant') result[target] = source;
    else throw new TypeError(`unknown shared projection instruction ${mode}`);
  }
  return readSurfaceReferencePayloadOperand(referenceForm)
    ? { k: referenceForm, value: result, ...(value.span !== undefined ? { span: value.span } : {}) }
    : result;
}
