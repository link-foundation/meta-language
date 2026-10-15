// Header/owner projection only. The existing complete-token Rust parser owns bodies.
import { acceptRustPureUnitNameCharacter } from '../../js/src/translation/rust-pure-unit-call-rules.js';
import { acceptRustRawBindingIdentifierName } from './rules.mjs';
import { checkGeneratedOwnedRustFields as fields } from '../generated-owned-rust/model.mjs';

export const fail = reason => { throw new TypeError('Experimental Rust raw binding identifiers: ' + reason); };
const roles = ['declaration', 'binding', 'literal'];
const owned = token => roles.some(role => Object.hasOwn(token, role));
export function validName(name, raw) {
  return typeof name === 'string' && acceptRustRawBindingIdentifierName(name, raw,
    [...name].every((letter, index) => acceptRustPureUnitNameCharacter(letter.codePointAt(0), index === 0)));
}
function groups(tokens) {
  const result = [];
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token.kind === 'identifier' && token.raw === 'r' && tokens[index + 1]?.raw === '#') {
      const hash = tokens[index + 1], name = tokens[index + 2];
      if (owned(token) || owned(hash) || name?.kind !== 'identifier' || token.span.end !== hash.span.start || hash.span.end !== name.span.start || !validName(name.raw, true)) fail('a raw name requires its three actual contiguous ASCII tokens');
      result.push({ tokens: [token, hash, name], tail: name, raw: true, name: name.raw }); index += 2;
    } else result.push({ tokens: [token], tail: token, raw: false, name: token.raw });
  }
  return result;
}
export function readRawBindingIdentifierSyntax(proof, policy, program = null) {
  const logical = groups(proof.tokens), functions = [], descriptors = [], assignments = new Map();
  let cursor = 0, mainSeen = false;
  const current = () => logical[cursor];
  const take = () => { if (!current()) fail('the complete source ended before its grammar'); return logical[cursor++]; };
  const syntax = (group, value) => group && !group.raw && group.tokens.length === 1 && !owned(group.tail) && group.tail.value === value;
  const literal = value => { const group = take(); if (!syntax(group, value)) fail('the complete grammar expected ' + value); return group; };
  const name = () => { const group = take(); if (group.tail.kind !== 'identifier' || !validName(group.name, group.raw)) fail('the actual binder requires an admitted identifier spelling'); return group; };
  const type = () => { const group = take(); if (!policy.types.some(value => syntax(group, value))) fail('only direct i32 and Boolean type tokens are admitted'); return group; };
  function alias(kind, value, group, owner, sourceName) {
    const index = descriptors.length, record = program ? proof.aliases[index] : null;
    if (program) {
      fields(record, policy.fields.alias);
      if (record.kind !== kind || record.value !== value || record.token !== group.tail || record.checkedOwner !== owner || group.tail[kind === 'declaration' ? 'declaration' : 'binding'] !== owner) fail('an alias inverse lost its actual source header or checked owner');
    }
    const semanticName = program ? owner?.name : sourceName;
    if (!validName(semanticName, true) || semanticName === 'main' && kind === 'declaration') fail('a checked alias owner has an invalid current semantic name');
    const descriptor = { kind, value, group, checkedOwner: owner, semanticName, record };
    descriptors.push(descriptor); assignments.set(group, descriptor); return descriptor;
  }
  while (current() && current().tail.kind !== 'eof') {
    const publicItem = syntax(current(), 'pub');
    if (publicItem) literal('pub');
    literal('fn'); const header = name();
    if (!header.raw && header.name === 'main') {
      if (publicItem || owned(header.tail)) fail('the ordinary empty entry is not a public value declaration alias');
      literal('('); literal(')'); literal('{'); literal('}');
      if (current()?.tail.kind !== 'eof') fail('the ordinary empty main must terminate the entire module');
      take(); mainSeen = true; break;
    }
    const ordinal = functions.length, entry = program?.items[ordinal];
    if (program && (!entry || entry.k !== 'fn' || program.declarations.get(entry.fullName) !== entry || entry.fullName !== entry.name)) fail('the complete header lost its actual flat declaration owner');
    const target = alias('declaration', policy.aliases.declarationPrefix + ordinal, header, entry, header.name);
    literal('('); const parameters = [];
    while (!syntax(current(), ')')) {
      const group = name(); literal(':'); type();
      const parameter = alias('parameter', policy.aliases.parameterPrefix + ordinal + '_' + parameters.length, group, entry?.params[parameters.length], group.name);
      if (parameters.some(row => row.semanticName === parameter.semanticName)) fail('raw and ordinary spellings cannot duplicate a parameter owner');
      parameters.push(parameter);
      if (!syntax(current(), ',')) break; literal(',');
    }
    literal(')'); literal('->'); type(); literal('{'); const body = [];
    while (current() && !syntax(current(), '}')) {
      const group = take();
      if (['{', ';'].some(value => syntax(group, value)) || group.tail.kind === 'eof') fail('nested blocks, statements and an incomplete body require another grammar');
      body.push(group);
    }
    literal('}');
    if (program && entry.params.length !== parameters.length) fail('the actual parameter array differs from its complete header');
    functions.push({ target, parameters, body });
  }
  if (cursor !== logical.length || !functions.length || !mainSeen || logical.at(-1)?.tail.kind !== 'eof') fail('the grammar requires value functions and one complete ordinary empty main');
  if (program && (program.items.length !== functions.length || proof.aliases.length !== descriptors.length)) fail('the complete alias inventory differs from the actual declarations');
  const declarations = new Map();
  for (const row of functions) {
    if (declarations.has(row.target.semanticName)) fail('raw and ordinary spellings cannot duplicate a declaration owner');
    declarations.set(row.target.semanticName, row.target);
  }
  for (const row of functions) {
    const scope = new Map(row.parameters.map(parameter => [parameter.semanticName, parameter]));
    for (const [index, group] of row.body.entries()) {
      const token = group.tail;
      if (token.kind === 'number') {
        if (owned(token) || token.floating || !['', 'i32'].includes(token.suffix ?? '')) fail('only unchanged direct i32 literal spellings are admitted');
      } else if (token.kind === 'punct') {
        if (!['(', ')', ','].includes(token.raw) || owned(token)) fail('this body operation requires another source interpretation');
      } else if (token.kind === 'identifier') {
        if (!group.raw && ['true', 'false'].includes(group.name)) { if (owned(token)) fail('a Boolean literal cannot acquire a binding interpretation'); continue; }
        if (!validName(group.name, group.raw)) fail('a keyword needs its actual raw prefix in this source');
        const owner = program ? token.binding ?? token.declaration : null;
        const semanticName = program ? owner?.name : group.name;
        const target = scope.get(semanticName) ?? declarations.get(semanticName);
        if (!target || program && (owner !== target.checkedOwner || token[target.kind === 'parameter' ? 'binding' : 'declaration'] !== target.checkedOwner)) fail('an occurrence has no actual owner in its lexical scope');
        if (target.kind === 'declaration' && !syntax(row.body[index + 1], '(')) fail('a value declaration occurrence requires its actual direct call parentheses');
        if (target.kind === 'parameter' && syntax(row.body[index + 1], '(')) fail('a primitive parameter cannot become a callable source owner');
        assignments.set(group, target);
      } else fail('the primitive body contains an unrepresented source token');
    }
  }
  const tokens = logical.map(group => {
    const target = assignments.get(group), first = group.tokens[0], last = group.tail;
    return { kind: last.kind, value: target?.value ?? last.value, raw: group.tokens.map(token => token.raw).join(''), start: first.span.start, end: last.span.end,
      ...(last.kind === 'number' ? { suffix: last.suffix, floating: last.floating } : {}) };
  });
  return { tokens, descriptors, functions, assignments, logical };
}

export function renameRawBindingIdentifierSurface(surface, plan) {
  const names = new Map(plan.descriptors.map(row => [row.value, row.semanticName]));
  function expression(node) {
    if (node.k === 'name') {
      if (node.path.length !== 1 || !names.has(node.path[0])) fail('a primitive occurrence escaped its actual alias inverse');
      node.path = [names.get(node.path[0])];
    } else if (node.k === 'app') { expression(node.fn); node.args.forEach(expression); }
    else if (!['num', 'bool'].includes(node.k)) fail('the complete parser produced an unsupported primitive body');
  }
  for (const item of surface.items) {
    if (item.k !== 'fn' || !names.has(item.name)) fail('the complete parser produced an unowned value declaration');
    item.name = names.get(item.name);
    for (const parameter of item.params) { if (!names.has(parameter.name)) fail('the actual parameter lost its alias inverse'); parameter.name = names.get(parameter.name); }
    expression(item.body);
  }
  return surface;
}
