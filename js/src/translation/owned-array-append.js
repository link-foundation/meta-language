// Source occurrence transport for the shared fresh-local construction proof.
// Every accepted reader refers to an inspected declaration, never its spelling.
import {
  acceptOwnedArrayAppendShape, acceptOwnedArrayRemovalShape, readOwnedArrayRemovalSliceBounds, acceptOwnedArrayAppendUses, acceptOwnedArrayRead,
  acceptOwnedArrayReaderResult,
} from './owned-array-append-rules.js';
import { FLOAT } from './types.js';

export function readOwnedArrayReaderParameters(declaration, readers, statements = null) {
  if (!acceptOwnedArrayReaderResult(declaration.ret?.kind ?? '')) return new Set();
  const admitted = new Set();
  for (const [index, parameter] of declaration.params.entries()) {
    let safe = true;
    const visit = (value, context = '', operation = '', reader = false, captured = false, locals = new Set(declaration.params.map(param => param.name))) => {
      if (!value || typeof value !== 'object') return;
      if (Array.isArray(value)) { value.forEach(child => visit(child, context, operation, reader, captured, locals)); return; }
      if (value.k === 'name' && value.path.length === 1 && value.path[0] === parameter.name) {
        if (captured || !acceptOwnedArrayRead(context, operation, reader)) safe = false;
        return;
      }
      if (value.k === 'app') {
        visit(value.fn, '', '', false, captured, locals);
        const target = value.fn.k === 'name' && value.fn.path.length === 2 && value.fn.path[0] === 'crate' ? value.fn.path[1] : null;
        value.args.forEach((argument, at) => visit(argument, 'argument', '', readers.get(target)?.has(at) === true && !locals.has(target), captured, locals));
        return;
      }
      if (value.k === 'let') {
        visit(value.value, '', '', false, captured, locals);
        // A rebound parameter requires a separate binding identity. Refuse
        // this reader summary instead of conflating its two occurrences.
        if (value.name === parameter.name) safe = false;
        visit(value.body, '', '', false, captured, new Set([...locals, value.name])); return;
      }
      if (value.k === 'index') {
        visit(value.object, 'index-receiver', '', false, captured, locals); visit(value.index, '', '', false, captured, locals); return;
      }
      if (value.k === 'field') {
        visit(value.object, value.field === 'length' ? 'length-receiver' : '', '', false, captured, locals); return;
      }
      if (value.k === 'method' || value.k === 'stringTest') {
        visit(value.object, 'method-receiver', value.op, false, captured, locals);
        (value.args ?? [value.search]).forEach(argument => visit(argument, '', '', false, captured, locals)); return;
      }
      for (const [field, child] of Object.entries(value)) {
        if (!['span', 'type', 'ret', 'from', 'to'].includes(field)) visit(child, '', '', false, captured || value.k === 'lambda', locals);
      }
    };
    const source = (entries, locals) => {
      for (const statement of entries) {
        const expression = value => visit(value, '', '', false, false, locals);
        if (statement.s === 'const' || statement.s === 'let') {
          expression(statement.value);
          if (statement.name === parameter.name) safe = false;
          locals.add(statement.name);
        } else if (statement.s === 'assign') {
          expression(statement.value);
          if (statement.name === parameter.name) safe = false;
        } else if (statement.s === 'block') source(statement.body, statement.inline ? locals : new Set(locals));
        else if (statement.s === 'if') {
          expression(statement.cond);
          source(statement.then, new Set(locals));
          if (statement.else) source(statement.else, new Set(locals));
        } else if (statement.s === 'while' || statement.s === 'doWhile') {
          expression(statement.cond); source(statement.body, new Set(locals));
        } else if (statement.s === 'for') {
          const inner = new Set(locals);
          source(statement.init, inner);
          visit(statement.cond, '', '', false, false, inner);
          source(statement.body, new Set(inner)); source(statement.update, inner);
        } else if (statement.s === 'switch') {
          expression(statement.discriminant);
          for (const clause of statement.clauses) {
            clause.tests.forEach(expression); source(clause.body, new Set(locals));
          }
        } else expression(statement.expr ?? statement);
      }
    };
    if (statements) source(statements, new Set(declaration.params.map(param => param.name)));
    else visit(declaration.body);
    if (safe) admitted.add(index);
  }
  return admitted;
}

/** Rebind a proven private source construction as an ordinary pure value. */
export function lowerOwnedArrayAppends(statements, parameters, readers) {
  const proofs = readOwnedArrayAppendProofs(statements, parameters, readers);
  for (const proof of proofs) {
    const { statement, name } = proof;
    const expression = statement.expr;
    // This interval ends before the first argument, so an unrelated nested
    // mutating call inside that argument can never be hidden by this proof.
    if (expression.object.span && expression.args[0]?.span) proof.operationSpan = {
      start: expression.object.span.end, end: expression.args[0].span.start,
    };
    if (expression.op === 'pop' && expression.object.span && expression.span) proof.operationSpan = {
      start: expression.object.span.end, end: expression.span.end,
    };
    statement.s = 'assign';
    statement.name = name;
    statement.value = expression.op === 'pop' ? { k: 'method', op: 'slice', object: expression.object,
      args: readOwnedArrayRemovalSliceBounds().map(value => ({ k: 'num', value: String(Math.abs(value)),
        ...(value < 0 ? { negative: true } : {}), type: FLOAT, span: expression.span })), span: expression.span } : { k: 'array', items: [
      { spread: true, value: expression.object },
      ...expression.args.map(value => ({ value })),
    ], span: expression.span };
    delete statement.expr;
  }
  return proofs;
}

export function readOwnedArrayAppendProofs(statements, parameters = [], readers = new Map()) {
  const allocations = [];
  const appends = [];
  let position = 0;
  let nextLoop = 0;
  const advance = () => position++;
  const escape = (allocation, loops) => allocation.escapes.push({ position: advance(), loops: [...loops] });
  const read = (value, scope, loops, context = '', operation = '', reader = false, captured = false) => {
    if (!value || typeof value !== 'object') return;
    if (Array.isArray(value)) { value.forEach(child => read(child, scope, loops, context, operation, reader, captured)); return; }
    if (value.k === 'name' && value.path.length === 1) {
      const allocation = scope.get(value.path[0]);
      if (allocation && (captured || !acceptOwnedArrayRead(context, operation, reader))) escape(allocation, loops);
      return;
    }
    if (value.k === 'app') {
      read(value.fn, scope, loops, '', '', false, captured);
      const target = value.fn.k === 'name' && value.fn.path.length === 2 && value.fn.path[0] === 'crate' ? value.fn.path[1] : null;
      value.args.forEach((argument, index) => read(argument, scope, loops, 'argument', '', readers.get(target)?.has(index) === true && !scope.has(target), captured));
      return;
    }
    if (value.k === 'index') {
      read(value.object, scope, loops, 'index-receiver', '', false, captured);
      read(value.index, scope, loops, '', '', false, captured); return;
    }
    if (value.k === 'field') { read(value.object, scope, loops, value.field === 'length' ? 'length-receiver' : '', '', false, captured); return; }
    if (value.k === 'method' || value.k === 'stringTest') {
      read(value.object, scope, loops, 'method-receiver', value.op, false, captured);
      (value.args ?? [value.search]).forEach(argument => read(argument, scope, loops, '', '', false, captured)); return;
    }
    for (const [field, child] of Object.entries(value)) {
      if (!['span', 'type', 'ret', 'from', 'to'].includes(field)) read(child, scope, loops, '', '', false, captured || value.k === 'lambda');
    }
  };
  const list = (entries, scope, loops) => {
    for (const statement of entries) {
      if (statement.s === 'const' || statement.s === 'let') {
        read(statement.value, scope, loops);
        if (statement.value?.k === 'array') {
          const allocation = { id: allocations.length, name: statement.name, literal: statement.value, loops: [...loops], position: advance(), escapes: [] };
          allocations.push(allocation); scope.set(statement.name, allocation);
        } else scope.set(statement.name, null);
      } else if (statement.s === 'expr' && (acceptOwnedArrayAppendShape(statement.s, statement.expr.k, statement.expr.op ?? '', statement.expr.object?.k ?? '', statement.expr.object?.path?.length ?? 0, statement.expr.args?.length ?? 0)
        || acceptOwnedArrayRemovalShape(statement.s, statement.expr.k, statement.expr.op ?? '', statement.expr.object?.k ?? '', statement.expr.object?.path?.length ?? 0, statement.expr.args?.length ?? 0))) {
        const allocation = scope.get(statement.expr.object.path[0]);
        statement.expr.args.forEach(argument => read(argument, scope, loops));
        if (allocation) appends.push({ statement, allocation, loops: [...loops], position: advance() });
      } else if (statement.s === 'assign') {
        read(statement.value, scope, loops);
        const allocation = scope.get(statement.name);
        if (allocation) escape(allocation, loops);
      } else if (statement.s === 'return' || statement.s === 'throw' || statement.s === 'break' || statement.s === 'continue') {
        // No owner reaches another construction after this control transfer.
        break;
      } else if (statement.s === 'block') list(statement.body, statement.inline ? scope : new Map(scope), loops);
      else if (statement.s === 'if') {
        read(statement.cond, scope, loops); list(statement.then, new Map(scope), loops); if (statement.else) list(statement.else, new Map(scope), loops);
      } else if (statement.s === 'while' || statement.s === 'doWhile' || statement.s === 'for') {
        const inner = new Map(scope); const nested = [...loops, nextLoop++];
        if (statement.init) list(statement.init, inner, nested);
        read(statement.cond, inner, nested); list(statement.body, inner, nested); if (statement.update) list(statement.update, inner, nested);
      } else if (statement.s === 'switch') {
        read(statement.discriminant, scope, loops);
        for (const clause of statement.clauses) { clause.tests.forEach(test => read(test, scope, loops)); list(clause.body, new Map(scope), loops); }
      } else {
        read(statement, scope, loops);
      }
    }
  };
  list(statements, new Map(parameters.map(parameter => [parameter.name, null])), []);
  return appends.filter(append => {
    const loops = [], starts = [], counts = [], positions = [];
    for (const entry of append.allocation.escapes) { starts.push(loops.length); counts.push(entry.loops.length); loops.push(...entry.loops); positions.push(entry.position); }
    return acceptOwnedArrayAppendUses(append.allocation.loops, append.loops, loops, starts, counts, positions, append.position);
  }).map(append => ({ statement: append.statement, allocation: append.allocation.literal, name: append.allocation.name,
    facts: { allocationLoops: append.allocation.loops, appendLoops: append.loops, escapeLoops: append.allocation.escapes.map(escape => escape.loops), escapePositions: append.allocation.escapes.map(escape => escape.position), appendPosition: append.position } }));
}
