// Bulk grammar diagnostics retain the parser's original rejection and limits.
// Resource failures do not have a source location.
export function describeGrammarRejection(rejection) {
  const bounded = value => String(value?.message ?? value).split('\n')[0].slice(0, 300);
  if (!rejection?.reason) return bounded(rejection?.message ?? JSON.stringify(rejection));
  const located = Number.isSafeInteger(rejection.line) && rejection.line > 0
    && Number.isSafeInteger(rejection.column) && rejection.column > 0;
  const location = located ? ` at ${rejection.line}:${rejection.column}` : '';
  const limited = Number.isSafeInteger(rejection.limit) && rejection.limit >= 0;
  const limit = limited ? ` (limit ${rejection.limit})` : '';
  const expected = rejection.expected?.length ? `, expected ${rejection.expected.join(' or ')}` : '';
  return bounded(`${rejection.reason}${location}${limit}${expected}`);
}
