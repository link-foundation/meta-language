// Experimental packaging for the closed checked decimal generator, not a
// general Rust source rewriter. Preserve every algorithm token; qualify only
// crate-root paths whose declarations belong to this generated program.
import { tokenize } from '../js/src/translation/lexer.js';

export function qualifyGeneratedRustRootPaths(source, namespace) {
  if (typeof source !== 'string') throw new TypeError('a generated program must be source text');
  if (typeof namespace !== 'string' || !/^ml_[a-z][a-z0-9_]*$/u.test(namespace)) {
    throw new TypeError('a generated namespace must be a reserved ml_ identifier');
  }
  const { tokens } = tokenize(source, 'Rust');
  const roots = new Set();
  let depth = 0;
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    // This lexer does not admit raw Rust strings. Their embedded quotes could
    // otherwise expose literal text as paths, so reject that wider input scope.
    if (token.kind === 'identifier' && ['r', 'br'].includes(token.value)
      && (tokens[index + 1]?.value === '#' || tokens[index + 1]?.kind === 'string')) {
      throw new TypeError('the closed generated program does not admit raw string literals');
    }
    if (!depth && token.kind === 'identifier' && ['fn', 'mod', 'struct', 'enum', 'type', 'const', 'static'].includes(token.value)
      && tokens[index + 1]?.kind === 'identifier') {
      const name = token.value === 'static' && tokens[index + 1].value === 'mut' ? tokens[index + 2] : tokens[index + 1];
      if (name?.kind === 'identifier' && !(token.value === 'const' && name.value === 'fn')) roots.add(name.value);
    }
    if (token.kind === 'punct' && token.value === '{') depth += 1;
    else if (token.kind === 'punct' && token.value === '}') depth -= 1;
    if (depth < 0) throw new TypeError('a generated program has unbalanced module scope');
  }
  if (depth !== 0) throw new TypeError('a generated program has unclosed module scope');
  let output = '', cursor = 0;
  for (let index = 0; index < tokens.length - 2; index += 1) {
    if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'crate' || tokens[index + 1].value !== '::') continue;
    const next = tokens[index + 2];
    if (next.kind !== 'identifier' || !roots.has(next.value)) {
      throw new TypeError('a generated program has an external crate-root dependency');
    }
    const end = tokens[index + 1].end;
    output += source.slice(cursor, end) + namespace + '::';
    cursor = end;
  }
  return output + source.slice(cursor);
}

export function wrapGeneratedRustNamespace(source, namespace = 'ml_decimal_draft') {
  return `pub mod ${namespace} {\n${qualifyGeneratedRustRootPaths(source, namespace)}\n}\n`;
}
