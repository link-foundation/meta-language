// The selected reverse boundary reads actual emitted scalar state loops.
import {unsupported} from './diagnostics.js';
import {RUST_SCALAR_LOOP_POLICY, checkRustScalarLoopProof} from './rust-scalar-loops.js';

const policy = RUST_SCALAR_LOOP_POLICY;
const fail = (reason, span) => { throw unsupported('JavaScript scalar loop', reason, span); };
const range = (first, last) => ({start: first.start, end: last.end});
const snapshot = name => new RegExp(policy.snapshotBindingPattern, 'u').test(name);

export function isJavaScriptScalarLoop(cursor) {
  return ['{', 'for', '(', ';', ';', ')', '{'].every((value, index) => cursor.peek(index).value === value);
}

export function finishJavaScriptScalarLoops(program) {
  function items(values) {
    for (const item of values) {
      if (item.k === 'module') items(item.items);
      else if (item.sourceLoop) {
        item.sourceLoop.interface = {model: policy.javaScriptInterfaceModel,
          parameters: item.params.map(param => ({kind: param.type.kind})), result: {kind: item.ret.kind}};
        checkRustScalarLoopProof(item, 'JavaScript');
      }
    }
  }
  items(program.items);
}

export function readJavaScriptScalarLoop(parser, name, params, isAsync) {
  if (isAsync || params.some(param => param.default != null || param.optional)) fail('loop parameters are direct fixed-arity Number or Boolean values', parser.cursor.peek());
  const c = parser.cursor, open = c.expect('{', 'scalar loop function');
  c.expect('for', 'scalar loop'); c.expect('(', 'scalar loop');
  c.expect(';', 'scalar loop'); c.expect(';', 'scalar loop'); c.expect(')', 'scalar loop');
  const state = params.map(param => param.name), known = new Set(state), declared = [];
  const proof = {model: policy.model, language: 'JavaScript', sourceName: name, lowering: policy.lowering,
    state: state.map(name => ({name, mutable: true})), transitions: []};

  function block() {
    const start = c.expect('{', 'scalar loop block'), outer = parser.scope;
    parser.scope = {locals: new Set(outer.locals), tdz: new Set(), mutable: new Set(outer.mutable)};
    try {
      const bindings = [];
      while (c.eat('const')) {
        const token = c.identifier('scalar loop binding');
        if (known.has(token.value) || /^ml_/u.test(token.value) && !snapshot(token.value)) fail('loop locals have distinct identities; reserved names require exact write snapshots', token);
        known.add(token.value);
        declared.push(token.value);
        c.expect('=', 'scalar loop binding');
        const value = parser.expr();
        c.expect(';', 'scalar loop binding');
        parser.declareLocal(token.value);
        bindings.push({name: token.value, type: null, value, span: range(token, c.peek())});
      }
      let result;
      if (c.eat('return')) { result = parser.expr(); c.expect(';', 'scalar loop return'); }
      else if (c.is('if')) result = conditional();
      else if (c.is('[') || c.is('continue')) result = continuation();
      else fail('every loop path explicitly returns a scalar or continues owned state', c.peek());
      const close = c.expect('}', 'scalar loop block');
      return bindings.reduceRight((body, binding) => ({k: 'let', ...binding, body, span: range(start, close)}), result);
    } finally { parser.scope = outer; }
  }
  function conditional() {
    const start = c.expect('if', 'scalar loop condition'); c.expect('(', 'scalar loop condition');
    const cond = parser.expr(); c.expect(')', 'scalar loop condition');
    const then = block(); c.expect('else', 'scalar loop branches');
    const otherwise = c.is('if') ? conditional() : block();
    return {k: 'if', cond, then, else: otherwise, span: range(start, c.peek())};
  }
  function continuation() {
    const start = c.peek(), arguments_ = [...state], writes = [];
    if (c.eat('[')) {
      for (const [index, target] of state.entries()) {
        if (index) c.expect(',', 'scalar state targets');
        if (c.identifier('scalar state target').value !== target) fail('state targets retain the exact header order', start);
      }
      c.expect(']', 'scalar state targets'); c.expect('=', 'scalar state snapshots'); c.expect('[', 'scalar state snapshots');
      for (const [index, target] of state.entries()) {
        if (index) c.expect(',', 'scalar state snapshots');
        const binding = c.identifier('scalar state snapshot').value;
        if (binding !== target) {
          if (!snapshot(binding) || !parser.scope.locals.has(binding)) fail('each changed slot reads its actual immutable write snapshot', start);
          writes.push({target, binding}); arguments_[index] = binding;
        }
      }
      c.expect(']', 'scalar state snapshots'); c.expect(';', 'scalar state snapshots');
      writes.sort((left, right) => declared.indexOf(left.binding) - declared.indexOf(right.binding));
    }
    c.expect('continue', 'scalar continuation'); c.expect(';', 'scalar continuation');
    proof.transitions.push({writes, arguments: arguments_});
    return {k: 'app', fn: {k: 'name', path: [name]}, args: arguments_.map(value => ({k: 'name', path: [value]})), span: range(start, c.peek())};
  }
  const body = block(); c.expect('}', 'scalar loop function');
  return {body, proof, span: range(open, c.peek())};
}
