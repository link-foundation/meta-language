import {readJavaScriptOptionalChainOperation} from './javascript-optional-chain-rules.js';
import {readJavaScriptOptionalCallOperation,acceptJavaScriptOptionalCallHead,requireJavaScriptOptionalCallChain} from './javascript-optional-call-rules.js';
import {readJavaScriptArrayClassificationBuiltinOperation} from './array-classification-rules.js';
import {readJavaScriptOwnPropertyBuiltinOperation} from './javascript-own-property-rules.js';
import {readJavaScriptBooleanBuiltinOperation} from './optional-value-rules.js';
// JavaScript frontend for the portable core. It reads the subset of modern
// JavaScript the core can represent faithfully: Number and BigInt arithmetic,
// strings and booleans; top-level functions and object-literal namespaces of methods
// whose types come from JSDoc (`@param`, `@returns`, and `@typedef` unions of
// `{ $: 'tag', … }` object types for data types) or, where JSDoc is silent,
// are inferred from how the program uses them; bodies made of `const`,
// `if`, `return`, `throw` and `switch` statements; and a top level of
// `console.log`, `const` and `node:assert` statements, which are the
// program's effects. A function whose leading statements throw on a negative
// argument takes a natural number. Everything else is rejected with a precise
// obligation.

import { TranslationError, typeError, unsupported } from './diagnostics.js';
import { requireJavaScriptIdentifierRole } from './javascript-identifier-roles.js';
import { acceptJavaScriptSourceKeywordToken } from './javascript-identifier-rules.js';
import { constructScalarAssignmentExpression, constructJavaScriptSequenceExpression, constructJavaScriptVoidExpression } from './scalar-assignments.js';
import { readScalarAssignmentOperation } from './scalar-assignment-rules.js';
import { readSourceParserIdentifierSupply } from './source-identifiers.js';
import { readJavaScriptFixedIntegerCast } from './javascript-fixed-integer-casts.js';
import {readJavaScriptNumberBitwiseOperation,readJavaScriptNumberBitwiseAssignment,acceptJavaScriptNumberBitwisePrecedence} from './number-bitwise-rules.js';
import {readJavaScriptNumberBuiltinOperation,readJavaScriptPrimitiveUnaryOperation} from './string-number-rules.js';
import {readJavaScriptExceptionConstructorOperation} from './source-exception-rules.js';
import {readJavaScriptBigIntegerBuiltinOperation,acceptJavaScriptExponentiationBase} from './big-integer-conversion-rules.js';
import {createJavaScriptStringLiteral} from './javascript-string-literals.js';
import {readJavaScriptStringConstructionOperation} from './string-construction-rules.js';
import {readJavaScriptLooseEqualityOperation} from './javascript-comparison-rules.js';
import { findDocumentationParameterRange, findDefaultParameterReference, readArrayMethodForm, acceptArrayMethodArguments, readStringTestOperation, readStringMapOperation, readJavaScriptTemporalBindingRead, readJavaScriptLexicalDeclarationStep, readJavaScriptFieldSubstitutionPlan, acceptJavaScriptMutableArrayBinding, acceptJavaScriptMutableArrayBindingOwner } from './frontend-rules.js';
import { inferJavaScriptTypes } from './javascript-infer.js';
import { readJavaScriptTagOwnerPosition, readJavaScriptTagReceiverForm, readJavaScriptTagBindingStep, acceptJavaScriptTagHeaderFact, readJavaScriptTagDeclaredOwner } from './javascript-tag-context-rules.js';
import { readCallableReferenceConstruct, readCallableReferenceObligation, readCallableSurfaceConstructor } from './callable-source-rules.js';
import { imperative, lowerImperative, lowerTopLevel, statementUses } from './javascript-lower.js';
import { TokenCursor, describe, tokenize } from './lexer.js';
import { BOOL, FLOAT, INT, NAT, STRING, array } from './types.js';
import { arrayBindingNames, bindArrayElements, isLiteralBinding, readArrayBinding, readArrayBindingInitializer, readClosure, readCompositeType, validateClosureCaptures } from './javascript-bindings.js';
import { hasStructuralIdentity, requiresReferenceSemantics } from './reference-analysis.js';
import { markSharedReferenceTypes } from './javascript-cells.js';
import { acceptSurfaceRawMetadataField, selectSurfaceReferenceSemantics } from './surface-raw-metadata-rules.js';
import { readOptionalLogicalMixingError, readOptionalSourceLiteral } from './optional-value-rules.js';
import { acceptObjectFreezeOperand, acceptObjectPropertyDefinition } from './primitive-value-rules.js';
import { validateJavaScriptProducts } from './javascript-products.js';
import { readNamedJavaScriptRecordParameters, readNamedJavaScriptRecordParameterBody } from './javascript-record-parameters.js';
import { acceptJavaScriptStandardBuiltinOwner } from './javascript-record-parameter-rules.js';
import {acceptRustScalarLoopModel} from './rust-scalar-loops.js';
import {isJavaScriptScalarLoop, readJavaScriptScalarLoop, finishJavaScriptScalarLoops} from './javascript-scalar-loops.js';
import { findJavaScriptLineTerminator } from './javascript-line-terminators.js';
import { lowerOwnedArrayAppends, readOwnedArrayReaderParameters } from './owned-array-append.js';
import { lowerGuardedPrivateArrayWrites } from './guarded-private-array-writes.js';
import { lowerConstructedGuardedArrayWrites } from './constructed-guarded-array-writes.js';
import { acceptCompleteDeclarationSourceInterval } from '../source-span-rules.js';
import { acceptJavaScriptFunctionStart, acceptJavaScriptNamespaceMethod,
  readJavaScriptCollectionConstruction, readJavaScriptConstructionObligation,
  readJavaScriptDocumentationTypeForm, readJavaScriptValueCaseForm,
  acceptJavaScriptValueSwitch, readJavaScriptValueSwitchInstruction,
  readJavaScriptValueSwitchPosition, readJavaScriptObjectStatementForm,
  readJavaScriptObjectPropertyForm, readJavaScriptObjectValueForm,
  readJavaScriptArrayMethodSurfaceForm, acceptJavaScriptTerminalTagGuard,
  acceptJavaScriptTagGuardSequence } from './javascript-surface-rules.js';

// The string predicates of the portable core: each reads the whole string,
// so it means the same over UTF-16, UTF-8 and code points.
// String-to-string maps: the Unicode case mappings, which Rust's
// to_lowercase/to_uppercase also follow, and trimming of the JavaScript
// whitespace set.
const ROOT = 'crate';
const EQUALITY = { '===': 'eq', '!==': 'ne' };
const RELATIONAL = { '<': 'lt', '<=': 'le', '>': 'gt', '>=': 'ge' };
const MULTIPLICATIVE = { '*': 'mul', '/': 'div', '%': 'rem' };
const ASSIGNMENTS = new Set(['=', '+=', '-=', '*=', '/=', '%=', '**=', '<<=', '>>=', '>>>=', '&=', '|=', '^=', '&&=', '||=', '??=']);
// `x op= e` is `x = x op e`; `+=` adds numbers or concatenates strings like `+`.
const COMPOUND = { '+=': 'plus', '-=': 'sub', '*=': 'mul', '/=': 'div', '%=': 'rem', '&&=': 'and', '||=': 'or' };
const GLOBALS = new Set([
  'Math', 'Number', 'BigInt', 'parseInt', 'parseFloat', 'JSON', 'Array', 'Object', 'Symbol', 'Date', 'Promise', 'Reflect',
  'Proxy', 'Map', 'Set', 'WeakMap', 'WeakSet', 'globalThis', 'process', 'require', 'console', 'setTimeout', 'isNaN', 'isFinite',
]);
// Owned Math and Number builtins use shared kernels. Power's canonical
// definition retains its separate implementation-approximation obligations.
const MATH = {
  'Object.is': 'testJavaScriptPrimitiveSameValue',
  'Math.abs': 'abs', 'Math.floor': 'floor', 'Math.ceil': 'ceil', 'Math.trunc': 'trunc', 'Math.round': 'round',
  'Math.sign': 'sign', 'Math.sqrt': 'sqrt', 'Math.max': 'max', 'Math.min': 'min',
  'Math.pow': 'powerJavaScriptNumberArguments',
  'Number.isInteger': 'isInteger', 'Number.isSafeInteger': 'isSafeInteger', 'Number.isFinite': 'isFinite', 'Number.isNaN': 'isNaN',
};
// Their constants, as the source text of the Number each one is.
const CONSTANTS = {
  'Math.PI': '3.141592653589793', 'Math.E': '2.718281828459045', 'Math.LN2': '0.6931471805599453', 'Math.LN10': '2.302585092994046',
  'Math.LOG2E': '1.4426950408889634', 'Math.LOG10E': '0.4342944819032518', 'Math.SQRT2': '1.4142135623730951', 'Math.SQRT1_2': '0.7071067811865476',
  'Number.MAX_SAFE_INTEGER': '9007199254740991', 'Number.MIN_SAFE_INTEGER': '-9007199254740991', 'Number.EPSILON': '2.220446049250313e-16',
  'Number.MAX_VALUE': '1.7976931348623157e+308', 'Number.MIN_VALUE': '5e-324', 'Number.POSITIVE_INFINITY': 'Infinity',
  'Number.NEGATIVE_INFINITY': '-Infinity', 'Number.NaN': 'NaN',
};
const ASSERTION_MODULES = new Set(['node:assert', 'node:assert/strict', 'assert', 'assert/strict']);
const NAMESPACE_IMPORT = 'import the items of a module by name, as in import { f } from \'./m.mjs\', or the assertion module as a whole, as in import assert from \'node:assert/strict\'';
const STRICT_ASSERTIONS = { equal: 'eq', notEqual: 'ne', deepEqual: 'deep', notDeepEqual: 'notDeep' };
const ASSERTIONS = { strictEqual: 'eq', notStrictEqual: 'ne', deepStrictEqual: 'deep', notDeepStrictEqual: 'notDeep' };

/**
 * Reads `source` as a portable-core program. Self-translation reads one
 * top-level item of a module at a time and passes its `context`:
 * `externals`, the signatures of the module's other items and of the items
 * it imports, which the item may call and read, and `moduleDirectory`, the
 * module's directory inside its crate as path segments, which makes relative
 * imports items of the crate.
 */
export function parseJavaScript(source, context = {}) {
  if (!acceptRustScalarLoopModel(context.scalarLoopModel ?? '')) throw unsupported('JavaScript scalar loop model', 'the selected model is not registered');
  const { tokens, comments } = tokenize(source, 'JavaScript');
  return new JavaScriptParser(source, tokens, comments, context).file();
}

class JavaScriptParser {
  constructor(source, tokens, comments, { externals = [], moduleDirectory = null, literalBindings = new Map(), referenceSemantics, readonlyModuleConstants = false, scalarLoopModel = '', moduleBindingNames = [], moduleScopeKnown = true } = {}) {
    this.source = source;
    if (typeof moduleScopeKnown !== 'boolean' || !Array.isArray(moduleBindingNames)
      || moduleBindingNames.length > 100_000) {
      throw unsupported('JavaScript module binding context', 'known actual outer binding names and an explicit Boolean scope fact are required');
    }
    for (const name of moduleBindingNames) if (typeof name !== 'string' || name.length === 0) {
      throw unsupported('JavaScript module binding context', 'actual outer binding names must be nonempty strings without sparse slots');
    }
    this.moduleScopeKnown = moduleScopeKnown;
    this.scalarLoopModel = scalarLoopModel;
    this.readonlyModuleConstants = readonlyModuleConstants;
    this.readonlyConstantInitializer = false;
    this.referenceSemantics = referenceSemantics ?? requiresReferenceSemantics(source);
    this.referenceSemanticsEnabled = referenceSemantics !== false;
    this.referenceSemanticsOverride = referenceSemantics;
    this.referenceSemanticsForced = false;
    this.eliminatedOwnedArrayAppendOperations = [];
    // Private source construction identities; never serialized into surface/IR.
    this.ownedArrayIterationBlocks = new WeakMap();
    // The other items of the module an item may call and read, by name.
    this.externals = new Map(externals.map((external) => [external.name, external]));
    this.moduleDirectory = moduleDirectory;
    this.imports = [];
    this.cursor = new TokenCursor(tokens, 'JavaScript');
    this.literalBindings = literalBindings;
    // This private parser owns its tokenize result; no parser operation
    // mutates these token records. Reserve all module/import/context names
    // once, then retain generated logical argument names in the same Set.
    this.recordParameterTokenOwner = tokens;
    this.recordParameterNames = new Set([...this.externals.keys(), ...literalBindings.keys(), ...moduleBindingNames]);
    this.recordParameterOrdinal = 0;
    for (const token of tokens) if (token.kind === 'identifier') this.recordParameterNames.add(token.value);
    this.docs = comments.filter((comment) => comment.text.startsWith('/**') && comment.text !== '/**/');
    // Tags of every @typedef data type, for `switch (x.$)`.
    this.dataTypes = new Map();
    this.typeAliases = new Map();
    // Parser-only facts from actual lexical owners and actual source headers.
    // They never become checked IR or substitute for semantic validation.
    this.tagBindingTypes = new WeakMap();
    this.tagFunctionResults = new Map([...this.externals.values()]
      .filter(entry => entry.k === 'fn').map(entry => [entry.name, { type: entry.ret }]));
    // Locals in scope, the ones `let` or a parameter makes assignable, and
    // names of the current block still in their temporal dead zone.
    this.scope = { locals: new Set(), tdz: new Set(), mutable: new Set() };
    // Parser-local origins survive early builtin normalization and body
    // lowering. They are validated before type inference, never emitted as IR.
    this.lexicalScopes = [{ parent: -1, boundary: 0, bindings: new Map(), declarationCounts: new Map(), parameters: new Set() }];
    this.lexicalScope = 0;
    this.lexicalReads = [];
    // Enclosing loops and switches, for `break` and `continue`.
    this.jumps = { loops: 0, switches: 0 };
    // Whether a top-level statement is being read, where `return` is a syntax error.
    this.topLevel = false;
    // Functions and data types that loops and joins of statements lower to, and their count.
    this.generated = [];
    this.generatedCount = 0;
    this.generatedPrefix = readSourceParserIdentifierSupply(source, tokens);
    this.indexedReferenceWrites = 0;
    this.ownedArrayReaders = new Map();
    this.ownedArrayReaderNamespace = 0;
    this.assertion = null;
    // Async functions run sequentially: each call of one is awaited where it is
    // made, so nothing else runs until its result is back. `await` is allowed at
    // the top level of the module and in async functions.
    this.asyncNames = new Set();
    this.functionNames = new Set([...this.externals.values()].filter((item) => item.k === 'fn').map((item) => item.name));
    this.inAsync = true;
    this.unawaited = [];
    this.sequentialAsync = false;
    this.exportedNames = new Set();
    this.moduleBindings = new Set([...moduleBindingNames, ...this.externals.keys(), ...this.literalBindings.keys()]);
  }

  // Current lexical values resolve before special-global normalization. Known
  // outer names are actual source facts, never invented typed declarations.
  // Unknown module surfaces cannot prove that omitted binders/effects leave
  // a special intrinsic available; this does not invent a complete name list.
  acceptStandardBuiltinOwner(name) {
    return acceptJavaScriptStandardBuiltinOwner(name, !this.moduleScopeKnown
      || this.scope.locals.has(name) || this.scope.tdz.has(name)
      || this.moduleBindings.has(name) || this.externals.has(name)
      || this.literalBindings.has(name) || this.functionNames.has(name));
  }

  fail(message, token = this.cursor.peek()) {
    return new TranslationError('syntax', `${message} but found ${describe(token)}`, token);
  }

  file() {
    const c = this.cursor;
    const items = this.typedefs();
    const effects = [];
    if (c.isKind('string') && c.peek().value === 'use strict') {
      c.next();
      c.eat(';');
    }
    this.scope = { locals: new Set(), tdz: this.blockDeclarations(c.index), mutable: new Set() };
    for (const name of this.scope.tdz) this.lexicalBinding(name);
    this.asyncNames = this.asyncDeclarations(c.index);
    let depth = 0;
    for (let index = c.index; index < c.tokens.length; index += 1) {
      const token = c.tokens[index];
      if (depth === 0 && token.value === 'import' && token.kind === 'identifier') {
        let braces = 0;
        for (let at = index + 1; at < c.tokens.length; at += 1) {
          const imported = c.tokens[at];
          if (!braces && (imported.kind === 'string' || imported.value === 'from' || imported.value === ';')) break;
          const previous = c.tokens[at - 1]?.value;
          if (imported.kind === 'identifier' && (previous === 'import' || previous === 'as' || ['{', ','].includes(previous) && c.tokens[at + 1]?.value !== 'as')) this.moduleBindings.add(imported.value);
          if (imported.value === '{') braces += 1;
          if (imported.value === '}') braces -= 1;
        }
      }
      if (depth === 0 && token.kind === 'identifier') {
        if (token.value === 'function' && c.tokens[index + 1]?.kind === 'identifier') {
          const name = c.tokens[index + 1].value;
          const asynchronous = c.tokens[index - 1]?.value === 'async';
          let head = index - (asynchronous ? 1 : 0);
          if (c.tokens[head - 1]?.value === 'export') head -= 1;
          this.functionNames.add(name);
          const expression = c.tokens[index - 1]?.value === '=' || asynchronous && c.tokens[index - 2]?.value === '=';
          if (!expression) this.rememberTagFunction(name, c.tokens[head], asynchronous);
        }
        if (token.value === 'const' && c.tokens[index + 1]?.kind === 'identifier' && c.tokens[index + 2]?.value === '=' && this.startsFunction(index + 3)) {
          const name = c.tokens[index + 1].value;
          const asynchronous = c.tokens[index + 3]?.value === 'async' && c.tokens[index + 4]?.value !== '=>';
          const head = c.tokens[index - 1]?.value === 'export' ? index - 1 : index;
          this.functionNames.add(name);
          this.rememberTagFunction(name, c.tokens[head], asynchronous);
        }
      }
      if (token.kind === 'punct' && ['(', '[', '{'].includes(token.value)) depth += 1;
      if (token.kind === 'punct' && [')', ']', '}'].includes(token.value)) depth -= 1;
    }
    for (const name of [...this.scope.tdz, ...this.functionNames]) this.moduleBindings.add(name);
    // Top-level statements that declare or assign variables, lowered together
    // before the next statement that prints, binds a constant or asserts.
    let pending = [];
    const flush = () => {
      if (pending.length) effects.push(...this.lowerTopLevel(pending, effects));
      pending = [];
    };
    while (!c.atEnd()) {
      const start = c.peek();
      if (c.is('import')) {
        this.importDeclaration();
        continue;
      }
      const exported = Boolean(c.eat('export'));
      if (exported) {
        if (!c.is('function') && !c.is('const') && !(c.is('async') && c.is('function', 1))) {
          throw unsupported(`export ${describe(c.peek())}`, 'only function and const declarations are exported', span(start, c.peek()));
        }
        const name = c.is('async') ? c.peek(2) : c.peek(1);
        if (name.kind === 'identifier') this.exportedNames.add(name.value);
      }
      if (c.is('async') && c.is('function', 1)) {
        c.next();
        items.push(this.functionDeclaration(start, true));
        continue;
      }
      if (c.is('async')) throw unsupported('async function', 'only async functions declared by name or bound to a top-level constant are portable', span(start, c.peek()));
      if (c.is('function')) {
        items.push(this.functionDeclaration(start));
        continue;
      }
      if (c.is('const') && c.peek(1).kind === 'identifier' && c.is('=', 2) && this.startsFunction(c.index + 3)) {
        if (effects.length || pending.length) {
          throw unsupported(
            'function after a top-level statement',
            `the statements before const ${c.peek(1).value} could call it before it is initialised; declare every function first`,
            span(start, c.peek(1)),
          );
        }
        items.push(this.constFunction(start));
        continue;
      }
      if (c.is('const') && c.peek(1).kind === 'identifier' && c.is('=', 2) && c.is('{', 3) && !(c.is('$', 4) && c.is(':', 5))) {
        if (readJavaScriptObjectStatementForm(false, false, effects.length > 0 || pending.length > 0) === 'refuse-order') {
          throw unsupported(
            'namespace after a top-level statement',
            `the statements before const ${c.peek(1).value} would run before it is initialised; declare every namespace first`,
            span(start, c.peek(1)),
          );
        }
        const methods = this.namespaceObjectHasMethods(c.index + 3);
        if (readJavaScriptObjectStatementForm(false, methods, false) === 'namespace') { items.push(this.namespace(start)); continue; }
      }
      if (this.startsTopLevelImperative()) {
        pending.push(this.topLevelStatement());
        continue;
      }
      flush();
      const effect = this.mainStatement();
      const entries = Array.isArray(effect) ? effect : [effect];
      if (exported) for (const entry of entries) {
        if (entry.k === 'let' && !entry.name.startsWith(this.generatedPrefix)) this.exportedNames.add(entry.name);
      }
      effects.push(...entries);
    }
    flush();
    this.validateLexicalBindings();
    if (this.unawaited.length) {
      const [call] = this.unawaited;
      throw unsupported(`call of async function ${call.name} without await`, 'the Promise it returns is outside the portable core; await it where it is called', call.span);
    }
    items.push(...this.generated);
    const main = { effects, span: { start: 0, end: this.source.length } };
    if (this.sequentialAsync) main.sequentialAsync = true;
    const program = { language: 'JavaScript', items, main };
    if (this.eliminatedOwnedArrayAppendOperations.length) {
      this.referenceSemantics = this.referenceSemanticsForced || (this.referenceSemanticsOverride
        ?? requiresReferenceSemantics(this.source, this.eliminatedOwnedArrayAppendOperations));
    }
    if (selectSurfaceReferenceSemantics(this.referenceSemantics, false, this.referenceSemanticsEnabled)) program.referenceSemantics = true;
    if (this.imports.length) program.imports = this.imports;
    if (this.externals.size) program.externals = [...this.externals.values()];
    const inferred = inferJavaScriptTypes(program);
    finishJavaScriptScalarLoops(inferred);
    validateJavaScriptProducts(inferred, this.exportedNames);
    if (selectSurfaceReferenceSemantics(false, this.referenceSemanticsEnabled && hasStructuralIdentity(inferred), this.referenceSemanticsEnabled)) {
      const selected = inferred.referenceSemantics !== true;
      inferred.referenceSemantics = true;
      if (selected) return markSharedReferenceTypes(inferred);
    }
    return inferred;
  }

  /** Names of the top-level async functions, which every call must await. */
  asyncDeclarations(index) {
    const tokens = this.cursor.tokens;
    const names = new Set();
    let depth = 0;
    for (let at = index; at < tokens.length; at += 1) {
      const token = tokens[at];
      if (token.kind === 'punct' && ['(', '[', '{'].includes(token.value)) depth += 1;
      if (token.kind === 'punct' && [')', ']', '}'].includes(token.value)) depth -= 1;
      if (depth !== 0 || token.kind !== 'identifier') continue;
      const next = tokens[at + 1];
      if (token.value === 'async' && next?.value === 'function' && tokens[at + 2]?.kind === 'identifier') names.add(tokens[at + 2].value);
      if (token.value === 'const' && next?.kind === 'identifier' && tokens[at + 2]?.value === '=' && tokens[at + 3]?.value === 'async' &&
        tokens[at + 4]?.value !== '=>' && this.startsFunction(at + 3)) {
        names.add(next.value);
      }
    }
    return names;
  }

  /** Whether the tokens from `index` are a function expression or an arrow function, either maybe async. */
  startsFunction(index) {
    const tokens = this.cursor.tokens;
    let token = tokens[index];
    if (!token) return false;
    if (token.kind === 'identifier' && !acceptJavaScriptSourceKeywordToken(token.value, token.raw) && tokens[index + 1]?.value !== '=>') return false;
    if (token.kind === 'identifier' && token.value === 'async' && tokens[index + 1]?.value !== '=>') {
      index += 1;
      token = tokens[index];
      if (!token) return false;
    }
    const afterGroup = token.kind === 'punct' && token.value === '(' ? tokens[this.matching(index) + 1]?.value ?? '' : '';
    return acceptJavaScriptFunctionStart(token.kind, token.value, tokens[index + 1]?.value ?? '', afterGroup);
  }

  /**
   * `import assert from 'node:assert/strict'` binds the assertion module. In a
   * module of a crate, which self-translation reads, `import { a, b as c } from
   * './m.mjs'` imports items of another module of the crate. Every other import
   * is outside the portable core.
   */
  importDeclaration() {
    const c = this.cursor;
    const start = c.next();
    if (c.is('*')) throw unsupported('namespace import', NAMESPACE_IMPORT, span(start, c.peek()));
    let local = null;
    let braced = false;
    const names = [];
    if (c.eat('{')) {
      braced = true;
      while (!c.is('}')) {
        const imported = c.identifier('import');
        // A refused named import of node:assert ends at the token after its name.
        const after = c.peek();
        const alias = c.eat('as') ? c.identifier('import') : null;
        requireJavaScriptIdentifierRole(alias ?? imported, true);
        names.push({ imported: imported.value, local: (alias ?? imported).value, aliased: alias !== null, after });
        if (!c.eat(',')) break;
      }
      c.expect('}', 'import');
    } else {
      local = c.identifier('import');
      requireJavaScriptIdentifierRole(local, true);
    }
    c.expect('from', 'import');
    const module = c.next();
    if (module.kind !== 'string') throw this.fail('expected a module name', module);
    this.requireScalarStringToken(module,'module specifier');
    c.eat(';');
    const specifier = module.value;
    if (braced && !names.length) throw unsupported(`import {} from '${specifier}'`, 'an import names the items it imports', span(start, module));
    if (ASSERTION_MODULES.has(specifier)) {
      const refused = names.find((name, index) => index > 0 || name.imported !== 'strict' || !name.aliased);
      if (refused) {
        throw unsupported(`import { ${refused.imported} }`, 'import the assertion module as a whole, e.g. import assert from \'node:assert/strict\'', span(start, refused.after));
      }
      if (this.assertion) throw unsupported('second assertion import', 'import node:assert once', span(start, module));
      const name = names.length ? names[0].local : local.value;
      this.assertion = { name, strict: names.length > 0 || specifier.endsWith('/strict') };
      this.scope.tdz.delete(name);
      return;
    }
    if (!/^\.\.?\//u.test(specifier)) {
      const reason = specifier.startsWith('node:')
        ? 'Node.js built-in modules are outside the portable core'
        : 'packages are outside the portable core; a module imports the items of the other modules of its crate by relative path, as in import { f } from \'./m.mjs\'';
      throw unsupported(`import from '${specifier}'`, reason, span(start, module));
    }
    if (!names.length) throw unsupported(`default import from '${specifier}'`, 'a translated module has no default export; import its items by name, as in import { f } from \'./m.mjs\'', span(start, module));
    if (!this.moduleDirectory) {
      throw unsupported(`import from '${specifier}'`, 'a relative import names another module of a crate, and self-translation translates a crate module by module', span(start, module));
    }
    const path = cratePath(specifier, this.moduleDirectory);
    if (!path) throw unsupported(`import from '${specifier}'`, 'the specifier names no module file inside the crate', span(start, module));
    this.imports.push({ module: path, names: names.map(({ imported, local: name }) => ({ imported, local: name })), span: span(start, module) });
  }

  /**
   * `@typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint }} Tree`
   * declares a data type: each alternative is a constructor named by its
   * `$` tag, and its other properties are the constructor's fields.
   */
  typedefs() {
    const items = [];
    for (const comment of this.docs) {
      for (const tag of jsdocTags(comment)) {
        if (tag.tag !== 'typedef') continue;
        const range = { start: comment.start, end: comment.end };
        if (!tag.type || !tag.name) throw new TranslationError('syntax', '@typedef needs a type and a name', range);
        if (readJavaScriptDocumentationTypeForm(/\$\s*:/u.test(tag.type)) === 'alias') {
          if (this.typeAliases.has(tag.name) || this.dataTypes.has(tag.name)) throw typeError(`duplicate @typedef ${tag.name}`, range);
          this.typeAliases.set(tag.name, tag.type);
          continue;
        }
        const { tokens } = tokenize(tag.type, 'JavaScript');
        const c = new TokenCursor(tokens, 'JavaScript');
        const ctors = [];
        do {
          if (!c.is('{')) {
            throw unsupported(`@typedef ${tag.name}`, 'a portable data type is a union of { $: \'tag\', … } object types', range);
          }
          c.next();
          let name = null;
          const fields = [];
          while (!c.is('}')) {
            const key = c.identifier('object type');
            c.expect(':', 'object type');
            if (key.value === '$') {
              const value = c.next();
              if (value.kind !== 'string') throw unsupported(`@typedef ${tag.name}`, 'the $ tag must be a string literal type', range);
              this.requireScalarStringToken(value,'datatype constructor tag');
              name = value.value;
            } else {
              let type = c.identifier('field type').value;
              if (c.is('<')) {
                // Array<T>, whose closing brackets may be read as one >> token.
                let depth = 0;
                do {
                  const token = c.next();
                  type += token.value;
                  for (const char of token.value) depth += char === '<' ? 1 : char === '>' ? -1 : 0;
                } while (depth > 0 && !c.atEnd());
              }
              while (c.is('[')) {
                c.next();
                c.expect(']', 'field type');
                type += '[]';
              }
              if (!c.is(',') && !c.is(';') && !c.is('}')) {
                throw unsupported(`field type of ${key.value}`, 'field types are number, bigint, boolean, string, a @typedef name or an array T[] of one', range);
              }
              fields.push({ name: key.value, type: this.type(type, range) });
            }
            if (!c.eat(',') && !c.eat(';')) break;
          }
          c.expect('}', 'object type');
          if (name === null) throw unsupported(`@typedef ${tag.name}`, 'every alternative needs a $ tag', range);
          if (ctors.some((ctor) => ctor.name === name)) throw typeError(`duplicate tag ${name} in ${tag.name}`, range);
          ctors.push({ name, fields });
        } while (c.eat('|'));
        if (!c.atEnd()) throw unsupported(`@typedef ${tag.name}`, 'a portable data type is a union of object types', range);
        if (this.dataTypes.has(tag.name)) throw typeError(`duplicate @typedef ${tag.name}`, range);
        this.dataTypes.set(tag.name, ctors);
        items.push({ k: 'data', name: tag.name, ctors, span: range });
      }
    }
    return items;
  }

  type(text, range) {
    const name = text.trim();
    const composite = readCompositeType(this, name, range);
    if (composite) return composite;
    if (name === 'bigint') return INT;
    if (name === 'boolean') return BOOL;
    if (name === 'string') return STRING;
    if (name === 'number') return FLOAT;
    if (name.endsWith('[]')) return array(this.type(name.slice(0, -2), range));
    const generic = /^(?:Array|ReadonlyArray)<(.+)>$/u.exec(name);
    if (generic) return array(this.type(generic[1], range));
    if (this.typeAliases.has(name)) {
      const expanded = this.typeAliases.get(name);
      this.typeAliases.delete(name);
      try { return this.type(expanded, range); }
      finally { this.typeAliases.set(name, expanded); }
    }
    if (!/^[A-Za-z_$][\w$]*$/u.test(name) || ['object', 'any', 'unknown', 'void', 'undefined', 'null', 'Object', 'Function', 'Array', 'symbol'].includes(name)) {
      throw unsupported(`JSDoc type {${name}}`, 'portable types are number, bigint, boolean, string, arrays T[] of them and @typedef data types', range);
    }
    return { kind: 'named', path: [ROOT, name], span: range };
  }

  /** The JSDoc block directly before a declaration. */
  jsdocFor(token) {
    const doc = this.docs.findLast((comment) => comment.end <= token.start && this.source.slice(comment.end, token.start).trim() === '');
    if (!doc) return null;
    const params = new Map();
    const optionalParams = new Set();
    let returns = null;
    for (const tag of jsdocTags(doc)) {
      if (tag.tag === 'param') {
        if (!tag.type || !tag.name) throw new TranslationError('syntax', '@param needs a type and a name', doc);
        if (params.has(tag.name)) throw typeError(`duplicate @param ${tag.name}`, doc);
        params.set(tag.name, tag.type);
        if (tag.optional) optionalParams.add(tag.name);
      } else if (tag.tag === 'returns' || tag.tag === 'return') {
        returns = tag.type;
      }
    }
    return { params, optionalParams, returns, range: { start: doc.start, end: doc.end } };
  }

  functionDeclaration(start, isAsync = false) {
    const c = this.cursor;
    c.expect('function', 'function declaration');
    if (c.is('*')) throw unsupported('generator function', 'generators are outside the portable core', span(start, c.peek()));
    const name = c.identifier('function');
    global(name);
    return this.functionRest(start, name, isAsync, true);
  }

  /** Parameters and body of a function or method; `docToken` is where its JSDoc ends. */
  functionRest(docToken, nameToken, isAsync = false, namedRecordParameters = false) {
    const c = this.cursor;
    const doc = this.jsdocFor(docToken);
    c.expect('(', nameToken.value);
    const records = namedRecordParameters ? readNamedJavaScriptRecordParameters(this, doc, isAsync) : null;
    const tokens = records ? [] : this.parameters(nameToken.value);
    const body = () => records ? readNamedJavaScriptRecordParameterBody(this, records, () => this.blockStatements()) : this.blockStatements();
    return this.functionBody(docToken, nameToken, doc, tokens, body, isAsync, records?.parameters ?? null);
  }

  /**
   * `const name = (…) => …` or `const name = function (…) { … }` before any
   * top-level statement: a function, as nothing can call it before it is
   * initialised. An arrow's expression body is the value it returns.
   */
  constFunction(start) {
    const c = this.cursor;
    const doc = this.jsdocFor(start);
    c.expect('const', 'function');
    const nameToken = c.identifier('function');
    global(nameToken);
    c.expect('=', 'function');
    this.scope.tdz.delete(nameToken.value);
    const isAsync = c.is('async') && !c.is('=>', 1);
    if (isAsync) c.next();
    let fn;
    if (c.eat('function')) {
      if (c.is('*')) throw unsupported('generator function', 'generators are outside the portable core', span(start, c.peek()));
      if (c.isKind('identifier') && c.peek().value !== nameToken.value) {
        const inner = c.peek();
        throw unsupported(`function expression ${inner.value}`, `its own name is visible only inside it; call it ${nameToken.value}`, span(inner, inner));
      }
      if (c.isKind('identifier')) c.next();
      c.expect('(', nameToken.value);
      const tokens = this.parameters(nameToken.value);
      fn = this.functionBody(start, nameToken, doc, tokens, () => this.blockStatements(), isAsync);
    } else {
      let tokens;
      if (c.eat('(')) tokens = this.parameters(nameToken.value);
      else tokens = [c.identifier('parameter')];
      c.expect('=>', nameToken.value);
      fn = this.functionBody(start, nameToken, doc, tokens, () => {
        if (c.is('{')) return this.blockStatements();
        const token = c.peek();
        const expr = isAsync ? this.awaited(() => this.expr()) : this.expr();
        return [{ s: 'return', expr, span: span(token, c.peek()) }];
      }, isAsync);
    }
    c.eat(';');
    return fn;
  }

  /** A parameter list after its `(`, through its `)`. */
  parameters(name) {
    const c = this.cursor;
    const tokens = [];
    const names = [];
    let depth = 0;
    let binding = true;
    for (let at = c.index; at < c.tokens.length; at += 1) {
      const token = c.tokens[at];
      if (!depth && token.value === ')') break;
      if (!depth && binding && token.kind === 'identifier') { names.push(token.value); binding = false; }
      if (token.kind === 'punct' && ['(', '[', '{'].includes(token.value)) depth += 1;
      else if (token.kind === 'punct' && [')', ']', '}'].includes(token.value)) depth -= 1;
      else if (!depth && token.value === ',') binding = true;
    }
    while (!c.is(')')) {
      const token = c.peek();
      if (c.is('...')) throw unsupported('rest parameter', 'functions take a fixed number of arguments', span(token, token));
      if (c.is('{') || c.is('[')) throw unsupported('destructured parameter', 'name each parameter', span(token, token));
      const param = c.identifier('parameter');
      // A default is filled in where a call leaves the argument out; it reads no parameter.
      if (c.eat('=')) {
        const remaining = c.tokens.slice(c.index);
        const reference = findDefaultParameterReference(remaining.map((token) => token.kind), remaining.map((token) => token.value), tokens.map((token) => token.value));
        if (reference >= 0) {
          const token = c.peek(reference);
          throw unsupported(`default reading parameter ${token.value}`, 'a call fills in a default where it leaves the argument out, so a default reads no parameter', span(token, token));
        }
        const outer = this.scope;
        this.scope = { ...outer, tdz: new Set([...outer.tdz, ...names]) };
        try { tokens.push({ ...param, defaultValue: this.withLexicalScope([], () => this.expr(), 2) }); } finally { this.scope = outer; }
      }
      else if (tokens.at(-1)?.defaultValue) throw unsupported(`parameter ${param.value} after a default`, 'a call fills in trailing defaults only; give it a default too', span(param, param));
      else tokens.push(param);
      if (!c.eat(',')) break;
    }
    c.expect(')', name);
    return tokens;
  }

  /**
   * A function from its parameters; `statementsOf` reads its body in the
   * parameters' scope. An async function is the function its body computes,
   * and its JSDoc may declare the result as `Promise<T>`.
   */
  functionBody(docToken, nameToken, doc, tokens, statementsOf, isAsync = false, logicalParameters = null) {
    const c = this.cursor;
    const name = nameToken.value;
    if (isAsync) this.sequentialAsync = true;
    // A type JSDoc does not declare is inferred once the whole program is read.
    // Existing SParam records can own a lowered logical pattern argument;
    // their spans are origins, not assertions of source identifier spelling.
    const params = logicalParameters ?? tokens.map((token) => {
      reserved(token);
      const text = doc?.params.get(token.value);
      const optional = doc?.optionalParams.has(token.value) && !token.defaultValue;
      const declared = text ? this.type(text, doc.range) : null;
      const type = optional && declared ? { kind: 'option', element: declared, absence: 'undefined' } : declared;
      return {
        name: token.value,
        type,
        ...(token.defaultValue ? { default: token.defaultValue } : optional ? { optional: true, default: { k: 'none', absence: 'undefined', span: span(token, token) } } : {}),
        span: span(token, token),
      };
    });
    for (const documented of doc?.params.keys() ?? []) {
      if (!params.some((param) => param.name === documented)) throw typeError(`@param ${documented} is not a parameter of ${name}`, doc.range);
    }
    const promised = isAsync ? /^\s*Promise\s*<([\s\S]*)>\s*$/u.exec(doc?.returns ?? '') : null;
    const returns = promised ? promised[1] : doc?.returns;
    const ret = returns ? this.type(returns, doc.range) : null;
    const outer = this.scope;
    const outerAsync = this.inAsync;
    const outerJumps = this.jumps;
    const outerTopLevel = this.topLevel;
    this.inAsync = isAsync;
    this.jumps = { loops: 0, switches: 0 };
    this.topLevel = false;
    const names = params.map((param) => param.name);
    this.scope = { locals: new Set(names), tdz: new Set(), mutable: new Set(names) };
    // Record entry bindings must execute before the original body. The old
    // scalar-loop shortcut skips statementsOf(), so it cannot own this route.
    if (logicalParameters === null && this.scalarLoopModel && isJavaScriptScalarLoop(c)) {
      let loop;
      try { loop = this.withLexicalScope(names, () => readJavaScriptScalarLoop(this, name, params, isAsync), 2); }
      finally { this.scope = outer; this.inAsync = outerAsync; this.jumps = outerJumps; this.topLevel = outerTopLevel; }
      return {k: 'fn', name, params, ret, body: loop.body, sourceLoop: loop.proof, span: readCompleteDeclarationSpan(docToken, c)};
    }
    const indexedWritesBefore = this.indexedReferenceWrites;
    const statements = this.withLexicalScope(names, () => {
      for (const parameter of params) this.lexicalParameter(parameter.name);
      for (const parameter of params) this.rememberTagBinding(parameter.name, { type: parameter.type });
      return statementsOf();
    }, 2);
    validateClosureCaptures(statements, this.referenceSemantics);
    // An explicit or forced reference context keeps actual heap operations.
    // Only the inferred construction context may become immutable rebinding.
    if (!selectSurfaceReferenceSemantics(this.referenceSemanticsOverride === true, this.referenceSemanticsForced, true)) {
      if (this.indexedReferenceWrites !== indexedWritesBefore) {
        for (const proof of lowerConstructedGuardedArrayWrites(statements, params, this)) {
          this.eliminatedOwnedArrayAppendOperations.push(proof.operationSpan);
        }
      }
      for (const proof of lowerOwnedArrayAppends(statements, params, this.ownedArrayReaders, this.ownedArrayIterationBlocks)) {
        if (proof.operationSpan) this.eliminatedOwnedArrayAppendOperations.push(proof.operationSpan);
      }
      // This observed parse count avoids building duplicate fact columns when
      // no indexed write exists. Only the shared whole-body plan admits one.
      if (this.indexedReferenceWrites !== indexedWritesBefore) {
        for (const proof of lowerGuardedPrivateArrayWrites(statements, params, this)) {
          this.eliminatedOwnedArrayAppendOperations.push(proof.operationSpan);
        }
      }
    }
    this.scope = outer;
    this.inAsync = outerAsync;
    this.jumps = outerJumps;
    this.topLevel = outerTopLevel;
    // `if (n < 0n) throw …` as a leading statement makes `n` a natural number,
    // unless the body assigns it another value.
    const { assigned } = statementUses(statements);
    let index = 0;
    for (; index < statements.length; index += 1) {
      const param = guardedParameter(statements[index], params);
      if (!param || assigned.has(param.name)) break;
      param.type = NAT;
      // A negative argument aborts with the guard's message, the guards in their order.
      const [thrown] = statements[index].then.filter((inner) => inner.s !== 'empty');
      param.guard = { message: thrown.message, order: index };
    }
    const where = readCompleteDeclarationSpan(nameToken, c);
    const rest = statements.slice(index);
    const body = imperative(rest, this) ? lowerImperative(this, name, params, rest, where) : this.lower(rest, where);
    const declaration = { k: 'fn', name, params, ret, body, span: readCompleteDeclarationSpan(docToken, c) };
    if (!this.ownedArrayReaderNamespace) this.ownedArrayReaders.set(name, readOwnedArrayReaderParameters(declaration, this.ownedArrayReaders, statements));
    return declaration;
  }

  /** Inspect property headers, not calls or closures inside their values. */
  namespaceObjectHasMethods(index) {
    const tokens = this.cursor.tokens;
    const close = this.matching(index);
    let member = index + 1;
    while (member < close) {
      // Identifier modifiers retain the namespace parser's existing accessor
      // and asynchronous-method diagnostics; neither a colon nor a value is
      // part of this header. Generator prefixes retain their refusal there too.
      let header = member;
      while (header < close && (tokens[header].kind === 'identifier' || tokens[header].kind === 'punct' && tokens[header].value === '*')) {
        if (acceptJavaScriptNamespaceMethod(tokens[header].kind, tokens[header + 1]?.value ?? '')) return true;
        header += 1;
      }
      if (tokens[member].kind === 'identifier' && tokens[member + 1]?.value === ':' && tokens[member + 2]?.value === '{'
        && this.namespaceObjectHasMethods(member + 2)) return true;
      // Balanced groups belong to this one property's value (or computed key).
      // Opaque string, template and regex tokens cannot introduce boundaries.
      while (member < close) {
        const token = tokens[member];
        if (token.kind === 'punct' && token.value === ',') { member += 1; break; }
        if (token.kind === 'punct' && ['(', '[', '{'].includes(token.value)) member = this.matching(member) + 1;
        else member += 1;
      }
    }
    return false;
  }

  /** `const Name = { method(…) { … }, Nested: { … } };` is a module. */
  namespace(start) {
    const c = this.cursor;
    c.expect('const', 'namespace');
    const name = c.identifier('namespace');
    global(name);
    c.expect('=', 'namespace');
    this.scope.tdz.delete(name.value);
    const module = this.namespaceBody(name.value, start);
    c.eat(';');
    return module;
  }

  namespaceBody(name, start) {
    const c = this.cursor;
    c.expect('{', 'namespace');
    const items = [];
    while (!c.is('}')) {
      const member = c.peek();
      if (member.kind !== 'identifier') {
        throw unsupported(`namespace member ${describe(member)}`, 'members are methods or nested namespaces with identifier names', span(member, member));
      }
      if (['get', 'set', 'async', 'static'].includes(member.value) && c.peek(1).kind === 'identifier') {
        throw unsupported(`${member.value} member`, 'accessors and asynchronous methods are outside the portable core', span(member, c.peek(1)));
      }
      c.next();
      if (!acceptObjectPropertyDefinition(member.value, false, c.is(':'))) {
        throw unsupported('object prototype definition', 'an uncomputed __proto__: entry is prototype-definition syntax rather than an own data property; the portable core does not represent object prototypes', span(member, member));
      }
      if (acceptJavaScriptNamespaceMethod(member.kind, c.peek().value)) {
        this.functionNames.add(`${name}.${member.value}`);
        this.ownedArrayReaderNamespace += 1;
        try { items.push(this.functionRest(member, member)); }
        finally { this.ownedArrayReaderNamespace -= 1; }
      } else if (c.eat(':')) {
        if (c.is('{')) {
          items.push(this.namespaceBody(member.value, member));
        } else {
          throw unsupported(`namespace property ${member.value}`, 'namespaces hold methods, name(…) { … }, and nested namespaces only', span(member, c.peek()));
        }
      } else {
        throw this.fail(`expected ( or : after ${member.value}`);
      }
      if (!c.eat(',')) break;
    }
    c.expect('}', 'namespace');
    return { k: 'module', name, items, span: span(start, c.peek()) };
  }

  /** Every lexical declaration's names remain in their temporal dead zone until initialized. */
  blockDeclarations(from, _end = null, counts = null) {
    const c = this.cursor;
    const names = new Set();
    const collect = name => {
      names.add(name);
      if (counts) counts.set(name, (counts.get(name) ?? 0) + 1);
    };
    const saved = c.index;
    c.index = from;
    let depth = 0;
    let declaration = false;
    while (!c.atEnd()) {
      const token = c.next();
      const step = readJavaScriptLexicalDeclarationStep(token.kind, token.value, depth, declaration);
      if (step === -1) break;
      if (step === 1) depth += 1;
      else if (step === 2) depth -= 1;
      else if (step === 5) declaration = false;
      if (step !== 3 && step !== 4) continue;
      declaration = true;
      if (c.peek().kind === 'identifier') collect(c.peek().value);
      else if (c.is('[')) {
        for (const binding of arrayBindingNames(this, c.index)) collect(binding.value);
      }
      else if (c.is('{')) {
        for (let offset = 1; !c.is('}', offset) && !c.isKind('eof', offset); offset += 1) {
          if (c.peek(offset).kind === 'identifier' && (c.is(',', offset + 1) || c.is('}', offset + 1))) collect(c.peek(offset).value);
        }
      }
    }
    c.index = saved;
    return names;
  }

  withBlockScope(parse) {
    const c = this.cursor;
    const outer = this.scope;
    const tdz = new Set(outer.tdz);
    const counts = new Map();
    const names = this.blockDeclarations(c.index, null, counts);
    for (const name of names) tdz.add(name);
    this.scope = { locals: new Set(outer.locals), tdz, mutable: new Set(outer.mutable) };
    try {
      return this.withLexicalScope(names, () => {
        this.lexicalScopes[this.lexicalScope].declarationCounts = counts;
        return parse();
      });
    } finally {
      this.scope = outer;
    }
  }

  declareLocal(name, mutable = false) {
    this.lexicalBinding(name);
    this.scope.locals.add(name);
    this.scope.tdz.delete(name);
    if (mutable) this.scope.mutable.add(name);
    else this.scope.mutable.delete(name);
  }

  withLexicalScope(names, parse, boundary = 0) {
    const outer = this.lexicalScope;
    this.lexicalScope = this.lexicalScopes.length;
    this.lexicalScopes.push({ parent: outer, boundary, bindings: new Map(), declarationCounts: new Map(), parameters: new Set() });
    for (const name of names) this.lexicalBinding(name);
    try { return parse(); } finally { this.lexicalScope = outer; }
  }

  lexicalBinding(name, initializedAt = -1) {
    const bindings = this.lexicalScopes[this.lexicalScope].bindings;
    if (initializedAt >= 0 || !bindings.has(name)) bindings.set(name, initializedAt);
  }

  lexicalParameter(name) {
    this.lexicalScopes[this.lexicalScope].parameters.add(name);
    this.lexicalBinding(name);
  }

  lexicalRead(token, assignment = false, where = span(token, token)) {
    requireJavaScriptIdentifierRole(token, assignment);
    this.lexicalReads.push({ scope: this.lexicalScope, name: token.value, start: token.start, span: where, assignment });
  }

  closure(method = false) {
    return this.withLexicalScope([], () => {
      const value = readClosure(this, method);
      for (const parameter of value.params) this.lexicalBinding(parameter.name);
      return value;
    }, 1);
  }

  validateLexicalBindings() {
    const bounds = [], names = [], ends = [];
    for (const scope of this.lexicalScopes) {
      bounds.push(names.length, scope.bindings.size, scope.boundary);
      for (const [name, end] of scope.bindings) { names.push(name); ends.push(end); }
    }
    const plan = readJavaScriptTemporalBindingRead(this.lexicalScopes.map(scope => scope.parent), bounds, names, ends,
      this.lexicalReads.map(read => read.scope), this.lexicalReads.map(read => read.name), this.lexicalReads.map(read => read.start));
    if (!plan.length) return;
    const read = this.lexicalReads[plan[0]];
    if (plan[1] === 2) throw unsupported(`function reading ${read.name}`,
      'a mutable module binding requires module-capture representation', read.span);
    if (plan[1] === 1) throw unsupported(`closure reading ${read.name} before its declaration`,
      'the captured binding may still be in its temporal dead zone; invoking the closure after initialization needs an invocation-order proof', read.span);
    if (read.assignment) throw unsupported(`assignment of ${read.name} before its declaration`, 'the binding is in its temporal dead zone, where assigning it throws a ReferenceError', read.span);
    throw unsupported(readCallableReferenceConstruct(read.name, true), readCallableReferenceObligation(true, true), read.span);
  }

  blockStatements() {
    const c = this.cursor;
    c.expect('{', 'block');
    return this.withBlockScope(() => {
      const statements = [];
      while (!c.is('}')) {
        if (c.atEnd()) throw this.fail('expected }');
        statements.push(this.statement());
      }
      c.expect('}', 'block');
      return statements;
    });
  }

  statement() {
    const c = this.cursor;
    const token = c.peek();
    if (c.is('const')) return this.constStatement(this.topLevel && this.readonlyModuleConstants);
    if (c.is('let')) {
      const statements = this.letDeclarations();
      c.eat(';');
      return statements.length === 1 ? statements[0] : { s: 'block', body: statements, inline: true, span: span(token, c.peek()) };
    }
    if (c.is('var')) {
      throw unsupported('var declaration', 'var bindings are hoisted to the function and shared by its blocks; use let or const', span(token, token));
    }
    if (c.is('return')) {
      if (this.topLevel) throw unsupported('top-level return statement', 'return leaves a function, and a module has none to leave', span(token, token));
      c.next();
      // `return;` returns undefined, the unit value.
      if (c.is(';') || c.is('}') || findJavaScriptLineTerminator(this.source, token.end, c.peek().start) < c.peek().start) {
        c.eat(';');
        return { s: 'return', expr: { k: 'unit', span: span(token, token) }, span: span(token, c.peek()) };
      }
      // An async function returning a call of another adopts the Promise it
      // returns, which awaits it.
      const expr = this.inAsync ? this.awaited(() => this.sequence()) : this.sequence();
      c.eat(';');
      return { s: 'return', expr, span: span(token, c.peek()) };
    }
    if (c.is('throw')) return this.throwStatement();
    if (c.is('if')) return this.ifStatement();
    if (c.is('switch')) return this.switchStatement();
    if (c.is('{')) return { s: 'block', body: this.blockStatements(), span: span(token, c.peek()) };
    if (c.eat(';')) return { s: 'empty', span: span(token, token) };
    if (c.is('while')) return this.whileStatement();
    if (c.is('do')) return this.doStatement();
    if (c.is('for')) return this.forStatement();
    if (c.is('break') || c.is('continue')) return this.jumpStatement();
    if (['try', 'function', 'class', 'with', 'debugger'].includes(token.value) && token.kind === 'identifier') {
      throw unsupported(`${token.value} statement`, 'outside the portable core', span(token, token));
    }
    if (token.kind === 'identifier' && c.peek(1).kind === 'punct' && c.peek(1).value === ':') {
      throw unsupported(`label ${token.value}`, 'labels are outside the portable core; a break or continue applies to the innermost loop', span(token, c.peek(1)));
    }
    const statements = this.expressionStatements();
    c.eat(';');
    return statements.length === 1 ? statements[0] : {s:'block',inline:true,body:statements,span:span(token,c.peek())};
  }

  expressionStatements() {
    const c = this.cursor, statements = [];
    do {
      const start = c.peek();
      if (c.is('console') && c.is('.',1)) {
        const {expr,style,span:where} = this.consoleStatement(false);
        statements.push({s:'print',expr,style,span:where});
      } else if (this.startsReferenceAssignment()) statements.push(this.referenceAssignment());
      else if (this.startsAssignment()) statements.push(this.assignment());
      else {
        const expr = this.expr();
        statements.push({s:'expr',expr,span:span(start,c.peek())});
      }
    } while (c.eat(','));
    return statements;
  }

  constStatement(readonly = false, consumeSemicolon = true) {
    const c = this.cursor;
    const start = c.next();
    const statements = [];
    const names = new Set();
    do {
      const binding = this.constDeclarator(start, readonly);
      const entries = binding.s === 'block' && binding.inline ? binding.body : [binding];
      for (const entry of entries) {
        if (names.has(entry.name)) throw new TranslationError('syntax', `duplicate lexical binding ${entry.name}`, entry.span);
        names.add(entry.name);
      }
      statements.push(...entries);
    } while (c.eat(','));
    if (consumeSemicolon) c.eat(';');
    return statements.length === 1 ? statements[0] : { s: 'block', inline: true, body: statements, span: span(start, c.peek()) };
  }

  constDeclarator(start, readonly) {
    const c = this.cursor;
    if (c.is('{')) return this.destructuring(start);
    if (c.is('[')) {
      const bindings = readArrayBinding(this);
      if (!bindings.length) throw unsupported('empty array binding', 'the iterator and destructibility need a checked representation', span(start, c.peek()));
      c.expect('=', 'array binding');
      const value = readArrayBindingInitializer(this, bindings, () => this.expr());
      const where = span(start, c.peek());
      const name = `${this.generatedPrefix}temporary_binding${++this.generatedCount}`;
      const object = { k: 'name', path: [name], span: where };
      for (const binding of bindings) this.declareLocal(binding.name);
      return { s: 'block', inline: true, body: [{ s: 'const', name, value, span: where }, ...bindArrayElements(bindings, object, 'const', where)], span: where };
    }
    const name = c.identifier('const');
    reserved(name);
    this.scope.tdz.add(name.value);
    if (!c.eat('=')) throw this.fail('expected = in const');
    const previous = this.readonlyConstantInitializer;
    this.readonlyConstantInitializer = readonly;
    let value;
    try { value = this.expr(); } finally { this.readonlyConstantInitializer = previous; }
    const receiverFact = this.readTagReceiverFact(value);
    this.declareLocal(name.value);
    if (receiverFact) this.rememberTagBinding(name.value, receiverFact);
    return { s: 'const', name: name.value, value, span: span(start, c.peek()) };
  }

  /** `const { left, value: v } = t;` binds fields of a switch subject. */
  destructuring(start) {
    const c = this.cursor;
    c.expect('{', 'destructuring');
    const pairs = [];
    while (!c.is('}')) {
      if (c.is('...')) throw unsupported('rest property', 'name every field', span(c.peek(), c.peek()));
      const key = c.identifier('destructuring');
      const local = c.eat(':') ? c.identifier('destructuring') : key;
      reserved(local);
      if (c.is('=')) throw unsupported('default value', 'every field has a value', span(key, c.peek()));
      pairs.push({ field: key.value, name: local.value, span: span(key, local) });
      if (!c.eat(',')) break;
    }
    c.expect('}', 'destructuring');
    if (!pairs.length) throw unsupported('empty object binding', 'object destructibility needs a checked representation', span(start, c.peek()));
    c.expect('=', 'destructuring');
    const value = this.expr();
    const where = span(start, c.peek());
    const name = `${this.generatedPrefix}temporary_binding${++this.generatedCount}`;
    const object = { k: 'name', path: [name], span: where };
    const statements = pairs.map((pair) => ({
      s: 'const',
      name: pair.name,
      value: { k: 'field', object, field: pair.field, span: pair.span },
      span: span(start, c.peek()),
    }));
    for (const pair of pairs) this.declareLocal(pair.name);
    return { s: 'block', body: [{ s: 'const', name, value, span: where }, ...statements], inline: true, span: where };
  }

  /** `let a = 1n, b = a;` declares assignable locals; each needs a value, since undefined is not portable. */
  letDeclarations() {
    const c = this.cursor;
    const start = c.next();
    if (c.is('{')) throw unsupported('let destructuring', 'declare each binding with its own let', span(start, c.peek()));
    const statements = [];
    const names = new Set();
    do {
      if (c.is('[')) {
        const pattern = c.peek();
        const bindings = readArrayBinding(this);
        if (!c.eat('=')) throw unsupported('let destructuring', 'declare each binding with its own let', span(start, pattern));
        const value = readArrayBindingInitializer(this, bindings, () => this.expr());
        const items = value.k === 'array' ? value.items : [];
        const plain = bindings.every((binding, index) => binding.index === index && !binding.path
          && !binding.rest && !Object.hasOwn(binding, 'defaultValue'));
        if (!acceptJavaScriptMutableArrayBinding(value.k, items.length, bindings.length, plain,
          items.some(item => item.spread))) {
          throw unsupported('let destructuring', 'declare each binding with its own let', span(start, pattern));
        }
        const owner = this.lexicalScopes[this.lexicalScope];
        const parent = this.lexicalScopes[owner.parent];
        for (const binding of bindings) {
          if (!acceptJavaScriptMutableArrayBindingOwner('JavaScript', (owner.declarationCounts.get(binding.name) ?? 0) > 1,
            parent?.parameters.has(binding.name) === true, this.topLevel)) {
            if (this.topLevel) throw unsupported('let destructuring', 'mutable literal bindings require a function-owned lexical scope', span(start, pattern));
            throw new TranslationError('syntax', `duplicate lexical binding ${binding.name}`, binding.span);
          }
        }
        const where = span(start, c.peek());
        const name = `${this.generatedPrefix}temporary_binding${++this.generatedCount}`;
        const object = { k: 'name', path: [name], span: where };
        const declarations = bindArrayElements(bindings, object, 'let', where).map(statement => ({
          ...statement, value: { ...statement.value, provenInBounds: true },
        }));
        statements.push({ s: 'const', name, value, span: where }, ...declarations);
        for (const binding of bindings) {
          if (names.has(binding.name)) throw new TranslationError('syntax', `duplicate lexical binding ${binding.name}`, binding.span);
          names.add(binding.name);
          this.declareLocal(binding.name, true);
          this.lexicalBinding(binding.name, c.peek().start);
        }
        continue;
      }
      if (c.is('{')) throw unsupported('let destructuring', 'declare each binding with its own let', span(start, c.peek()));
      const name = c.identifier('let');
      reserved(name);
      if (names.has(name.value)) throw new TranslationError('syntax', `duplicate lexical binding ${name.value}`, span(name, name));
      names.add(name.value);
      // The binding is in its temporal dead zone in its own initialiser.
      this.scope.tdz.add(name.value);
      const value = c.eat('=') ? this.expr() : { k: 'none', absence: 'undefined', span: span(start, name) };
      this.declareLocal(name.value, true);
      this.lexicalBinding(name.value, c.peek().start);
      statements.push({ s: 'let', name: name.value, value, span: span(start, c.peek()) });
    } while (c.eat(','));
    return statements;
  }

  /** Whether the statement is `x = e`, `x op= e`, `x++`, `x--`, `++x` or `--x`. */
  startsAssignment() {
    const c = this.cursor;
    if ((c.is('++') || c.is('--')) && c.isKind('identifier', 1)) return true;
    if (!c.isKind('identifier')) return false;
    const next = c.peek(1);
    return next.kind === 'punct' && (ASSIGNMENTS.has(next.value) || next.value === '++' || next.value === '--');
  }

  startsReferenceAssignment() {
    const c = this.cursor;
    let index = c.index;
    if (c.tokens[index]?.value === '++' || c.tokens[index]?.value === '--') index += 1;
    if (c.tokens[index]?.kind !== 'identifier') return false;
    index += 1;
    let member = false;
    for (;;) {
      if (c.tokens[index]?.value === '.' && c.tokens[index + 1]?.kind === 'identifier') { member = true; index += 2; continue; }
      if (c.tokens[index]?.value === '[') { member = true; index = this.matching(index) + 1; continue; }
      break;
    }
    const operator = c.tokens[index];
    return member && (operator?.kind === 'punct' && (ASSIGNMENTS.has(operator.value) || operator.value === '++' || operator.value === '--') || c.is('++') || c.is('--'));
  }

  referenceAssignment() {
    const c = this.cursor;
    const start = c.peek();
    const prefix = c.is('++') || c.is('--') ? c.next() : null;
    const token = c.identifier('reference assignment');
    let object = this.reference(token);
    let index = null;
    let field = null;
    for (;;) {
      if (c.eat('.')) {
        const member = c.identifier('reference field');
        field = member.value;
        index = null;
      } else if (c.eat('[')) {
        index = this.sequence();
        c.expect(']', 'reference index');
        field = null;
      } else throw this.fail('expected a field or index in reference assignment');
      if (!c.is('.') && !c.is('[')) break;
      object = field === null ? { k: 'index', object, index, span: span(start, c.peek()) } : { k: 'field', object, field, span: span(start, c.peek()) };
    }
    const op = prefix ? prefix.value : c.next().value;
    if (field === null) this.indexedReferenceWrites += 1;
    const where = span(start, c.peek());
    const target = { object, ...(field === null ? { index } : { field }) };
    if (op === '++' || op === '--') return { s: 'assignReference', ...target, op, span: where };
    if (op !== '=' && !COMPOUND[op]) throw unsupported(`${op} reference assignment`, 'the operator needs a source-language read and write rule', where);
    const value = this.expr();
    return { s: 'assignReference', ...target, value, op, span: span(start, c.peek()) };
  }

  /** An assignment of a local: `x op= e` is `x = x op e`, and `x++` is `x = x + 1`. */
  assignment() {
    const c = this.cursor;
    const start = c.peek();
    const prefix = c.is('++') || c.is('--') ? c.next() : null;
    const target = c.identifier('assignment');
    const operator = prefix ?? c.next();
    const op = operator.value;
    const where = span(start, c.peek());
    this.lexicalRead(target, true, where);
    if (this.scope.tdz.has(target.value)) {
      throw unsupported(`assignment of ${target.value} before its declaration`, 'the binding is in its temporal dead zone, where assigning it throws a ReferenceError', where);
    }
    if (!this.scope.locals.has(target.value)) {
      throw unsupported(`assignment of ${target.value}`, 'only local variables declared with let, and parameters, are assignable', where);
    }
    if (!this.scope.mutable.has(target.value)) {
      throw unsupported(`assignment of constant ${target.value}`, 'assigning a const binding throws a TypeError; declare it with let', where);
    }
    const read = { k: 'name', path: [target.value], span: span(target, target) };
    if (op === '??=') {
      const expr = this.scalarAssignmentExpression(read,operator,false,()=>this.expr());
      return {s:'expr',expr,span:span(start,c.peek())};
    }
    let value;
    if (op === '++' || op === '--') {
      // One of the variable's own type: a Number or a BigInt.
      const one = { k: 'num', value: '1', unit: true, span: where };
      value = { k: 'binary', op: op === '++' ? 'add' : 'sub', left: read, right: one, span: where };
    } else if (op === '=') {
      value = this.expr();
    } else if (COMPOUND[op]) {
      const right = this.expr();
      value = { k: 'binary', op: COMPOUND[op], left: read, right, span: joined(read, right, start) };
    } else if(op==='**='){
      const right=this.expr();
      value={k:'math',op:'exponentiateJavaScriptNumericValue',name:op,args:[{value:read,spread:false},{value:right,spread:false}],span:joined(read,right,start)};
    } else if (readJavaScriptNumberBitwiseAssignment(op)) {
      const right = this.expr();
      value = {k:'math', op:readJavaScriptNumberBitwiseAssignment(op), name:op,
        args:[{value:read,spread:false},{value:right,spread:false}], span:joined(read,right,start)};
    } else {
      throw unsupported(`${op} assignment`, 'the portable compound assignments are +=, -=, *=, /=, %=, &&= and ||=', where);
    }
    return { s: 'assign', name: target.value, value, span: span(start, c.peek()) };
  }

  /** The body of a loop, where `break` and `continue` apply to it. */
  loopBody() {
    this.jumps.loops += 1;
    try {
      return this.branch();
    } finally {
      this.jumps.loops -= 1;
    }
  }

  whileStatement() {
    const c = this.cursor;
    const start = c.next();
    c.expect('(', 'while');
    const cond = this.sequence();
    c.expect(')', 'while');
    const body = this.loopBody();
    return { s: 'while', cond, body, span: span(start, c.peek()) };
  }

  doStatement() {
    const c = this.cursor;
    const start = c.next();
    const body = this.loopBody();
    c.expect('while', 'do');
    c.expect('(', 'do');
    const cond = this.sequence();
    c.expect(')', 'do');
    c.eat(';');
    return { s: 'doWhile', body, cond, span: span(start, c.peek()) };
  }

  /** `for (let i = 0n; i < n; i++) …`: the variables of its head belong to the loop. */
  forStatement() {
    return this.withLexicalScope([], () => this.forStatementInScope());
  }

  forStatementInScope() {
    const c = this.cursor;
    const start = c.next();
    if (c.is('await')) throw unsupported('for await', 'asynchronous iteration is outside the portable core', span(start, c.peek()));
    c.expect('(', 'for');
    const declared = c.is('let') || c.is('const') || c.is('var');
    if ((c.is('const') || c.is('let')) && c.is('[', 1) && c.tokens[this.matching(c.index + 1) + 1]?.value === 'of') return this.forOf(start);
    if (c.isKind('identifier', declared ? 1 : 0) && (c.is('of', declared ? 2 : 1) || c.is('in', declared ? 2 : 1))) {
      const kind = c.peek(declared ? 2 : 1).value;
      if (kind === 'in') throw unsupported('for…in loop', 'iteration over the keys of objects is outside the portable core; count with for (let i = …; …; …)', span(start, c.peek()));
      if (!declared || c.is('var')) throw unsupported('for…of without const or let', 'declare the loop variable with const or let, so each iteration has its own', span(start, c.peek()));
      return this.forOf(start);
    }
    const outer = this.scope;
    this.scope = { locals: new Set(outer.locals), tdz: new Set(outer.tdz), mutable: new Set(outer.mutable) };
    try {
      const init = [];
      if (c.is('let')) {
        init.push(...this.letDeclarations());
      } else if (c.is('const')) {
        const binding = this.constStatement(false,false);
        init.push(...(binding.s === 'block' && binding.inline ? binding.body : [binding]));
      } else if (c.is('var')) {
        throw unsupported(`for with ${c.peek().value}`, 'declare the loop variables with let', span(c.peek(), c.peek()));
      } else {
        if (!c.is(';')) init.push(...this.expressionStatements());
      }
      c.expect(';', 'for');
      const cond = c.is(';') ? null : this.sequence();
      c.expect(';', 'for');
      const update = [];
      if (!c.is(')')) update.push(...this.expressionStatements());
      c.expect(')', 'for');
      const body = this.loopBody();
      return { s: 'for', init, cond, update, body, span: span(start, c.peek()) };
    } finally {
      this.scope = outer;
    }
  }

  /**
   * `for (const x of xs) body` reads the elements of an array in order: it
   * is a loop over the indices of the array, whose body first binds `x` to
   * the element at the index. The array is evaluated once, before the loop.
   */
  forOf(start) {
    const c = this.cursor;
    const binding = c.next().value;
    const bindings = c.is('[') ? readArrayBinding(this) : null;
    const name = bindings ? null : c.identifier('for…of');
    if (name) reserved(name);
    c.expect('of', 'for…of');
    const value = bindings ? readArrayBindingInitializer(this, bindings, () => this.expr()) : this.expr();
    const iterable = { k: 'forOfIterable', object: value, span: value.span };
    c.expect(')', 'for');
    this.generatedCount += 1;
    const values = `${this.generatedPrefix}temporary_values${this.generatedCount}`;
    const index = `${this.generatedPrefix}temporary_index${this.generatedCount}`;
    const outer = this.scope;
    this.scope = { locals: new Set(outer.locals), tdz: new Set(outer.tdz), mutable: new Set(outer.mutable) };
    let body;
    try {
      for (const local of bindings ?? [{ name: name.value }]) {
        this.declareLocal(local.name, binding === 'let');
        this.lexicalBinding(local.name, c.peek().start);
      }
      body = this.loopBody();
    } finally {
      this.scope = outer;
    }
    const where = span(start, c.peek());
    const read = (local) => ({ k: 'name', path: [local], span: where });
    const element = { k: 'index', object: read(values), index: read(index), provenInBounds: true, span: where };
    const declarations = bindings ? [{ s: 'const', name: `${this.generatedPrefix}temporary_element${this.generatedCount}`, value: element, span: where }, ...bindArrayElements(bindings, read(`${this.generatedPrefix}temporary_element${this.generatedCount}`), binding, where)] : [{ s: binding, name: name.value, value: element, span: where }];
    const sourceBlock = {
      s: 'block',
      body: [
        { s: 'const', name: values, value: iterable, span: where },
        {
          s: 'for',
          init: [{ s: 'let', name: index, value: { k: 'num', value: '0', span: where }, span: where }],
          cond: { k: 'binary', op: 'lt', left: read(index), right: { k: 'length', object: read(values), integer: true, span: where }, span: where },
          update: [{ s: 'assign', name: index, value: { k: 'binary', op: 'add', left: read(index), right: { k: 'num', value: '1', span: where }, span: where }, span: where }],
          body: [...declarations, { s: 'block', body, span: where }],
          span: where,
        },
      ],
      span: where,
    };
    this.ownedArrayIterationBlocks.set(sourceBlock, {binding: sourceBlock.body[0], loop: sourceBlock.body[1], value: iterable, span: where});
    return sourceBlock;
  }

  jumpStatement() {
    const c = this.cursor;
    const token = c.next();
    const where = span(token, token);
    if (c.isKind('identifier') && !c.is('case') && !c.is('default') && findJavaScriptLineTerminator(this.source, token.end, c.peek().start) === c.peek().start) {
      throw unsupported(`labelled ${token.value}`, 'labels are outside the portable core; a break or continue applies to the innermost loop', span(token, c.peek()));
    }
    if (token.value === 'continue' && !this.jumps.loops) throw new TranslationError('syntax', 'continue outside a loop', where);
    if (token.value === 'break' && !this.jumps.loops && !this.jumps.switches) throw new TranslationError('syntax', 'break outside a loop or switch', where);
    c.eat(';');
    return { s: token.value, span: where };
  }

  throwStatement() {
    const c = this.cursor;
    const start = c.next();
    if (findJavaScriptLineTerminator(this.source, start.end, c.peek().start) < c.peek().start) {
      throw new TranslationError('syntax', 'line terminator after throw', span(start, start));
    }
    const constructed=c.eat('new');
    if (!constructed&&!(c.peek().kind==='identifier'&&readJavaScriptExceptionConstructorOperation(c.peek().value)&&c.is('(',1))) throw unsupported('throw of a non-error value', 'throw new Error(\'…\'), which aborts the program', span(start, c.peek()));
    const kind = c.identifier('throw');
    this.lexicalRead(kind);
    const operation=readJavaScriptExceptionConstructorOperation(kind.value);
    if (!operation) throw unsupported(`throw new ${kind.value}`, 'the error constructor needs its actual exception definition', span(kind, kind));
    if (!this.acceptStandardBuiltinOwner(kind.value)) throw unsupported('shadowed standard error owner',
      'the actual lexical/module/context error constructor must be the standard unshadowed intrinsic', kind);
    const args=this.spreadArguments('throw');
    c.eat(';');
    const where=span(start,c.peek());
    if(kind.value==='Error'&&args.length===1&&!args[0].spread&&args[0].value.k==='str')return{s:'throw',message:args[0].value.value,span:where};
    return{s:'throw',message:'',exception:{k:'math',op:operation,name:kind.value,args,span:where},span:where};
  }

  ifStatement() {
    const c = this.cursor;
    const start = c.next();
    c.expect('(', 'if');
    const cond = this.sequence();
    c.expect(')', 'if');
    const then = this.branch();
    const otherwise = c.eat('else') ? this.branch() : null;
    return { s: 'if', cond, then, else: otherwise, span: span(start, c.peek()) };
  }

  branch() {
    const c = this.cursor;
    if (c.is('{')) return this.blockStatements();
    if (c.is('const') || c.is('let') || c.is('function')) throw this.fail('expected a statement');
    return [this.statement()];
  }

  /**
   * `switch (x.$) { case 'tag': … }` matches the constructors of a data
   * type; `switch (n) { case 0n: … default: … }` matches literals.
   */
  switchStatement() {
    const c = this.cursor;
    const start = c.next();
    c.expect('(', 'switch');
    const discriminant = this.sequence();
    c.expect(')', 'switch');
    c.expect('{', 'switch');
    const tagged = discriminant.k === 'field' && discriminant.field === '$';
    if (tagged && !(discriminant.object.k === 'name' && discriminant.object.path.length === 1)) {
      throw unsupported('switch on the tag of an expression', 'bind the value with const and switch on its tag', discriminant.span);
    }
    const clauses = [];
    let tests = [];
    while (!c.is('}')) {
      const clauseStart = c.peek();
      if (c.eat('default')) {
        tests.push({ k: 'default', span: span(clauseStart, clauseStart) });
      } else {
        c.expect('case', 'switch');
        tests.push(this.caseTest(tagged));
      }
      c.expect(':', 'case');
      if (c.is('case') || c.is('default')) continue;
      const body = [];
      this.jumps.switches += 1;
      while (!c.is('case') && !c.is('default') && !c.is('}')) {
        if (c.is('const') || c.is('let')) {
          throw unsupported('declaration in a case clause', 'case clauses share one scope; wrap the case body in braces', span(c.peek(), c.peek()));
        }
        body.push(this.statement());
      }
      this.jumps.switches -= 1;
      clauses.push({ tests, body, span: span(clauseStart, c.peek()) });
      tests = [];
    }
    if (tests.length) clauses.push({ tests, body: [], span: span(start, c.peek()) });
    c.expect('}', 'switch');
    const node = { s: 'switch', discriminant, tagged, clauses, span: span(start, c.peek()) };
    if (tagged) node.data = this.switchData(node);
    return node;
  }

  caseTest(tagged) {
    const c = this.cursor;
    const token = c.peek();
    if (tagged) {
      if (!c.isKind('string')) throw unsupported('case test', 'the cases of a tag switch are string literal tags', span(token, token));
      this.requireScalarStringToken(token,'datatype switch tag');
      c.next();
      return { k: 'tag', tag: token.value, span: span(token, token) };
    }
    const literal = c.is(':',c.is('-') ? 2 : 1);
    if (!literal) return {k:'value',value:this.sequence(),span:span(token,c.peek())};
    const negative = Boolean(c.eat('-'));
    const value = c.peek();
    const form = readJavaScriptValueCaseForm(value.kind, value.value, value.suffix ?? '', negative);
    if (value.kind === 'number') {
      c.next();
      if (form === 'value') {
        return { k: 'value', value: { ...this.numberLiteral(value), ...(negative && { negative: true }) }, span: span(token, value) };
      }
      return { k: 'numLit', value: value.value, ...(negative && { negative }), span: span(token, value) };
    }
    if (form === 'boolLit') {
      c.next();
      return { k: 'boolLit', value: token.value === 'true', span: span(token, token) };
    }
    if (form === 'value') return { k: 'value', value: this.sequence(), span: span(token, c.peek()) };
    throw unsupported('case test', 'the cases of a value switch are BigInt or boolean literals', span(token, value));
  }

  /** The @typedef whose tags a tag switch names. */
  switchData(node) {
    const tags = node.clauses.flatMap((clause) => clause.tests.filter((test) => test.k === 'tag').map((test) => test.tag));
    const candidates = [...this.dataTypes].filter(([, ctors]) => tags.every((tag) => ctors.some((ctor) => ctor.name === tag)));
    if (candidates.length !== 1) {
      throw typeError(candidates.length ? `tags ${tags.join(', ')} are ambiguous between data types` : `no @typedef has tags ${tags.join(', ')}`, node.span);
    }
    const [name, ctors] = candidates[0];
    return { name, ctors };
  }

  /** Whether the next top-level statement declares or assigns variables, or is a block, if or loop. */
  startsTopLevelImperative() {
    const c = this.cursor;
    if (c.is('const')) return false;
    if (['let', 'var', 'if', 'for', 'while', 'do'].some((word) => c.is(word)) && c.peek().kind === 'identifier') return true;
    return c.is('{') || this.startsAssignment() || this.startsReferenceAssignment();
  }

  /** A top-level statement read as a function body's, where `return` has no function to leave. */
  topLevelStatement() {
    this.topLevel = true;
    try {
      return this.statement();
    } finally {
      this.topLevel = false;
    }
  }

  /**
   * Top-level statements that declare or assign variables are one
   * expression, whose value carries the top-level variables they declare
   * or assign; main binds each of them for the statements after it.
   */
  lowerTopLevel(statements, effects) {
    const variables = effects.filter((effect) => effect.k === 'let' && !effect.name.startsWith(this.generatedPrefix)).map((effect) => effect.name);
    const where = { start: statements[0].span.start, end: statements.at(-1).span.end };
    return lowerTopLevel(this, statements, variables, where);
  }

  mainStatement() {
    const c = this.cursor;
    const token = c.peek();
    if (c.is('const')) {
      if (c.is('{', 1)) throw unsupported('top-level destructuring', 'bind each value with const', span(token, c.peek(1)));
      const binding = this.constStatement(this.readonlyModuleConstants);
      if (binding.s === 'block' && binding.inline) return binding.body.map(entry => {
        if (isLiteralBinding(entry.value)) this.literalBindings.set(entry.name, entry.value);
        return { k: 'let', name: entry.name, value: entry.value, constant: true, span: entry.span };
      });
      if (isLiteralBinding(binding.value)) this.literalBindings.set(binding.name, binding.value);
      return { k: 'let', name: binding.name, value: binding.value, constant: true, span: binding.span };
    }
    if (c.is('console') && c.is('.', 1)) return this.consoleStatement();
    if (this.assertion && c.is(this.assertion.name)) return this.assertStatement();
    if (token.kind === 'identifier' && ['switch', 'try', 'throw', 'class', 'return'].includes(token.value)) {
      throw unsupported(`top-level ${token.value} statement`, 'the top level prints with console.log, binds with const or let, assigns, branches with if, loops and asserts', span(token, token));
    }
    // A statement that discards its value still runs it, for the lines it prints and the aborts it may reach.
    const expr = this.expr();
    c.eat(';');
    this.generatedCount += 1;
    return { k: 'let', name: `${this.generatedPrefix}temporary_main_ignored${this.generatedCount}`, value: expr, span: span(token, c.peek()) };
  }

  consoleStatement(consumeSemicolon = true) {
    const c = this.cursor;
    const start = c.next();
    this.lexicalRead(start);
    if (!this.acceptStandardBuiltinOwner(start.value)) throw unsupported('shadowed standard console owner',
      'a local, module or supplied context console binding cannot be replaced with standard output', start);
    c.expect('.', 'console');
    const method = c.identifier('console');
    if (method.value !== 'log') {
      throw unsupported(`console.${method.value}`, 'console.log, which writes a line to standard output, is the portable output', span(start, method));
    }
    const args = this.arguments('console.log');
    if (consumeSemicolon) c.eat(';');
    const where = span(start, c.peek());
    const expr = args.length > 1 ? consoleFormat(args) : (args[0] ?? { k: 'str', value: '', span: where });
    return { k: 'print', expr, style: 'js-console', span: where };
  }

  assertStatement() {
    const c = this.cursor;
    const start = c.next();
    let method = null;
    if (c.eat('.')) method = c.identifier('assertion').value;
    const args = this.arguments('assertion');
    c.eat(';');
    const where = span(start, c.peek());
    const name = method ? `${this.assertion.name}.${method}` : this.assertion.name;
    if (method === null || method === 'ok') {
      if (args.length !== 1) throw unsupported(`${name} message`, 'custom assertion messages are not kept', where);
      return { k: 'assert', prop: { ...propOf(args[0]), span: where }, span: where };
    }
    const kind = ASSERTIONS[method] ?? (this.assertion.strict ? STRICT_ASSERTIONS[method] : undefined);
    if (!kind) throw unsupported(name, 'the portable assertions are assert, ok, strictEqual, notStrictEqual and deepStrictEqual', where);
    if (args.length !== 2) throw unsupported(`${name} message`, 'custom assertion messages are not kept', where);
    const [left, right] = args;
    // These assertions compare primitives with Object.is (SameValue): NaN equals NaN, and 0 differs from -0.
    if (kind === 'eq' || kind === 'ne') return { k: 'assert', prop: { p: kind, left, right, reference: true, sameValue: true, span: where }, span: where };
    return { k: 'assert', prop: { p: kind === 'deep' ? 'eq' : 'ne', left, right, sameValue: true, span: where }, span: where };
  }

  arguments(context) {
    const c = this.cursor;
    c.expect('(', context);
    const args = [];
    while (!c.is(')')) {
      if (c.is('...')) throw unsupported('spread argument', 'pass each argument', span(c.peek(), c.peek()));
      args.push(this.expr());
      if (!c.eat(',')) break;
    }
    c.expect(')', context);
    return args;
  }

  /** Arguments that may spread arrays, `(a, ...xs)`, as array items. */
  spreadArguments(context) {
    const c = this.cursor;
    c.expect('(', context);
    const args = [];
    while (!c.is(')')) {
      const spread = c.is('...');
      if (spread) c.next();
      args.push({ spread, value: this.expr() });
      if (!c.eat(',')) break;
    }
    c.expect(')', context);
    return args;
  }

  /**
   * Statements to one expression: `const` is `let`, `if` with a returning
   * branch selects between the branch and the rest, and every path must end
   * in `return` or `throw`.
   */
  lower(statements, where) {
    const list = statements.flatMap((statement) => (statement.s === 'block' && statement.inline ? statement.body : [statement]))
      .filter((statement) => statement.s !== 'empty');
    const unreachable = (index) => {
      if (index < list.length - 1) {
        throw unsupported('unreachable statement', 'statements after return, throw or a complete if are never executed', list[index + 1].span);
      }
    };
    const at = (index) => {
      // A function that finishes without returning returns undefined, the unit value.
      if (index === list.length) return { k: 'unit', span: where };
      const statement = list[index];
      const guards = this.tagGuardSequence(list, index);
      if (guards) return this.lowerSwitch(guards, null, () => {});
      switch (statement.s) {
        case 'const':
          return { k: 'let', name: statement.name, value: statement.value, body: at(index + 1), span: statement.span };
        case 'return':
          unreachable(index);
          return statement.expr;
        case 'throw':
          unreachable(index);
          return statement.exception??{ k: 'abort', message: statement.message, span: statement.span };
        case 'block':
          if (!this.terminates(statement.body)) throw unsupported('block without a result', 'the block must end in return or throw', statement.span);
          unreachable(index);
          return this.lower(statement.body, statement.span);
        case 'if': {
          if (!this.terminates(statement.then) || (statement.else && !this.terminates(statement.else))) {
            throw unsupported('if branch without a result', 'every branch must end in return or throw; statements without effects are outside the portable core', statement.span);
          }
          const then = this.lower(statement.then, statement.span);
          const test = this.tagCondition(statement.cond);
          if (test) {
            if (statement.else) unreachable(index);
            const otherwise = statement.else ? this.lower(statement.else, statement.span) : at(index + 1);
            return this.lowerTagIf(test, then, otherwise, statement.span);
          }
          if (statement.else) {
            unreachable(index);
            return { k: 'if', cond: statement.cond, then, else: this.lower(statement.else, statement.span), span: statement.span };
          }
          return { k: 'if', cond: statement.cond, then, else: at(index + 1), span: statement.span };
        }
        case 'switch':
          return this.lowerSwitch(statement, index === list.length - 1 ? null : () => at(index + 1), () => unreachable(index));
        case 'expr':
          throw unsupported('expression statement', 'statements with effects are outside the portable core in function bodies', statement.span);
        default:
          throw new TranslationError('syntax', `unknown statement ${statement.s}`, statement.span);
      }
    };
    return at(0);
  }

  terminates(statements) {
    const list = statements.filter((statement) => statement.s !== 'empty' && !(statement.s === 'block' && statement.inline));
    const last = list.at(-1);
    if (!last) return false;
    if (last.s === 'return' || last.s === 'throw') return true;
    if (last.s === 'block') return this.terminates(last.body);
    if (last.s === 'if') return Boolean(last.else) && this.terminates(last.then) && this.terminates(last.else);
    if (last.s === 'switch') return this.switchTerminates(last);
    return false;
  }

  switchTerminates(node) {
    if (!node.clauses.every((clause) => this.terminates(clause.body))) return false;
    if (node.clauses.some((clause) => clause.tests.some((test) => test.k === 'default'))) return true;
    if (!node.tagged) return false;
    const tags = new Set(node.clauses.flatMap((clause) => clause.tests.map((test) => test.tag)));
    return node.data.ctors.every((ctor) => tags.has(ctor.name));
  }

  /**
   * A switch is a match. In a tag switch, `x.field` of the subject in a case
   * is a pattern variable of that case's constructor; the variables get
   * names that nothing in the case body uses, so no reference is captured.
   */
  lowerSwitch(node, rest, unreachable) {
    for (const clause of node.clauses) {
      if (!this.terminates(clause.body)) {
        throw unsupported('case without a result', 'every case must end in return or throw; falling through is outside the portable core', clause.span);
      }
    }
    if (acceptJavaScriptValueSwitch(node.tagged, node.clauses.some((clause) => clause.tests.some((test) => test.k === 'value')))) {
      const fallback = node.clauses.find((clause) => clause.tests.some((test) => test.k === 'default'));
      const where = node.span;
      const clauses = node.clauses.flatMap((clause) => clause.tests.filter((test) => test.k !== 'default').map((test) => ({ test, clause })));
      let name, read, body, cond, then;
      for (let step = 0; ; step += 1) {
        const instruction = readJavaScriptValueSwitchInstruction(false, step, clauses.length);
        if (instruction === 'capture') {
          name = `${this.generatedPrefix}temporary_switch${++this.generatedCount}`;
          read = { k: 'name', path: [name], span: where };
        } else if (instruction === 'fallback') {
          body = fallback ? this.lower(fallback.body, fallback.span) : rest ? rest() : { k: 'unit', span: where };
        } else if (instruction === 'reachability') {
          if (this.switchTerminates(node)) unreachable();
        } else if (instruction === 'bind') {
          return { k: 'let', name, value: node.discriminant, body, span: where };
        } else {
          const { test, clause } = clauses[readJavaScriptValueSwitchPosition(false, step, clauses.length)];
          if (instruction === 'test') {
            const right = test.k === 'value' ? test.value : test.k === 'boolLit' ? { k: 'bool', value: test.value, span: test.span } : { k: 'num', value: test.value, negative: test.negative, span: test.span };
            cond = { k: 'binary', op: 'eq', left: read, right, span: test.span };
          } else if (instruction === 'body') {
            then = this.lower(clause.body, clause.span);
          } else if (instruction === 'select') {
            body = { k: 'if', cond, then, else: body, span: clause.span };
          }
        }
      }
    }
    const rows = [];
    // `default` applies only when no case matches, wherever it stands.
    let defaults = [];
    let hasDefault = false;
    for (const clause of node.clauses) {
      for (const test of clause.tests) {
        const body = this.lower(clause.body, clause.span);
        if (test.k === 'default') {
          hasDefault = true;
          defaults = node.tagged ? this.defaultRows(node, body, clause.span) : [{ patterns: [{ k: 'wild', span: test.span }], body, span: clause.span }];
        } else if (test.k === 'tag') {
          rows.push(this.tagRow(node, test, body, clause.span));
        } else {
          rows.push({ patterns: [{ k: test.k, value: test.value, ...(test.negative && { negative: true }), span: test.span }], body, span: clause.span });
        }
      }
    }
    rows.push(...defaults);
    if (this.switchTerminates(node)) {
      unreachable();
    } else if (rest && !hasDefault) {
      rows.push({ patterns: [{ k: 'wild', span: node.span }], body: rest(), span: node.span });
    } else if (hasDefault) {
      unreachable();
    }
    const scrutinee = node.tagged ? node.discriminant.object : node.discriminant;
    return { k: 'match', scrutinees: [scrutinee], rows, span: node.span };
  }

  /**
   * The `default` of a tag switch covers the remaining alternatives; when it
   * reads fields of the subject, it is one row per remaining alternative.
   */
  defaultRows(node, body, where) {
    const subject = node.discriminant.object.path[0];
    const covered = new Set(node.clauses.flatMap((clause) => clause.tests.filter((test) => test.k === 'tag').map((test) => test.tag)));
    if (!readsFields(body, subject)) return [{ patterns: [{ k: 'wild', span: where }], body, span: where }];
    return node.data.ctors.filter((ctor) => !covered.has(ctor.name)).map((ctor) => this.tagRow(node, { k: 'tag', tag: ctor.name, span: where }, body, where));
  }

  /** Consecutive returning tag guards are one existing exhaustive switch.
   * Keeping their common default together lets its fields use every preceding
   * exclusion before any constructor row is projected. No condition, binding,
   * assignment or effect between guards is skipped.
   */
  tagGuardSequence(statements, index) {
    const first = statements[index];
    const subject = first?.s === 'if' ? this.tagCondition(first.cond) : null;
    if (!subject) return null;
    const clauses = [], covered = new Set();
    let next = index;
    while (next < statements.length) {
      const statement = statements[next];
      const test = statement.s === 'if' ? this.tagCondition(statement.cond) : null;
      if (!test) break;
      const sameBinding = test.object.path[0] === subject.object.path[0]
        && test.data.name === subject.data.name && test.data.ctors === subject.data.ctors;
      if (!acceptJavaScriptTerminalTagGuard(!test.negated, Boolean(statement.else),
        this.terminates(statement.then), sameBinding, covered.has(test.tag))) break;
      covered.add(test.tag);
      clauses.push({ tests: [{ k: 'tag', tag: test.tag, span: statement.cond.span }],
        body: statement.then, span: statement.span });
      next += 1;
    }
    const remainder = statements.slice(next);
    if (!acceptJavaScriptTagGuardSequence(clauses.length > 1, this.terminates(remainder))) return null;
    const last = statements.at(-1);
    const span = { start: first.span.start, end: last.span.end };
    const tail = { start: remainder[0].span.start, end: last.span.end };
    clauses.push({ tests: [{ k: 'default', span: tail }], body: remainder, span: tail });
    return { s: 'switch', tagged: true, data: subject.data,
      discriminant: { k: 'field', object: subject.object, field: '$', span: first.cond.span }, clauses, span };
  }

  /** `x.$ === 'tag'` on a local `x`: the data type and the tag, when it narrows `x`. */
  tagCondition(cond) {
    const test = cond.tagTest;
    if (!test || test.object.k !== 'name' || test.object.path.length !== 1) return null;
    return test;
  }

  /** `if (x.$ === 'tag') A else B` is a match in which `A` reads the fields of `tag`. */
  lowerTagIf(test, then, otherwise, where) {
    const node = { discriminant: { k: 'field', object: test.object, field: '$' }, data: test.data, clauses: [{ tests: [{ k: 'tag', tag: test.tag }] }] };
    const [matching, other] = test.negated ? [otherwise, then] : [then, otherwise];
    const rows = [this.tagRow(node, { k: 'tag', tag: test.tag, span: where }, matching, where), ...this.defaultRows(node, other, where)];
    return { k: 'match', scrutinees: [test.object], rows, span: where };
  }

  tagRow(node, test, caseBody, where) {
    let body = caseBody;
    const subject = node.discriminant.object.path[0];
    const ctor = node.data.ctors.find((candidate) => candidate.name === test.tag);
    if (!ctor) throw typeError(`${node.data.name} has no tag ${test.tag}`, test.span);
    const binders = new Map();
    // `const { left, value } = t;` at the start of the case names the pattern variables itself.
    let rest = body;
    const direct = [];
    while (rest.k === 'let' && rest.value.k === 'field' && rest.value.object.k === 'name' && rest.value.object.path.length === 1
      && rest.value.object.path[0] === subject && ctor.fields.some((field) => field.name === rest.value.field)
      && !binders.has(rest.value.field) && rest.name !== subject && !direct.includes(rest.name)) {
      binders.set(rest.value.field, rest.name);
      direct.push(rest.name);
      rest = rest.body;
    }
    const rebound = new Set();
    collectBinders(rest, rebound);
    if (direct.some((name) => rebound.has(name))) {
      binders.clear();
      rest = body;
    }
    body = rest;
    const used = new Set();
    collectNames(body, used);
    for (const name of binders.values()) used.add(name);
    const binderFor = (field) => {
      if (!ctor.fields.some((candidate) => candidate.name === field)) {
        throw typeError(`the ${test.tag} alternative of ${node.data.name} has no field ${field}`, where);
      }
      if (!binders.has(field)) {
        let name = used.has(field) || field === subject ? `${subject}_${field}` : field;
        for (let index = 2; used.has(name); index += 1) name = `${subject}_${field}${index}`;
        used.add(name);
        binders.set(field, name);
      }
      return binders.get(field);
    };
    const substituted = substituteFields(body, subject, binderFor);
    const args = ctor.fields.map((field) => (binders.has(field.name)
      ? { k: 'bindOrCtor', name: binders.get(field.name), span: test.span }
      : { k: 'wild', span: test.span }));
    return { patterns: [{ k: 'ctor', path: [ROOT, node.data.name, test.tag], args, span: test.span }], body: substituted, span: where };
  }

  /** Expressions, by JavaScript precedence. */
  sequence() {
    const c = this.cursor;
    let value = this.expr();
    while (c.is(',')) {
      const comma = c.next(), right = this.expr();
      const name = this.generatedPrefix+'temporary_sequence'+(++this.generatedCount);
      value = constructJavaScriptSequenceExpression(value,right,name,joined(value,right,comma));
    }
    return value;
  }

  /** AssignmentExpression: list separators belong to the caller. */
  expr() {
    const c = this.cursor;
    const start = c.peek();
    if (this.startsFunction(c.index)) return this.closure();
    let value = this.or();
    if (c.eat('?')) {
      const then = this.expr();
      c.expect(':', 'conditional expression');
      const otherwise = this.expr();
      value = {k:'if',cond:value,then,else:otherwise,span:joined(value,otherwise,start)};
    }
    if (ASSIGNMENTS.has(c.peek().value) && c.peek().kind === 'punct') {
      const operator = c.next();
      return this.scalarAssignmentExpression(value,operator,false,()=>this.expr());
    }
    return value;
  }

  scalarAssignmentExpression(target, operator, postfix, readRight) {
    const c = this.cursor, name = target.k === 'name' && target.path.length === 1 ? target.path[0] : null;
    const where = {start:Math.min(target.span?.start ?? operator.start,operator.start),end:Math.max(target.span?.end ?? operator.end,operator.end)};
    if (!name) throw unsupported('assignment expression target','this target needs its actual reference and property state operation',where);
    const token = c.tokens.find(token=>token.kind === 'identifier' && token.value === name && token.start >= (target.span?.start ?? 0) && token.end <= (target.span?.end ?? operator.start));
    if (!token) throw unsupported('assignment expression source','the target needs its actual identifier token',where);
    this.lexicalRead(token,true,where);
    if (this.scope.tdz.has(name)) throw unsupported('assignment of '+name+' before its declaration','the binding is in its temporal dead zone, where assigning it throws a ReferenceError',where);
    if (!this.scope.locals.has(name)) throw unsupported('assignment of '+name,'only local variables declared with let, and parameters, are assignable',where);
    if (!this.scope.mutable.has(name)) throw unsupported('assignment of constant '+name,'assigning a const binding throws a TypeError; declare it with let',where);
    if (!this.referenceSemanticsEnabled || !readScalarAssignmentOperation(operator.value)) throw unsupported('scalar assignment expression','the selected source profile needs the complete shared cell operation',where);
    this.referenceSemantics = true;this.referenceSemanticsForced = true;
    const right = readRight ? readRight() : null;
    where.end = Math.max(where.end,right?.span?.end ?? 0);
    const ordinal = ++this.generatedCount;
    const names = [this.generatedPrefix+'temporary_assignment_previous'+ordinal,this.generatedPrefix+'temporary_assignment_updated'+ordinal];
    return constructScalarAssignmentExpression({k:'name',path:[name],span:span(token,token)},right,operator.value,postfix,names,where);
  }

  or() {
    const c = this.cursor;
    let left = this.and();
    let coalesced = false;
    for (;;) {
      const token = c.peek();
      if (c.eat('??')) {
        const leftError = readOptionalLogicalMixingError('left', coalesced, Boolean(left.parenthesized), left.k, left.op ?? '');
        if (leftError) throw new TranslationError('syntax', leftError, token);
        const right = this.and();
        const rightError = readOptionalLogicalMixingError('right', coalesced, Boolean(right.parenthesized), right.k, right.op ?? '');
        if (rightError) throw new TranslationError('syntax', rightError, token);
        left = { k: 'coalesce', left, right, span: joined(left, right, token) };
        coalesced = true;
        continue;
      }
      if (!c.eat('||')) return left;
      const logicalError = readOptionalLogicalMixingError('or', coalesced, false, '', '');
      if (logicalError) throw new TranslationError('syntax', logicalError, token);
      const right = this.and();
      left = { k: 'binary', op: 'or', left, right, span: joined(left, right, token) };
    }
  }

  and() {
    const c = this.cursor;
    let left = this.bitwise();
    while (c.is('&&')) {
      const token = c.next();
      const right = this.bitwise();
      left = { k: 'binary', op: 'and', left, right, span: joined(left, right, token) };
    }
    return left;
  }

  bitwise() {
    return this.readBitwisePrecedence(0);
  }

  readBitwisePrecedence(level) {
    const c=this.cursor,next=()=>level<2?this.readBitwisePrecedence(level+1):this.equality();
    let left=next();
    while(c.peek().kind==='punct'&&acceptJavaScriptNumberBitwisePrecedence(c.peek().value,level)){
      const token=c.next(),right=next();
      left={k:'math',op:readJavaScriptNumberBitwiseOperation(token.value),name:token.value,args:[{value:left,spread:false},{value:right,spread:false}],span:joined(left,right,token)};
    }
    return left;
  }

  equality() {
    const c = this.cursor;
    let left = this.relational();
    for (;;) {
      const token = c.peek();
      const loose=readJavaScriptLooseEqualityOperation(token.kind==='punct'?token.value:'');
      if(loose){c.next();const right=this.relational();left={k:'math',op:loose,name:token.value,args:[{value:left,spread:false},{value:right,spread:false}],span:joined(left,right,token)};continue;}
      const op = token.kind === 'punct' ? EQUALITY[token.value] : undefined;
      if (!op) return left;
      c.next();
      const right = this.relational();
      left = this.tagTest(op, left, right, joined(left, right, token)) ?? { k: 'binary', op, left, right, span: joined(left, right, token) };
    }
  }

  /** Retain the actual source header; parse its annotation only if an
   * ambiguous tag test actually needs the result context. Duplicate source
   * declarations/imports supply no unique owner. No unrelated diagnostic is
   * moved into this ordinary declaration census.
   */
  rememberTagFunction(name, token, asynchronous) {
    if (this.tagFunctionResults.has(name)) this.tagFunctionResults.set(name, null);
    else this.tagFunctionResults.set(name, { token, asynchronous });
  }

  rememberTagBinding(name, fact) {
    const owner = this.lexicalScopes[this.lexicalScope];
    let bindings = this.tagBindingTypes.get(owner);
    if (!bindings) { bindings = new Map(); this.tagBindingTypes.set(owner, bindings); }
    bindings.set(name, fact);
  }

  /** Only declared parameters, actual named call results, and immutable
   * aliases of those facts contribute context. A nearer lexical declaration
   * stops lookup even when it has no type fact. Closure/function boundaries
   * never borrow a same-spelled outer parameter's context.
   */
  readTagReceiverFact(node) {
    const target = node.k === 'app' ? node.fn : node;
    const path = target.path ?? [];
    const form = readJavaScriptTagReceiverForm(node.k, node.k === 'app' ? target.k : '', path, ROOT);
    if (form === 1) return this.tagFunctionResults.get(path[1]) ?? null;
    if (form !== 2) return null;
    for (let index = this.lexicalScope; index >= 0; index = this.lexicalScopes[index].parent) {
      const scope = this.lexicalScopes[index];
      const step = readJavaScriptTagBindingStep(scope.bindings.has(path[0]), scope.boundary);
      if (step === 1) return this.tagBindingTypes.get(scope)?.get(path[0]) ?? null;
      if (step === 2) break;
    }
    return null;
  }

  readTagReceiverOwner(node) {
    const fact = this.readTagReceiverFact(node);
    if (!fact) return '';
    let type = fact.type;
    if (acceptJavaScriptTagHeaderFact(Boolean(type), Boolean(fact.token), Boolean(fact.asynchronous))) {
      // This is speculative context, not ordinary declaration validation.
      // A later malformed header must not replace the current tag refusal.
      try {
        const doc = this.jsdocFor(fact.token);
        if (doc?.returns) type = this.type(doc.returns, doc.range);
      } catch (error) {
        if (error instanceof TranslationError) return '';
        throw error;
      }
    }
    return readJavaScriptTagDeclaredOwner(type?.kind ?? '', type?.path ?? [], type?.name ?? '', ROOT);
  }

  /** `x.$ === 'tag'` tests the alternative of a data value. */
  tagTest(op, left, right, where) {
    const [field, tag] = left.k === 'field' && left.field === '$' ? [left, right] : [right, left];
    if (field.k !== 'field' || field.field !== '$') return null;
    if (tag.k !== 'str') throw unsupported('computed tag test', 'compare the $ tag with a string literal', where);
    const candidates = [...this.dataTypes].filter(([, ctors]) => ctors.some((ctor) => ctor.name === tag.value));
    const receiver = candidates.length > 1 ? this.readTagReceiverOwner(field.object) : '';
    const position = readJavaScriptTagOwnerPosition(candidates.map(([name]) => name), receiver);
    if (position < 0) throw typeError(candidates.length ? `tag ${tag.value} is ambiguous between data types` : `no @typedef has tag ${tag.value}`, where);
    const [name, ctors] = candidates[position];
    const ctor = ctors.find((candidate) => candidate.name === tag.value);
    const pattern = { k: 'ctor', path: [ROOT, name, tag.value], args: ctor.fields.map(() => ({ k: 'wild', span: where })), span: where };
    return {
      k: 'match',
      scrutinees: [field.object],
      rows: [
        { patterns: [pattern], body: { k: 'bool', value: op === 'eq', span: where }, span: where },
        { patterns: [{ k: 'wild', span: where }], body: { k: 'bool', value: op !== 'eq', span: where }, span: where },
      ],
      tagTest: { object: field.object, tag: tag.value, data: { name, ctors }, negated: op === 'ne' },
      span: where,
    };
  }

  relational() {
    const c = this.cursor;
    let left = this.shift();
    for (;;) {
      const token = c.peek();
      if (c.is('in') || c.is('instanceof')) throw unsupported(`${token.value} operator`, 'objects are outside the portable core', span(token, token));
      const op = token.kind === 'punct' ? RELATIONAL[token.value] : undefined;
      if (!op) return left;
      c.next();
      const right = this.shift();
      left = { k: 'binary', op, left, right, span: joined(left, right, token) };
    }
  }

  shift() {
    const c = this.cursor;
    let left = this.additive();
    while(c.is('<<')||c.is('>>')||c.is('>>>')){
      const token=c.next(),right=this.additive();
      left={k:'math',op:readJavaScriptNumberBitwiseOperation(token.value),name:token.value,args:[{value:left,spread:false},{value:right,spread:false}],span:joined(left,right,token)};
    }
    return left;
  }

  /** `+` adds numbers and concatenates strings; the checker decides which by the operand types. */
  additive() {
    const c = this.cursor;
    let left = this.multiplicative();
    for (;;) {
      const token = c.peek();
      if (!c.is('+') && !c.is('-')) return left;
      c.next();
      const right = this.multiplicative();
      left = { k: 'binary', op: token.value === '+' ? 'plus' : 'sub', left, right, span: joined(left, right, token) };
    }
  }

  multiplicative() {
    const c = this.cursor;
    let left = this.exponent();
    for (;;) {
      const token = c.peek();
      const op = token.kind === 'punct' ? MULTIPLICATIVE[token.value] : undefined;
      if (!op) return left;
      c.next();
      const right = this.exponent();
      left = { k: 'binary', op, left, right, span: joined(left, right, token) };
    }
  }

  exponent() {
    const c = this.cursor;
    const start=c.peek();
    const left = this.unary();
    if(c.eat('**')){
      if(!acceptJavaScriptExponentiationBase(start.value,Boolean(left.parenthesized)))throw new TranslationError('syntax','an unparenthesized unary expression cannot be the left operand of exponentiation',span(start,start));
      const right=this.exponent();
      return{k:'math',op:'exponentiateJavaScriptNumericValue',name:'**',args:[{value:left,spread:false},{value:right,spread:false}],span:joined(left,right,start)};
    }
    return left;
  }

  unary() {
    const c = this.cursor;
    const token = c.peek();
    if (token.kind === 'identifier' && c.eat('void')) {
      const arg = this.unary(), name = this.generatedPrefix+'temporary_void'+(++this.generatedCount);
      return constructJavaScriptVoidExpression(arg,name,joined(token,arg,token));
    }
    if (token.kind === 'identifier' && c.eat('typeof')) {
      const arg = this.unary();
      return { k: 'typeOf', arg, span: joined(token, arg, token) };
    }
    if (c.eat('!')) {
      const arg = this.unary();
      return { k: 'unary', op: 'not', arg, span: joined(token, arg, token) };
    }
    if (c.eat('-')) {
      const arg = this.unary();
      return { k: 'unary', op: 'neg', arg, span: joined(token, arg, token) };
    }
    if(c.eat('+')){
      const arg=this.unary();
      return{k:'math',op:readJavaScriptPrimitiveUnaryOperation('+'),name:'+',args:[{value:arg,spread:false}],span:joined(token,arg,token)};
    }
    if(c.eat('~')){
      const arg=this.unary();
      return{k:'math',op:readJavaScriptNumberBitwiseOperation('~'),name:'~',args:[{value:arg,spread:false}],span:joined(token,arg,token)};
    }
    if (c.is('++') || c.is('--')) {
      const operator=c.next(),target=this.unary();
      return this.scalarAssignmentExpression(target,operator,false,null);
    }
    if (token.kind === 'identifier' && token.value === 'await') {
      if (!this.inAsync) {
        throw unsupported('await outside an async function', 'await is only valid in async functions and at the top level of a module', span(token, token));
      }
      c.next();
      this.sequentialAsync = true;
      return this.awaited(() => this.unary());
    }
    if (token.kind === 'identifier' && ['typeof', 'void', 'delete', 'await', 'yield'].includes(token.value)) {
      throw unsupported(`${token.value} operator`, 'outside the portable core', span(token, token));
    }
    return this.postfix();
  }

  postfix() {
    const c = this.cursor;
    let expr = this.primary();
    const continuations=[];
    const finish=value=>{for(const entry of continuations.toReversed()){const where={start:entry.receiver.span.start,end:value.span.end};value={k:'math',op:readJavaScriptOptionalChainOperation(),name:'',args:[{value:{k:'let',name:entry.name,value:entry.receiver,body:value,span:where},spread:false}],span:where};}return value;};
    const bind=receiver=>{const name=this.generatedPrefix+'temporary_optional_chain'+(++this.generatedCount);continuations.push({receiver,name});return {k:'name',path:[name],span:receiver.span};};
    for (;;) {
      const token = c.peek();
      if (c.is('.') && c.peek(1).kind === 'identifier') {
        c.next();
        const field = c.next();
        const arrayForm = readArrayMethodForm(field.value);
        if (c.is('(') && arrayForm !== '') {
          const args = this.arguments(field.value);
          if(field.value==='concat'){
            expr={k:'method',object:expr,op:field.value,args,span:joined(expr,{span:span(token,c.peek())},token)};
            continue;
          }
          const form = readJavaScriptArrayMethodSurfaceForm(field.value, args.length);
          if (form === 'copySlice') {
            expr = { k: 'copySlice', object: expr, span: joined(expr, { span: span(token, c.peek()) }, token) };
            continue;
          }
          if (form === 'method') {
            expr = { k: 'method', object: expr, op: field.value, args, span: joined(expr, { span: span(token, c.peek()) }, token) };
            continue;
          }
          if (!acceptArrayMethodArguments(arrayForm, args.length > 0, args.length === 1)) throw unsupported(`.${field.value}() with arguments`, 'array copying takes no index arguments until their bounds are modeled', span(token, c.peek()));
          expr = { k: 'array', items: [expr, ...args].map((value) => ({ spread: true, value })), span: joined(expr, { span: span(token, c.peek()) }, token) };
          continue;
        }
        if (c.is('(') && readStringTestOperation('JavaScript', field.value) !== '') {
          const args = this.arguments(field.value);
          expr = { k: 'method', op: field.value, object: expr, args, span: joined(expr, { span: span(token, c.peek()) }, token) };
          continue;
        }
        if (c.is('(') && readStringMapOperation('JavaScript', field.value) !== '') {
          const args = this.arguments(field.value);
          expr = { k: 'method', op: field.value, object: expr, args, span: joined(expr, { span: span(token, c.peek()) }, token) };
          continue;
        }
        if (c.is('(')) {
          const args = this.arguments(field.value);
          expr = { k: 'method', object: expr, op: field.value, args, span: joined(expr, { span: span(token, c.peek()) }, token) };
          continue;
        }
        expr = { k: 'field', object: expr, field: field.value, span: joined(expr, { span: span(field, field) }, token) };
        continue;
      }
      if (c.eat('?.')) {
        if(c.is('(')){
          if(!acceptJavaScriptOptionalCallHead(expr.k,expr.path?.length??0))throw unsupported('optional method call','the property reference needs its complete original receiver and chain extent',joined(expr,token,token));
          const args=this.arguments('optional callable');
          if(requireJavaScriptOptionalCallChain(c.peek().value)){
            const receiver=expr,fn=bind(receiver);
            expr={k:'app',fn,args,span:joined(receiver,c.peek(),token)};
          }else expr={k:'math',op:readJavaScriptOptionalCallOperation(),name:'',args:[expr,...args].map(value=>({value,spread:false})),span:joined(expr,c.peek(),token)};
          continue;
        }
        if(c.eat('[')){
          const receiver=expr,index=this.sequence(),close=c.expect(']','optional index');
          expr={k:'index',object:bind(receiver),index,span:joined(receiver,close,token)};
          continue;
        }
        const field = c.identifier('optional field');
        const receiver=expr,object=bind(receiver);
        if(c.is('(')){
          const args=this.arguments(field.value);
          expr={k:'method',object,op:field.value,args,span:joined(receiver,c.peek(),token)};
        }else expr={k:'field',object,field:field.value,span:joined(receiver,field,token)};
        continue;
      }
      if (c.is('[')) {
        c.next();
        const index = this.sequence();
        const close = c.expect(']', 'index');
        expr = { k: 'index', object: expr, index, span: joined(expr, close, token) };
        if (c.peek().kind === 'punct' && (ASSIGNMENTS.has(c.peek().value) || c.is('++') || c.is('--'))) {
          throw unsupported('assignment of an array element', 'the portable core reads arrays and does not mutate them; build a new array with [...xs, value]', expr.span);
        }
        continue;
      }
      if (c.is('(')) {
        const args = this.arguments('function value');
        expr = expr.k==='index'&&expr.index.k==='str'
          ?{k:'method',object:expr.object,op:expr.index.value,args,span:joined(expr,c.peek(),token)}
          :{ k: readCallableSurfaceConstructor('computed-call', false), fn: expr, args, span: joined(expr, c.peek(), token) };
        continue;
      }
      if (c.isKind('template')) throw unsupported('tagged template', 'template tags are functions as values', span(token, token));
      if (c.is('++') || c.is('--')) {
        if (findJavaScriptLineTerminator(this.source,c.tokens[c.index-1].end,token.start)<token.start) return finish(expr);
        if(continuations.length)throw this.fail('an optional chain cannot be an assignment target',token);
        c.next();return this.scalarAssignmentExpression(expr,token,true,null);
      }
      return finish(expr);
    }
  }

  primary() {
    const c = this.cursor;
    const token = c.peek();
    if (this.startsFunction(c.index)) return this.closure();
    switch (token.kind) {
      case 'number':
        c.next();
        return this.numberLiteral(token);
      case 'string':
        c.next();
        return createJavaScriptStringLiteral(token.value,span(token,token),token.stringUnits);
      case 'template':
        c.next();
        return this.template(token);
      case 'identifier':
        return this.identifier();
      default:
        break;
    }
    if (c.is('(')) {
      c.next();
      const inner = this.sequence();
      c.expect(')', 'parenthesised expression');
      return { ...inner, parenthesized: acceptSurfaceRawMetadataField('expression', 'parenthesized'), span: span(token, c.peek()) };
    }
    if (c.is('{')) return this.objectLiteral();
    if (c.is('[')) return this.arrayLiteral();
    if (token.kind === 'regex' || c.is('/')) throw unsupported('regular expression', 'outside the portable core', span(token, token));
    throw this.fail('expected an expression');
  }

  /** `[a, ...xs, b]`: the elements in order, with the elements of each spread array in place. */
  arrayLiteral() {
    const c = this.cursor;
    const start = c.next();
    const items = [];
    while (!c.is(']')) {
      if (c.is(',')) throw unsupported('array hole', 'a hole is an element that reads undefined, which is not a portable value', span(c.peek(), c.peek()));
      const spread = c.is('...');
      if (spread) c.next();
      const value = this.expr();
      items.push({ spread, value: spread ? { k: 'forOfIterable', object: value, span: value.span } : value });
      if (!c.eat(',')) break;
    }
    c.expect(']', 'array literal');
    return { k: 'array', items, span: span(start, c.peek()) };
  }

  matching(index) {
    const tokens = this.cursor.tokens;
    let depth = 0;
    for (let at = index; at < tokens.length; at += 1) {
      if (tokens[at].kind !== 'punct') continue;
      if (['(', '[', '{'].includes(tokens[at].value)) depth += 1;
      if ([')', ']', '}'].includes(tokens[at].value)) {
        depth -= 1;
        if (depth === 0) return at;
      }
    }
    return tokens.length - 1;
  }

  /**
   * `5n` and `0x1fn` are BigInt literals, which the lexer keeps as decimal
   * digits; `42`, `1.5` and `1e21` are Numbers, IEEE-754 doubles, which the
   * lexer keeps as `String(value)`.
   */
  numberLiteral(token) {
    if (token.suffix === 'n') return { k: 'num', value: token.value, span: span(token, token) };
    return { k: 'num', value: token.value, type: FLOAT, span: span(token, token) };
  }

  /** Template literals concatenate their text with `String(value)` of each substitution. */
  template(token) {
    const pieces = [];
    for (const part of token.parts) {
      if (part.text||part.stringUnits?.length) pieces.push(createJavaScriptStringLiteral(part.text,span(token,token),part.stringUnits));
      if (part.expression) {
        const value = this.subexpression(part.expression);
        pieces.push({ k: 'show', arg: value, style: 'js-template', span: value.span });
      }
    }
    if (!pieces.length) return { k: 'str', value: '', span: span(token, token) };
    return pieces.reduce((left, right) => ({ k: 'binary', op: 'concat', left, right, span: span(token, token) }));
  }

  subexpression({ source, offset }) {
    const { tokens } = tokenize(source, 'JavaScript');
    const shifted = tokens.map((token) => ({ ...token, start: token.start + offset, end: token.end + offset }));
    const outer = this.cursor;
    this.cursor = new TokenCursor(shifted, 'JavaScript');
    try {
      const expr = this.sequence();
      if (!this.cursor.atEnd()) throw this.fail('expected } after the template substitution');
      return expr;
    } finally {
      this.cursor = outer;
    }
  }

  /** `{ $: 'node', left, value: v }` constructs the `node` alternative of a data type. */
  requireScalarStringToken(token,role) {
    if(token.stringUnits)throw unsupported('isolated UTF16 '+role,'this name requires its complete code-unit identity instead of scalar text',span(token,token));
  }

  objectLiteral() {
    const c = this.cursor;
    const start = c.next();
    let tag = null;
    const fields = Object.create(null);
    const entries = [];
    let duplicate = null;
    let prototypeDefinition = null;
    let propertyIndex = 0;
    while (!c.is('}')) {
      let key = c.peek();
      this.requireScalarStringToken(key,'record property key');
      const index = propertyIndex++;
      if (c.eat('...')) {
        entries.push({ spread: true, value: this.expr() });
        if (!c.eat(',')) break;
        continue;
      }
      const computed = c.eat('[');
      if (computed) {
        const startKey = key;
        const value = this.expr();
        const close = c.expect(']', 'computed object key');
        if (value.k !== 'str') throw unsupported('computed object key', 'the portable record shape needs a literal string key', span(startKey, close));
        key = { kind: 'string', value: value.value, start: startKey.start, end: close.end };
      } else {
        if (key.kind !== 'identifier' && key.kind !== 'string') throw unsupported(`object key ${describe(key)}`, 'keys are literal names', span(key, key));
        c.next();
      }
      if (!acceptObjectPropertyDefinition(key.value, Boolean(computed), c.is(':'))) {
        if (prototypeDefinition) throw new TranslationError('syntax', 'duplicate object prototype definition', span(key, key));
        c.expect(':', 'object prototype definition');
        prototypeDefinition = { value: this.expr(), span: span(key, key), index };
      } else if (readJavaScriptObjectPropertyForm(key.value, c.is('('), c.is(':'), !computed && key.kind === 'identifier') === 'tag') {
        c.expect(':', 'object');
        const value = c.next();
        if (value.kind !== 'string') throw unsupported('computed tag', 'the $ tag is a string literal', span(value, value));
        this.requireScalarStringToken(value,'datatype constructor tag');
        tag = value.value;
      } else {
        if (Object.hasOwn(fields, key.value)) duplicate ??= key;
        const form = readJavaScriptObjectPropertyForm(key.value, c.is('('), c.is(':'), !computed && key.kind === 'identifier');
        if (form === 'method') fields[key.value] = this.closure(true);
        else if (form === 'field') { c.next(); fields[key.value] = this.expr(); }
        else {
          if (form === 'missing-colon') throw this.fail('expected : after the object property name', key);
          fields[key.value] = this.reference(key);
        }
        entries.push({ name: key.value, spread: false, value: fields[key.value] });
      }
      if (!c.eat(',')) break;
    }
    c.expect('}', 'object');
    const prototype = prototypeDefinition ? { prototypeDefinition } : {};
    const form = readJavaScriptObjectValueForm(tag !== null, duplicate !== null, entries.some((entry) => entry.spread));
    if (form === 'record') return { k: 'record', entries, ...prototype, span: span(start, c.peek()) };
    if (form === 'duplicate') throw typeError(`duplicate field ${duplicate.value}`, span(duplicate, duplicate));
    if (form === 'spread') throw unsupported('tagged object spread', 'constructor spreads require a complete tagged field projection', span(start, c.peek()));
    return { k: 'ctorObject', tag, fields, ...prototype, span: span(start, c.peek()) };
  }

  /**
   * An expression whose value is awaited: when it is a call of an async
   * function, that call is awaited where it is made, which is its ordinary
   * call. Any other value is awaited as itself.
   */
  awaited(read) {
    const before = this.unawaited.length;
    const expr = read();
    const call = this.unawaited[this.unawaited.length - 1];
    const path = expr.k === 'app' ? expr.fn.path : expr.path;
    if (this.unawaited.length > before && ['app', 'name'].includes(expr.k) && path?.[0] === ROOT && path.slice(1).join('.') === call.name) {
      this.unawaited.pop();
    }
    return expr;
  }

  identifier() {
    const c = this.cursor;
    const token = c.peek();
    requireJavaScriptIdentifierRole(token, false);
    if (token.value === 'true' || token.value === 'false') {
      c.next();
      return { k: 'bool', value: token.value === 'true', span: span(token, token) };
    }
    this.lexicalRead(token);
    // These are global bindings rather than reserved literal keywords. A
    // parameter or local declaration takes precedence, including its TDZ.
    if (['NaN', 'Infinity', 'undefined'].includes(token.value) && (this.scope.locals.has(token.value) || this.scope.tdz.has(token.value))) {
      c.next();
      return this.reference(token);
    }
    const boundGlobal = this.moduleBindings.has(token.value) || this.externals.has(token.value) || this.literalBindings.has(token.value) || this.functionNames.has(token.value);
    if (['NaN', 'Infinity', 'undefined'].includes(token.value) && boundGlobal && !this.externals.has(token.value) && !this.literalBindings.has(token.value) && !this.functionNames.has(token.value)) throw unsupported(`global binding ${token.value}`, 'a delayed function read needs an owned module binding; a mutable module binding requires module-capture representation, and neither can be replaced with the built-in constant', span(token, token));
    if (!boundGlobal && (token.value === 'NaN' || token.value === 'Infinity')) {
      if (!this.acceptStandardBuiltinOwner(token.value)) throw unsupported('unproved standard global owner',
        'the original module scope does not prove this special global binding', token);
      c.next();
      return { k: 'num', value: token.value, type: FLOAT, span: span(token, token) };
    }
    if (token.value === 'undefined' && !boundGlobal && !this.acceptStandardBuiltinOwner(token.value)) {
      throw unsupported('unproved standard global owner', 'the original module scope does not prove this special global binding', token);
    }
    const absent = readOptionalSourceLiteral(token.value, boundGlobal);
    if (absent) { c.next(); return { k: 'none', absence: absent, span: span(token, token) }; }
    if (token.value === 'new') {
      c.next();
      const constructor = c.identifier('constructor');
      this.lexicalRead(constructor);
      const args = c.is('(') ? this.arguments(constructor.value) : [];
      const collection = readJavaScriptCollectionConstruction(constructor.value, args.length > 1);
      if (collection !== '') {
        if (!this.acceptStandardBuiltinOwner(constructor.value)) throw unsupported('shadowed standard collection owner',
          'a local, module or supplied context constructor cannot be replaced with the standard Map or Set constructor', constructor);
        const items = args[0] ?? { k: 'array', items: [], span: span(token, c.peek()) };
        return { k: collection, [collection === 'set' ? 'items' : 'entries']: items, span: span(token, c.peek()) };
      }
      throw unsupported(`new ${constructor.value}`, readJavaScriptConstructionObligation(), span(token, c.peek()));
    }
    if (['this', 'super', 'new', 'function', 'class', 'import', 'arguments'].includes(token.value)) {
      throw unsupported(`${token.value} expression`, 'outside the portable core', span(token, token));
    }
    c.next();
    if (this.scope.locals.has(token.value) || this.scope.tdz.has(token.value)) return this.reference(token);
    if (this.literalBindings.has(token.value)) return { ...this.literalBindings.get(token.value), span: span(token, token) };
    if (this.assertion?.name === token.value) throw unsupported('assertion in an expression', 'assertions are top-level statements', span(token, token));
    // A constant of another item of the module is read by name.
    if (this.externals.get(token.value)?.k === 'const' && !c.is('(')) return { k: 'name', path: [ROOT, token.value], span: span(token, token) };
    // A global: a function, or a namespace path to a method, which must be called.
    const segments = [token.value];
    while (c.is('.') && c.peek(1).kind === 'identifier') {
      c.next();
      segments.push(c.next().value);
    }
    const name = segments.join('.');
    const where = span(token, c.peek());
    const standardOwner = this.acceptStandardBuiltinOwner(segments[0]);
    if (Object.hasOwn(CONSTANTS, name) && !c.is('(')) {
      if (!standardOwner) throw unsupported('shadowed standard constant owner',
        'an actual lexical/module/context owner cannot be replaced with the standard numeric constant', token);
      const value = CONSTANTS[name];
      return { k: 'num', value: value.replace(/^-/u, ''), ...(value.startsWith('-') && { negative: true }), type: FLOAT, span: where };
    }
    const numericBuiltin=standardOwner?(readJavaScriptNumberBuiltinOperation(name)||readJavaScriptBigIntegerBuiltinOperation(name)||readJavaScriptStringConstructionOperation(name)||readJavaScriptBooleanBuiltinOperation(name)||readJavaScriptArrayClassificationBuiltinOperation(name)||readJavaScriptOwnPropertyBuiltinOperation(name)):'';
    if(numericBuiltin&&c.is('('))return{k:'math',op:numericBuiltin,name,args:this.spreadArguments(name),span:span(token,c.peek())};
    if (Object.hasOwn(MATH, name) && c.is('(') && standardOwner) {
      const args = this.spreadArguments(name);
      return { k: 'math', op: MATH[name], name, args, span: span(token, c.peek()) };
    }
    if (!c.is('(')) {
      const constructor = readCallableSurfaceConstructor('name', this.functionNames.has(name));
      if (constructor) return { k: constructor, path: [ROOT, ...segments], span: where };
      throw unsupported(`function value ${name}`, 'functions and namespaces are only portable when a function is called', where);
    }
    const args = this.arguments(name);
    const called = span(token, c.peek());
    if (standardOwner) {
      const fixedCast = readJavaScriptFixedIntegerCast(this, name, args, called);
      if (fixedCast) return fixedCast;
    }
    if (name === 'Array.from' && standardOwner) return { k: 'arrayFrom', args, span: called };
    if (name === 'Array.isArray' && args.length === 1 && standardOwner) return { k: 'arrayIsArray', arg: args[0], span: called };
    const arrayForm = standardOwner ? readArrayMethodForm(name) : '';
    if (arrayForm === 'copy-from' || arrayForm === 'construct') {
      if (!acceptArrayMethodArguments(arrayForm, args.length > 0, args.length === 1)) throw unsupported(`${name}() with ${args.length} arguments`, 'array copying takes one array and no mapping function', called);
      return { k: 'array', items: args.map(value => ({ spread: arrayForm === 'copy-from', value })), span: called };
    }
    if (name === 'String' && args.length === 1 && standardOwner) return { k: 'toString', arg: args[0], span: called };
    if (name === 'Object.freeze' && args.length === 1 && standardOwner) {
      if (this.functionNames.has('Object') || this.externals.has('Object')) throw unsupported('shadowed Object.freeze', 'immutable products require the actual unshadowed Object.freeze built-in', called);
      const value = args[0];
      const literalArray = (item) => ['num', 'bool', 'str', 'none'].includes(item.k)
        || item.k === 'array' && item.items.every((entry) => !entry.spread && literalArray(entry.value));
      const readonlyLiteral = this.readonlyConstantInitializer && value.k === 'array' && literalArray(value);
      if (this.readonlyConstantInitializer && value.k === 'array' && !readonlyLiteral) throw unsupported('Object.freeze operand',
        'readonly module constants require literal values; evaluated elements need a separate module initialization model', called);
      if (!readonlyLiteral && value.k === 'array') {
        if (!value.items.length || value.items.some((item) => item.spread || item.value.k === 'array')) throw unsupported('immutable tuple product', 'a fresh nonempty dense array needs scalar Copy elements or separately frozen nested products', called);
        return { k: 'tuple', items: value.items.map(item => item.value), frozen: true, productModel: 'immutable value product', span: called };
      }
      if (!acceptObjectFreezeOperand(value.k, readonlyLiteral)) throw unsupported('Object.freeze operand',
        'freezing this value needs an explicit property-descriptor operation; only direct tagged objects and readonly literal module constants are represented', called);
      return { ...value, ...(value.k === 'ctorObject' && { frozen: true }), span: called };
    }
    if ((GLOBALS.has(segments[0]) || name === 'String') && !boundGlobal) {
      throw unsupported(name, 'the JavaScript standard library is outside the portable core', called);
    }
    if (segments.length === 1 && this.asyncNames.has(name)) this.unawaited.push({ name, span: called });
    if (!args.length) return { k: 'name', path: [ROOT, ...segments], span: called };
    return { k: 'app', fn: { k: 'name', path: [ROOT, ...segments], span: where }, args, span: called };
  }

  /** A local variable; reading one before its declaration throws in JavaScript. */
  reference(token) {
    this.lexicalRead(token);
    const temporal = this.scope.tdz.has(token.value);
    if (temporal) {
      const bounds = [], names = [], ends = [];
      for (const scope of this.lexicalScopes) {
        bounds.push(names.length, scope.bindings.size, scope.boundary);
        for (const [name, end] of scope.bindings) {
          names.push(name);
          // Only this already-established TDZ read is checked. Its actual
          // lexical owner and closure boundaries select the diagnostic.
          ends.push(name === token.value ? token.start + 1 : end);
        }
      }
      const plan = readJavaScriptTemporalBindingRead(this.lexicalScopes.map(scope => scope.parent), bounds, names, ends,
        [this.lexicalScope], [token.value], [token.start]);
      if (plan[1] === 2) throw unsupported(`function reading ${token.value}`,
        'a mutable module binding requires module-capture representation', span(token, token));
      if (plan[1] === 1) throw unsupported(`closure reading ${token.value} before its declaration`,
        'the captured binding may still be in its temporal dead zone; invoking the closure after initialization needs an invocation-order proof', span(token, token));
    }
    const obligation = readCallableReferenceObligation(temporal, this.scope.locals.has(token.value));
    if (obligation) throw unsupported(readCallableReferenceConstruct(token.value, temporal), obligation, span(token, token));
    return { k: 'name', path: [token.value], span: span(token, token) };
  }
}

const FORMAT_STYLES = { s: 'js-console', d: 'js-format-number', i: 'js-format-integer' };

/**
 * `console.log(a, b, …)` prints `util.format(a, b, …)`: a literal first
 * string reads its `%s`, `%d`, `%i`, `%c` and `%%` directives, each taking
 * the next argument, and the arguments left over follow, each after a space,
 * strings as they are and other values as the console shows them. A first
 * argument that is not a literal string is shown like the rest, and the
 * checker refuses it when it is a string, whose directives only the run
 * reads.
 */
function consoleFormat([first, ...values]) {
  const pieces = [];
  let next = 0;
  if (first.k === 'str') {
    const source = first.value;
    let text = '';
    for (let index = 0; index < source.length; index += 1) {
      const char = source[index];
      const directive = source[index + 1];
      if (char !== '%' || directive === undefined) {
        text += char;
        continue;
      }
      if (directive === '%') {
        text += '%';
        index += 1;
        continue;
      }
      // A directive with no argument left, or an unknown one, stays as it is written.
      if (next === values.length || !'sdifjoOc'.includes(directive)) {
        text += char;
        continue;
      }
      const value = values[next];
      next += 1;
      index += 1;
      if (directive === 'c') {
        if (value.k !== 'str') throw unsupported('%c with a computed style', 'console.log discards the CSS a %c directive takes; pass it as a string literal', value.span);
        continue;
      }
      if (!FORMAT_STYLES[directive]) {
        throw unsupported(`console.log %${directive} directive`, 'the portable directives are %s, %d, %i, %c and %%', first.span);
      }
      if (text) pieces.push({ k: 'str', value: text, span: first.span });
      text = '';
      pieces.push({ k: 'show', arg: value, style: FORMAT_STYLES[directive], span: value.span });
    }
    if (text || !pieces.length) pieces.push({ k: 'str', value: text, span: first.span });
  } else {
    pieces.push({ k: 'show', arg: first, style: 'js-format-first', span: first.span });
  }
  for (const value of values.slice(next)) {
    pieces.push({ k: 'str', value: ' ', span: value.span }, { k: 'show', arg: value, style: 'js-console', span: value.span });
  }
  return pieces.reduce((left, right) => ({ k: 'binary', op: 'concat', left, right, span: first.span }));
}

/** Source binding roles retain the ECMAScript early errors. */
function reserved(token) {
  requireJavaScriptIdentifierRole(token, true);
}

/** A top-level declaration may not shadow a global the translation reads as the built-in. */
function global(token) {
  if (GLOBALS.has(token.value) || token.value === 'String') {
    throw unsupported(`declaration of ${token.value}`, `it shadows the JavaScript global ${token.value}, which the translation reads as the built-in; rename it`, span(token, token));
  }
}

/** The parameter a leading `if (n < 0n) throw …` restricts to the naturals. */
function guardedParameter(statement, params) {
  if (statement.s !== 'if' || statement.else) return null;
  const then = statement.then.filter((inner) => inner.s !== 'empty');
  if (then.length !== 1 || then[0].s !== 'throw' || then[0].exception) return null;
  const { cond } = statement;
  if (cond.k !== 'binary') return null;
  const isZero = (node) => node.k === 'num' && node.value === '0';
  let variable = null;
  if (cond.op === 'lt' && isZero(cond.right)) variable = cond.left;
  if (cond.op === 'gt' && isZero(cond.left)) variable = cond.right;
  if (variable?.k !== 'name' || variable.path.length !== 1) return null;
  const param = params.find((candidate) => candidate.name === variable.path[0]);
  // An undeclared parameter compared with the BigInt 0n is a bigint.
  const zero = cond.op === 'lt' ? cond.right : cond.left;
  return param && (param.type === INT || (param.type === null && !zero.type)) ? param : null;
}

/**
 * JSDoc tags of a comment: `@tag {type} name`. Leading `*` of each line is
 * decoration.
 */
export function readJavaScriptDocumentationTags(comment) {
  return jsdocTags(comment);
}

function jsdocTags(comment) {
  const text = comment.text.slice(3, -2).split('\n').map((line) => line.replace(/^\s*\*?/u, '')).join('\n');
  const tags = [];
  const pattern = /@([A-Za-z]+)/gu;
  let match = pattern.exec(text);
  while (match) {
    let index = match.index + match[0].length;
    while (text[index] === ' ' || text[index] === '\t') index += 1;
    let type = null;
    if (text[index] === '{') {
      let depth = 0;
      let end = index;
      for (; end < text.length; end += 1) {
        if (text[end] === '{') depth += 1;
        if (text[end] === '}') {
          depth -= 1;
          if (depth === 0) break;
        }
      }
      if (depth !== 0) throw new TranslationError('syntax', `unbalanced braces in the JSDoc type of @${match[1]}`, comment);
      type = text.slice(index + 1, end).replaceAll('\n', ' ');
      index = end + 1;
    }
    const tail = text.slice(index);
    const range = findDocumentationParameterRange(Array.from({ length: tail.length }, (_, offset) => tail.charCodeAt(offset)));
    const name = range.length ? tail.slice(range[0], range[1]) : null;
    tags.push({ tag: match[1], type, name, ...(tail.trimStart().startsWith('[') ? { optional: true } : {}) });
    pattern.lastIndex = index;
    match = pattern.exec(text);
  }
  return tags;
}

/** Every name a surface expression binds or mentions. */
function collectNames(node, names) {
  if (!node || typeof node !== 'object') return;
  if (Array.isArray(node)) {
    for (const item of node) collectNames(item, names);
    return;
  }
  if (node.k === 'name' && node.path.length === 1) names.add(node.path[0]);
  if (node.k === 'let' || node.k === 'bind' || node.k === 'bindOrCtor') names.add(node.name);
  for (const [key, value] of Object.entries(node)) {
    if (key !== 'span' && key !== 'type') collectNames(value, names);
  }
}

/** Whether an expression reads a field of `subject`. */
function readsFields(node, subject) {
  if (!node || typeof node !== 'object') return false;
  if (Array.isArray(node)) return node.some((item) => readsFields(item, subject));
  if (node.k === 'field' && node.object.k === 'name' && node.object.path.length === 1 && node.object.path[0] === subject) return true;
  return Object.entries(node).some(([key, value]) => key !== 'span' && key !== 'type' && value && typeof value === 'object' && readsFields(value, subject));
}

/** Every name a surface expression binds. */
function collectBinders(node, names) {
  if (!node || typeof node !== 'object') return;
  if (Array.isArray(node)) {
    for (const item of node) collectBinders(item, names);
    return;
  }
  if (node.k === 'let' || node.k === 'bind' || node.k === 'bindOrCtor') names.add(node.name);
  for (const [key, value] of Object.entries(node)) {
    if (key !== 'span' && key !== 'type') collectBinders(value, names);
  }
}

/** Replaces `subject.field` by the case's pattern variables, up to a rebinding of `subject`. */
function substituteFields(node, subject, binderFor) {
  if (!node || typeof node !== 'object') return node;
  if (Array.isArray(node)) return node.map((item) => substituteFields(item, subject, binderFor));
  if (node.k === 'field' && node.object.k === 'name' && node.object.path.length === 1 && node.object.path[0] === subject) {
    return { k: 'name', path: [binderFor(node.field)], span: node.span };
  }
  if (node.k === 'let') {
    const plan = readJavaScriptFieldSubstitutionPlan(node.name, subject, node.value.k,
      node.value.path?.length ?? 0, node.value.path?.[0] ?? '');
    const value = substituteFields(node.value, subject, binderFor);
    const body = plan === 0 ? node.body : substituteFields(node.body, subject, binderFor);
    return { ...node, value, body: plan === 2 ? substituteFields(body, node.name, binderFor) : body };
  }
  if (node.k === 'match') {
    const scrutinees = substituteFields(node.scrutinees, subject, binderFor);
    const rows = node.rows.map((row) => {
      const bound = new Set();
      collectNames(row.patterns, bound);
      return bound.has(subject) ? row : { ...row, body: substituteFields(row.body, subject, binderFor) };
    });
    return { ...node, scrutinees, rows };
  }
  const copy = { ...node };
  for (const [key, value] of Object.entries(node)) {
    if (key !== 'span' && key !== 'type' && value && typeof value === 'object') copy[key] = substituteFields(value, subject, binderFor);
  }
  return copy;
}

/** A proposition from an asserted condition; `===` on objects compares identities. */
function propOf(expr) {
  if (expr.k === 'binary' && (expr.op === 'eq' || expr.op === 'ne')) return { p: expr.op, left: expr.left, right: expr.right, reference: true };
  if (expr.k === 'binary' && ['lt', 'le', 'gt', 'ge'].includes(expr.op)) return { p: expr.op, left: expr.left, right: expr.right };
  if (expr.k === 'binary' && (expr.op === 'and' || expr.op === 'or')) return { p: expr.op, left: propOf(expr.left), right: propOf(expr.right) };
  if (expr.k === 'unary' && expr.op === 'not') return { p: 'not', arg: propOf(expr.arg) };
  return { p: 'bool', expr };
}

/**
 * The path inside the crate of the module a relative `specifier` names from
 * the module directory `directory`, or null when it climbs above the crate
 * root or names no file.
 */
function cratePath(specifier, directory) {
  const parts = specifier.split('/');
  const file = parts.pop().replace(/\.[cm]?[jt]s$/u, '');
  const path = [...directory];
  for (const part of parts) {
    if (part === '..') {
      if (!path.length) return null;
      path.pop();
    } else if (part !== '.') {
      if (!part) return null;
      path.push(part);
    }
  }
  return file && file !== '.' && file !== '..' ? [...path, file] : null;
}

function joined(left, right, token) {
  return { start: left.span?.start ?? left.start ?? token.start, end: right.span?.end ?? right.end ?? token.end };
}

function readCompleteDeclarationSpan(from, cursor) {
  const consumed = cursor.tokens[cursor.index - 1];
  if (!consumed || consumed.kind === 'eof'
      || !acceptCompleteDeclarationSourceInterval(from.start, from.end, consumed.end)) {
    throw new TypeError('an actual completed declaration requires its last consumed ordered source token');
  }
  return { start: from.start, end: consumed.end };
}

function span(from, to) {
  return { start: from.start, end: Math.max(from.end, to && to.kind !== 'eof' ? to.start : from.end) };
}
