// Direct Surface inputs deliberately bypass parser inference. An unannotated
// numeral must retain the checker's BigInt default rather than a made-up Float
// return context; only callback parameters come from the surrounding operation.
const integer = () => ({ k: 'num', value: '7' });
const scalar = value => ({ k: 'num', value: String(value), type: { kind: 'float' } });
const name = value => ({ k: 'name', path: [value] });
const lambda = () => ({ k: 'lambda', params: [{ name: 'element', span: { start: 30, end: 37 } }],
  captures: [], body: integer(), span: { start: 30, end: 45 } });

export function callableExpectedContextSurface(callback, items = []) {
  return { language: 'JavaScript', items, main: { effects: [
    { k: 'let', name: 'mapped', value: { k: 'arrayFrom', args: [
      { k: 'array', items: [scalar(1), scalar(2)].map(value => ({ value, spread: false })) }, callback,
    ], span: { start: 10, end: 90 } } },
    { k: 'print', style: 'js-console', expr: { k: 'length', object: name('mapped'), integer: false } },
  ] } };
}

export function callableExpectedContextCases() {
  const wrappers = [
    ['lambda', value => value, ['2']],
    ['if', value => ({ k: 'if', cond: { k: 'bool', value: true }, then: value, else: lambda() }), ['2']],
    ['let', value => ({ k: 'let', name: 'temporary', value: { k: 'bool', value: true }, body: value }), ['2']],
    ['print', value => ({ k: 'print', style: 'js-console', expr: { k: 'str', value: 'created' }, body: value }), ['created', '2']],
    ['cellWrite', value => ({ k: 'let', name: 'cell', value: { k: 'cellAllocate', value: scalar(0) }, body: {
      k: 'cellWrite', cell: name('cell'), value: scalar(1), body: value,
    } }), ['2']],
    ['match', value => ({ k: 'match', scrutinees: [integer()], rows: [{ patterns: [{ k: 'wild' }], body: value }] }), ['2']],
    // Minimal old bug: Match1 kept the partial expected Function as its result,
    // then coerced this valid one-parameter Lambda to a type with no return.
    ['match1', value => ({ k: 'match1', scrutinee: integer(), cases: [{ pattern: { k: 'wild' }, body: value }] }), ['2']],
  ];
  const cases = wrappers.map(([id, wrap, lines]) => ({ id, surface: callableExpectedContextSurface(wrap(lambda())), lines }));
  cases.push({ id: 'name', surface: callableExpectedContextSurface(name('callback'), [{
    k: 'fn', name: 'callback', params: [{ name: 'element', type: { kind: 'float' } }],
    ret: { kind: 'int' }, body: integer(),
  }]), lines: ['2'] });
  return cases;
}

export function callableExpectedContextSummary(program) {
  const value = program.main.effects[0].value;
  const callables = [];
  const visit = node => {
    if (!node || typeof node !== 'object') return;
    if (node.k === 'lambda') callables.push({ k: node.k, type: node.type, params: node.params });
    if (node.k === 'functionRef') callables.push({ k: node.k, type: node.type, fn: node.fn });
    for (const [key, child] of Object.entries(node)) if (key !== 'type' && key !== 'span') {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(value);
  return { type: value.type, callables };
}
