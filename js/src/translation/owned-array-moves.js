// Mechanical control-flow facts over the already renamed checked tree.
// Each name denotes one actual binder, and the shared decision follows its
// reachable continuation. No source spelling or boolean grants ownership.
import { acceptOwnedArrayFinalUse } from './owned-array-append-rules.js';
import { readProgramExpressionOperandFields, readProgramExpressionEntryOperandField } from '../typed-program-link-rules.js';

export function readOwnedArrayFinalUses(body) {
  const bindings = [], starts = [], counts = [], successors = [];
  const names = new Map();
  const candidates = [];
  const name = value => {
    if (!names.has(value)) names.set(value, names.size);
    return names.get(value);
  };
  const event = (binding, next) => {
    const position = bindings.length;
    bindings.push(binding); starts.push(successors.length); counts.push(next.length); successors.push(...next);
    return position;
  };
  const children = node => {
    if (node.k === 'let') return [node.value, node.body];
    if (node.k === 'lambda') return [node.body];
    if (node.k === 'match') return [node.scrutinee, ...node.cases.map(branch => branch.body)];
    return readProgramExpressionOperandFields(node.k).split(' ').filter(Boolean).flatMap(field => {
      const value = node[field];
      const entryField = readProgramExpressionEntryOperandField(node.k, field);
      if (Array.isArray(value)) return value.map(child => entryField ? child[entryField] : child).flat(Infinity).filter(child => child?.k);
      return value?.k ? [value] : [];
    });
  };
  const sequence = (nodes, continuation) => nodes.reduceRight((next, child) => build(child, next), continuation);
  const build = (node, continuation) => {
    if (node.k === 'var') {
      const position = event(name(node.name), continuation);
      if (node.type.kind === 'array' && !node.type.reference) candidates.push({ node, position });
      return [position];
    }
    if (node.k === 'lambda') {
      // A closure gets owned copies at creation. Its body's lexical reads
      // consume those copies later, never the enclosing binding in place.
      const captured = new Set();
      const visit = child => {
        if (child.k === 'var') captured.add(child.name);
        else children(child).forEach(visit);
      };
      visit(node.body);
      return [...captured].reduceRight((next, binding) => [event(name(binding), next)], continuation);
    }
    if (node.k === 'if') {
      return build(node.cond, [...build(node.then, continuation), ...build(node.else, continuation)]);
    }
    if (node.k === 'match') {
      return build(node.scrutinee, node.cases.flatMap(branch => build(branch.body, continuation)));
    }
    if (node.k === 'append' && node.left.k === 'var') {
      // Reading a variable has no effect. Evaluate the right value before
      // consuming the left storage so a right-hand length/index can borrow it.
      return build(node.right, build(node.left, continuation));
    }
    if (node.k === 'recur') {
      return node.args.reduceRight((next, argument) => build(argument, next), continuation);
    }
    return sequence(children(node), continuation);
  };
  build(body, []);
  return new Set(candidates.filter(candidate => acceptOwnedArrayFinalUse(candidate.position, bindings, starts, counts, successors)).map(candidate => candidate.node));
}
