// Actual token/source-owner transport into the complete shared pattern plan.
import { unsupported } from './diagnostics.js';
import { BOOL } from './types.js';
import { acceptRustLiteralMatchReceiver, acceptRustLiteralMatchCatalogOwner, planRustLiteralMatchPattern } from './rust-literal-match-rules.js';

export function readRustLiteralMatch(parser, token, path) {
  const c = parser.cursor;
  const close = { '(': ')', '[': ']', '{': '}' }[c.peek().value];
  if (!close) throw parser.fail('expected ( after matches!');
  c.next();
  const receiver = parser.expr(path);
  c.expect(',', 'matches!');
  const plan = planRustLiteralMatchPattern(c.tokens.map(item => item.kind),
    c.tokens.map(item => item.value), c.index, close);
  if (!plan.length) throw unsupported('Rust literal matches!',
    'only a nonempty string-literal alternative list without guards or bindings is represented',
    { start: token.start, end: c.peek().end });
  const range = { start: token.start, end: c.tokens[plan[0] - 1].end };
  const origin = parser.origins.origin(receiver, path);
  const resolved = parser.resolve(['str'], origin.path);
  const shadowed = [parser.origins.lookup(['str'], origin.path, ['data']),
    parser.origins.lookup(resolved, origin.path, ['data'])]
    .some(owner => acceptRustLiteralMatchCatalogOwner(owner?.k ?? '')) || resolved.join('::') !== 'str';
  if ((origin.proven || !parser.origins.collecting) && !acceptRustLiteralMatchReceiver(origin.type?.kind ?? '',
    origin.textSourceModel ?? '', origin.proven, origin.wrappers.join('|'), shadowed)) {
    throw unsupported('Rust literal matches! receiver',
      'the scrutinee needs its actual proven unshadowed borrowed str source owner', range);
  }
  c.index = plan[0];
  let count = 0, name;
  do { name = 'rust_literal_match_value' + count++; } while (parser.reservedNames.has(name));
  parser.reservedNames.add(name);
  const alternatives = plan.slice(1).map(index => {
    const literal = c.tokens[index];
    return { k: 'binary', op: 'eq', left: { k: 'name', path: [name], span: receiver.span ?? range },
      right: { k: 'str', value: literal.value, span: { start: literal.start, end: literal.end } }, span: range };
  });
  const body = alternatives.reduce((left, right) => ({ k: 'binary', op: 'or', left, right, span: range }));
  // Binding the scrutinee preserves one evaluation even when an earlier
  // alternative succeeds. Only immutable string values are compared below it.
  return parser.origins.mark({ k: 'let', name, value: receiver, body, span: range },
    parser.origins.typeOrigin(BOOL, path));
}
