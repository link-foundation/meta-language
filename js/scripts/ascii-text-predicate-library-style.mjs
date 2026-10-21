// Source qualification only. Every complete generated owner, signature and
// body stays exact; the original JavaScript decisions remain the semantics.
import { tokenize } from '../src/translation/lexer.js';
import { readRustLibraryItems } from './rust-library-packaging.mjs';

export function readAsciiTextPredicateLibraryStyles(source, { rustVersionRange = '>=1.90.0 <1.100.0' } = {}) {
  if (rustVersionRange !== '>=1.90.0 <1.100.0') return { source, actions: [] };
  const { items } = readRustLibraryItems(source);
  // Unresolved imports or a distinct primitive type owner cannot borrow the
  // actual standard primitive const qualification from this generated family.
  if (items.some(item => item.kind === 'use'
    || (['type', 'struct', 'enum', 'mod'].includes(item.kind) && ['f64', 'bool'].includes(item.name)))) {
    return { source, actions: [] };
  }
  const specifications = [
    { name: 'accept_rust_const_function_interval',
      declaration: 'accept_rust_const_function_interval(start:f64,end:f64,nonempty:bool)->bool{(start<=end)&&(!nonempty||(start<end))}' },
    { name: 'accept_rust_const_function_finite_literal',
      declaration: 'accept_rust_const_function_finite_literal(value:f64)->bool{value.is_finite()}' },
  ];
  const actions = [];
  let current = source;
  for (const specification of specifications) {
    const owners = items.filter(item => item.kind === 'fn' && item.name === specification.name);
    if (owners.length !== 1) continue;
    const owner = owners[0], tokens = tokenize(owner.source, 'Rust').tokens;
    const at = tokens.findIndex((token, index) => token.value === 'fn' && tokens[index + 1]?.value === specification.name);
    if (at < 1) continue;
    const constant = tokens[at - 1].value === 'const';
    if (tokens[at - (constant ? 2 : 1)]?.value !== 'pub') continue;
    const declaration = tokens.slice(at + 1).filter(token => token.kind !== 'eof').map(token => token.raw).join('');
    if (declaration !== specification.declaration) continue;
    if (constant) continue;
    const replacement = owner.source.slice(0, tokens[at].start) + 'const ' + owner.source.slice(tokens[at].start);
    if (current.split(owner.source).length !== 2) continue;
    actions.push({ op: 'replace', field: 'source', from: owner.source, to: replacement });
    current = current.replace(owner.source, () => replacement);
  }
  return { source: current, actions };
}
