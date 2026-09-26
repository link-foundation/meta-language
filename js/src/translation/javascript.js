// JavaScript frontend for the portable core. It reads the subset of modern
// JavaScript the core can represent faithfully: BigInt arithmetic, strings
// and booleans; top-level functions and object-literal namespaces of methods
// whose types come from JSDoc (`@param`, `@returns`, and `@typedef` unions of
// `{ $: 'tag', … }` object types for data types); bodies made of `const`,
// `if`, `return`, `throw` and `switch` statements; and a top level of
// `console.log`, `const` and `node:assert` statements, which are the
// program's effects. A function whose leading statements throw on a negative
// argument takes a natural number. Everything else is rejected with a precise
// obligation.

import { TranslationError, typeError, unsupported } from './diagnostics.js';
import { TokenCursor, describe, tokenize } from './lexer.js';
import { BOOL, INT, NAT, STRING } from './types.js';

const ROOT = 'crate';
const EQUALITY = { '===': 'eq', '!==': 'ne' };
const RELATIONAL = { '<': 'lt', '<=': 'le', '>': 'gt', '>=': 'ge' };
const MULTIPLICATIVE = { '*': 'mul', '/': 'div', '%': 'rem' };
const ASSIGNMENTS = new Set(['=', '+=', '-=', '*=', '/=', '%=', '**=', '<<=', '>>=', '>>>=', '&=', '|=', '^=', '&&=', '||=', '??=']);
const ERRORS = new Set(['Error', 'RangeError', 'TypeError']);
const GLOBALS = new Set([
  'Math', 'Number', 'BigInt', 'parseInt', 'parseFloat', 'JSON', 'Array', 'Object', 'Symbol', 'Date', 'Promise', 'Reflect',
  'Proxy', 'Map', 'Set', 'WeakMap', 'WeakSet', 'globalThis', 'process', 'require', 'console', 'setTimeout', 'isNaN', 'isFinite',
]);
const STRICT_ASSERTIONS = { equal: 'eq', notEqual: 'ne', deepEqual: 'deep', notDeepEqual: 'notDeep' };
const ASSERTIONS = { strictEqual: 'eq', notStrictEqual: 'ne', deepStrictEqual: 'deep', notDeepStrictEqual: 'notDeep' };

export function parseJavaScript(source) {
  const { tokens, comments } = tokenize(source, 'JavaScript');
  return new JavaScriptParser(source, tokens, comments).file();
}

class JavaScriptParser {
  constructor(source, tokens, comments) {
    this.source = source;
    this.cursor = new TokenCursor(tokens, 'JavaScript');
    this.docs = comments.filter((comment) => comment.text.startsWith('/**') && comment.text !== '/**/');
    // Tags of every @typedef data type, for `switch (x.$)`.
    this.dataTypes = new Map();
    // Locals in scope, and names of the current block still in their temporal dead zone.
    this.scope = { locals: new Set(), tdz: new Set() };
    this.assertion = null;
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
    this.scope = { locals: new Set(), tdz: this.blockDeclarations(c.index, () => c.atEnd()) };
    while (!c.atEnd()) {
      const start = c.peek();
      if (c.is('import')) {
        this.importDeclaration();
        continue;
      }
      if (c.eat('export')) {
        if (!c.is('function') && !c.is('const')) {
          throw unsupported(`export ${describe(c.peek())}`, 'only function and const declarations are exported', span(start, c.peek()));
        }
      }
      if (c.is('async')) throw unsupported('async function', 'asynchronous code is outside the portable core', span(start, c.peek()));
      if (c.is('function')) {
        items.push(this.functionDeclaration(start));
        continue;
      }
      if (c.is('const') && c.peek(1).kind === 'identifier' && c.is('=', 2) && c.is('{', 3) && !(c.is('$', 4) && c.is(':', 5))) {
        if (effects.length) {
          throw unsupported(
            'namespace after a top-level statement',
            `the statements before const ${c.peek(1).value} would run before it is initialised; declare every namespace first`,
            span(start, c.peek(1)),
          );
        }
        items.push(this.namespace(start));
        continue;
      }
      effects.push(this.mainStatement());
    }
    return { language: 'JavaScript', items, main: { effects, span: { start: 0, end: this.source.length } } };
  }

  /** `import assert from 'node:assert/strict'` is the only portable import. */
  importDeclaration() {
    const c = this.cursor;
    const start = c.next();
    let local;
    let strict = false;
    if (c.eat('{')) {
      const imported = c.identifier('import');
      if (imported.value !== 'strict' || !c.eat('as')) {
        throw unsupported(`import { ${imported.value} }`, 'import the assertion module as a whole, e.g. import assert from \'node:assert/strict\'', span(start, c.peek()));
      }
      local = c.identifier('import');
      c.expect('}', 'import');
      strict = true;
    } else if (c.is('*')) {
      throw unsupported('namespace import', 'only node:assert can be imported', span(start, c.peek()));
    } else {
      local = c.identifier('import');
    }
    c.expect('from', 'import');
    const module = c.next();
    if (module.kind !== 'string') throw this.fail('expected a module name', module);
    c.eat(';');
    if (module.value === 'node:assert/strict' || module.value === 'assert/strict') strict = true;
    else if (module.value !== 'node:assert' && module.value !== 'assert') {
      throw unsupported(`import from '${module.value}'`, 'modules other than node:assert are outside the portable core', span(start, module));
    }
    if (this.assertion) throw unsupported('second assertion import', 'import node:assert once', span(start, module));
    this.assertion = { name: local.value, strict };
    this.scope.tdz.delete(local.value);
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
              name = value.value;
            } else {
              const type = c.identifier('field type');
              if (!c.is(',') && !c.is(';') && !c.is('}')) {
                throw unsupported(`field type of ${key.value}`, 'field types are bigint, boolean, string or a @typedef name', range);
              }
              fields.push({ name: key.value, type: this.type(type.value, range) });
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
    if (name === 'bigint') return INT;
    if (name === 'boolean') return BOOL;
    if (name === 'string') return STRING;
    if (name === 'number') {
      throw unsupported('JavaScript number', 'numbers are IEEE-754 doubles, which are outside the portable core; use bigint', range);
    }
    if (!/^[A-Za-z_$][\w$]*$/u.test(name) || ['object', 'any', 'unknown', 'void', 'undefined', 'null', 'Object', 'Function', 'Array', 'symbol'].includes(name)) {
      throw unsupported(`JSDoc type {${name}}`, 'portable types are bigint, boolean, string and @typedef data types', range);
    }
    return { kind: 'named', path: [ROOT, name], span: range };
  }

  /** The JSDoc block directly before a declaration. */
  jsdocFor(token) {
    const doc = this.docs.findLast((comment) => comment.end <= token.start && this.source.slice(comment.end, token.start).trim() === '');
    if (!doc) return null;
    const params = new Map();
    let returns = null;
    for (const tag of jsdocTags(doc)) {
      if (tag.tag === 'param') {
        if (!tag.type || !tag.name) throw new TranslationError('syntax', '@param needs a type and a name', doc);
        if (params.has(tag.name)) throw typeError(`duplicate @param ${tag.name}`, doc);
        params.set(tag.name, tag.type);
      } else if (tag.tag === 'returns' || tag.tag === 'return') {
        returns = tag.type;
      }
    }
    return { params, returns, range: { start: doc.start, end: doc.end } };
  }

  functionDeclaration(start) {
    const c = this.cursor;
    c.expect('function', 'function declaration');
    if (c.is('*')) throw unsupported('generator function', 'generators are outside the portable core', span(start, c.peek()));
    const name = c.identifier('function');
    return this.functionRest(start, name);
  }

  /** Parameters and body of a function or method; `docToken` is where its JSDoc ends. */
  functionRest(docToken, nameToken) {
    const c = this.cursor;
    const doc = this.jsdocFor(docToken);
    const name = nameToken.value;
    c.expect('(', name);
    const tokens = [];
    while (!c.is(')')) {
      const token = c.peek();
      if (c.is('...')) throw unsupported('rest parameter', 'functions take a fixed number of arguments', span(token, token));
      if (c.is('{') || c.is('[')) throw unsupported('destructured parameter', 'name each parameter', span(token, token));
      const param = c.identifier('parameter');
      if (c.is('=')) throw unsupported('default parameter', 'every argument must be passed', span(param, c.peek()));
      tokens.push(param);
      if (!c.eat(',')) break;
    }
    c.expect(')', name);
    const where = span(docToken, c.peek());
    if (!doc) throw unsupported(`untyped function ${name}`, 'declare its types with JSDoc @param and @returns tags', where);
    const params = tokens.map((token) => {
      const text = doc.params.get(token.value);
      if (!text) throw unsupported(`untyped parameter ${token.value}`, 'give every parameter a JSDoc @param type', span(token, token));
      return { name: token.value, type: this.type(text, doc.range), span: span(token, token) };
    });
    for (const documented of doc.params.keys()) {
      if (!params.some((param) => param.name === documented)) throw typeError(`@param ${documented} is not a parameter of ${name}`, doc.range);
    }
    if (!doc.returns) throw unsupported(`function ${name} without @returns`, 'declare the result type with a JSDoc @returns tag', where);
    const ret = this.type(doc.returns, doc.range);
    const outer = this.scope;
    this.scope = { locals: new Set(params.map((param) => param.name)), tdz: new Set() };
    const statements = this.blockStatements();
    this.scope = outer;
    // `if (n < 0n) throw …` as a leading statement makes `n` a natural number.
    let index = 0;
    for (; index < statements.length; index += 1) {
      const param = guardedParameter(statements[index], params);
      if (!param) break;
      param.type = NAT;
      param.guard = true;
    }
    const body = this.lower(statements.slice(index), span(nameToken, c.peek()));
    return { k: 'fn', name, params, ret, body, span: span(docToken, c.peek()) };
  }

  /** `const Name = { method(…) { … }, Nested: { … } };` is a module. */
  namespace(start) {
    const c = this.cursor;
    c.expect('const', 'namespace');
    const name = c.identifier('namespace');
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
      if (c.is('(')) {
        items.push(this.functionRest(member, member));
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

  /** Names a block declares with `const`, which are in their temporal dead zone until declared. */
  blockDeclarations(from, done) {
    const c = this.cursor;
    const names = new Set();
    const saved = c.index;
    c.index = from;
    let depth = 0;
    while (!c.atEnd() && !(depth === 0 && done())) {
      const token = c.next();
      if (['{', '(', '['].includes(token.value) && token.kind === 'punct') depth += 1;
      else if (['}', ')', ']'].includes(token.value) && token.kind === 'punct') depth -= 1;
      else if (depth === 0 && token.kind === 'identifier' && token.value === 'const') {
        if (c.peek().kind === 'identifier') names.add(c.peek().value);
        else if (c.is('{')) {
          for (let offset = 1; !c.is('}', offset) && !c.isKind('eof', offset); offset += 1) {
            if (c.peek(offset).kind === 'identifier' && (c.is(',', offset + 1) || c.is('}', offset + 1))) names.add(c.peek(offset).value);
          }
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
    for (const name of this.blockDeclarations(c.index, () => c.is('}'))) tdz.add(name);
    this.scope = { locals: new Set(outer.locals), tdz };
    try {
      return parse();
    } finally {
      this.scope = outer;
    }
  }

  declareLocal(name) {
    this.scope.locals.add(name);
    this.scope.tdz.delete(name);
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
    if (c.is('const')) return this.constStatement();
    if (c.is('let') || c.is('var')) {
      throw unsupported(`${token.value} declaration`, 'mutable bindings are outside the portable core; use const', span(token, token));
    }
    if (c.is('return')) {
      c.next();
      if (c.is(';') || c.is('}') || this.source.slice(token.end, c.peek().start).includes('\n')) {
        throw unsupported('return without a value', 'the function would return undefined, which is not a portable value', span(token, c.peek()));
      }
      const expr = this.expr();
      c.eat(';');
      return { s: 'return', expr, span: span(token, c.peek()) };
    }
    if (c.is('throw')) return this.throwStatement();
    if (c.is('if')) return this.ifStatement();
    if (c.is('switch')) return this.switchStatement();
    if (c.is('{')) return { s: 'block', body: this.blockStatements(), span: span(token, c.peek()) };
    if (c.eat(';')) return { s: 'empty', span: span(token, token) };
    if (['for', 'while', 'do'].includes(token.value) && token.kind === 'identifier') {
      throw unsupported(`${token.value} loop`, 'loops over mutable state are outside the portable core; use recursion', span(token, token));
    }
    if (['break', 'continue', 'try', 'function', 'class', 'label', 'with', 'debugger'].includes(token.value) && token.kind === 'identifier') {
      throw unsupported(`${token.value} statement`, 'outside the portable core', span(token, token));
    }
    const expr = this.expr();
    c.eat(';');
    return { s: 'expr', expr, span: span(token, c.peek()) };
  }

  constStatement() {
    const c = this.cursor;
    const start = c.next();
    if (c.is('{')) return this.destructuring(start);
    if (c.is('[')) throw unsupported('array destructuring', 'arrays are outside the portable core', span(start, c.peek()));
    const name = c.identifier('const');
    if (!c.eat('=')) throw this.fail('expected = in const');
    const value = this.expr();
    if (c.is(',')) throw unsupported('several declarators', 'declare one binding per const', span(start, c.peek()));
    c.eat(';');
    this.declareLocal(name.value);
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
      if (c.is('=')) throw unsupported('default value', 'every field has a value', span(key, c.peek()));
      pairs.push({ field: key.value, name: local.value, span: span(key, local) });
      if (!c.eat(',')) break;
    }
    c.expect('}', 'destructuring');
    c.expect('=', 'destructuring');
    const object = this.expr();
    c.eat(';');
    const statements = pairs.map((pair) => ({
      s: 'const',
      name: pair.name,
      value: { k: 'field', object, field: pair.field, span: pair.span },
      span: span(start, c.peek()),
    }));
    for (const pair of pairs) this.declareLocal(pair.name);
    return { s: 'block', body: statements, inline: true, span: span(start, c.peek()) };
  }

  throwStatement() {
    const c = this.cursor;
    const start = c.next();
    if (!c.eat('new')) throw unsupported('throw of a non-error value', 'throw new Error(\'…\'), which aborts the program', span(start, c.peek()));
    const kind = c.identifier('throw');
    if (!ERRORS.has(kind.value)) throw unsupported(`throw new ${kind.value}`, 'throw an Error, RangeError or TypeError', span(kind, kind));
    c.expect('(', 'throw');
    let message = '';
    if (!c.is(')')) {
      const token = c.next();
      if (token.kind !== 'string') throw unsupported('computed error message', 'the message of an abort is a string literal', span(token, token));
      message = token.value;
    }
    c.expect(')', 'throw');
    c.eat(';');
    return { s: 'throw', message, span: span(start, c.peek()) };
  }

  ifStatement() {
    const c = this.cursor;
    const start = c.next();
    c.expect('(', 'if');
    const cond = this.expr();
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
    const discriminant = this.expr();
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
      while (!c.is('case') && !c.is('default') && !c.is('}')) {
        if (c.is('const') || c.is('let')) {
          throw unsupported('declaration in a case clause', 'case clauses share one scope; wrap the case body in braces', span(c.peek(), c.peek()));
        }
        body.push(this.statement());
      }
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
      c.next();
      return { k: 'tag', tag: token.value, span: span(token, token) };
    }
    const negative = Boolean(c.eat('-'));
    const value = c.peek();
    if (value.kind === 'number') {
      c.next();
      return { k: 'numLit', value: this.bigint(value), negative, span: span(token, value) };
    }
    if (!negative && (c.is('true') || c.is('false'))) {
      c.next();
      return { k: 'boolLit', value: token.value === 'true', span: span(token, token) };
    }
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

  mainStatement() {
    const c = this.cursor;
    const token = c.peek();
    if (c.is('const')) {
      if (c.is('{', 1)) throw unsupported('top-level destructuring', 'bind each value with const', span(token, c.peek(1)));
      const binding = this.constStatement();
      return { k: 'let', name: binding.name, value: binding.value, span: binding.span };
    }
    if (c.is('console') && c.is('.', 1)) return this.consoleStatement();
    if (this.assertion && c.is(this.assertion.name)) return this.assertStatement();
    if (token.kind === 'identifier' && ['let', 'var', 'if', 'for', 'while', 'do', 'switch', 'try', 'throw', 'class', 'return'].includes(token.value)) {
      throw unsupported(`top-level ${token.value} statement`, 'the top level may only print with console.log, bind with const and assert', span(token, token));
    }
    throw unsupported('top-level expression statement', 'a statement that discards its value has no portable effect', span(token, token));
  }

  consoleStatement() {
    const c = this.cursor;
    const start = c.next();
    c.expect('.', 'console');
    const method = c.identifier('console');
    if (method.value !== 'log') {
      throw unsupported(`console.${method.value}`, 'console.log, which writes a line to standard output, is the portable output', span(start, method));
    }
    const args = this.arguments('console.log');
    c.eat(';');
    const where = span(start, c.peek());
    if (args.length > 1) throw unsupported('console.log with several arguments', 'several arguments are formatted by util.format; pass one string', where);
    const expr = args[0] ?? { k: 'str', value: '', span: where };
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
    if (kind === 'eq' || kind === 'ne') return { k: 'assert', prop: { p: kind, left, right, reference: true, span: where }, span: where };
    return { k: 'assert', prop: { p: kind === 'deep' ? 'eq' : 'ne', left, right, span: where }, span: where };
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
      if (index === list.length) {
        throw unsupported('missing return', 'the function can finish without returning and return undefined, which is not a portable value', where);
      }
      const statement = list[index];
      switch (statement.s) {
        case 'const':
          return { k: 'let', name: statement.name, value: statement.value, body: at(index + 1), span: statement.span };
        case 'return':
          unreachable(index);
          return statement.expr;
        case 'throw':
          unreachable(index);
          return { k: 'abort', message: statement.message, span: statement.span };
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
          rows.push({ patterns: [{ k: test.k, value: test.value, negative: test.negative, span: test.span }], body, span: clause.span });
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
  expr() {
    const c = this.cursor;
    const start = c.peek();
    if (start.kind === 'identifier' && c.is('=>', 1)) throw unsupported('arrow function', 'functions as values are outside the portable core', span(start, c.peek(1)));
    const cond = this.or();
    if (ASSIGNMENTS.has(c.peek().value) && c.peek().kind === 'punct') {
      throw unsupported('assignment', 'mutation is outside the portable core', span(start, c.peek()));
    }
    if (!c.eat('?')) return cond;
    const then = this.expr();
    c.expect(':', 'conditional expression');
    const otherwise = this.expr();
    return { k: 'if', cond, then, else: otherwise, span: joined(cond, otherwise, start) };
  }

  or() {
    const c = this.cursor;
    let left = this.and();
    for (;;) {
      const token = c.peek();
      if (c.is('??')) throw unsupported('nullish coalescing', 'null and undefined are outside the portable core', span(token, token));
      if (!c.eat('||')) return left;
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
    const c = this.cursor;
    const left = this.equality();
    const token = c.peek();
    if (token.kind === 'punct' && ['|', '^', '&'].includes(token.value)) {
      throw unsupported(`bitwise ${token.value}`, 'bitwise operators are outside the portable core', span(token, token));
    }
    return left;
  }

  equality() {
    const c = this.cursor;
    let left = this.relational();
    for (;;) {
      const token = c.peek();
      if (c.is('==') || c.is('!=')) throw unsupported(`loose equality ${token.value}`, 'use === or !==', span(token, token));
      const op = token.kind === 'punct' ? EQUALITY[token.value] : undefined;
      if (!op) return left;
      c.next();
      const right = this.relational();
      left = this.tagTest(op, left, right, joined(left, right, token)) ?? { k: 'binary', op, left, right, span: joined(left, right, token) };
    }
  }

  /** `x.$ === 'tag'` tests the alternative of a data value. */
  tagTest(op, left, right, where) {
    const [field, tag] = left.k === 'field' && left.field === '$' ? [left, right] : [right, left];
    if (field.k !== 'field' || field.field !== '$') return null;
    if (tag.k !== 'str') throw unsupported('computed tag test', 'compare the $ tag with a string literal', where);
    const candidates = [...this.dataTypes].filter(([, ctors]) => ctors.some((ctor) => ctor.name === tag.value));
    if (candidates.length !== 1) throw typeError(candidates.length ? `tag ${tag.value} is ambiguous between data types` : `no @typedef has tag ${tag.value}`, where);
    const [name, ctors] = candidates[0];
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
    const left = this.additive();
    const token = c.peek();
    if (c.is('<<') || c.is('>>') || c.is('>>>')) throw unsupported(`shift ${token.value}`, 'bit shifts are outside the portable core', span(token, token));
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
    const left = this.unary();
    if (c.is('**')) throw unsupported('exponentiation', 'powers are outside the portable core; use recursion', span(c.peek(), c.peek()));
    return left;
  }

  unary() {
    const c = this.cursor;
    const token = c.peek();
    if (c.eat('!')) {
      const arg = this.unary();
      return { k: 'unary', op: 'not', arg, span: joined(token, arg, token) };
    }
    if (c.eat('-')) {
      const arg = this.unary();
      return { k: 'unary', op: 'neg', arg, span: joined(token, arg, token) };
    }
    if (c.is('+')) throw unsupported('unary +', 'unary plus throws a TypeError on BigInt values', span(token, token));
    if (c.is('~')) throw unsupported('bitwise ~', 'bitwise operators are outside the portable core', span(token, token));
    if (c.is('++') || c.is('--')) throw unsupported(`${token.value} operator`, 'mutation is outside the portable core', span(token, token));
    if (token.kind === 'identifier' && ['typeof', 'void', 'delete', 'await', 'yield'].includes(token.value)) {
      throw unsupported(`${token.value} operator`, 'outside the portable core', span(token, token));
    }
    return this.postfix();
  }

  postfix() {
    const c = this.cursor;
    let expr = this.primary();
    for (;;) {
      const token = c.peek();
      if (c.is('.') && c.peek(1).kind === 'identifier') {
        c.next();
        const field = c.next();
        if (c.is('(')) {
          if (field.value !== 'toString') throw unsupported(`method call .${field.value}()`, 'methods of values are outside the portable core', span(token, c.peek()));
          const args = this.arguments('toString');
          if (args.length) throw unsupported('toString with a radix', 'only decimal text is portable', span(token, c.peek()));
          expr = { k: 'toString', arg: expr, span: joined(expr, { span: span(token, c.peek()) }, token) };
          continue;
        }
        expr = { k: 'field', object: expr, field: field.value, span: joined(expr, { span: span(field, field) }, token) };
        continue;
      }
      if (c.is('?.')) throw unsupported('optional chaining', 'null and undefined are outside the portable core', span(token, token));
      if (c.is('[')) throw unsupported('indexing', 'arrays and computed properties are outside the portable core', span(token, token));
      if (c.is('(')) throw unsupported('call of a computed function', 'only named functions can be called', span(token, token));
      if (c.isKind('template')) throw unsupported('tagged template', 'template tags are functions as values', span(token, token));
      if (c.is('++') || c.is('--')) throw unsupported(`${token.value} operator`, 'mutation is outside the portable core', span(token, token));
      return expr;
    }
  }

  primary() {
    const c = this.cursor;
    const token = c.peek();
    switch (token.kind) {
      case 'number':
        c.next();
        if (c.is('.') && c.peek(1).kind === 'number' && c.peek(1).start === c.peek().end) {
          throw unsupported('JavaScript number', 'numbers are IEEE-754 doubles, which are outside the portable core; use BigInt literals such as 5n', span(token, c.peek(1)));
        }
        return { k: 'num', value: this.bigint(token), span: span(token, token) };
      case 'string':
        c.next();
        return { k: 'str', value: token.value, span: span(token, token) };
      case 'template':
        c.next();
        return this.template(token);
      case 'identifier':
        return this.identifier();
      default:
        break;
    }
    if (c.is('(')) {
      const close = this.matching(c.index);
      if (this.cursor.tokens[close + 1]?.value === '=>') {
        throw unsupported('arrow function', 'functions as values are outside the portable core', span(token, token));
      }
      c.next();
      const inner = this.expr();
      c.expect(')', 'parenthesised expression');
      return { ...inner, span: span(token, c.peek()) };
    }
    if (c.is('{')) return this.objectLiteral();
    if (c.is('[')) throw unsupported('array literal', 'arrays are outside the portable core', span(token, token));
    if (c.is('/')) throw unsupported('regular expression', 'outside the portable core', span(token, token));
    throw this.fail('expected an expression');
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

  /** `5n`, `0x1fn`: BigInt literals. Plain numbers are doubles and are rejected. */
  bigint(token) {
    const radix = /^([xob])([0-9a-f]*)n$/u.exec(token.suffix);
    if (token.value === '0' && radix) return BigInt(`0${radix[1]}${radix[2]}`).toString();
    if (token.suffix === 'n') return token.value;
    throw unsupported('JavaScript number', 'numbers are IEEE-754 doubles, which are outside the portable core; use BigInt literals such as 5n', span(token, token));
  }

  /** Template literals concatenate their text with `String(value)` of each substitution. */
  template(token) {
    const pieces = [];
    for (const part of token.parts) {
      if (part.text) pieces.push({ k: 'str', value: part.text, span: span(token, token) });
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
      const expr = this.expr();
      if (!this.cursor.atEnd()) throw this.fail('expected } after the template substitution');
      return expr;
    } finally {
      this.cursor = outer;
    }
  }

  /** `{ $: 'node', left, value: v }` constructs the `node` alternative of a data type. */
  objectLiteral() {
    const c = this.cursor;
    const start = c.next();
    let tag = null;
    const fields = {};
    while (!c.is('}')) {
      const key = c.peek();
      if (c.is('...')) throw unsupported('object spread', 'name every field', span(key, key));
      if (key.kind !== 'identifier') throw unsupported(`object key ${describe(key)}`, 'keys are identifiers', span(key, key));
      c.next();
      if (key.value === '$') {
        c.expect(':', 'object');
        const value = c.next();
        if (value.kind !== 'string') throw unsupported('computed tag', 'the $ tag is a string literal', span(value, value));
        tag = value.value;
      } else {
        if (Object.hasOwn(fields, key.value)) throw typeError(`duplicate field ${key.value}`, span(key, key));
        if (c.is('(')) throw unsupported('method in an object value', 'functions as values are outside the portable core', span(key, key));
        fields[key.value] = c.eat(':') ? this.expr() : this.reference(key);
      }
      if (!c.eat(',')) break;
    }
    c.expect('}', 'object');
    if (tag === null) throw unsupported('object without a $ tag', 'a portable object is an alternative of a @typedef data type', span(start, c.peek()));
    return { k: 'ctorObject', tag, fields, span: span(start, c.peek()) };
  }

  identifier() {
    const c = this.cursor;
    const token = c.peek();
    if (token.value === 'true' || token.value === 'false') {
      c.next();
      return { k: 'bool', value: token.value === 'true', span: span(token, token) };
    }
    if (['null', 'undefined', 'NaN', 'Infinity'].includes(token.value)) {
      throw unsupported(token.value, 'outside the portable core', span(token, token));
    }
    if (['this', 'super', 'new', 'function', 'class', 'import', 'arguments'].includes(token.value)) {
      throw unsupported(`${token.value} expression`, 'outside the portable core', span(token, token));
    }
    c.next();
    if (this.scope.locals.has(token.value) || this.scope.tdz.has(token.value)) return this.reference(token);
    if (this.assertion?.name === token.value) throw unsupported('assertion in an expression', 'assertions are top-level statements', span(token, token));
    // A global: a function, or a namespace path to a method, which must be called.
    const segments = [token.value];
    while (c.is('.') && c.peek(1).kind === 'identifier') {
      c.next();
      segments.push(c.next().value);
    }
    const name = segments.join('.');
    const where = span(token, c.peek());
    if (!c.is('(')) {
      throw unsupported(`function value ${name}`, 'functions and namespaces are only portable when a function is called', where);
    }
    const args = this.arguments(name);
    const called = span(token, c.peek());
    if (name === 'String' && args.length === 1) return { k: 'toString', arg: args[0], span: called };
    if (name === 'Object.freeze' && args.length === 1) return { ...args[0], span: called };
    if (GLOBALS.has(segments[0]) || name === 'String') {
      throw unsupported(name, 'the JavaScript standard library is outside the portable core', called);
    }
    if (!args.length) return { k: 'name', path: [ROOT, ...segments], span: called };
    return { k: 'app', fn: { k: 'name', path: [ROOT, ...segments], span: where }, args, span: called };
  }

  /** A local variable; reading one before its declaration throws in JavaScript. */
  reference(token) {
    if (this.scope.tdz.has(token.value)) {
      throw unsupported(`use of ${token.value} before its declaration`, 'the binding is in its temporal dead zone, where reading it throws a ReferenceError', span(token, token));
    }
    if (!this.scope.locals.has(token.value)) {
      throw unsupported(`function value ${token.value}`, 'functions are only portable when called', span(token, token));
    }
    return { k: 'name', path: [token.value], span: span(token, token) };
  }
}

/** The parameter a leading `if (n < 0n) throw …` restricts to the naturals. */
function guardedParameter(statement, params) {
  if (statement.s !== 'if' || statement.else) return null;
  const then = statement.then.filter((inner) => inner.s !== 'empty');
  if (then.length !== 1 || then[0].s !== 'throw') return null;
  const { cond } = statement;
  if (cond.k !== 'binary') return null;
  const isZero = (node) => node.k === 'num' && node.value === '0';
  let variable = null;
  if (cond.op === 'lt' && isZero(cond.right)) variable = cond.left;
  if (cond.op === 'gt' && isZero(cond.left)) variable = cond.right;
  if (variable?.k !== 'name' || variable.path.length !== 1) return null;
  const param = params.find((candidate) => candidate.name === variable.path[0]);
  return param && param.type === INT ? param : null;
}

/**
 * JSDoc tags of a comment: `@tag {type} name`. Leading `*` of each line is
 * decoration.
 */
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
    const name = /^[ \t]*([A-Za-z_$][\w$]*)/u.exec(text.slice(index))?.[1] ?? null;
    tags.push({ tag: match[1], type, name });
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
    const value = substituteFields(node.value, subject, binderFor);
    return { ...node, value, body: node.name === subject ? node.body : substituteFields(node.body, subject, binderFor) };
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

function joined(left, right, token) {
  return { start: left.span?.start ?? left.start ?? token.start, end: right.span?.end ?? right.end ?? token.end };
}

function span(from, to) {
  return { start: from.start, end: Math.max(from.end, to && to.kind !== 'eof' ? to.start : from.end) };
}
