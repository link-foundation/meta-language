// Structural host transport for the shared lexical literal-extension plan.
import { readGrammarLexerEntryForm, readGrammarLexerLiteralExtension } from './translation/grammar-lexer-rules.js';

// Project original owner bodies in declaration order. The portable rule owns
// literal priority, implicit identities and all existing-entry rename decisions.
export function extendGrammarLexerLiteral(grammar, lexer, owner, value, retainedLiterals = []) {
  const entries = lexer.expression.items;
  const parserLiterals = [];
  const visit = (expression) => {
    if (expression.kind === 'literal' && expression.value !== '') parserLiterals.push(expression.value);
    if (expression.item) visit(expression.item);
    if (expression.kind !== 'charClass' && expression.kind !== 'byteClass') for (const item of expression.items ?? []) if (item?.kind) visit(item);
  };
  for (const entry of entries) if (readGrammarLexerEntryForm(entry.kind) === 1) {
    const rule = grammar.rule(entry.item.name);
    if (rule) visit(rule.expression);
  }
  // A transformed target may no longer use an original literal. Its actual
  // retained binding still competes lexically and stays in the source pool.
  parserLiterals.push(...retainedLiterals);
  const plan = readGrammarLexerLiteralExtension(owner, value, entries.map(({ kind }) => kind), entries.map(({ name }) => name ?? ''),
    entries.map((entry) => entry.value ?? ''), entries.map(({ item }) => item?.kind === 'ref' ? item.name : ''),
    entries.map(({ item }) => item?.kind ?? ''), entries.map(({ item }) => item?.kind === 'literal' ? item.value : ''), parserLiterals);
  if (plan[0] < 0) throw new TypeError(`invalid lexical literal extension in lexer ${lexer.name}`);
  if (plan[0] === 0) return lexer;
  const name = plan[0] === 2 ? entries[plan[2]].name : `@literal:${plan[2]}`;
  const added = plan[0] === 3 ? [{ kind: 'lexicalObligation', name: '', category: 'literal', source: value }]
    : [{ kind: 'lexicalLiteral', value, name }, ...(plan[0] === 1 ? [{ kind: 'lexicalRule', name, type: name,
      mode: 'default', channel: 'default', disposition: 'emit', item: { kind: 'literal', value }, action: [] }] : [])];
  const renames = new Map();
  for (let index = 3; index < plan.length; index += 2) renames.set(plan[index], `@literal:${plan[index + 1]}`);
  const items = [];
  for (let index = 0; index <= entries.length; index += 1) {
    if (index === plan[1]) items.push(...added);
    if (index < entries.length) {
      const entry = entries[index];
      items.push(renames.has(index) ? { ...entry, name: renames.get(index),
        ...(readGrammarLexerEntryForm(entry.kind) === 7 ? { type: renames.get(index) } : {}) } : entry);
    }
  }
  return { ...lexer, expression: { ...lexer.expression, items } };
}

