// Independent grammar-tool cycles. The target AST and parser both come from
// Peggy; its AST is translated back to native expressions before native replay.
import { createHash } from 'node:crypto';
import { Grammar, emitPeggy } from '../src/grammar.js';
import { grammarImporter } from '../src/grammar-interchange.js';
import { extendGrammarLexerLiteral } from '../src/grammar-lexer-literal-extension.js';
import { readGrammarLexerEntryForm } from '../src/translation/grammar-lexer-rules.js';
import { isGrammarToolSyntaxRejection, nativeGrammarToolOutcome, targetGrammarToolOutcome } from './grammar-tool-outcomes.mjs';

const digest = (source) => createHash('sha256').update(source).digest('hex');

function lexicalOwnerFor(grammar, name) {
  const owners = (grammar.declarations.lexers ?? []).filter(({ expression }) => expression.items.some((entry) =>
    readGrammarLexerEntryForm(entry.kind) === 1 && entry.item?.kind === 'ref' && entry.item.name === name));
  if (owners.length > 1) throw new TypeError(`the target AST rule ${name} has multiple source lexical owners`);
  return owners[0] ?? null;
}

// Peggy 5.1.0 merges adjacent single-character literal alternatives into a
// class. Under a retained source lexer, parser terminals denote token types.
// Project a finite optimized set back to its exact source literal bindings;
// cardinality equality forbids silently dropping an added or unbound member.
function lowerIndependentPeggyLexicalClass(node, lexer) {
  if (node.inverted) throw new TypeError('an inverted target character class has no finite source lexical lowering');
  const intervals = node.parts.map((part) => {
    const endpoints = Array.isArray(part) ? part : [part, part];
    if (endpoints.length !== 2 || endpoints.some((value) => typeof value !== 'string' || [...value].length !== 1)) {
      throw new TypeError('the target character class has an unsupported source lexical endpoint');
    }
    const [start, end] = endpoints.map((value) => value.codePointAt(0));
    if (start > end || (start <= 0xdfff && end >= 0xd800) || (!node.unicode && end > 0xffff)) {
      throw new TypeError('the target character class has no exact Unicode source lexical lowering');
    }
    return [start, end];
  }).sort(([first], [second]) => first - second);
  const merged = [];
  for (const [start, end] of intervals) {
    const previous = merged.at(-1);
    if (previous && start <= previous[1] + 1) previous[1] = Math.max(previous[1], end);
    else merged.push([start, end]);
  }
  const cardinality = merged.reduce((count, [start, end]) => count + end - start + 1, 0);
  const selected = new Map();
  for (const entry of lexer.expression.items) {
    if (readGrammarLexerEntryForm(entry.kind) !== 3 || [...entry.value].length !== 1) continue;
    const point = entry.value.codePointAt(0);
    if (!merged.some(([start, end]) => point >= start && point <= end)) continue;
    if (selected.has(entry.value)) throw new TypeError('the target character class has duplicate source lexical bindings');
    selected.set(entry.value, { kind: 'literal', value: entry.value });
  }
  if (selected.size !== cardinality) throw new TypeError('the target character class has unbound source lexical members');
  const items = [...selected.values()];
  if (items.length === 0) return { kind: 'not', item: { kind: 'empty' } };
  if (items.length === 1) return items[0];
  return { kind: 'choice', ordered: true, items };
}

export function importIndependentPeggyAst(ast, original) {
  if (!Array.isArray(ast?.rules) || ast.rules.length !== original.rules.size + 1) {
    throw new TypeError('the independent target AST does not contain every source rule');
  }
  const names = new Map(ast.rules.slice(1).map((rule, index) => [rule.name, original.ruleNames()[index]]));
  function expression(node, lexer) {
    const unary = { optional: 'optional', zero_or_more: 'repeat0', one_or_more: 'repeat1', simple_and: 'and', simple_not: 'not' };
    if (unary[node.type]) return { kind: unary[node.type], item: expression(node.expression, lexer) };
    switch (node.type) {
      case 'literal': return { kind: node.ignoreCase ? 'literalInsensitive' : 'literal', value: node.value };
      case 'any': return { kind: 'any' };
      case 'rule_ref': {
        if (!names.has(node.name)) throw new TypeError(`the target AST references an unknown rule ${node.name}`);
        return { kind: 'ref', name: names.get(node.name) };
      }
      case 'sequence': return { kind: 'seq', items: node.elements.map((node) => expression(node, lexer)) };
      case 'choice': return { kind: 'choice', ordered: true, items: node.alternatives.map((node) => expression(node, lexer)) };
      case 'class': {
        if (node.ignoreCase) throw new TypeError('a folded target character class needs explicit native lowering');
        if (lexer) return lowerIndependentPeggyLexicalClass(node, lexer);
        return { kind: 'charClass', negated: node.inverted, items: node.parts.map((part) => Array.isArray(part)
          ? { kind: 'range', start: part[0], end: part[1] } : { kind: 'char', value: part }) };
      }
      case 'labeled': return { kind: 'capture', label: node.label, item: expression(node.expression, lexer) };
      case 'group': return expression(node.expression, lexer);
      default: throw new TypeError(`the target AST construct ${node.type} has no native lowering`);
    }
  }
  const rules = new Map(ast.rules.slice(1).map((rule) => [names.get(rule.name), {
    ...original.rule(names.get(rule.name)), expression: expression(rule.expression, lexicalOwnerFor(original, names.get(rule.name))),
  }]));
  const replay = new Grammar(original.start, rules, 'peg', original.declarations);
  const originalLexers = original.declarations.lexers ?? [];
  const lexers = originalLexers.map((originalLexer) => {
    const retained = originalLexer.expression.items.filter(({ kind }) => readGrammarLexerEntryForm(kind) === 3).map(({ value }) => value);
    let lexer = originalLexer;
    const visit = (node, owner) => {
      if (node.kind === 'literal') lexer = extendGrammarLexerLiteral(replay, lexer, owner, node.value, retained);
      if (node.item) visit(node.item, owner);
      if (node.kind !== 'charClass' && node.kind !== 'byteClass') for (const child of node.items ?? []) if (child?.kind) visit(child, owner);
    };
    for (const entry of originalLexer.expression.items) if (readGrammarLexerEntryForm(entry.kind) === 1) {
      const rule = replay.rule(entry.item.name);
      if (rule) visit(rule.expression, rule.name);
    }
    return lexer;
  });
  if (lexers.every((lexer, index) => lexer === originalLexers[index])) return replay;
  return new Grammar(original.start, rules, 'peg', { ...original.declarations, lexers });
}

function namedTree(tree, source) {
  if (tree.type !== 'node') return null;
  return {
    kind: tree.kind, start: tree.start, end: tree.end,
    text: Buffer.from(source).subarray(tree.start, tree.end).toString(),
    children: tree.children.map((child) => namedTree(child, source)).filter(Boolean),
  };
}

function targetActions(source, grammar) {
  const lines = source.split('\n');
  let index = 0;
  return `{
    function children(value) {
      if (Array.isArray(value)) return value.flatMap(children);
      return value && typeof value === 'object' && 'kind' in value ? [value] : [];
    }
    function bytes(offset) { return new TextEncoder().encode(input.slice(0, offset)).length; }
  }\n` + lines.map((line, lineIndex) => {
    if (lineIndex < 2 || line.length === 0) return line;
    const match = /^(\w+) = (.+)$/u.exec(line);
    if (!match) throw new TypeError('the emitted target rule is not a named expression');
    const kind = grammar.ruleNames()[index++];
    return `${match[1]} = value:(${match[2]}) { return {
      kind: ${JSON.stringify(kind)}, start: bytes(location().start.offset),
      end: bytes(location().end.offset), text: text(), children: children(value)
    }; }`;
  }).join('\n');
}

function nativeOutcome(grammar, source) {
  return nativeGrammarToolOutcome(grammar, source, namedTree);
}

export function checkIndependentGrammarCycle(fixture, peggy) {
  const grammar = grammarImporter(fixture.format)(fixture.source);
  const source = emitPeggy(grammar);
  const ast = peggy.generate(source, { output: 'ast' });
  const reimported = importIndependentPeggyAst(ast, grammar);
  const parser = peggy.generate(targetActions(source, grammar));
  const samples = [...fixture.accepts.map((entry) => ({ ...entry, accepted: true })),
    ...fixture.rejects.map((entry) => ({ ...entry, accepted: false }))];
  const observations = samples.map((sample) => {
    const native = nativeOutcome(grammar, sample.source);
    const target = targetGrammarToolOutcome(parser, sample.source);
    const replay = nativeOutcome(reimported, sample.source);
    const failures = [];
    for (const [name, outcome] of Object.entries({ native, target, replay })) {
      if (outcome.accepted !== sample.accepted) failures.push(`${name} acceptance differs from the fixture`);
      if (outcome.accepted === null) failures.push(`${name} operation failed: ${outcome.error.name ?? outcome.error.thrownType}: ${outcome.error.message ?? outcome.error.description}`);
      if (!sample.accepted && !isGrammarToolSyntaxRejection(outcome)) failures.push(`${name} did not produce a syntax rejection`);
      if (!sample.accepted && outcome.offset !== sample.offset) failures.push(`${name} diagnostic offset differs from the fixture`);
      if (sample.tree && JSON.stringify(outcome.tree) !== JSON.stringify(sample.tree)) failures.push(`${name} parse structure differs from the fixture`);
    }
    if (sample.accepted && JSON.stringify(native.tree) !== JSON.stringify(target.tree)) failures.push('the independent target changes the native parse structure');
    if (sample.accepted && JSON.stringify(native.tree) !== JSON.stringify(replay.tree)) failures.push('target AST reimport changes the native parse structure');
    return { source: sample.source, native, target, replay, failures };
  });
  // A target action is executed, and a changed target literal must change both
  // the target and AST-reimported native execution. Cached source cannot pass.
  const firstLiteral = /"(?:[^"\\]|\\.)+"/u.exec(source);
  if (!firstLiteral) throw new TypeError('an independent cycle needs a literal mutation witness');
  const marker = 'independent-target-mutation';
  const changed = source.slice(0, firstLiteral.index) + JSON.stringify(marker) + source.slice(firstLiteral.index + firstLiteral[0].length);
  const changedAst = peggy.generate(changed, { output: 'ast' });
  const changedGrammar = importIndependentPeggyAst(changedAst, grammar);
  const changedParser = peggy.generate(targetActions(changed, grammar));
  const witness = fixture.accepts[0].source;
  const target = targetGrammarToolOutcome(changedParser, witness);
  const replay = nativeOutcome(changedGrammar, witness);
  const mutation = {
    sourceSha256: digest(changed),
    target, replay,
    originalWitnessRejectedByTarget: isGrammarToolSyntaxRejection(target),
    originalWitnessRejectedByNativeReplay: isGrammarToolSyntaxRejection(replay),
  };
  const failures = observations.flatMap((observation) => observation.failures.map((failure) => `${JSON.stringify(observation.source)}: ${failure}`));
  if (!mutation.originalWitnessRejectedByTarget || !mutation.originalWitnessRejectedByNativeReplay) failures.push('the target mutation did not change both executions');
  return { fixture: fixture.id, sourceFormat: fixture.format, targetTool: 'peggy', targetVersion: peggy.VERSION,
    sourceSha256: digest(fixture.source), targetSha256: digest(source),
    observations, mutation, failures };
}
