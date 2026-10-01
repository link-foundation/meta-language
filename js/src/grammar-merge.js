// Deterministic, meaning-aware merging of grammars from several sources, and
// binding-aware renaming of grammar rules. It mirrors
// rust/src/grammar/merge.rs: both runtimes normalize rule definitions the same
// way, prove equivalence by the same recursive structural bisimulation and
// print the same canonical definitions, so their decisions agree.
import { createHash } from 'node:crypto';

import { Grammar } from './grammar.js';

/** How an accepted equivalence is justified. */
export const GRAMMAR_MERGE_METHOD = 'recursive-structural-bisimulation';

/** Raised for malformed merge input and by `assertMergeComplete`. */
export class GrammarMergeError extends Error {
  constructor(message, failures = []) {
    super(message);
    this.name = 'GrammarMergeError';
    this.failures = failures;
  }
}

/** Raised when a rename is unknown, invalid or would capture another name. */
export class GrammarRenameError extends Error {
  constructor(kind, message) {
    super(message);
    this.name = 'GrammarRenameError';
    this.kind = kind;
  }
}

/**
 * Merges grammars of several sources without user interaction.
 *
 * Each source is `{ id, language, edition, precedence, grammar }`. Sources of
 * different languages or editions are never merged with each other. Within one
 * language edition, rules are merged only when their normalized definitions are
 * equivalent under a recursive structural bisimulation; similar names and
 * identical caller-supplied samples (`options.samples`, keyed by
 * `sourceId:ruleName`) only nominate candidates. `options.requiredEquivalences`
 * lists `[aliasA, aliasB]` pairs that must end up merged, otherwise the result
 * is `incomplete`. `options.previous` is an earlier result: unchanged groups are
 * reused and established canonical names are kept.
 */
export function mergeGrammars(sources, options = {}) {
  const prepared = prepareSources(sources);
  const samples = normalizeSamples(options.samples ?? {});
  const required = normalizeRequired(options.requiredEquivalences ?? []);
  const previousGroups = new Map((options.previous?.groups ?? []).map((group) => [group.key, group]));

  const grouped = new Map();
  for (const source of prepared) {
    const key = groupKey(source.language, source.edition);
    if (!grouped.has(key)) grouped.set(key, { language: source.language, edition: source.edition, sources: [] });
    grouped.get(key).sources.push(source);
  }
  const ordered = [...grouped.values()].sort((left, right) =>
    compareText(left.language, right.language) || compareText(left.edition, right.edition));

  const groups = [];
  const reused = [];
  const recomputed = [];
  for (const entry of ordered) {
    const key = groupKey(entry.language, entry.edition);
    const ids = new Set(entry.sources.map(({ id }) => id));
    const groupRequired = required.filter((pair) => pair.some((alias) => ids.has(sourceOf(alias))));
    const fingerprint = groupFingerprint(entry, groupRequired, samples);
    const previous = previousGroups.get(key);
    if (previous && previous.fingerprint === fingerprint) {
      groups.push(previous);
      reused.push(key);
      continue;
    }
    groups.push(mergeGroup(entry, fingerprint, samples, previous?.identities ?? {}));
    recomputed.push(key);
  }

  const alternatives = editionAlternatives(groups);
  const failures = requiredFailures(required, groups);
  return {
    status: failures.length === 0 ? 'complete' : 'incomplete',
    groups,
    alternatives,
    failures,
    reused,
    recomputed,
  };
}

/** Throws when a merge left a required equivalence unresolved. */
export function assertMergeComplete(result) {
  if (result.failures.length === 0) return result;
  const detail = result.failures.map(({ members, reason }) => `${members.join(' = ')} (${reason})`).join('; ');
  throw new GrammarMergeError(`unresolved required equivalence: ${detail}`, result.failures);
}

/**
 * Renames one rule and every reference to it, including recursive references,
 * references inside captures and references qualified with `namespace` (as
 * `namespace.rule` or `namespace::rule`). Capture labels are a separate scope
 * and are never renamed. The returned aliases map canonical names back to the
 * original source names, so the grammar can be exported with them.
 */
export function renameGrammarRule(grammar, from, to, { namespace = null, aliases = [] } = {}) {
  if (typeof to !== 'string' || !/^\S+$/u.test(to)) {
    throw new GrammarRenameError('invalid-name', `invalid rule name ${JSON.stringify(to)}`);
  }
  if (!grammar.rules.has(from)) {
    throw new GrammarRenameError('unknown-rule', `grammar has no rule ${from}`);
  }
  if (from === to) return { grammar, aliases: [...aliases] };
  const mapping = new Map([[from, to]]);
  if (namespace !== null) {
    mapping.set(`${namespace}.${from}`, `${namespace}.${to}`);
    mapping.set(`${namespace}::${from}`, `${namespace}::${to}`);
  }
  const taken = new Set([...grammar.ruleNames(), ...grammar.referencedNonterminals()]);
  for (const target of mapping.values()) {
    if (taken.has(target)) {
      throw new GrammarRenameError('collision', `renaming ${from} to ${to} would capture the existing name ${target}`);
    }
  }
  let chained = false;
  const nextAliases = aliases.map((alias) => {
    if (alias.canonical !== from) return { ...alias };
    chained = true;
    return { canonical: to, original: alias.original };
  });
  if (!chained) nextAliases.push({ canonical: to, original: from });
  return { grammar: renameAll(grammar, mapping), aliases: nextAliases };
}

/** Renames canonical rule names back to their source names for export. */
export function restoreSourceNames(grammar, aliases, { namespace = null } = {}) {
  const mapping = new Map();
  for (const { canonical, original } of aliases) {
    if (!grammar.rules.has(canonical) || canonical === original) continue;
    mapping.set(canonical, original);
    if (namespace !== null) {
      mapping.set(`${namespace}.${canonical}`, `${namespace}.${original}`);
      mapping.set(`${namespace}::${canonical}`, `${namespace}::${original}`);
    }
  }
  const rename = (name) => mapping.get(name) ?? name;
  const ruleNames = grammar.ruleNames().map(rename);
  const externals = grammar.referencedNonterminals().filter((name) => !grammar.rules.has(name)).map(rename);
  if (new Set(ruleNames).size !== ruleNames.length || externals.some((name) => ruleNames.includes(name))) {
    throw new GrammarRenameError('collision', 'restoring source names would give two bindings the same name');
  }
  return renameAll(grammar, mapping);
}

/** The canonical text of a rule definition after meaning-aware normalization. */
export function normalizedRuleDefinition(rule) {
  return `${rule.kind ?? 'normal'}:${normalize(rule.expression, nameLabel).text}`;
}

function renameAll(grammar, mapping) {
  const rename = (name) => mapping.get(name) ?? name;
  const rules = new Map();
  for (const rule of grammar.rules.values()) {
    rules.set(rename(rule.name), { kind: rule.kind, expression: mapReferences(rule.expression, rename) });
  }
  return new Grammar(grammar.start === null ? null : rename(grammar.start), rules, grammar.sourceFormat);
}

function mapReferences(expression, rename) {
  if (expression.kind === 'ref') return { ...expression, name: rename(expression.name) };
  const copy = { ...expression };
  if (Array.isArray(expression.items) && expression.kind !== 'charClass') {
    copy.items = expression.items.map((item) => mapReferences(item, rename));
  }
  if (expression.item) copy.item = mapReferences(expression.item, rename);
  return copy;
}

function prepareSources(sources) {
  if (!Array.isArray(sources)) throw new GrammarMergeError('merge sources must be an array');
  const seen = new Set();
  const prepared = sources.map((source) => {
    const { id, language, edition = '', precedence = 0, grammar } = source ?? {};
    if (typeof id !== 'string' || id.length === 0 || id.includes(':')) {
      throw new GrammarMergeError(`invalid merge source id ${JSON.stringify(id)}`);
    }
    if (seen.has(id)) throw new GrammarMergeError(`duplicate merge source id ${id}`);
    seen.add(id);
    if (typeof language !== 'string' || language.length === 0) {
      throw new GrammarMergeError(`merge source ${id} has no language`);
    }
    if (typeof edition !== 'string') throw new GrammarMergeError(`merge source ${id} has an invalid edition`);
    if (!Number.isSafeInteger(precedence) || precedence < 0) {
      throw new GrammarMergeError(`merge source ${id} has an invalid precedence`);
    }
    if (!(grammar instanceof Grammar)) throw new GrammarMergeError(`merge source ${id} has no grammar`);
    return { id, language, edition, precedence, grammar };
  });
  return prepared.sort((left, right) => left.precedence - right.precedence || compareText(left.id, right.id));
}

function normalizeSamples(samples) {
  const normalized = new Map();
  for (const alias of Object.keys(samples).sort(compareText)) {
    const values = [...new Set(samples[alias].map(String))].sort(compareText);
    if (values.length > 0) normalized.set(alias, values);
  }
  return normalized;
}

function normalizeRequired(pairs) {
  const normalized = pairs.map((pair) => {
    if (!Array.isArray(pair) || pair.length !== 2 || pair.some((alias) => typeof alias !== 'string')) {
      throw new GrammarMergeError('a required equivalence is a pair of sourceId:ruleName aliases');
    }
    return [...pair].sort(compareText);
  });
  const unique = new Map(normalized.map((pair) => [pair.join('\u0000'), pair]));
  return [...unique.values()].sort((left, right) => compareText(left[0], right[0]) || compareText(left[1], right[1]));
}

function groupKey(language, edition) {
  return `${language}@${edition}`;
}

function sourceOf(alias) {
  const colon = alias.indexOf(':');
  return colon < 0 ? alias : alias.slice(0, colon);
}

function formatOf(source) {
  return source.grammar.sourceFormat ?? 'none';
}

function groupFingerprint(entry, required, samples) {
  const lines = ['grammar-merge v1', `group ${q(entry.language)} ${q(entry.edition)}`];
  const ids = new Set();
  for (const source of entry.sources) {
    ids.add(source.id);
    const start = source.grammar.startRule()?.name;
    lines.push(`source ${q(source.id)} ${source.precedence} ${q(formatOf(source))} ${start === undefined ? '-' : q(start)}`);
    const label = sourceLabel(source, nameLabel);
    for (const rule of source.grammar.rules.values()) {
      lines.push(`rule ${q(rule.name)} ${rule.kind}:${normalize(rule.expression, label).text}`);
    }
  }
  for (const [first, second] of required) lines.push(`required ${q(first)} ${q(second)}`);
  for (const [alias, values] of samples) {
    if (ids.has(sourceOf(alias))) lines.push(`samples ${q(alias)} ${values.map(q).join(',')}`);
  }
  return createHash('sha256').update(`${lines.join('\n')}\n`).digest('hex');
}

function sourceLabel(source, internal) {
  const format = formatOf(source);
  return (name) => (source.grammar.rules.has(name) ? internal(name) : `ext(${q(format)},${q(name)})`);
}

function nameLabel(name) {
  return `ref(${q(name)})`;
}

function mergeGroup(entry, fingerprint, samples, previousIdentities) {
  const nodes = [];
  const index = new Map();
  const externals = new Set();
  for (const source of entry.sources) {
    let position = 0;
    for (const rule of source.grammar.rules.values()) {
      const alias = `${source.id}:${rule.name}`;
      index.set(alias, nodes.length);
      nodes.push({ alias, source, position, name: rule.name, kind: rule.kind, expression: rule.expression });
      position += 1;
    }
    for (const name of source.grammar.referencedNonterminals()) {
      if (!source.grammar.rules.has(name)) externals.add(name);
    }
  }

  const classes = refine(nodes, index);
  const members = new Map();
  for (const [position, node] of nodes.entries()) {
    const id = classes[position];
    if (!members.has(id)) members.set(id, []);
    members.get(id).push(node);
  }
  const classOrder = [...members.keys()];

  const names = new Map();
  const taken = new Set(externals);
  const renamed = new Map();
  for (const id of classOrder) {
    for (const node of members.get(id)) {
      const previous = previousIdentities[node.alias];
      if (previous !== undefined && !taken.has(previous)) {
        names.set(id, previous);
        taken.add(previous);
        break;
      }
    }
  }
  for (const id of classOrder) {
    if (names.has(id)) continue;
    const best = members.get(id)[0];
    let name = best.name;
    if (taken.has(name)) {
      const base = `${best.name}_from_${best.source.id.replace(/[^A-Za-z0-9_]/gu, '_')}`;
      name = base;
      for (let suffix = 2; taken.has(name); suffix += 1) name = `${base}_${suffix}`;
      renamed.set(id, best.name);
    }
    names.set(id, name);
    taken.add(name);
  }

  const canonicalOf = (node) => names.get(classes[index.get(`${node.source.id}:${node.name}`)]);
  const rules = new Map();
  const definitions = new Map();
  for (const id of classOrder) {
    const representative = members.get(id)[0];
    const rename = (name) => (representative.source.grammar.rules.has(name)
      ? names.get(classes[index.get(`${representative.source.id}:${name}`)])
      : name);
    const form = normalize(mapReferences(representative.expression, rename), nameLabel);
    rules.set(names.get(id), { kind: representative.kind, expression: form.expr });
    definitions.set(id, `${representative.kind}:${form.text}`);
  }

  const startClasses = [];
  for (const source of entry.sources) {
    const start = source.grammar.startRule();
    if (!start) continue;
    const id = classes[index.get(`${source.id}:${start.name}`)];
    if (!startClasses.includes(id)) startClasses.push(id);
  }
  const formats = new Set(entry.sources.map(formatOf));
  const grammar = new Grammar(
    startClasses.length === 0 ? null : names.get(startClasses[0]),
    rules,
    formats.size === 1 ? entry.sources[0].grammar.sourceFormat : 'meta-language',
  );

  const identities = {};
  for (const node of nodes) identities[node.alias] = canonicalOf(node);

  const decisions = [];
  for (const id of classOrder) {
    const group = members.get(id);
    decisions.push({
      kind: group.length > 1 ? 'merged' : 'kept-unique',
      name: names.get(id),
      members: group.map(({ alias }) => alias),
      basis: group.length > 1 ? GRAMMAR_MERGE_METHOD : 'no-equivalent-rule',
      definition: definitions.get(id),
    });
    if (renamed.has(id)) {
      decisions.push({
        kind: 'renamed-for-collision',
        name: names.get(id),
        members: [group[0].alias],
        basis: 'precedence',
        definition: null,
      });
    }
  }

  const alternatives = [];
  if (startClasses.length > 1) {
    alternatives.push({
      reason: 'start-rule',
      name: names.get(startClasses[0]),
      options: startClasses.slice(1).map((id) => names.get(id)),
    });
  }
  const byName = new Map();
  for (const id of classOrder) {
    for (const node of members.get(id)) {
      if (!byName.has(node.name)) byName.set(node.name, []);
      if (!byName.get(node.name).includes(id)) byName.get(node.name).push(id);
    }
  }
  for (const name of [...byName.keys()].sort(compareText)) {
    const ids = byName.get(name);
    if (ids.length < 2) continue;
    const options = ids.map((id) => names.get(id));
    decisions.push({ kind: 'homonym-kept-distinct', name, members: options, basis: 'different-definitions', definition: null });
    alternatives.push({ reason: 'distinct-meaning', name, options });
  }

  const nominations = nominate(nodes, classes, samples);
  for (const nomination of nominations) {
    if (nomination.outcome !== 'unproven') continue;
    const [first, second] = nomination.members.map((alias) => identities[alias]);
    decisions.push({ kind: 'uncertain', name: first, members: nomination.members, basis: nomination.basis, definition: null });
    alternatives.push({ reason: 'uncertain-match', name: nomination.basis, options: [first, second] });
  }

  return {
    key: groupKey(entry.language, entry.edition),
    language: entry.language,
    edition: entry.edition,
    fingerprint,
    sources: entry.sources.map(({ id }) => id),
    grammar,
    identities,
    decisions,
    nominations,
    alternatives,
  };
}

// Greatest structural bisimulation by partition refinement: rules start in one
// class per normalized shape and are split until every member of a class
// references members of the same classes in the same way. Class numbers are
// ranks of sorted signatures, so they do not depend on the input order.
function refine(nodes, index) {
  let classes = nodes.map(() => 0);
  let count = 0;
  for (;;) {
    const current = classes;
    const keys = nodes.map((node, position) => {
      const label = sourceLabel(node.source, (name) => `#${current[index.get(`${node.source.id}:${name}`)]}`);
      return `${count === 0 ? '' : current[position]}|${node.kind}:${normalize(node.expression, label).text}`;
    });
    const ranks = new Map([...new Set(keys)].sort(compareText).map((key, rank) => [key, rank]));
    classes = keys.map((key) => ranks.get(key));
    if (ranks.size === count) return classes;
    count = ranks.size;
  }
}

function nominate(nodes, classes, samples) {
  const nominations = [];
  const seen = new Set();
  const add = (basis, first, second) => {
    const key = `${basis}\u0000${first.alias}\u0000${second.alias}`;
    if (seen.has(key)) return;
    seen.add(key);
    nominations.push({
      basis,
      members: [first.alias, second.alias],
      outcome: classes[first.position] === classes[second.position] ? 'proven' : 'unproven',
    });
  };
  const located = nodes.map((node, position) => ({ ...node, position }));
  const byKey = new Map();
  for (const node of located) {
    const key = node.name.toLowerCase().replace(/[-_]/gu, '');
    if (!byKey.has(key)) byKey.set(key, []);
    byKey.get(key).push(node);
  }
  for (const group of byKey.values()) {
    for (let left = 0; left < group.length; left += 1) {
      for (let right = left + 1; right < group.length; right += 1) {
        if (group[left].name !== group[right].name) add('name-similarity', group[left], group[right]);
      }
    }
  }
  const bySamples = new Map();
  for (const node of located) {
    const values = samples.get(node.alias);
    if (!values) continue;
    const key = values.map(q).join(',');
    if (!bySamples.has(key)) bySamples.set(key, []);
    bySamples.get(key).push(node);
  }
  for (const group of bySamples.values()) {
    for (let left = 0; left < group.length; left += 1) {
      for (let right = left + 1; right < group.length; right += 1) add('identical-samples', group[left], group[right]);
    }
  }
  return nominations;
}

function editionAlternatives(groups) {
  const editions = new Map();
  for (const group of groups) {
    if (!editions.has(group.language)) editions.set(group.language, []);
    editions.get(group.language).push(group.edition);
  }
  return [...editions.entries()]
    .filter(([, list]) => list.length > 1)
    .map(([language, list]) => ({ reason: 'edition', name: language, options: list }));
}

function requiredFailures(required, groups) {
  const groupOf = new Map();
  for (const group of groups) for (const id of group.sources) groupOf.set(id, group);
  const failures = [];
  for (const members of required) {
    const owners = members.map((alias) => groupOf.get(sourceOf(alias)));
    const names = members.map((alias, position) => owners[position]?.identities[alias]);
    let reason = null;
    if (names.some((name) => name === undefined)) reason = 'unknown-rule';
    else if (owners[0] !== owners[1]) reason = 'different-language-or-edition';
    else if (names[0] !== names[1]) reason = 'not-proven';
    if (reason !== null) failures.push({ kind: 'unresolved-required-equivalence', members, reason });
  }
  return failures;
}

// Meaning-aware normalization. Each form carries the normalized expression and
// its canonical text; `label` prints a reference, which lets the same pass
// serve source names, external names and bisimulation classes.
const EMPTY = Object.freeze({ expr: Object.freeze({ kind: 'empty' }), text: 'empty' });

function normalize(expression, label) {
  switch (expression.kind) {
    case 'empty': return EMPTY;
    case 'literal': return literal(expression.value);
    case 'literalInsensitive':
      return hasCase(expression.value)
        ? { expr: { kind: 'literalInsensitive', value: expression.value }, text: `ilit(${q(expression.value)})` }
        : literal(expression.value);
    case 'charRange': return charRange(expression.start, expression.end);
    case 'charClass': return charClass(expression);
    case 'regex': return { expr: { kind: 'regex', value: expression.value }, text: `regex(${q(expression.value)})` };
    case 'any': return { expr: { kind: 'any' }, text: 'any' };
    case 'ref': return { expr: { kind: 'ref', name: expression.name }, text: label(expression.name) };
    case 'seq': return sequenceForm(expression.items.map((item) => normalize(item, label)));
    case 'choice': return choiceForm(expression.items.map((item) => normalize(item, label)), expression.ordered === true);
    case 'repeat0': return repeatForm(normalize(expression.item, label), 0, null);
    case 'repeat1': return repeatForm(normalize(expression.item, label), 1, null);
    case 'optional': return repeatForm(normalize(expression.item, label), 0, 1);
    case 'repeat': return repeatForm(normalize(expression.item, label), expression.min, expression.max ?? null);
    case 'and':
    case 'not': {
      const inner = normalize(expression.item, label);
      return { expr: { kind: expression.kind, item: inner.expr }, text: `${expression.kind}(${inner.text})`, inner };
    }
    case 'capture': {
      const inner = normalize(expression.item, label);
      const name = expression.label ?? null;
      return {
        expr: { kind: 'capture', label: name, item: inner.expr },
        text: `capture(${name === null ? '_' : q(name)},${inner.text})`,
        inner,
      };
    }
    default: throw new GrammarMergeError(`unsupported grammar expression kind: ${expression.kind}`);
  }
}

function literal(value) {
  return { expr: { kind: 'literal', value }, text: `lit(${q(value)})` };
}

function charRange(start, end) {
  if (start === end) return literal(start);
  return { expr: { kind: 'charRange', start, end }, text: `range(${q(start)},${q(end)})` };
}

function charClass(expression) {
  const negated = expression.negated === true;
  if (typeof expression.value === 'string') {
    return {
      expr: { kind: 'charClass', value: expression.value, negated },
      text: `rawclass(${negated ? '!' : ''}${q(expression.value)})`,
    };
  }
  const items = new Map();
  for (const item of expression.items) {
    const single = item.kind === 'range' && item.start === item.end ? item.start : null;
    if (item.kind === 'range' && single === null) {
      items.set(`${q(item.start)}-${q(item.end)}`, { kind: 'range', start: item.start, end: item.end });
    } else {
      const value = single ?? item.value;
      items.set(q(value), { kind: 'char', value });
    }
  }
  const texts = [...items.keys()].sort(compareText);
  if (!negated && texts.length === 1) {
    const only = items.get(texts[0]);
    return only.kind === 'range' ? charRange(only.start, only.end) : literal(only.value);
  }
  return {
    expr: { kind: 'charClass', items: texts.map((text) => items.get(text)), negated },
    text: `class(${negated ? '!' : ''}[${texts.join(',')}])`,
  };
}

function sequenceForm(forms) {
  const items = [];
  for (const form of forms) {
    for (const part of form.expr.kind === 'seq' ? form.items : [form]) {
      if (part.expr.kind === 'empty') continue;
      const previous = items.at(-1);
      // `x x*` matches exactly what `x+` matches, in PEG and in CFG alike.
      if (previous && part.expr.kind === 'repeat0' && part.inner.text === previous.text) {
        items[items.length - 1] = repeatForm(previous, 1, null);
      } else {
        items.push(part);
      }
    }
  }
  if (items.length === 0) return EMPTY;
  if (items.length === 1) return items[0];
  return {
    expr: { kind: 'seq', items: items.map(({ expr }) => expr) },
    text: `seq(${items.map(({ text }) => text).join(',')})`,
    items,
  };
}

function choiceForm(forms, ordered) {
  const flattened = [];
  for (const form of forms) {
    if (form.expr.kind === 'choice' && form.ordered === ordered) flattened.push(...form.items);
    else flattened.push(form);
  }
  // A repeated alternative adds nothing: in an ordered choice the later copy
  // can never match where the earlier one failed, and union is idempotent.
  const unique = new Map();
  for (const form of flattened) if (!unique.has(form.text)) unique.set(form.text, form);
  const items = [...unique.values()];
  if (!ordered) items.sort((left, right) => compareText(left.text, right.text));
  if (items.length === 1) return items[0];
  return {
    expr: { kind: 'choice', items: items.map(({ expr }) => expr), ordered },
    text: `${ordered ? 'first' : 'alt'}(${items.map(({ text }) => text).join(',')})`,
    items,
    ordered,
  };
}

function repeatForm(inner, min, max) {
  if (!Number.isSafeInteger(min) || min < 0 || (max !== null && (!Number.isSafeInteger(max) || max < min))) {
    throw new GrammarMergeError(`invalid repetition bounds ${min}..${max ?? ''}`);
  }
  if (inner.expr.kind === 'empty' || max === 0) return EMPTY;
  if (min === 1 && max === 1) return inner;
  if (min === 0 && max === null) return { expr: { kind: 'repeat0', item: inner.expr }, text: `many(${inner.text})`, inner };
  if (min === 1 && max === null) return { expr: { kind: 'repeat1', item: inner.expr }, text: `some(${inner.text})`, inner };
  if (min === 0 && max === 1) return { expr: { kind: 'optional', item: inner.expr }, text: `opt(${inner.text})`, inner };
  return {
    expr: { kind: 'repeat', item: inner.expr, min, max },
    text: `rep(${min},${max ?? '*'},${inner.text})`,
    inner,
  };
}

function hasCase(value) {
  return value.toLowerCase() !== value.toUpperCase();
}

function q(value) {
  return JSON.stringify(value);
}

// Code point order, which is the byte order of UTF-8 strings in Rust.
function compareText(left, right) {
  let i = 0;
  let j = 0;
  while (i < left.length && j < right.length) {
    const x = left.codePointAt(i);
    const y = right.codePointAt(j);
    if (x !== y) return x < y ? -1 : 1;
    i += x > 0xffff ? 2 : 1;
    j += y > 0xffff ? 2 : 1;
  }
  if (i < left.length) return 1;
  if (j < right.length) return -1;
  return 0;
}
