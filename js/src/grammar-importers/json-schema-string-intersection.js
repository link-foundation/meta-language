import { GrammarBuilder as Expression } from '../grammar.js';
import { stringIntersectionLimits } from './json-schema-pattern-policy.js';
import { complementScalarRanges } from './json-schema-patterns.js';

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
      case 'charClass': {
        if (!node.items) throw exhausted;
        const intervals = node.items.map(item => item.kind === 'char'
          ? [item.value.codePointAt(0), item.value.codePointAt(0)]
          : [item.start.codePointAt(0), item.end.codePointAt(0)]);
        if (node.negated) {
          charge(intervals.length);
          intervals.sort(([first], [second]) => first - second);
        }
        ranges(node.negated ? complementScalarRanges(intervals) : intervals, start, end);
        break;
      }
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

export function intersectJsonStringExpressions(leftExpression, rightExpression, resolve, rulePrefix) {
  return selectExclusiveJsonStringExpressions([[leftExpression, rightExpression]], resolve, rulePrefix);
}

/** Each branch is a conjunction. The decoder accepts when exactly one branch matches. */
export function selectExclusiveJsonStringExpressions(branches, resolve, rulePrefix) {
  if (branches.length === 0) return null;
  if (branches.length === 1 && branches[0].length === 1) return { expression: branches[0][0], rules: [] };
  return withProductBudget(charge => {
    const rows = buildProduct(branches, resolve, charge, 2);
    const accepting = rows.flatMap((row, index) => row.matches.length === 1 ? [index] : []);
    return compileProduct(rows, accepting, rulePrefix);
  });
}

/** Each returned expression accepts strings with exactly its listed branch memberships. */
export function partitionJsonStringExpressions(branches, resolve, rulePrefix, budget) {
  return withProductBudget(charge => {
    const rows = buildProduct(branches, resolve, charge, branches.length);
    const groups = new Map();
    let compilationWork = 0;
    for (const [index, row] of rows.entries()) {
      compilationWork += 1 + 2 * row.edges.length;
      if (!row.matches.length) continue;
      const key = row.matches.join(',');
      if (!groups.has(key)) groups.set(key, { matches: row.matches, accepted: [] });
      groups.get(key).accepted.push(index);
    }
    charge(compilationWork * groups.size);
    return [...groups.values()].map(({ matches, accepted }, index) => ({
      matches,
      ...compileProduct(rows, accepted, `${rulePrefix}_${index}`),
    }));
  }, budget);
}

function withProductBudget(build, budget = { remaining: maximumWork }) {
  try {
    return build(cost => {
      budget.remaining -= cost;
      if (budget.remaining < 0) throw exhausted;
    });
  } catch (error) {
    if (error !== exhausted) throw error;
    return undefined;
  }
}

function buildProduct(branches, resolve, charge, maximumMatches) {
  const machines = [];
  const machineIndices = new Map();
  const groups = branches.map(expressions => expressions.map(expression => {
    charge(1);
    if (!machineIndices.has(expression)) {
      machineIndices.set(expression, machines.length);
      machines.push(automaton(expression, resolve, charge));
    }
    return machineIndices.get(expression);
  }));
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
  function product(states) {
    const key = states.map(group => group.join(',')).join('/');
    if (indices.has(key)) return indices.get(key);
    if (rows.length === maximumProductStates) throw exhausted;
    const index = rows.length;
    indices.set(key, index);
    const matches = [];
    for (const [index, group] of groups.entries()) {
      charge(group.length);
      if (group.every(machine => states[machine].includes(1))) {
        matches.push(index);
        if (matches.length === maximumMatches) break;
      }
    }
    rows.push({ states, matches, edges: [], incoming: [] });
    return index;
  }
  product(machines.map(machine => closure(machine, [0])));
  for (let index = 0; index < rows.length; index += 1) {
    const row = rows[index];
    const machineEdges = machines.map((machine, offset) => row.states[offset].flatMap(state => machine[state].edges));
    const edgeCount = machineEdges.reduce((count, edges) => count + edges.length, 0);
    const bounds = [...new Set(machineEdges.flatMap(edges => edges.flatMap(edge => [edge.first, edge.last + 1])))]
      .sort((first, last) => first - last);
    for (let part = 1; part < bounds.length; part += 1) {
      charge(1 + edgeCount + machineEdges.length);
      const first = bounds[part - 1];
      const last = bounds[part] - 1;
      const targets = machineEdges.map(edges => edges.filter(edge => edge.first <= first && first <= edge.last).map(edge => edge.end));
      if (!groups.some(group => {
        charge(group.length);
        return group.every(machine => targets[machine].length);
      })) continue;
      const target = product(machines.map((machine, offset) => closure(machine, targets[offset])));
      const previous = row.edges.at(-1);
      if (previous?.target === target && previous.last + 1 === first) previous.last = last;
      else {
        row.edges.push({ first, last, target });
        rows[target].incoming.push(index);
      }
    }
  }
  return rows;
}

function compileProduct(rows, accepted, rulePrefix) {
  const accepting = new Set(accepted);
  const productive = new Set(accepted);
  const pending = [...accepted];
  for (const index of pending) {
    for (const previous of rows[index].incoming) {
      if (productive.has(previous)) continue;
      productive.add(previous);
      pending.push(previous);
    }
  }
  if (!productive.has(0)) return null;
  const name = index => `${rulePrefix}_${index}`;
  const rules = [];
  for (const [index, row] of rows.entries()) {
    if (!productive.has(index)) continue;
    const alternatives = accepting.has(index) ? [empty()] : [];
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
}
