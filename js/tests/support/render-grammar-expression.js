// Runtime-neutral rendering of grammar IR rules. The Rust twin lives in
// rust/tests/unit/grammar_render.rs; both render the expected `expressions`
// recorded in the shared grammar parity fixtures.

const quote = (value) => JSON.stringify(value);

export function renderGrammarRule(rule) {
  return `${rule.kind} ${renderGrammarExpression(rule.expression)}`;
}

export function renderGrammarExpression(expression) {
  const inner = (item) => renderGrammarExpression(item);
  const list = (items) => items.map(inner).join(', ');
  switch (expression.kind) {
    case 'empty': return 'empty';
    case 'any': return 'any';
    case 'literal': return `literal(${quote(expression.value)})`;
    case 'literalInsensitive': return `literalInsensitive(${quote(expression.value)})`;
    case 'charRange': return `range(${quote(expression.start)}, ${quote(expression.end)})`;
    case 'charClass':
      return `${expression.negated ? 'notClass' : 'class'}(${expression.items.map((item) =>
        item.kind === 'range'
          ? `range(${quote(item.start)}, ${quote(item.end)})`
          // `category` and `script` items name Unicode properties.
          : `${item.kind}(${quote(item.value)})`).join(', ')})`;
    case 'ref': return `ref(${expression.name})`;
    case 'choice': return `${expression.ordered ? 'orderedChoice' : 'choice'}(${list(expression.items)})`;
    case 'seq': return `seq(${list(expression.items)})`;
    case 'optional': case 'repeat0': case 'repeat1': case 'and': case 'not':
      return `${expression.kind}(${inner(expression.item)})`;
    case 'repeat':
      return `repeat(${inner(expression.item)}, ${expression.min}, ${expression.max ?? 'unbounded'})`;
    case 'capture':
      return `capture(${expression.label === null ? 'null' : quote(expression.label)}, ${inner(expression.item)})`;
    default: throw new TypeError(`unrenderable grammar expression kind ${expression.kind}`);
  }
}
