// ANTLR-family plans are separate from contextual native extras. All matches
// use source-local lexical definitions; canonical rule unions are projections,
// never candidates for an imported lexer's maximal-munch decision.
import {
  readGrammarLexerDeclarationIssue,
  readGrammarLexerEntryForm,
  readGrammarLexerLazyNormalization,
  readGrammarLexerObligationPositions,
} from '../translation/grammar-lexer-rules.js';

/** Source-production scope, before a merge changes any canonical rule body. */
export function lexerScopeOf(grammar, name) {
  for (const lexer of grammar.declarations.lexers ?? []) {
    if (lexer.expression.items.some((entry) => (entry.kind === 'lexicalOwner' || entry.kind === 'lexicalBinding') && entry.item.name === name)) return lexer.name;
  }
  return '';
}

/** Renames public bindings; source-local matcher references remain local. */
export function mapLexerDeclaration(lexer, rename, name = lexer.name) {
  return { name, expression: { ...lexer.expression, items: lexer.expression.items.map((entry) =>
    entry.kind === 'lexicalOwner' || entry.kind === 'lexicalBinding'
      ? { ...entry, item: { ...entry.item, name: rename(entry.item.name) } } : entry) } };
}

// A lazy unrestricted middle with a literal final delimiter stops at its
// first delimiter. This exact lowering does not infer a prefix from FIRST.
// Other non-greedy decisions remain explicit unsupported obligations.
export function lowerLexerExpression(expression, obligations, name) {
  const nodes = [], inner = [], children = [], edgeSources = [], edgeTargets = [];
  const capture = (node) => {
    const index = nodes.length;
    nodes.push(node); inner.push(-1); children.push([]);
    if (node.kind === 'charClass' || node.kind === 'byteClass') return index;
    if (node.item) inner[index] = capture(node.item);
    for (const item of node.items ?? []) {
      const child = capture(item);
      children[index].push(child); edgeSources.push(index); edgeTargets.push(child);
    }
    return index;
  };
  capture(expression);
  const normalization = readGrammarLexerLazyNormalization(nodes.map(({ kind }) => kind), nodes.map(({ label }) => label ?? ''),
    nodes.map(({ value }) => typeof value === 'string' ? value : ''), inner, edgeSources, edgeTargets);
  if (normalization[0] === -1) throw new TypeError('invalid lexical expression graph projection');
  const replacements = new Map();
  for (let index = 0; index < normalization.length; index += 2) {
    const delimiter = normalization[index + 1];
    if (delimiter < 0) obligations.push({ name, category: 'non-greedy', source: 'unsupported non-greedy lexical decision' });
    else replacements.set(normalization[index], delimiter);
  }
  const materialize = (index) => {
    const node = nodes[index];
    if (replacements.has(index)) return { kind: 'repeat0', item: { kind: 'seq', items: [{ kind: 'not', item: nodes[replacements.get(index)] }, { kind: 'any' }] } };
    if (node.kind === 'charClass' || node.kind === 'byteClass') return node;
    return { ...node, ...(node.item ? { item: materialize(inner[index]) } : {}), ...(node.items ? { items: children[index].map(materialize) } : {}) };
  };
  return materialize(0);
}

/** Validates the inspectable declaration and prepares the local matcher input. */
export function readLexerDeclaration(declaration, fail) {
  if (declaration.expression?.kind !== 'seq') fail(`lexer ${declaration.name} needs an ordered declaration list`);
  const entries = declaration.expression.items;
  const actionOwners = [], actions = [], actionModes = [];
  entries.forEach((entry, index) => { for (const action of entry.action ?? []) {
    actionOwners.push(index); actions.push(action.operation); actionModes.push(action.mode ?? '');
  } });
  const issue = readGrammarLexerDeclarationIssue(entries.map(({ kind }) => kind), entries.map(({ name }) => name ?? ''),
    entries.map(({ item }) => item?.kind ?? ''), entries.map(({ value }) => value ?? ''),
    entries.map(({ mode }) => mode ?? ''), entries.map(({ channel }) => channel ?? ''), entries.map(({ disposition }) => disposition ?? ''), actionOwners, actions, actionModes);
  const entry = entries[issue[1]];
  const messages = [null, 'invalid lexical declaration fact columns', `unknown declaration ${entry?.kind} in lexer ${declaration.name}`,
    'a lexical owner must name one unique original parser owner and rule', 'a lexical binding must name one unique source token and rule',
    'a parser literal has multiple lexical bindings', `duplicate lexical rule ${entry?.name}`,
    `undefined mode ${entry?.mode} in lexer ${declaration.name}`, `undefined channel ${entry?.channel} in lexer ${declaration.name}`,
    `unsupported lexical command ${actions[issue[2]]}`, `undefined command mode ${actionModes[issue[2]]}`, `unsupported lexical disposition ${entry?.disposition}`];
  if (issue[0] !== 0) fail(messages[issue[0]]);
  const plan = { name: declaration.name, rules: [], bindings: new Map(), literals: new Map(), owners: new Map(), originalOwners: new Set(), obligations: [], modes: new Set(['default']), channels: new Set(['default', 'HIDDEN']), types: new Set(), localRules: new Map() };
  for (const entry of entries) {
    switch (readGrammarLexerEntryForm(entry.kind)) {
      case 1:
        plan.owners.set(entry.item.name, entry.name);
        plan.originalOwners.add(entry.name);
        break;
      case 2:
        plan.bindings.set(entry.name, entry.item.name);
        break;
      case 3:
        plan.literals.set(entry.value, entry.name);
        break;
      case 4: plan.modes.add(entry.name); break;
      case 5: plan.channels.add(entry.name); break;
      case 6: plan.obligations.push(entry); break;
      case 7: case 8: {
        const item = lowerLexerExpression(entry.item, plan.obligations, entry.name);
        plan.localRules.set(entry.name, { name: entry.name, kind: 'silent', expression: item });
        if (readGrammarLexerEntryForm(entry.kind) === 7) plan.rules.push(entry);
        break;
      }
      case 9: plan.types.add(entry.name); break;
    }
  }
  return plan;
}

/** The unresolved executable obligations reachable in one token's local body. */
export function lexerObligationsFor(plan, name) {
  const names = [...plan.localRules.keys()];
  const positions = new Map(names.map((entry, index) => [entry, index]));
  const edgeSources = [], edgeTargets = [];
  names.forEach((name, index) => {
    const pending = [plan.localRules.get(name).expression];
    while (pending.length > 0) {
      const expression = pending.pop();
      if (expression.kind === 'ref' && positions.has(expression.name)) { edgeSources.push(index); edgeTargets.push(positions.get(expression.name)); }
      if (expression.item) pending.push(expression.item);
      if (expression.kind !== 'charClass' && expression.kind !== 'byteClass') pending.push(...expression.items ?? []);
    }
  });
  const selected = readGrammarLexerObligationPositions(names.length, positions.get(name), edgeSources, edgeTargets,
    plan.obligations.map((entry) => positions.get(entry.name) ?? -1), plan.obligations.map(({ category }) => category));
  if (selected[0] === -1) throw new TypeError('invalid source-local lexical obligation graph projection');
  return selected.map((position) => plan.obligations[position]);
}
