// Deterministic, meaning-aware merging of grammars from several sources, and
// binding-aware renaming of grammar rules. It mirrors
// rust/src/grammar/merge/: both runtimes normalize rule definitions the same
// way, prove equivalence by the same recursive structural bisimulation and
// print the same canonical definitions, so their decisions agree.
import { createHash } from 'node:crypto';

import { Grammar } from './grammar.js';
import { grammarDeclarations } from './grammar-feature-forms.js';
import { renderDeclarationLinks, renderLinksExpression, renderRuleFieldLinks } from './grammar-links.js';
import { DecoratorSet, decoratorSet } from './decorators.js';
import { mapDeclarations, mapReferences, renamedRule } from './grammar-rename.js';

export { GrammarRenameError, renameGrammarRule, restoreSourceNames } from './grammar-rename.js';

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
 * reused and established canonical names are kept. `options.decorators` holds
 * `merge-decision` decorators, which see each decision as `{ kind, name,
 * members, basis, definition }` (members joined by spaces): they may change
 * its `kind` or `basis`, or drop it from the report. They never change the
 * merged grammar, and a group is reused only under the same decorators.
 */
export function mergeGrammars(sources, options = {}) {
  const prepared = prepareSources(sources);
  const decorators = decoratorSet(options.decorators);
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
    const fingerprint = groupFingerprint(entry, groupRequired, samples, decorators);
    const previous = previousGroups.get(key);
    if (previous && previous.fingerprint === fingerprint) {
      groups.push(previous);
      reused.push(key);
      continue;
    }
    groups.push(mergeGroup(entry, fingerprint, samples, previous?.identities ?? {}, decorators));
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

/** The canonical text of a rule definition after meaning-aware normalization. */
export function normalizedRuleDefinition(rule) {
  return `${rule.kind ?? 'normal'}:${normalize(rule.expression, nameLabel).text}`;
}

// The rule fields (parameters, channel, modes and action) as they take part
// in a rule's signature, with action references printed by `label`; empty for
// a rule without them, so plain rules keep their definitions.
function ruleFields(rule, label) {
  const action = rule.action === undefined ? undefined : mapReferences(rule.action, label);
  const fields = renderRuleFieldLinks({ ...rule, action });
  return fields.length === 0 ? '' : ` ${fields.join(' ')}`;
}

// The declarations of one grammar as one line of links.
function declarationsText(declarations) {
  const matching = declarations.matching === null ? [] : [`(matching ${declarations.matching})`];
  return [...matching, ...renderDeclarationLinks(declarations)].join(' ');
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

function groupFingerprint(entry, required, samples, decorators) {
  const lines = ['grammar-merge v1', `group ${q(entry.language)} ${q(entry.edition)}`];
  const ids = new Set();
  for (const source of entry.sources) {
    ids.add(source.id);
    const start = source.grammar.startRule()?.name;
    lines.push(`source ${q(source.id)} ${source.precedence} ${q(formatOf(source))} ${start === undefined ? '-' : q(start)}`);
    const label = sourceLabel(source, nameLabel);
    for (const rule of source.grammar.rules.values()) {
      lines.push(`rule ${q(rule.name)} ${rule.kind}:${normalize(rule.expression, label).text}${ruleFields(rule, label)}`);
    }
    const declarations = declarationsText(grammarDeclarations(source.grammar));
    if (declarations.length > 0) lines.push(`declarations ${q(source.id)} ${q(declarations)}`);
  }
  for (const [first, second] of required) lines.push(`required ${q(first)} ${q(second)}`);
  for (const [alias, values] of samples) {
    if (ids.has(sourceOf(alias))) lines.push(`samples ${q(alias)} ${values.map(q).join(',')}`);
  }
  for (const entry of decorators.forLevel('merge-decision')) lines.push(`decorator ${q(new DecoratorSet([entry]).toLino().trim())}`);
  return createHash('sha256').update(`${lines.join('\n')}\n`).digest('hex');
}

function sourceLabel(source, internal) {
  const format = formatOf(source);
  return (name) => (source.grammar.rules.has(name) ? internal(name) : `ext(${q(format)},${q(name)})`);
}

function nameLabel(name) {
  return `ref(${q(name)})`;
}

function mergeGroup(entry, fingerprint, samples, previousIdentities, decorators) {
  const nodes = [];
  const index = new Map();
  const externals = new Set();
  for (const source of entry.sources) {
    let position = 0;
    for (const rule of source.grammar.rules.values()) {
      const alias = `${source.id}:${rule.name}`;
      index.set(alias, nodes.length);
      nodes.push({ alias, source, position, name: rule.name, kind: rule.kind, expression: rule.expression, rule });
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
    const renamed = renamedRule(representative.rule, rename);
    const form = normalize(renamed.expression, nameLabel);
    rules.set(names.get(id), { ...renamed, expression: form.expr });
    definitions.set(id, `${representative.kind}:${form.text}${ruleFields(renamed, nameLabel)}`);
  }
  const merged = mergeDeclarations(entry.sources, (source, name) => (source.grammar.rules.has(name)
    ? names.get(classes[index.get(`${source.id}:${name}`)])
    : name));

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
    merged.declarations,
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
  for (const conflict of merged.conflicts) {
    decisions.push({ kind: 'declaration-conflict', name: conflict.name, members: conflict.members, basis: conflict.basis, definition: null });
    alternatives.push({ reason: 'declaration-conflict', name: conflict.name, options: conflict.members });
  }

  return {
    key: groupKey(entry.language, entry.edition),
    language: entry.language,
    edition: entry.edition,
    fingerprint,
    sources: entry.sources.map(({ id }) => id),
    grammar,
    identities,
    decisions: decorateDecisions(decisions, decorators),
    nominations,
    alternatives,
  };
}

// The decisions the `merge-decision` decorators leave, with `kind` and
// `basis` as they set them; the other fields describe the merged grammar and
// stay as they are.
function decorateDecisions(decisions, decorators) {
  if (!decorators.has('merge-decision')) return decisions;
  return decisions.flatMap((decision) => {
    const decorated = decorators.decorate('merge-decision', {
      kind: decision.kind,
      name: decision.name,
      members: decision.members.join(' '),
      basis: decision.basis,
      definition: decision.definition ?? '',
    });
    return decorated === null ? [] : [{ ...decision, kind: decorated.kind, basis: decorated.basis }];
  });
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
      return `${count === 0 ? '' : current[position]}|${node.kind}:${normalize(node.expression, label).text}${ruleFields(node.rule, label)}`;
    });
    const ranks = new Map([...new Set(keys)].sort(compareText).map((key, rank) => [key, rank]));
    classes = keys.map((key) => ranks.get(key));
    if (ranks.size === count) return classes;
    count = ranks.size;
  }
}

// Merges the declarations of the sources in precedence order, with each
// source's rule names renamed to their canonical names. Imports, modes,
// extras, conflict groups and precedence orders are united; the first declared matching, macro
// or scanner of a name wins, and a later different one is a conflict.
function mergeDeclarations(sources, canonical) {
  const declarations = { matching: null, imports: [], modes: [], extras: [], conflicts: [], precedences: [], macros: [], scanners: [] };
  const conflicts = [];
  const seen = { extras: new Set(), conflicts: new Set(), precedences: new Set(), macros: new Map(), scanners: new Map() };
  let matchingOwner = null;
  const line = (entry) =>
    renderDeclarationLinks({ imports: [], modes: [], extras: [], conflicts: [], precedences: [], macros: [], scanners: [], ...entry })[0];
  for (const source of sources) {
    const own = mapDeclarations(grammarDeclarations(source.grammar), (name) => canonical(source, name));
    if (own.matching !== undefined) {
      if (declarations.matching === null) {
        declarations.matching = own.matching;
        matchingOwner = source.id;
      } else if (declarations.matching !== own.matching) {
        conflicts.push({ name: 'matching', basis: 'different-matching', members: [matchingOwner, source.id] });
      }
    }
    for (const name of own.imports) if (!declarations.imports.includes(name)) declarations.imports.push(name);
    for (const name of own.modes) if (!declarations.modes.includes(name)) declarations.modes.push(name);
    for (const [field, key] of [['extras', renderLinksExpression], ['conflicts', (group) => line({ conflicts: [group] })], ['precedences', (order) => line({ precedences: [order] })]]) {
      for (const entry of own[field]) {
        const text = key(entry);
        if (seen[field].has(text)) continue;
        seen[field].add(text);
        declarations[field].push(entry);
      }
    }
    for (const [field, basis] of [['macros', 'different-macro'], ['scanners', 'different-scanner']]) {
      for (const entry of own[field]) {
        const text = line({ [field]: [entry] });
        const first = seen[field].get(entry.name);
        if (first === undefined) {
          seen[field].set(entry.name, { text, owner: source.id });
          declarations[field].push(entry);
        } else if (first.text !== text) {
          conflicts.push({ name: entry.name, basis, members: [first.owner, source.id] });
        }
      }
    }
  }
  if (declarations.matching === null) delete declarations.matching;
  return { declarations, conflicts };
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
  if (items.length === 1) return items[0];
  // The comparison text of an unordered choice is order-free, but the merged
  // expression keeps the source order: a PEG-style parser (the native
  // executor under `matching peg`, or an exported Peggy grammar) commits to
  // the first matching alternative, so `letter | letter word` would stop
  // after one letter.
  const texts = items.map(({ text }) => text);
  if (!ordered) texts.sort(compareText);
  return {
    expr: { kind: 'choice', items: items.map(({ expr }) => expr), ordered },
    text: `${ordered ? 'first' : 'alt'}(${texts.join(',')})`,
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
