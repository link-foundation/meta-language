// The existing bounded evaluator for actual private formal output IR.
// It observes recorded constructors, child order and real function calls; it
// is test support and does not claim general source or target interpretation.
import assert from 'node:assert/strict';

export function observeThreadedOptionalProgram(program) {
  const lines = [];
  const evaluate = (node, env) => {
    switch (node.k) {
      case 'var': return env.get(node.name);
      case 'lit': return node.type.kind === 'float' ? Number(node.value) : node.type.kind === 'bool' ? node.value : node.value;
      case 'unit': return undefined;
      case 'none': return node.absence === 'null' ? null : undefined;
      case 'some': case 'flattenOption': return evaluate(node.value, env);
      case 'unwrapOption': return evaluate(node.arg, env);
      case 'coalesce': return evaluate(node.left, env) ?? evaluate(node.right, env);
      case 'strictEqual': return (evaluate(node.left, env) === evaluate(node.right, env)) !== node.negate;
      case 'binary':
        if (node.op === 'and') return evaluate(node.left, env) && evaluate(node.right, env);
        if (node.op === 'or') return evaluate(node.left, env) || evaluate(node.right, env);
        throw new Error('unexpected output control binary ' + node.op);
      case 'if': return evaluate(evaluate(node.cond, env) ? node.then : node.else, env);
      case 'let': { const scope = new Map(env); scope.set(node.name, evaluate(node.value, env)); return evaluate(node.body, scope); }
      case 'toString': case 'optionalText': return String(evaluate(node.arg, env));
      case 'outNil': return [];
      case 'outCons': return [evaluate(node.head, env), ...evaluate(node.tail, env)];
      case 'record': return Object.fromEntries(node.entries.map(entry => [entry.name, evaluate(entry.value, env)]));
      case 'recordField': return evaluate(node.object, env)[node.field];
      case 'ctor': return { tag: node.ctor, values: node.args.map(arg => evaluate(arg, env)) };
      case 'match': {
        const value = evaluate(node.scrutinee, env);
        const selected = node.cases.find(branch => branch.pattern.ctor === value.tag);
        assert.ok(selected, 'the actual output pair has one matching constructor');
        const scope = new Map(env);
        selected.pattern.binds.forEach((name, index) => scope.set(name, value.values[index]));
        return evaluate(selected.body, scope);
      }
      case 'call': {
        const declaration = program.declarations.get(node.fn);
        assert.ok(declaration, 'every actual output call resolves to a retained declaration');
        const values = node.args.map(arg => evaluate(arg, env));
        return evaluate(declaration.body, new Map(declaration.params.map((param, index) => [param.name, values[index]])));
      }
      default: throw new Error('unexpected output control node ' + node.k);
    }
  };
  const env = new Map();
  try {
    for (const effect of program.main.effects) {
      if (effect.k === 'let') env.set(effect.name, evaluate(effect.value, env));
      else if (effect.k === 'print') lines.push(evaluate(effect.expr, env));
      else if (effect.k === 'output') lines.push(...evaluate(effect.expr, env).toReversed());
      else if (effect.k === 'unwrap') {
        const pair = evaluate(effect.pair, env);
        if (pair.tag === effect.ctors[1]) throw new Error(pair.values[1]);
        assert.equal(pair.tag, effect.ctors[0]);
        env.set(effect.name, pair.values[1]);
      } else throw new Error('unexpected output control effect ' + effect.k);
    }
    return { lines, error: '' };
  } catch (error) { return { lines, error: error.message }; }
}
