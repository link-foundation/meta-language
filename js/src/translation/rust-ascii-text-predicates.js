// Actual source-owner/token transport only; all admissions and sets are shared.
import { unsupported } from './diagnostics.js';
import { BOOL } from './types.js';
import { acceptRustLiteralMatchReceiver, acceptRustLiteralMatchCatalogOwner } from './rust-literal-match-rules.js';
import { planRustAsciiTextQuery, planRustAsciiCharacterCallback, acceptRustAsciiCharacterOwner } from './ascii-text-predicate-rules.js';
import { createAsciiTextPredicateExpression } from './ascii-text-predicate-expression.js';
import { acceptRustTypeNamespaceMethodDiscovery, acceptRustTypeNamespaceBodyMethod } from './rust-type-namespace-rules.js';

function sourceShadowed(parser, spelling, path) {
  const resolved = parser.resolve([spelling], path);
  return { resolved: resolved.join('::'), shadowed: [
    parser.origins.lookup([spelling], path, ['data']), parser.origins.lookup(resolved, path, ['data']),
  ].some(owner => acceptRustLiteralMatchCatalogOwner(owner?.k ?? '')) };
}

/** Runs after actual .METHOD, before ordinary argument parsing. Unrelated
 * syntax returns null; a selected but unsupported callback/owner refuses. */
export function readRustAsciiTextPredicate(parser, receiver, method, path) {
  // Route only possible query spellings before projecting whole token columns.
  // This rejects no selected syntax and grants no source-owner admission.
  if (method.value !== 'starts_with' && method.value !== 'chars') return null;
  const c = parser.cursor;
  const kinds = c.tokens.map(token => token.kind), values = c.tokens.map(token => token.value);
  const query = planRustAsciiTextQuery(method.value, kinds, values, c.index);
  if (!query.length) return null;
  const discovery = acceptRustTypeNamespaceMethodDiscovery(parser.typeNamespaceDiscovery, method.value);
  if (!acceptRustTypeNamespaceBodyMethod(Boolean(parser.typeNamespaceModel) && !discovery, method.value, false)) {
    throw unsupported('Rust type namespace body method', `${method.value} requires a separate actual method owner proof`, method);
  }
  c.index = query[1];
  const selected = c.peek();
  // Use the canonical lexer's scalar rather than decoding source characters.
  const characters = c.tokens.map(token => token.kind === 'char' ? Number(token.value) : -1);
  const plan = planRustAsciiCharacterCallback(kinds, values, characters, c.index);
  if (!plan.length) throw unsupported('Rust ASCII character predicate',
    'the selected standard query needs a closed pure ASCII character predicate', selected);
  const range = { start: receiver.span?.start ?? method.start, end: c.tokens[plan[0]].end };
  const origin = parser.origins.origin(receiver, path);
  const text = sourceShadowed(parser, 'str', origin.path);
  if ((origin.proven || !parser.origins.collecting) && !acceptRustLiteralMatchReceiver(
    origin.type?.kind ?? '', origin.textSourceModel ?? '', origin.proven, origin.wrappers.join('|'),
    text.shadowed || text.resolved !== 'str')) throw unsupported('Rust ASCII text receiver',
    'the actual receiver needs its proven unshadowed immutable borrowed str source owner', range);
  const character = sourceShadowed(parser, 'char', path);
  if (!acceptRustAsciiCharacterOwner(plan[1] >= 0, character.resolved, character.shadowed)) {
    throw unsupported('Rust ASCII character owner', 'the annotation must denote the actual primitive char', range);
  }
  c.index = plan[0] + 1;
  return parser.origins.mark(createAsciiTextPredicateExpression(parser, receiver,
    query[0] === 1 ? 'first' : 'all', plan.slice(2), range), parser.origins.typeOrigin(BOOL, path));
}
