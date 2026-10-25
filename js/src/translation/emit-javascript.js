import {RECORD_PROPERTY_ORDER_JAVASCRIPT_PRELUDE} from './record-property-order-javascript-runtime.js';
import {prepareOrderedRecordValues} from './ordered-record-preparation.js';
import {emitOrderedRecordValue,emitOrderedRecordOrderRead} from './ordered-record-emission.js';
import {acceptUtf16TextMethod,renderUtf16TextMethod} from './utf16-method-emission-rules.js';
import {UNICODE_CASE_JAVASCRIPT_PRELUDE} from './unicode-case-javascript-runtime.js';
import {acceptUnicodeCaseMappingOperation,renderUnicodeCaseStringMap,readUnicodeCaseEncodingStatement} from './unicode-case-emission-rules.js';
import {acceptLinkedJavaScriptStringConstruction,renderJavaScriptStringConstruction} from './string-construction-rules.js';
import {UTF16_TEXT_JAVASCRIPT_PRELUDE} from './utf16-text-javascript-runtime.js';
import {checkRustRuntimeShell,recordRustRuntimeShellAssumptions,refuseRustRuntimeShellComposition} from './rust-runtime-shell.js';
import { renderWrappingFixedIntegerCast, readFixedIntegerCastStatement } from './fixed-integer-cast-rules.js';
import { renderCollectionSizeExpression, renderCollectionReadExpression } from './collection-access-rules.js';
import { acceptLinkedRecordFieldWrite, readRecordFieldWriteOperator, readRecordFieldWriteName, renderRecordFieldWrite, readRecordFieldWriteStatement } from './record-write-rules.js';
import { readReferenceMethodResult, readReferenceMethodIdentity, readReferenceMethodStatement, renderReferenceMethodExpression } from './reference-method-rules.js';
import {readJavaScriptNumberBitwiseSymbol,readJavaScriptNumberBitwiseArity} from './number-bitwise-rules.js';
import { readBigIntegerBitwiseArity, renderBigIntegerBitwiseExpression } from './big-integer-bitwise-rules.js';
import {acceptLinkedJavaScriptNumberConversion,renderJavaScriptNumberScalarConversion} from './string-number-rules.js';
import {acceptLinkedJavaScriptSourceException,renderJavaScriptSourceExceptionExpression} from './source-exception-rules.js';
import {acceptLinkedJavaScriptBigIntegerResult,readJavaScriptBigIntegerResultOperation,renderJavaScriptBigIntegerResultExpression,renderJavaScriptBigIntegerDivision,renderJavaScriptBigIntegerPower,renderJavaScriptNumberPower,renderJavaScriptBigIntegerComparison} from './big-integer-conversion-rules.js';
import {BIG_INTEGER_CONVERSION_JAVASCRIPT_PRELUDE} from './big-integer-conversion-javascript-runtime.js';
import { readRustPrimitiveBitwiseArity } from './rust-primitive-bitwise-rules.js';
import { emitRustPrimitiveBitwise } from './rust-primitive-bitwise.js';
import { acceptLinkedRustCheckedShift } from './rust-shift-rules.js';
import { emitRustCheckedShift } from './rust-shifts.js';
import { acceptLinkedArrayStateMethod, renderArrayStateMethodExpression,
  readArrayStateMethodIdentity, readArrayStateMethodStatement } from './array-state-method-rules.js';
import { renderStringMapExpression, renderStringTestExpression } from './frontend-rules.js';
import {requireStringOrderUnits,renderStringOrderExpression} from './string-order-rules.js';
// JavaScript emitter. Every portable integer is a BigInt, so naturals and
// integers stay unbounded and machine integers are range-checked exactly as
// Rust checks them; a Number stays a Number. Data values are plain objects whose `$` property names
// the constructor. Modules become object literals referenced by qualified
// names. Theorems cannot be proved in JavaScript: each becomes an executable
// property, checked over a bounded domain by `--ml-check-theorems`, while the
// proof obligation stays discharged by the source language's kernel.

import { castMessage, overflowMessage, zeroDivisorMessage } from './aborts.js';
import { renderCollectionConstruction } from './collection-construction-rules.js';
import { renderSurfaceReferenceOperation } from './surface-reference-analysis-rules.js';
import { renderCollectionIteration, renderOptionalArrayRead, renderStringPointIteration } from './array-value-rules.js';
import { renderArrayFromMapping } from './array-from-rules.js';
import { renderPrimitiveMethodArgument, renderPrimitiveMethodExpression } from './primitive-method-rules.js';
import { renderRecordField, renderRecordValue } from './record-emission-rules.js';
import { readOptionalJavaScriptTextPrelude, renderOptionalAbsentValue, renderOptionalCoalescing, renderOptionalFlattening, renderOptionalJavaScriptOperation, renderOptionalPresentValue, renderOptionalPrimitiveTruth, renderOptionalProjection, renderOptionalStrictEquality, renderOptionalUnwrapping } from './optional-value-rules.js';
import { renderCallableApplication, renderCallableCellOperation, renderCallableFunctionReference, renderCallableLambda } from './callable-source-rules.js';
import { requireDataObjectFreeze } from './primitive-value-rules.js';
import { renderJavaScriptUnit, renderTupleProjection, renderTupleValue } from './product-emission-rules.js';
import { acceptProgramTupleProductModel } from '../typed-program-link-rules.js';
import { unsupported } from './diagnostics.js';
import { fixedBounds, typeKey } from './types.js';
import { renameFunction, renameMain, renameTheorem, tailLoop } from './ir.js';
import {readJavaScriptScopedBindingEmission} from './javascript-scoped-binding-emission.js';
import {reserveJavaScriptScopedBindingDeclarationNames} from './javascript-scoped-binding-emission-rules.js';
import {checkRustConstFunctionProof} from './rust-const-functions.js';
import {readRustDiagnosticAttributeNormalization} from './rust-diagnostic-attributes.js';
import {checkRustOwnedLocalStateProof,RUST_OWNED_LOCAL_STATE_POLICY} from './rust-owned-local-state.js';
import {validateRustTypeNamespace} from './rust-type-namespace.js';
import {renderRustOwnedLocalTextInput} from './rust-owned-local-state-rules.js';
import { EmitState, withParts } from './emit-common.js';
import {emitRustStringByteLength} from './rust-string-byte-lengths.js';
import { acceptJavaScriptExternalBinding, renderJavaScriptExternalCall } from './external-emission-rules.js';
import {readJavaScriptIdentifierModel} from './javascript-identifier-model.js';
import {acceptLinkedJavaScriptStringUnitConstruction,renderJavaScriptStringUnitConstruction} from './javascript-string-unit-rules.js';

const KEYWORDS = new Set([
  'ml_unicode_case', 'ml_utf16_text',
  'TypeError','SyntaxError','ReferenceError','URIError','EvalError',
  'break', 'case', 'catch', 'class', 'const', 'continue', 'debugger', 'default', 'delete', 'do', 'else', 'enum',
  'export', 'extends', 'false', 'finally', 'for', 'function', 'if', 'import', 'in', 'instanceof', 'new', 'null',
  'return', 'super', 'switch', 'this', 'throw', 'true', 'try', 'typeof', 'var', 'void', 'while', 'with', 'yield',
  'await', 'let', 'static', 'implements', 'interface', 'package', 'private', 'protected', 'public', 'arguments',
  'eval', 'undefined', 'NaN', 'Infinity', 'globalThis', 'console', 'process', 'BigInt', 'String', 'Object', 'Error', 'encodeURIComponent',
  'ml_record_property_order', 'Object', 'RangeError', 'Math', 'Number', 'Array', 'JSON', 'Symbol', 'main', '__proto__', 'constructor', 'prototype', 'of',
  'async', 'get', 'set',
]);

const HELPERS = {
  recordPropertyOrder:RECORD_PROPERTY_ORDER_JAVASCRIPT_PRELUDE,
  utf16Text:UTF16_TEXT_JAVASCRIPT_PRELUDE,
  unicodeCase:UNICODE_CASE_JAVASCRIPT_PRELUDE,
  bigIntegerConversion: BIG_INTEGER_CONVERSION_JAVASCRIPT_PRELUDE,
  ownedTextInput: renderRustOwnedLocalTextInput(),
  natSub: `function ml_natSub(a, b) {
  return a > b ? a - b : 0n;
}`,
  fixed: `function ml_fixed(value, min, max, message) {
  if (value < min || value > max) throw new RangeError(message);
  return value;
}`,
  divide: `// Integer division with the source's rounding; by zero it aborts with the message \`zero\`
// or, when that is null, is total (x / 0 = 0, x % 0 = x); a machine-integer quotient
// out of [min, max] aborts with \`overflow\`, the remainder too.
function ml_divide(a, b, rounding, zero, remainder, bounds) {
  if (b === 0n) {
    if (zero !== null) throw new RangeError(zero);
    return remainder ? a : 0n;
  }
  let q = a / b;
  const r = a - q * b;
  if (r !== 0n) {
    if (rounding === 'floor' && (r < 0n) !== (b < 0n)) q -= 1n;
    if (rounding === 'euclid' && r < 0n) q = b > 0n ? q - 1n : q + 1n;
  }
  if (bounds && (q < bounds[0] || q > bounds[1])) throw new RangeError(bounds[2]);
  return remainder ? a - q * b : q;
}`,
  toNatChecked: `function ml_toNatChecked(value, message) {
  if (value < 0n) throw new RangeError(message);
  return value;
}`,
  abort: `function ml_abort(message) {
  throw new Error(message);
}`,
  assert: `function ml_assert(holds, statement) {
  if (!holds) throw new Error(\`assertion failed: \${statement}\`);
}`,
  equal: `function ml_equal(left, right) {
  if (typeof left !== 'object' || left === null) return left === right;
  const keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every((key) => ml_equal(left[key], right[key]));
}`,
  showNumber: `// console.log prints -0 as -0, where String(-0) is "0".
function ml_showNumber(value) {
  return Object.is(value, -0) ? '-0' : String(value);
}`,
  showOptional: readOptionalJavaScriptTextPrelude(),
  at: `// An element of an array; a read outside it, undefined in JavaScript, aborts.
function ml_at(values, index) {
  const at = Number(index);
  if (!Number.isInteger(at) || at < 0 || at >= values.length) throw new RangeError(\`array index \${index} out of range\`);
  return values[at];
}`,
  forall: `function ml_forall(values, property) {
  return values.every(property);
}`,
  domains: `// Bounded domains for executable theorem checks.
const ml_nat = [0n, 1n, 2n, 3n, 4n, 5n, 6n];
const ml_int = [-4n, -3n, -2n, -1n, 0n, 1n, 2n, 3n, 4n];
const ml_small_nat = [0n, 1n, 2n];
const ml_small_int = [-1n, 0n, 1n];
function ml_product(lists) {
  return lists.reduce((rows, list) => rows.flatMap((row) => list.map((value) => [...row, value])), [[]]);
}`,
};

function ident(name) {
  let result = name.replace(/[^A-Za-z0-9_]/gu, '_');
  if (/^[0-9]/u.test(result)) result = `x${result}`;
  if (KEYWORDS.has(result)) result = `${result}_`;
  return result;
}

export function emitJavaScript(program, options = {}) {
  program=prepareOrderedRecordValues(program);
  const targetIdentifier=readJavaScriptIdentifierModel(options.identifierModel,ident);
  if(options.identifierModel!==undefined&&options.scopedBindingModel!==undefined)throw unsupported('JavaScript identifier model composition','the original narrow scoped-binding model requires a separate composition proof');
  validateRustTypeNamespace(program);
  checkRustConstFunctionProof(program);
  const runtimeShell=checkRustRuntimeShell(program);
  if(runtimeShell&&options.scopedBindingModel!==undefined)refuseRustRuntimeShellComposition('scoped target bindings require an independent runtime composition proof');
  checkRustOwnedLocalStateProof(program);
  const scopedBindings = options.scopedBindingModel === undefined ? null : readJavaScriptScopedBindingEmission(program, options.scopedBindingModel);
  const reserved = new Set(KEYWORDS);
  const externals = new Map((program.externals ?? []).filter(external => !program.declarations.has(external.name)).map(external => [external.name, external]));
  for (const name of externals.keys()) {
    if (!acceptJavaScriptExternalBinding(name)) throw unsupported('JavaScript external binding', 'this provider name needs an explicit legal target binding alias');
    reserved.add(name);
  }
  if ([...program.declarations.values()].some(entry => Object.hasOwn(entry, 'sourceLocalState'))) reserved.add('ml_ownedTextInput');
  const state = new EmitState(program, 'JavaScript', targetIdentifier, reserved, { modulesShareTermSpace: true, typeSpace: true });
  recordRustRuntimeShellAssumptions(state,runtimeShell);
  return new JavaScriptEmitter(program, state, scopedBindings, externals).file();
}

class JavaScriptEmitter {
  constructor(program, state, scopedBindings = null, externals = new Map()) {
    this.program = program;
    this.state = state;
    this.scopedBindings = scopedBindings;
    this.externals = externals;
    this.helpers = new Set();
    this.temporaries = 0;
    this.theoremChecks = [];
  }

  file() {
    const tree = { items: [], modules: new Map() };
    for (const entry of this.program.declarations.values()) {
      let node = tree;
      for (const segment of entry.modulePath) {
        if (!node.modules.has(segment)) node.modules.set(segment, { items: [], modules: new Map() });
        node = node.modules.get(segment);
      }
      node.items.push(entry);
    }
    const blocks = [];
    for (const entry of tree.items) {
      const text = this.declaration(entry);
      if (text) blocks.push(text.replace(/^([A-Za-z0-9_$]+)\(/u, 'function $1('));
    }
    for (const [segment, node] of tree.modules) {
      blocks.push(`const ${this.state.moduleName([segment])} = ${this.moduleObject(node, [segment])};`);
    }
    const main = this.program.main ? this.main(this.program.main) : null;
    if (this.theoremChecks.length) this.helpers.add('domains').add('forall');
    const helperOrder = ['ownedTextInput', 'natSub', 'fixed', 'divide', 'toNatChecked', 'abort', 'assert', 'equal', 'showNumber', 'showOptional', 'at', 'forall', 'domains','bigIntegerConversion','utf16Text','unicodeCase','recordPropertyOrder'];
    const entry = [
      this.theoremChecks.length ? this.theoremRunner() : null,
      main,
      this.theoremChecks.length
        ? "if (process.argv.includes('--ml-check-theorems')) ml_checkTheorems();\nelse main();"
        : (main ? 'main();' : null),
    ].filter(Boolean);
    const text = [
      `// Translated from ${this.program.sourceLanguage} by meta-language: portable core, JavaScript target.`,
      "'use strict';",
      '',
      ...helperOrder.filter((name) => this.helpers.has(name)).flatMap((name) => [HELPERS[name], '']),
      ...blocks.flatMap((block) => [block, '']),
      ...entry.flatMap((block) => [block, '']),
    ].join('\n');
    this.state.encode('numbers', 'every natural, integer and machine integer is a BigInt; machine-integer results are range-checked and throw RangeError where Rust would panic');
    if (this.usesFloat) this.state.encode('floats', 'a Number is a JavaScript Number, an IEEE-754 double, with its own arithmetic, comparisons and String conversion');
    return withParts({
      language: 'JavaScript',
      text,
      mappings: this.state.mappings,
      assumptions: this.state.assumptionList(),
      encodings: this.state.encodingList(),
      theorems: this.state.theorems,
      entry: main ? 'main' : null,
    }, helperOrder.filter((name) => this.helpers.has(name)).map((name) => HELPERS[name]), blocks);
  }

  moduleObject(node, path) {
    const members = [];
    for (const entry of node.items) {
      const text = this.declaration(entry);
      if (text) members.push(indent(`${text},`, 1));
    }
    for (const [segment, child] of node.modules) {
      members.push(indent(`${this.state.moduleName([...path, segment])}: ${this.moduleObject(child, [...path, segment])},`, 1));
    }
    return `{\n${members.join('\n')}\n}`;
  }

  declaration(entry) {
    if (entry.k === 'data') {
      this.state.map(entry, this.state.localName(entry.fullName));
      this.state.encode('data', this.program.sourceLanguage === 'JavaScript'
        ? 'a JavaScript data value is an object whose $ property names its constructor and whose other properties retain their own field names; explicit source freezing is preserved'
        : 'a data value is a frozen object whose $ property names its constructor and whose other properties are its fields');
      return null;
    }
    if (entry.k === 'fn') return this.fn(entry);
    return this.theorem(entry);
  }

  fn(entry) {
    const local = this.scopedBindings ? name => this.state.ident(this.scopedBindings(name, entry)) : this.state.ident;
    // A revalidated closed loop reads only its locals, Math and its own tail
    // continuation, which becomes writes/continue before JavaScript emission.
    const reserved = this.scopedBindings && !reserveJavaScriptScopedBindingDeclarationNames(entry.sourceLoop !== undefined)
      ? KEYWORDS : this.state.localReserved();
    const { params, body } = renameFunction(entry, local, reserved);
    if (params.some((param) => param.type.kind === 'float') || entry.ret.kind === 'float') this.usesFloat = true;
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    const guards = params.flatMap((param) => this.parameterGuard(param, entry.sourceLocalState !== undefined));
    if(entry.sourceDiagnosticAttributes!==undefined){
      const normalization=readRustDiagnosticAttributeNormalization();
      this.state.encode(normalization.id,normalization.statement);
    }
    if(entry.sourceConstFunction!==undefined){
      for(const param of params){const kind=param.type.kind==='bool'?'boolean':'number';guards.push(`if (typeof ${param.name} !== ${JSON.stringify(kind)}) throw new RangeError(${JSON.stringify(`const predicate argument ${param.name} requires ${kind}`)});`);}
      this.state.encode('rust-const-predicate-inputs','the selected Rust const predicate model admits only primitive Boolean and binary64 input values; const evaluation requests remain outside this source interpretation');
    }
    if (entry.sourceLocalState !== undefined) {
      this.state.encode('owned-text-input', 'the selected Rust local value model admits primitive Unicode scalar text, Boolean and signed i32 inputs; JavaScript rejects other input values before evaluating the body');
      this.state.assume(RUST_OWNED_LOCAL_STATE_POLICY.targetInputAssumption);
    }
    const loop = tailLoop(entry, body);
    // A lifted loop runs as a loop: each iteration assigns the parameters their next values.
    this.loopParams = loop && params.map((param) => param.name);
    const lines = [...guards, ...(loop ? ['for (;;) {', indent(this.statements(loop).join('\n'), 1), '}'] : this.statements(body))];
    this.loopParams = null;
    return `${name}(${params.map((param) => param.name).join(', ')}) {\n${indent(lines.join('\n'), 1)}\n}`;
  }

  /** Machine-integer parameters are range-checked, as the Rust type guarantees. */
  parameterGuard(param, ownedLocalState = false) {
    const guards = [];
    if (ownedLocalState) {
      if (param.type.kind === 'string') {
        this.helpers.add('ownedTextInput');
        guards.push(`if (!ml_ownedTextInput(${param.name})) throw new RangeError(${JSON.stringify(`String argument ${param.name} requires Unicode scalar text`)});`);
      } else {
        const type = param.type.kind === 'bool' ? 'boolean' : 'bigint';
        guards.push(`if (typeof ${param.name} !== ${JSON.stringify(type)}) throw new RangeError(${JSON.stringify(`${typeKey(param.type)} argument ${param.name} requires ${type}`)});`);
      }
    }
    if (param.type.kind !== 'fixed') return guards;
    this.helpers.add('fixed');
    const { min, max } = fixedBounds(param.type);
    return [...guards, `ml_fixed(${param.name}, ${min}n, ${max}n, ${JSON.stringify(`${typeKey(param.type)} argument ${param.name} out of range`)});`];
  }

  theorem(entry) {
    const { binders, prop } = renameTheorem(entry, this.state.ident, this.state.localReserved());
    const name = this.state.localName(entry.fullName);
    this.state.map(entry, name);
    this.state.theorem(entry, name, { closedGoal: binders.length === 0, discharge: 'source-kernel', check: 'bounded' });
    this.theoremChecks.push({ ref: this.state.ref(entry.fullName), binders, source: entry.fullName });
    return `${name}(${binders.map((binder) => binder.name).join(', ')}) {\n  return ${this.prop(prop)};\n}`;
  }

  theoremRunner() {
    this.state.encode('theorem-properties', 'each theorem is an executable property; --ml-check-theorems evaluates it on every input of a bounded domain, and its proof remains checked by the source kernel');
    const checks = this.theoremChecks.map(({ ref, binders, source }) => {
      const domains = binders.map((binder) => this.domain(binder.type, 3));
      const call = binders.length
        ? `ml_product([${domains.join(', ')}]).every((args) => ${ref}(...args))`
        : `${ref}()`;
      const failure = JSON.stringify(`theorem ${source} fails on a bounded input`);
      const success = JSON.stringify(`theorem ${source}: holds on the bounded domain`);
      return `  if (!(${call})) throw new Error(${failure});\n  console.log(${success});`;
    });
    return `function ml_checkTheorems() {\n${checks.join('\n')}\n}`;
  }

  /** Values of a type up to a constructor depth, as JavaScript source. */
  domain(type, depth) {
    this.helpers.add('domains');
    switch (type.kind) {
      case 'nat':
        return depth >= 3 ? 'ml_nat' : 'ml_small_nat';
      case 'int':
        return depth >= 3 ? 'ml_int' : 'ml_small_int';
      case 'fixed':
        return type.signed ? 'ml_small_int' : 'ml_small_nat';
      case 'bool':
        return '[false, true]';
      case 'string':
        return "['', 'a', 'ab']";
      case 'unit':
        return '[null]';
      case 'data': {
        const entry = this.program.declarations.get(type.name);
        const values = entry.ctors.flatMap((ctor) => {
          const recursive = ctor.fields.some((field) => field.type.kind === 'data');
          if (recursive && depth <= 1) return [];
          const fields = ctor.fields.map((field) => this.domain(field.type, field.type.kind === 'data' ? depth - 1 : 1));
          const object = (args) => this.ctorObject(entry, ctor, args);
          if (!fields.length) return [`[${object([])}]`];
          return [`ml_product([${fields.join(', ')}]).map((args) => ${object(ctor.fields.map((_, index) => `args[${index}]`))})`];
        });
        return values.length ? `[${values.map((value) => `...${value}`).join(', ')}]` : '[]';
      }
      default:
        throw new Error(`no JavaScript domain for ${type.kind}`);
    }
  }

  ctorObject(entry, ctor, args, explicitFrozen = false) {
    const fields = ctor.fields.map((field, index) => `[${fieldKey(field.name)}]: ${args[index]}`);
    const object = `{ $: ${JSON.stringify(ctor.name)}${fields.length ? `, ${fields.join(', ')}` : ''} }`;
    return requireDataObjectFreeze(this.program.sourceLanguage, explicitFrozen) ? `Object.freeze(${object})` : `(${object})`;
  }

  prop(prop) {
    switch (prop.p) {
      case 'forall': {
        this.helpers.add('forall');
        const body = this.prop(prop.body);
        return prop.binders.reduceRight(
          (inner, binder) => `ml_forall(${this.domain(binder.type, 3)}, (${binder.name}) => ${inner})`,
          body,
        );
      }
      case 'and':
        return `(${this.prop(prop.left)} && ${this.prop(prop.right)})`;
      case 'or':
        return `(${this.prop(prop.left)} || ${this.prop(prop.right)})`;
      case 'implies':
        return `(!${this.prop(prop.left)} || ${this.prop(prop.right)})`;
      case 'not':
        return `!${this.prop(prop.arg)}`;
      case 'bool':
        return this.expr(prop.expr);
      case 'eq':
      case 'ne': {
        const [left, right] = [this.expr(prop.left), this.expr(prop.right)];
        const structured = prop.left.type.kind === 'data' || prop.left.type.kind === 'unit';
        if (structured) this.helpers.add('equal');
        const same = prop.sameValue ? `Object.is(${left}, ${right})` : `(${left} === ${right})`;
        const equal = structured ? `ml_equal(${left}, ${right})` : same;
        return prop.p === 'eq' ? equal : `!${equal}`;
      }
      default: {
        const operator = { lt: '<', le: '<=', gt: '>', ge: '>=' }[prop.p];
        return `(${this.expr(prop.left)} ${operator} ${this.expr(prop.right)})`;
      }
    }
  }

  /** A function body as statements that return its value. */
  statements(e) {
    switch (e.k) {
      case 'let':
        return [`const ${e.name} = ${this.expr(e.value)};`, ...this.statements(e.body)];
      case 'print':
        return [`console.log(${this.expr(e.text)});`, ...this.statements(e.body)];
      case 'if':
        return [
          `if (${this.expr(e.cond)}) {`,
          indent(this.statements(e.then).join('\n'), 1),
          '} else {',
          indent(this.statements(e.else).join('\n'), 1),
          '}',
        ];
      case 'match':
        return this.matchStatements(e);
      case 'abort':
        this.helpers.add('abort');
        return [`return ml_abort(${JSON.stringify(e.message)});`];
      case 'recur': {
        const args = e.args.map((arg) => this.expr(arg));
        const names = this.loopParams;
        if (names.length === 1) return [`${names[0]} = ${args[0]};`, 'continue;'];
        return [...(names.length ? [`[${names.join(', ')}] = [${args.join(', ')}];`] : []), 'continue;'];
      }
      default:
        return [`return ${this.expr(e)};`];
    }
  }

  matchStatements(e) {
    const lines = [];
    let subject;
    if (e.scrutinee.k === 'var') subject = e.scrutinee.name;
    else {
      this.temporaries += 1;
      subject = `ml_subject${this.temporaries}`;
      lines.push(`const ${subject} = ${this.expr(e.scrutinee)};`);
    }
    const fallback = e.cases.find((kase) => kase.pattern.k === 'wild' || kase.pattern.k === 'bind');
    const fallbackBody = (kase) => [
      ...(kase.pattern.k === 'bind' ? [`const ${kase.pattern.name} = ${subject};`] : []),
      ...this.statements(kase.body),
    ];
    if (e.scrutinee.type.kind !== 'data') {
      const zero = e.cases.find((kase) => kase.pattern.k === 'natZero');
      const succ = e.cases.find((kase) => kase.pattern.k === 'natSucc');
      const zeroLines = zero ? this.statements(zero.body) : fallbackBody(fallback);
      const succLines = succ
        ? [`const ${succ.pattern.name} = ${subject} - 1n;`, ...this.statements(succ.body)]
        : fallbackBody(fallback);
      return [...lines, `if (${subject} === 0n) {`, indent(zeroLines.join('\n'), 1), '} else {', indent(succLines.join('\n'), 1), '}'];
    }
    const entry = this.program.declarations.get(e.scrutinee.type.name);
    lines.push(`switch (${subject}.$) {`);
    for (const kase of e.cases) {
      if (kase.pattern.k !== 'ctor') continue;
      const ctor = entry.ctors.find((candidate) => candidate.name === kase.pattern.ctor);
      const binds = kase.pattern.binds
        .map((bind, index) => (bind ? `const ${bind} = ${subject}[${fieldKey(ctor.fields[index].name)}];` : null))
        .filter(Boolean);
      lines.push(`  case ${JSON.stringify(ctor.name)}: {`);
      lines.push(indent([...binds, ...this.statements(kase.body)].join('\n'), 2));
      lines.push('  }');
    }
    lines.push('  default: {');
    if (fallback) lines.push(indent(fallbackBody(fallback).join('\n'), 2));
    else lines.push(`    throw new TypeError(\`unexpected constructor \${${subject}.$}\`);`);
    lines.push('  }');
    lines.push('}');
    return lines;
  }

  expr(e) {
    if (e.type?.kind === 'float') this.usesFloat = true;
    switch (e.k) {
      case 'lit':
        return this.literal(e);
      case 'unit':
        return renderJavaScriptUnit(this.program.sourceLanguage);
      case 'var':
        return e.name;
      case 'call':
        if (this.externals.has(e.fn)) {
          const source = renderJavaScriptExternalCall(this.externals.get(e.fn).k, e.fn, e.args.map(arg => this.expr(arg)).join(', '));
          if (!source) throw unsupported('JavaScript external call', 'the checked provider kind and argument shape need a supported target binding');
          return source;
        }
        return `${this.state.ref(e.fn)}(${e.args.map((arg) => this.expr(arg)).join(', ')})`;
      case 'functionRef':
        return renderCallableFunctionReference('JavaScript', this.externals.has(e.fn) ? e.fn : this.state.ref(e.fn));
      case 'apply':
        return renderCallableApplication('JavaScript', this.expr(e.fn), e.args.map(arg => this.expr(arg)));
      case 'lambda':
        return renderCallableLambda('JavaScript', e.params.map(param => param.name), [], '', this.expr(e.body), []);
      case 'method':
        if(e.op==='readJavaScriptRecordPropertyOrder')return emitOrderedRecordOrderRead(this,e,'JavaScript');
        if (acceptLinkedArrayStateMethod(e.k, e.op, e.object.type.kind, e.object.type.reference === true)) {
          const receiver = this.expr(e.object), args = e.args.map(argument => this.expr(argument));
          this.state.encode(readArrayStateMethodIdentity(e.op), readArrayStateMethodStatement());
          return renderArrayStateMethodExpression('JavaScript', e.op, receiver, args, [], [], '');
        }
        if (acceptLinkedRecordFieldWrite(e.k, e.op, e.object.type.kind, e.object.type.reference === true)) {
          const receiver = this.expr(e.object), args = e.args.map(argument => this.expr(argument));
          this.state.encode('write-record-field', readRecordFieldWriteStatement());
          return renderRecordFieldWrite('JavaScript', readRecordFieldWriteOperator(e.op), receiver,
            JSON.stringify(readRecordFieldWriteName(e.op)), '', args, '');
        }
        if (e.object.type.reference && readReferenceMethodResult(e.object.type.kind, e.op)) {
          const receiver = this.expr(e.object);
          const args = e.args.map(argument => {
            if (argument === null) throw unsupported('shared collection method argument', 'every source argument needs its actual expression', e.span);
            return this.expr(argument);
          });
          this.state.encode(readReferenceMethodIdentity(e.object.type.kind, e.op), readReferenceMethodStatement());
          return renderReferenceMethodExpression('JavaScript', e.object.type.kind, e.op, receiver, args, false, '');
        }
        if (e.op === 'map' && e.args.length === 1 && e.args[0] !== null) return renderArrayFromMapping('JavaScript', this.expr(e.object), this.expr(e.args[0]), 0, '');
        if (acceptUtf16TextMethod(e.k, e.object.type.kind, e.op)) {
          this.helpers.add('utf16Text');
          const object = this.expr(e.object);
          const values = e.args.map((arg, index) => renderPrimitiveMethodArgument('JavaScript', e.object.type.kind, e.op, index, arg === null, arg === null ? '' : this.expr(arg)));
          return renderUtf16TextMethod('JavaScript', e.op, object, values.join(', '));
        }
        return renderPrimitiveMethodExpression('JavaScript', e.object.type.kind, e.op, this.expr(e.object),
          e.args.map((arg, index) => renderPrimitiveMethodArgument('JavaScript', e.object.type.kind, e.op, index, arg === null, arg === null ? '' : this.expr(arg))).join(', '));
      case 'record':
        return emitOrderedRecordValue(this,e,'JavaScript')??renderRecordValue('JavaScript', e.entries.map(entry => JSON.stringify(entry.name)), e.entries.map(entry => this.expr(entry.value)));
      case 'cellAllocate':
        return renderCallableCellOperation('JavaScript', e.k, '', this.expr(e.value), '', '', '');
      case 'cellRead':
        return renderCallableCellOperation('JavaScript', e.k, this.expr(e.cell), '', '', '', '');
      case 'cellWrite':
        return renderCallableCellOperation('JavaScript', e.k, this.expr(e.cell), this.expr(e.value), this.expr(e.body), '', '');
      case 'referenceAllocate':
        return renderSurfaceReferenceOperation('JavaScript', e.k, this.expr(e.value), '');
      case 'referenceRead':
        return renderSurfaceReferenceOperation('JavaScript', e.k, this.expr(e.reference), '');
      case 'recordField':
        return renderRecordField('JavaScript', this.expr(e.object), JSON.stringify(e.field), '');
      case 'tuple':
        if (Object.hasOwn(e, 'productModel') && !acceptProgramTupleProductModel(e.productModel, e.frozen === true)) throw unsupported('immutable tuple product model', 'the declared immutable value product requires its retained frozen facet', e.span);
        return renderTupleValue('JavaScript', e.items.map((item) => this.expr(item)), e.frozen === true);
      case 'tupleIndex':
        return renderTupleProjection('JavaScript', this.expr(e.object), String(e.index));
      case 'none':
        return renderOptionalAbsentValue('JavaScript', e.type.absence, e.absence, '');
      case 'some':
        return renderOptionalPresentValue('JavaScript', e.type.absence, this.expr(e.value));
      case 'flattenOption':
        return renderOptionalFlattening('JavaScript', e.value.type.absence, e.value.type.element.absence, e.type.absence, this.expr(e.value));
      case 'nullishNumber':
        return renderOptionalJavaScriptOperation(e.k, this.expr(e.arg), '', false);
      case 'optionalIndexNumber':
        return renderOptionalJavaScriptOperation(e.k, this.expr(e.arg), '', false);
      case 'truthy':
        return renderOptionalPrimitiveTruth('JavaScript', e.arg.type.kind, this.expr(e.arg), '', false, false);
      case 'strictEqual':
        return renderOptionalStrictEquality('JavaScript', e.left.type.kind, e.left.type.absence ?? '', e.right.type.kind, e.right.type.absence ?? '', '', this.expr(e.left), this.expr(e.right), e.negate);
      case 'optionalTypeOf':
        return renderOptionalJavaScriptOperation(e.k, this.expr(e.arg), '', false);
      case 'optionalText':
        if (e.console) this.helpers.add('showOptional');
        return renderOptionalJavaScriptOperation(e.k, this.expr(e.arg), '', e.console);
      case 'unwrapOption':
        return renderOptionalUnwrapping('JavaScript', e.arg.type.absence, this.expr(e.arg), '', '');
      case 'optionalPredicate':
        return renderOptionalJavaScriptOperation(e.k, this.expr(e.arg), e.op, false);
      case 'coalesce':
        return renderOptionalCoalescing('JavaScript', e.left.type.absence, this.expr(e.left), '', this.expr(e.right));
      case 'optionalField':
      case 'optionalProject':
        return renderOptionalProjection('JavaScript', e.object.type.absence, this.expr(e.object), '', '', '', JSON.stringify(e.field));
      case 'optionalIndex':
        return renderOptionalArrayRead('JavaScript', e.index.type.kind, this.expr(e.array), this.expr(e.index));
      case 'stringPoints':
        return renderStringPointIteration('JavaScript', false, this.expr(e.object));
      case 'set':
      case 'map':
        return renderCollectionConstruction('JavaScript', e.k, '', this.expr(e.k === 'set' ? e.items : e.entries), '', '', '');
      case 'collectionSize':
        return renderCollectionSizeExpression('JavaScript', this.expr(e.object));
      case 'collectionRead':
        return renderCollectionReadExpression('JavaScript', e.object.type.kind, e.op, this.expr(e.object), this.expr(e.key), false, '', '', '');
      case 'collectionItems':
        return renderCollectionIteration('JavaScript', e.object.type.kind === 'map', e.op, this.expr(e.object), '', '');
      case 'ctor': {
        const entry = this.program.declarations.get(e.data);
        const ctor = entry.ctors.find((candidate) => candidate.name === e.ctor);
        return this.ctorObject(entry, ctor, e.args.map((arg) => this.expr(arg)), Boolean(e.frozen));
      }
      case 'unary':
        if (e.op === 'not') return `!${this.expr(e.arg)}`;
        return this.checked(negate(this.expr(e.arg)), e.type, 'neg');
      case 'binary':
        return this.binary(e);
      case 'if':
        return `(${this.expr(e.cond)} ? ${this.expr(e.then)} : ${this.expr(e.else)})`;
      case 'print':
        return `(console.log(${this.expr(e.text)}), ${this.expr(e.body)})`;
      case 'let':
      case 'match':
        return `(() => {\n${indent(this.statements(e).join('\n'), 1)}\n})()`;
      case 'toString':
        if (e.console && e.arg.type.kind === 'float') {
          this.helpers.add('showNumber');
          return `ml_showNumber(${this.expr(e.arg)})`;
        }
        return e.arg.type.kind === 'string' ? this.expr(e.arg) : this.toText(e.arg);
      case 'stringMap': {
        if(this.program.sourceLanguage==='JavaScript'&&acceptUnicodeCaseMappingOperation(e.op)){this.helpers.add('unicodeCase');this.state.encode('unicode-default-case-16',readUnicodeCaseEncodingStatement());return renderUnicodeCaseStringMap('JavaScript',e.op,false,this.expr(e.string));}
        const string = e.string.k === 'var' || e.string.k === 'lit' ? this.expr(e.string) : `(${this.expr(e.string)})`;
        if (e.op === 'rustTrim') return `${string}.replace(/^\\p{White_Space}+|\\p{White_Space}+$/gu, '')`;
        if (e.op === 'rustTrimStart') return `${string}.replace(/^\\p{White_Space}+/u, '')`;
        if (e.op === 'rustTrimEnd') return `${string}.replace(/\\p{White_Space}+$/u, '')`;
        return renderStringMapExpression('JavaScript', e.op, string);
      }
      case 'stringTest': {
        if (requireStringOrderUnits('stringTest', e.op)) return renderStringOrderExpression('JavaScript', e.op, this.expr(e.string), this.expr(e.search));
        const string = e.string.k === 'var' || e.string.k === 'lit' ? this.expr(e.string) : `(${this.expr(e.string)})`;
        return renderStringTestExpression('JavaScript', e.op, string, this.expr(e.search));
      }
      case 'cast':
        return this.cast(e);
      case 'abort':
        this.helpers.add('abort');
        return `ml_abort(${JSON.stringify(e.message)})`;
      case 'array':
        this.arrays();
        return `[${e.items.map((item) => this.expr(item)).join(', ')}]`;
      case 'append':
        return `[...${this.expr(e.left)}, ...${this.expr(e.right)}]`;
      case 'index':
        this.arrays();
        this.helpers.add('at');
        if (!e.provenInBounds) this.state.arrayRead();
        return `ml_at(${this.expr(e.array)}, ${this.expr(e.index)})`;
      case 'length':
        return e.type.kind === 'float' ? `${this.expr(e.array)}.length` : `BigInt(${this.expr(e.array)}.length)`;
      case 'math': {
        const byteLength=emitRustStringByteLength(this,e,'JavaScript',value=>this.expr(value));if(byteLength!==null)return byteLength;
        if(acceptLinkedJavaScriptStringConstruction(e.k,e.op)){this.helpers.add('utf16Text');return renderJavaScriptStringConstruction('JavaScript',e.op,this.expr(e.args[0]));}
        if(acceptLinkedJavaScriptStringUnitConstruction(e.k,e.op))return renderJavaScriptStringUnitConstruction('JavaScript',this.expr(e.args[0]));
        if(acceptLinkedJavaScriptBigIntegerResult(e.k,e.op)){
          this.helpers.add('bigIntegerConversion');
          if(['number-compare','string-compare'].includes(readJavaScriptBigIntegerResultOperation(e.op)))return renderJavaScriptBigIntegerComparison('JavaScript',readJavaScriptBigIntegerResultOperation(e.op),this.expr(e.args[0]),this.expr(e.args[1]),false);
          if(readJavaScriptBigIntegerResultOperation(e.op)==='number-power')return renderJavaScriptNumberPower('JavaScript',this.expr(e.args[0]),this.expr(e.args[1]));
          if(readJavaScriptBigIntegerResultOperation(e.op)==='power')return renderJavaScriptBigIntegerPower('JavaScript',this.expr(e.args[0]),this.expr(e.args[1]));
          if(readJavaScriptBigIntegerResultOperation(e.op)==='division')return renderJavaScriptBigIntegerDivision('JavaScript',this.expr(e.args[0]),this.expr(e.args[1]));
          return renderJavaScriptBigIntegerResultExpression('JavaScript',readJavaScriptBigIntegerResultOperation(e.op),this.expr(e.args[0]),false);
        }
        if(acceptLinkedJavaScriptSourceException(e.k,e.op))return renderJavaScriptSourceExceptionExpression('JavaScript',e.op,this.expr(e.args[0]),this.expr(e.args[1]),false);
        if(acceptLinkedJavaScriptNumberConversion(e.k,e.op))return renderJavaScriptNumberScalarConversion('JavaScript',e.op,'float',this.expr(e.args[0]),false);
        if (acceptLinkedRustCheckedShift(e.k,e.op)) return emitRustCheckedShift('JavaScript',e,argument=>this.expr(argument));
        if (readRustPrimitiveBitwiseArity(e.op)) return emitRustPrimitiveBitwise('JavaScript', e, argument => this.expr(argument));
        if (readBigIntegerBitwiseArity(e.op)) {
          const text = renderBigIntegerBitwiseExpression('JavaScript', e.op, e.args.map(argument => this.expr(argument)));
          if (!text) throw unsupported('BigInt bitwise arity', 'the operation requires its exact checked operand inventory', e.span);
          return text;
        }
        const args = e.args.map((arg) => this.expr(arg));
        const bitwise=readJavaScriptNumberBitwiseSymbol(e.op);
        if(bitwise){
          if(args.length!==readJavaScriptNumberBitwiseArity(e.op))throw unsupported('Number bitwise arity','the operation requires its exact checked operand inventory',e.span);
          return args.length===1?`(${bitwise}${args[0]})`:`(${args[0]} ${bitwise} ${args[1]})`;
        }
        if (e.op === 'maxOf' || e.op === 'minOf') return `Math.${e.op.slice(0, 3)}(...${args[0]})`;
        return `${e.type.kind === 'bool' ? 'Number' : 'Math'}.${e.op}(${args.join(', ')})`;
      }
      default:
        throw new Error(`no JavaScript expression for ${e.k}`);
    }
  }

  arrays() {
    this.state.encode('arrays', 'a JavaScript array stays an array, which the portable core never mutates; a read outside it throws RangeError where JavaScript reads undefined');
  }

  literal(e) {
    switch (e.type.kind) {
      case 'nat':
      case 'int':
      case 'fixed':
        return e.value.startsWith('-') ? `(${e.value}n)` : `${e.value}n`;
      case 'float':
        return e.value.startsWith('-') ? `(${e.value})` : e.value;
      case 'bool':
        return String(e.value);
      case 'string':
        return JSON.stringify(String(e.value));
      default:
        throw new Error(`no JavaScript literal for ${e.type.kind}`);
    }
  }

  toText(arg) {
    if (arg.type.kind === 'data' || arg.type.kind === 'unit') {
      throw unsupported('output of structured values', `a ${arg.type.kind} value has no portable textual form`, arg.span);
    }
    return `String(${this.expr(arg)})`;
  }

  /** A machine-integer result out of range panics as Rust does. */
  checked(text, type, op) {
    if (type.kind !== 'fixed') return text;
    this.helpers.add('fixed');
    const { min, max } = fixedBounds(type);
    return `ml_fixed(${text}, ${min}n, ${max}n, ${JSON.stringify(overflowMessage(op))})`;
  }

  binary(e) {
    const left = this.expr(e.left);
    const right = this.expr(e.right);
    switch (e.op) {
      case 'and':
        return `(${left} && ${right})`;
      case 'or':
        return `(${left} || ${right})`;
      case 'concat':
        return `(${left} + ${right})`;
      case 'eq':
        return `(${left} === ${right})`;
      case 'ne':
        return `(${left} !== ${right})`;
      case 'lt':
        return `(${left} < ${right})`;
      case 'le':
        return `(${left} <= ${right})`;
      case 'gt':
        return `(${left} > ${right})`;
      case 'ge':
        return `(${left} >= ${right})`;
      case 'add':
        return this.checked(`(${left} + ${right})`, e.type, 'add');
      case 'mul':
        return this.checked(`(${left} * ${right})`, e.type, 'mul');
      case 'sub':
        if (e.semantics === 'truncated') {
          this.helpers.add('natSub');
          return `ml_natSub(${left}, ${right})`;
        }
        return this.checked(`(${left} - ${right})`, e.type, 'sub');
      case 'div':
      case 'rem': {
        if (e.semantics === 'ieee') return `(${left} ${e.op === 'div' ? '/' : '%'} ${right})`;
        this.helpers.add('divide');
        const zero = e.byZero === 'abort' ? JSON.stringify(zeroDivisorMessage(e.op, e.type)) : 'null';
        if (e.type.kind !== 'fixed') return `ml_divide(${left}, ${right}, '${e.rounding}', ${zero}, ${e.op === 'rem'})`;
        const { min, max } = fixedBounds(e.type);
        return `ml_divide(${left}, ${right}, '${e.rounding}', ${zero}, ${e.op === 'rem'}, [${min}n, ${max}n, ${JSON.stringify(overflowMessage(e.op))}])`;
      }
      default:
        throw new Error(`no JavaScript operator ${e.op}`);
    }
  }

  cast(e) {
    const arg = this.expr(e.arg);
    if (e.flavor === 'wrapping') {
      if (e.from.kind !== 'fixed' || e.to.kind !== 'fixed') throw unsupported('wrapping integer cast', 'requires actual fixed integer source and target types', e.span);
      const code = renderWrappingFixedIntegerCast('JavaScript', arg, e.to.bits, e.to.signed, e.from.signed);
      if (!code) throw unsupported('wrapping integer cast', 'the integer width needs an exact target encoding', e.span);
      this.state.encode('fixed integer casts', readFixedIntegerCastStatement());
      return code;
    }
    if (e.flavor === 'exact') return arg;
    if (e.flavor === 'clamp') return `((value) => (value < 0n ? 0n : value))(${arg})`;
    this.helpers.add('toNatChecked');
    return `ml_toNatChecked(${arg}, ${JSON.stringify(castMessage(e))})`;
  }

  main(main) {
    const local = this.scopedBindings ? name => this.state.ident(this.scopedBindings(name, main)) : this.state.ident;
    const { effects } = renameMain(main, local, this.state.localReserved());
    const lines = [];
    let assertion = 0;
    for (const effect of effects) {
      if (effect.k === 'print') lines.push(`console.log(${this.expr(effect.expr)});`);
      else if (effect.k === 'let') lines.push(`const ${effect.name} = ${this.expr(effect.value)};`);
      else {
        assertion += 1;
        this.helpers.add('assert');
        lines.push(`ml_assert(${this.prop(effect.prop)}, ${JSON.stringify(`assertion ${assertion}`)});`);
        this.state.assertionTheorem(`assertion ${assertion}`, effect);
      }
    }
    if (main.sequentialAsync) this.state.encode('sequential-async', 'an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order');
    this.state.encode('program-output', 'main prints the lines the source program prints, in order, with console.log');
    return `function main() {\n${indent(lines.join('\n'), 1)}\n}`;
  }
}

/** `-x`, parenthesised so that `-(-x)` does not read as a decrement. */
function negate(text) {
  return text.startsWith('-') ? `-(${text})` : `-${text}`;
}

function fieldKey(name) {
  return JSON.stringify(name);
}

function indent(text, depth) {
  const pad = '  '.repeat(depth);
  return text.split('\n').map((line) => (line ? `${pad}${line}` : line)).join('\n');
}
