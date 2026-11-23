import { GrammarBuilder as Expression } from '../grammar.js';
import { stringIntersectionLimits } from './json-schema-pattern-policy.js';

const { charClass, choice, empty, ref, seq } = Expression;
const [maximumSourceStates, maximumProductStates, maximumWork] = stringIntersectionLimits();
const exhausted = Symbol('string intersection budget');

function automaton(expression, resolve, charge) {
  const states = [];
  function state() {
    if (states.length === maximumSourceStates) throw exhausted;
    return states.push({ empty: [], edges: [] }) - 1;
  }
  function ranges(items, start, end) {
    charge(items.length);
    for (const [first, last] of items) states[start].edges.push({ first, last, end });
  }
  function epsilon(start, end) {
    states[start].empty.push(end);
  }
  function lower(node, start, end, depth) {
    if (depth > 128) throw exhausted;
    charge(1);
    const descend = (item, first, last) => lower(item, first, last, depth + 1);
    switch (node.kind) {
      case 'empty':
        epsilon(start, end);
        break;
      case 'literal': {
        const points = Array.from(node.value, character => character.codePointAt(0));
        if (!points.length) epsilon(start, end);
        for (let index = 0; index < points.length; index += 1) {
          const next = index === points.length - 1 ? end : state();
          ranges([[points[index], points[index]]], start, next);
          start = next;
        }
        break;
      }
      case 'charRange':
        ranges([[node.start.codePointAt(0), node.end.codePointAt(0)]], start, end);
        break;
      case 'charClass':
        if (node.negated || !node.items) throw exhausted;
        ranges(node.items.map(item => item.kind === 'char'
          ? [item.value.codePointAt(0), item.value.codePointAt(0)]
          : [item.start.codePointAt(0), item.end.codePointAt(0)]), start, end);
        break;
      case 'ref': {
        const referenced = resolve(node.name);
        if (!referenced) throw exhausted;
        descend(referenced, start, end);
        break;
      }
      case 'seq':
        if (!node.items.length) epsilon(start, end);
        for (let index = 0; index < node.items.length; index += 1) {
          const next = index === node.items.length - 1 ? end : state();
          descend(node.items[index], start, next);
          start = next;
        }
        break;
      case 'choice':
        if (node.ordered) throw exhausted;
        for (const item of node.items) descend(item, start, end);
        break;
      case 'optional':
      case 'repeat0':
      case 'repeat1':
      case 'repeat': {
        const minimum = node.min ?? Number(node.kind === 'repeat1');
        const maximum = node.kind === 'optional' ? 1 : node.max ?? null;
        for (let index = 0; index < minimum; index += 1) {
          const next = state();
          descend(node.item, start, next);
          start = next;
        }
        if (maximum === null) {
          const loop = state();
          const next = state();
          epsilon(start, loop);
          epsilon(loop, end);
          descend(node.item, loop, next);
          epsilon(next, loop);
        } else {
          epsilon(start, end);
          for (let index = minimum; index < maximum; index += 1) {
            const next = state();
            descend(node.item, start, next);
            epsilon(next, end);
            start = next;
          }
        }
        break;
      }
      default:
        throw exhausted;
    }
  }
  lower(expression, state(), state(), 0);
  return states;
}

/** Build a deterministic product over JSON source characters. Undefined means the work bound was reached. */
export function intersectJsonStringExpressions(leftExpression, rightExpression, resolve, rulePrefix) {
  try {
    let work = maximumWork;
    function charge(cost) {
      work -= cost;
      if (work < 0) throw exhausted;
    }
    const left = automaton(leftExpression, resolve, charge);
    const right = automaton(rightExpression, resolve, charge);
    function closure(machine, starts) {
      const pending = [...new Set(starts)];
      const visited = new Set(pending);
      for (const state of pending) {
        charge(1 + machine[state].empty.length);
        for (const target of machine[state].empty) {
          if (visited.has(target)) continue;
          visited.add(target);
          pending.push(target);
        }
      }
      return pending.sort((first, last) => first - last);
    }
    const rows = [];
    const indices = new Map();
    function product(first, second) {
      const key = `${first}/${second}`;
      if (indices.has(key)) return indices.get(key);
      if (rows.length === maximumProductStates) throw exhausted;
      const index = rows.length;
      indices.set(key, index);
      rows.push({ first, second, accepts: first.includes(1) && second.includes(1), edges: [], incoming: [] });
      return index;
    }
    product(closure(left, [0]), closure(right, [0]));
    const accepting = [];
    for (let index = 0; index < rows.length; index += 1) {
      const row = rows[index];
      if (row.accepts) accepting.push(index);
      const leftEdges = row.first.flatMap(state => left[state].edges);
      const rightEdges = row.second.flatMap(state => right[state].edges);
      const bounds = [...new Set([...leftEdges, ...rightEdges].flatMap(edge => [edge.first, edge.last + 1]))]
        .sort((first, last) => first - last);
      for (let part = 1; part < bounds.length; part += 1) {
        charge(1 + leftEdges.length + rightEdges.length);
        const first = bounds[part - 1];
        const last = bounds[part] - 1;
        const advance = edges => edges.filter(edge => edge.first <= first && first <= edge.last).map(edge => edge.end);
        const leftTargets = advance(leftEdges);
        if (!leftTargets.length) continue;
        const rightTargets = advance(rightEdges);
        if (!rightTargets.length) continue;
        const target = product(closure(left, leftTargets), closure(right, rightTargets));
        const previous = row.edges.at(-1);
        if (previous?.target === target && previous.last + 1 === first) previous.last = last;
        else {
          row.edges.push({ first, last, target });
          rows[target].incoming.push(index);
        }
      }
    }
    const productive = new Set(accepting);
    for (const index of accepting) {
      for (const previous of rows[index].incoming) {
        if (productive.has(previous)) continue;
        productive.add(previous);
        accepting.push(previous);
      }
    }
    if (!productive.has(0)) return null;
    const name = index => `${rulePrefix}_${index}`;
    const rules = [];
    for (const [index, row] of rows.entries()) {
      if (!productive.has(index)) continue;
      const alternatives = row.accepts ? [empty()] : [];
      const transitions = new Map();
      for (const { first, last, target } of row.edges) {
        if (!productive.has(target)) continue;
        if (!transitions.has(target)) transitions.set(target, []);
        transitions.get(target).push(first === last
          ? { kind: 'char', value: String.fromCodePoint(first) }
          : { kind: 'range', start: String.fromCodePoint(first), end: String.fromCodePoint(last) });
      }
      for (const [target, characters] of transitions) alternatives.push(seq(charClass(characters), ref(name(target))));
      rules.push([name(index), choice(...alternatives)]);
    }
    return { expression: ref(name(0)), rules };
  } catch (error) {
    if (error !== exhausted) throw error;
    return undefined;
  }
}
