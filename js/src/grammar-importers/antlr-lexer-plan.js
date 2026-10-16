// Source-local ANTLR lexer plans. The historical structural rule bodies stay
// intact; these executable Links declarations retain the lexical decisions
// which cannot be recovered from a reconciled union of token definitions.
import { GrammarBuilder as G } from '../grammar.js';
import { readGrammarLexerCommandPlan, readGrammarLexerEntryForm, readGrammarLexerSectionRulePositions,
  readGrammarLexerLiteralRulePosition, readGrammarLexerBindingNames } from '../translation/grammar-lexer-rules.js';

function visit(expression, callback) {
  callback(expression);
  if (expression.item) visit(expression.item, callback);
  for (const item of expression.items ?? []) if (item?.kind) visit(item, callback);
}

function commandPlan(command, name) {
  const result = readGrammarLexerCommandPlan(command ?? '', name);
  const [type, channel, disposition] = result;
  const action = [];
  const obligations = [];
  for (let index = 3; index < result.length; index += 2) {
    const operation = result[index];
    const argument = result[index + 1];
    if (operation === 'unsupported') obligations.push({ category: 'command', source: argument });
    else action.push({ operation, ...(operation === 'popMode' ? {} : { mode: argument }) });
  }
  return { type, channel, disposition, action, obligations };
}

/** Builds lexer declarations without rewriting or removing imported rules. */
export function antlrLexerDeclarations(sections, rules) {
  const lexers = [];
  for (const section of sections.filter(({ kind }) => kind !== 'parser')) {
    const positions = readGrammarLexerSectionRulePositions(sections.indexOf(section), sections.map(({ kind }) => kind),
      sections.map(({ name }) => name), sections.map(({ tokenVocab }) => tokenVocab ?? ''),
      rules.map(({ section }) => sections.indexOf(section)), rules.map(({ kind }) => kind));
    const separator = positions.indexOf(-1);
    const lexical = positions.slice(0, separator).map((index) => rules[index]);
    const owners = positions.slice(separator + 1).map((index) => rules[index]);
    if (lexical.length === 0 && owners.length === 0) continue;
    const entries = [
      ...section.channels.map((name) => ({ kind: 'lexicalChannel', name })),
      ...section.types.map((name) => ({ kind: 'lexicalType', name })),
      ...section.modes.map((name) => ({ kind: 'lexicalMode', name })),
      ...owners.map(({ name }) => ({ kind: 'lexicalOwner', name, item: G.ref(name) })),
    ];
    for (const source of section.imports) entries.push({ kind: 'lexicalObligation', name: '', category: 'import', source });
    for (const source of section.unsupportedOptions) entries.push({ kind: 'lexicalObligation', name: '', category: 'option', source });
    for (const owner of owners) for (const obligation of owner.obligations) {
      entries.push({ kind: 'lexicalObligation', name: owner.name, ...obligation });
    }
    const parserLiterals = new Set();
    const parserReferences = new Set();
    for (const owner of owners) visit(owner.expression, (item) => {
      if (item.kind === 'literal' && item.value !== '') parserLiterals.add(item.value);
      if (item.kind === 'ref') parserReferences.add(item.name);
    });
    let implicit = 0;
    for (const value of parserLiterals) {
      const selected = readGrammarLexerLiteralRulePosition(value, section.kind, lexical.map(({ kind }) => kind),
        lexical.map(({ expression }) => expression.kind === 'literal' ? expression.value : ''), lexical.map(({ expression }) => expression.kind === 'literal'));
      if (selected >= 0) entries.push({ kind: 'lexicalLiteral', value, name: lexical[selected].name });
      else if (selected === -2) {
        entries.push({ kind: 'lexicalObligation', name: '', category: 'literal', source: value });
      } else {
        const name = `@literal:${implicit++}`;
        entries.push({ kind: 'lexicalLiteral', value, name });
        entries.push({ kind: 'lexicalRule', name, type: name, mode: 'default', channel: 'default', disposition: 'emit', item: G.literal(value), action: [] });
      }
    }
    // A declared but never emitted token still has a parser identity. Keep
    // unused declarations as metadata without adding historical public rules.
    const tokenPlans = [];
    for (const rule of lexical) {
      for (const obligation of rule.obligations) entries.push({ kind: 'lexicalObligation', name: rule.name, ...obligation });
      if (rule.kind === 'silent') {
        entries.push({ kind: 'lexicalFragment', name: rule.name, item: rule.expression });
        continue;
      }
      const plan = commandPlan(rule.command, rule.name);
      tokenPlans.push({ rule, plan });
      entries.push({ kind: 'lexicalRule', name: rule.name, type: plan.type, mode: rule.mode, channel: plan.channel, disposition: plan.disposition, item: rule.expression, action: plan.action });
      for (const obligation of plan.obligations) entries.push({ kind: 'lexicalObligation', name: rule.name, ...obligation });
    }
    const bindings = readGrammarLexerBindingNames(section.types, [...parserReferences], tokenPlans.map(({ rule }) => rule.name),
      tokenPlans.map(({ rule }) => rule.retype ?? ''), tokenPlans.map(({ plan }) => plan.type), tokenPlans.map(({ plan }) => plan.disposition));
    for (const name of bindings) entries.push({ kind: 'lexicalBinding', name, item: G.ref(name) });
    lexers.push({ name: section.name, expression: { kind: 'seq', items: entries } });
  }
  return lexers;
}

/** Adds only missing public token identities from the complete lexical plan. */
export function completeAntlrLexerBindings(lexers, rules) {
  const names = new Set(rules.map(({ name }) => name));
  for (const lexer of lexers) {
    const entries = lexer.expression.items;
    for (const entry of entries) {
      if (readGrammarLexerEntryForm(entry.kind) !== 2 || names.has(entry.item.name)) continue;
      const sources = entries.filter((candidate) => readGrammarLexerEntryForm(candidate.kind) === 7
        && candidate.disposition === 'emit' && candidate.type === entry.name).map(({ name }) => name);
      // No lexer rule can emit a declared-only identity. A failing structural
      // body preserves that fact when the declaration is inspected separately.
      const expression = sources.length === 0 ? G.not(G.empty()) : G.choice(...sources.map((name) => G.ref(name)));
      rules.push({ name: entry.item.name, kind: 'token', expression,
        doc: `lexer ${lexer.name} token ${entry.name}`, channel: null, retype: null });
      names.add(entry.item.name);
    }
  }
}
