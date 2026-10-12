// Independent grammar-tool cycles. The target AST and parser both come from
// Peggy; its AST is translated back to native expressions before native replay.
import { createHash } from 'node:crypto';
import { Grammar, emitPeggy, parseWithGrammar } from '../src/grammar.js';
import { grammarImporter } from '../src/grammar-interchange.js';

const digest = (source) => createHash('sha256').update(source).digest('hex');
const byteOffset = (source, offset) => Buffer.byteLength(source.slice(0, offset));

export function importIndependentPeggyAst(ast, original) {
  if (!Array.isArray(ast?.rules) || ast.rules.length !== original.rules.size + 1) {
    throw new TypeError('the independent target AST does not contain every source rule');
  }
  const names = new Map(ast.rules.slice(1).map((rule, index) => [rule.name, original.ruleNames()[index]]));
  function expression(node) {
    const unary = { optional: 'optional', zero_or_more: 'repeat0', one_or_more: 'repeat1', simple_and: 'and', simple_not: 'not' };
    if (unary[node.type]) return { kind: unary[node.type], item: expression(node.expression) };
    switch (node.type) {
      case 'literal': return { kind: node.ignoreCase ? 'literalInsensitive' : 'literal', value: node.value };
      case 'any': return { kind: 'any' };
      case 'rule_ref': {
        if (!names.has(node.name)) throw new TypeError(`the target AST references an unknown rule ${node.name}`);
        return { kind: 'ref', name: names.get(node.name) };
      }
      case 'sequence': return { kind: 'seq', items: node.elements.map(expression) };
      case 'choice': return { kind: 'choice', ordered: true, items: node.alternatives.map(expression) };
      case 'class': {
        if (node.ignoreCase) throw new TypeError('a folded target character class needs explicit native lowering');
        return { kind: 'charClass', negated: node.inverted, items: node.parts.map((part) => Array.isArray(part)
          ? { kind: 'range', start: part[0], end: part[1] } : { kind: 'char', value: part }) };
      }
      case 'labeled': return { kind: 'capture', label: node.label, item: expression(node.expression) };
      case 'group': return expression(node.expression);
      default: throw new TypeError(`the target AST construct ${node.type} has no native lowering`);
    }
  }
  const rules = new Map(ast.rules.slice(1).map((rule) => [names.get(rule.name), {
    ...original.rule(names.get(rule.name)), expression: expression(rule.expression),
  }]));
  return new Grammar(original.start, rules, 'peg', original.declarations);
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
  try { return { accepted: true, tree: namedTree(parseWithGrammar(grammar, source), source) }; }
  catch (error) { return { accepted: false, offset: error.offset, reason: error.reason, expected: error.expected }; }
}

function targetOutcome(parser, source) {
  try { return { accepted: true, tree: parser.parse(source) }; }
  catch (error) { return { accepted: false, offset: byteOffset(source, error.location?.start.offset ?? 0), reason: 'syntax' }; }
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
    const target = targetOutcome(parser, sample.source);
    const replay = nativeOutcome(reimported, sample.source);
    const failures = [];
    for (const [name, outcome] of Object.entries({ native, target, replay })) {
      if (outcome.accepted !== sample.accepted) failures.push(`${name} acceptance differs from the fixture`);
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
  const mutation = {
    sourceSha256: digest(changed),
    originalWitnessRejectedByTarget: !targetOutcome(changedParser, witness).accepted,
    originalWitnessRejectedByNativeReplay: !nativeOutcome(changedGrammar, witness).accepted,
  };
  const failures = observations.flatMap((observation) => observation.failures.map((failure) => `${JSON.stringify(observation.source)}: ${failure}`));
  if (!mutation.originalWitnessRejectedByTarget || !mutation.originalWitnessRejectedByNativeReplay) failures.push('the target mutation did not change both executions');
  return { fixture: fixture.id, sourceFormat: fixture.format, targetTool: 'peggy', targetVersion: peggy.VERSION,
    sourceSha256: digest(fixture.source), targetSha256: digest(source),
    observations, mutation, failures };
}
