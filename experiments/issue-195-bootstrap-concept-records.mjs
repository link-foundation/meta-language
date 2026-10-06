// Generates parity/naming/canonical-concepts.json: the concept record of every
// canonical name the sources define. Definitions and syntax aliases that a
// source concept table gives are copied from the source (the naming check
// requires them to match); former names are copied from the former-name
// tables; the rest is written here once and then maintained in the JSON file.
//
// Usage: node experiments/issue-195-bootstrap-concept-records.mjs [--write]

import { writeFileSync } from 'node:fs';

import { extractFormerNames, extractSourceDescriptions, phraseOf } from '../js/scripts/issue-195-naming.mjs';

const GRAMMAR_EXPRESSION = [
  'A grammar expression of this kind is one variant of the grammar expression tree.',
  'Importers and emitters of every grammar format map it to the same concept.',
];
const GRAMMAR_RULE_NAME = [
  'Grammar inference and the grammar surface translation use this concept to name rules.',
  'The English and Russian grammar surfaces translate the rule name through this concept.',
];
const GRAMMAR_VALUE = [
  'A grammar link stores the value under this tag so the grammar reads back unchanged.',
];
const STRUCTURAL = [
  'The structural concept is independent of any one programming language.',
  'Every language syntax that expresses it is a syntax alias of the concept.',
];
const DOCUMENT = [
  'Every document format that supports the element maps it to this concept.',
  'The formatting concept round-trips through Markdown and HTML templates.',
];
const PROGRAM = [
  'Each language frontend reports evidence for this construct or states why it is unavailable.',
];
const OPERATION = [
  'Every supported programming interface style reaches this operation.',
];
const STAGE = [
  'Each translation corpus program records a digest for this stage.',
];

// Hand data: role, definition when no source defines it, constraints, extra
// source aliases, `represents` for a representation of another concept, and
// `distinctFrom` for WordNet synonyms that name different concepts.
const HAND = {
  'grammar.rule': { constraints: ['A rule binds exactly one nonterminal name to one grammar expression.', ...GRAMMAR_RULE_NAME] },
  'grammar.sequence': { constraints: ['A sequence matches its expressions in order, each after the previous one.', ...GRAMMAR_EXPRESSION] },
  'grammar.ordered-choice': { constraints: ['The first alternative that matches wins; later alternatives are not tried.', ...GRAMMAR_EXPRESSION] },
  'grammar.unordered-choice': { constraints: ['Every alternative is a possible match; none is preferred.', ...GRAMMAR_EXPRESSION] },
  'grammar.choice': {
    definition: 'A grammar alternative among expressions, ordered or unordered.',
    constraints: ['A choice is either an ordered choice or an unordered choice.', ...GRAMMAR_EXPRESSION],
    aliases: [{ source: 'meta-language', name: 'choice' }],
  },
  'grammar.counted-repetition': { constraints: ['The minimum count is at most the maximum count when a maximum is given.', ...GRAMMAR_EXPRESSION] },
  'grammar.zero-or-more-repetition': { constraints: ['The repetition also accepts no occurrence at all.', ...GRAMMAR_EXPRESSION] },
  'grammar.one-or-more-repetition': { constraints: ['The repetition requires at least one occurrence.', ...GRAMMAR_EXPRESSION] },
  'grammar.optional-expression': {
    constraints: ['The expression accepts zero or one occurrence of its inner expression.', ...GRAMMAR_EXPRESSION],
    aliases: [{ source: 'peg', name: 'e?' }, { source: 'ebnf', name: '[ e ]' }, { source: 'meta-language', name: 'optional expression' }],
  },
  'grammar.terminal': { constraints: ['The literal text is matched exactly, letter case included.', ...GRAMMAR_EXPRESSION] },
  'grammar.case-insensitive-terminal': {
    definition: 'A literal terminal token matched in the input without regard to letter case.',
    constraints: ['The literal text is matched with letter case ignored.', ...GRAMMAR_EXPRESSION],
    aliases: [{ source: 'abnf', name: '"lit"' }, { source: 'lark', name: '"lit"i' }, { source: 'meta-language', name: 'case-insensitive terminal' }],
  },
  'grammar.nonterminal': { constraints: ['The referenced rule is defined in the same grammar.', ...GRAMMAR_EXPRESSION] },
  'grammar.character-class': {
    constraints: ['A character class lists characters and character ranges, and may be negated.', ...GRAMMAR_EXPRESSION],
    aliases: [{ source: 'peg', name: '[a-z]' }, { source: 'gbnf', name: '[a-z]' }, { source: 'meta-language', name: 'character class' }],
  },
  'grammar.character-range': { constraints: ['The first endpoint is not greater than the second endpoint.', ...GRAMMAR_EXPRESSION] },
  'grammar.any-character': { constraints: ['The expression consumes exactly one character and fails at the end of input.', ...GRAMMAR_EXPRESSION] },
  'grammar.positive-predicate': { constraints: ['The predicate consumes no input whether it succeeds or fails.', ...GRAMMAR_EXPRESSION] },
  'grammar.negative-predicate': { constraints: ['The predicate consumes no input whether it succeeds or fails.', ...GRAMMAR_EXPRESSION] },
  'grammar.capture': { constraints: ['A labelled capture carries its label as a readable name.', ...GRAMMAR_EXPRESSION] },
  'grammar.empty-expression': { constraints: ['The expression always succeeds and consumes no input.', ...GRAMMAR_EXPRESSION] },
  'grammar.atomic-rule': {
    definition: 'A grammar rule whose expression matches without implicit whitespace between its parts.',
    constraints: ['An atomic rule is a rule kind; its expression is an ordinary grammar expression.', 'Grammar formats without atomic rules keep the kind as a fidelity note.'],
    aliases: [{ source: 'pest', name: '@{ }' }],
  },
  'grammar.silent-rule': {
    definition: 'A grammar rule that matches without producing a parse tree node of its own.',
    constraints: ['A silent rule is a rule kind; its expression is an ordinary grammar expression.', 'Grammar formats without silent rules keep the kind as a fidelity note.'],
    aliases: [{ source: 'pest', name: '_{ }' }, { source: 'tree-sitter', name: '_name' }],
  },
  'grammar.token-rule': {
    definition: 'A grammar rule whose whole match is reported as one lexical token.',
    constraints: ['A token rule is a rule kind; its expression is an ordinary grammar expression.', 'Grammar formats without token rules keep the kind as a fidelity note.'],
    aliases: [{ source: 'tree-sitter', name: 'token(e)' }, { source: 'antlr', name: 'LexerRule' }],
  },
  'grammar.grammar': {
    definition: 'A named set of grammar rules with a start rule.',
    constraints: ['Every nonterminal a rule references names a rule of the grammar.', 'The grammar round-trips through links without losing a rule.'],
    aliases: [{ source: 'antlr', name: 'grammar Name;' }, { source: 'meta-language', name: 'grammar' }],
  },
  'grammar.character-class-item.character': {
    represents: 'grammar.value.character',
    definition: 'A single character listed in a character class.',
    constraints: ['The item matches exactly the listed character.'],
    aliases: [{ source: 'Rust', name: 'CharClassItem::Char' }],
  },
  'grammar.character-class-item.character-range': {
    represents: 'grammar.character-range',
    definition: 'A character range listed in a character class.',
    constraints: ['The item matches every character between its two endpoints inclusive.'],
    aliases: [{ source: 'Rust', name: 'CharClassItem::Range' }],
  },
  'grammar.value.absent-value': {
    definition: 'A grammar field value that is absent.',
    constraints: ['An absent value carries no inner value.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'None' }, { source: 'JavaScript', name: 'null' }],
  },
  'grammar.value.present-value': {
    definition: 'A grammar field value that is present and carries an inner value.',
    constraints: ['A present value carries exactly one inner value.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'Some' }],
  },
  'grammar.value.string': {
    represents: 'grammar.string',
    definition: 'A text value stored in a grammar link.',
    constraints: ['The text is stored as written, escapes resolved.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'String' }],
  },
  'grammar.value.character': {
    definition: 'A single Unicode scalar value stored in a grammar link.',
    constraints: ['The value is exactly one Unicode scalar value.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'char' }],
  },
  'grammar.value.boolean-value': {
    represents: 'grammar.boolean-value',
    definition: 'A truth value stored in a grammar link.',
    constraints: ['The value is true or false.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'bool' }],
  },
  'grammar.value.natural-number': {
    definition: 'A non-negative whole number stored in a grammar link.',
    constraints: ['The value is zero or a positive whole number.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'usize' }],
  },
  'grammar.value.rule-kind': {
    definition: 'The kind of a grammar rule: normal, atomic, silent or token.',
    constraints: ['The value is one of the rule kinds meta-language records.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'RuleKind' }],
  },
  'grammar.value.format': {
    definition: 'The grammar notation a grammar is imported from or emitted as.',
    constraints: ['The value is one of the grammar formats meta-language imports or emits.', ...GRAMMAR_VALUE],
    aliases: [{ source: 'Rust', name: 'GrammarFormat' }],
  },
  'grammar.expression': { definition: 'A rule for a complete expression of a grammar.', constraints: GRAMMAR_RULE_NAME },
  'grammar.term': { definition: 'A rule for an operand of addition or subtraction in an expression grammar.', constraints: GRAMMAR_RULE_NAME },
  'grammar.factor': { definition: 'A rule for an operand of multiplication or division in an expression grammar.', constraints: GRAMMAR_RULE_NAME },
  'grammar.number': { definition: 'A rule for a numeric literal.', constraints: GRAMMAR_RULE_NAME },
  'grammar.digit': { definition: 'A rule for one decimal digit.', constraints: GRAMMAR_RULE_NAME },
  'grammar.letter': { definition: 'A rule for one alphabetic letter.', constraints: GRAMMAR_RULE_NAME },
  'grammar.identifier': { definition: 'A rule for a name made of letters, digits and underscores that starts with a letter.', constraints: GRAMMAR_RULE_NAME },
  'grammar.statement': { definition: 'A rule for one statement of a program grammar.', constraints: GRAMMAR_RULE_NAME },
  'grammar.item': { definition: 'A rule for one element of a list.', constraints: GRAMMAR_RULE_NAME },
  'grammar.list': { definition: 'A rule for a delimited sequence of items.', constraints: GRAMMAR_RULE_NAME },
  'grammar.value': { definition: 'A rule for any value of a data grammar.', constraints: GRAMMAR_RULE_NAME },
  'grammar.name': { definition: 'A rule for the key of an object member.', constraints: GRAMMAR_RULE_NAME },
  'grammar.object': { definition: 'A rule for a delimited set of named members.', constraints: GRAMMAR_RULE_NAME },
  'grammar.member': { definition: 'A rule for one name and value pair of an object.', constraints: GRAMMAR_RULE_NAME },
  'grammar.string': { definition: 'A rule for a quoted text literal.', constraints: GRAMMAR_RULE_NAME },
  'grammar.boolean-value': { definition: 'A rule for the literals true and false.', constraints: GRAMMAR_RULE_NAME },
  'grammar.null': { definition: 'A rule for the literal that denotes no value.', constraints: GRAMMAR_RULE_NAME },
  function: { constraints: STRUCTURAL },
  binding: { constraints: STRUCTURAL },
  application: { constraints: STRUCTURAL },
  'sequential-composition': { constraints: STRUCTURAL },
  branch: { constraints: STRUCTURAL },
  loop: { constraints: STRUCTURAL },
  parameter: {
    constraints: STRUCTURAL,
    distinctFrom: [{ id: 'argument', reason: 'A parameter is the name a function declares; an argument is the value an application supplies for it.' }],
  },
  argument: {
    constraints: STRUCTURAL,
    distinctFrom: [{ id: 'grammar.statement', reason: 'An argument is a value supplied to a function application; a statement is a grammar rule for one step of a program.' }],
  },
  return: { constraints: STRUCTURAL },
  assignment: { constraints: STRUCTURAL },
  statehood: {
    constraints: [
      'The worked example links the Wikidata items Q782 and Q35657 through the statehood proposition.',
      'Every natural language syntax of the proposition is a syntax alias of the concept.',
    ],
  },
  'ontology.external-identifier': {
    definition: 'An identifier that an external vocabulary such as Wikidata assigns to a concept.',
    constraints: ['The link stores the identifier under the external-identifier: prefix and the vocabulary name.', 'Networks that use the former external-id: prefix still load.'],
    aliases: [{ source: 'Wikidata', name: 'external-id' }],
  },
  ...Object.fromEntries(
    ['emphasis', 'strong-emphasis', 'strikethrough', 'inline-code', 'hyperlink', 'image', 'line-break', 'heading', 'paragraph', 'block-quote', 'bullet-list', 'ordered-list', 'list-item', 'code-block', 'thematic-break', 'table', 'table-row', 'table-cell'].map((id) => [id, { constraints: DOCUMENT }]),
  ),
  'program.modules-and-imports': {
    definition: 'The modules a program defines and the modules it imports.',
    constraints: PROGRAM,
    aliases: [{ source: 'JavaScript', name: 'import' }, { source: 'Rust', name: 'use' }, { source: 'Lean', name: 'import' }, { source: 'Rocq', name: 'Require Import' }],
  },
  'program.scopes-and-bindings': {
    definition: 'The scopes of a program and the names bound in each scope.',
    constraints: PROGRAM,
    aliases: [{ source: 'JavaScript', name: 'let' }, { source: 'Rust', name: 'let' }, { source: 'Lean', name: 'let' }, { source: 'Rocq', name: 'let' }],
  },
  'program.recursive-definitions': {
    definition: 'The definitions of a program that refer to themselves.',
    constraints: PROGRAM,
    aliases: [{ source: 'Lean', name: 'partial def' }, { source: 'Rocq', name: 'Fixpoint' }],
  },
  'program.types-and-universes': {
    definition: 'The types a program declares and the universes those types live in.',
    constraints: PROGRAM,
    aliases: [{ source: 'Lean', name: 'Type u' }, { source: 'Rocq', name: 'Type' }, { source: 'Rust', name: 'struct' }],
  },
  'program.effects': {
    definition: 'The side effects a program performs, such as input, output and mutation.',
    constraints: PROGRAM,
    aliases: [{ source: 'Lean', name: 'IO' }, { source: 'JavaScript', name: 'await' }],
  },
  'program.attributes': {
    definition: 'The annotations attached to declarations of a program.',
    constraints: PROGRAM,
    aliases: [{ source: 'Rust', name: '#[attribute]' }, { source: 'Lean', name: '@[attribute]' }],
  },
  'program.macros-and-notation': {
    definition: 'The macros and custom notation a program defines or uses.',
    constraints: PROGRAM,
    aliases: [{ source: 'Rust', name: 'macro_rules!' }, { source: 'Lean', name: 'notation' }, { source: 'Rocq', name: 'Notation' }],
  },
  'program.proof-terms-and-tactics': {
    definition: 'The proofs of a program, written as terms or as tactic scripts.',
    constraints: PROGRAM,
    aliases: [{ source: 'Lean', name: 'by' }, { source: 'Rocq', name: 'Proof.' }],
  },
  'program.surface-expansion-elaboration-traces': {
    definition: 'The record of how surface syntax was expanded and elaborated into core terms.',
    constraints: [...PROGRAM, 'The construct is unavailable when no expansion or elaboration trace was produced.'],
  },
  'program.project-context-and-dependencies': {
    definition: 'The project a program belongs to and the packages it depends on.',
    constraints: PROGRAM,
    aliases: [{ source: 'JavaScript', name: 'package.json' }, { source: 'Rust', name: 'Cargo.toml' }, { source: 'Lean', name: 'lakefile.lean' }, { source: 'Rocq', name: '_CoqProject' }],
  },
  'operation.parse': { role: 'operation', definition: 'Parse source text into a links network.', constraints: OPERATION },
  'operation.query': { role: 'operation', definition: 'Query links in a network.', constraints: OPERATION },
  'operation.transform': { role: 'operation', definition: 'Transform query-selected links or source ranges.', constraints: OPERATION },
  'operation.substitute': { role: 'operation', definition: 'Apply structural substitutions.', constraints: OPERATION },
  'operation.serialize': { role: 'operation', definition: 'Serialize and load network data.', constraints: OPERATION },
  'operation.snapshot': { role: 'operation', definition: 'Capture immutable network versions.', constraints: OPERATION },
  'operation.translate': {
    role: 'operation',
    definition: 'Reconstruct through translation rules.',
    constraints: OPERATION,
    distinctFrom: [{ id: 'operation.transform', reason: 'Translating reconstructs a network in another language through translation rules; transforming rewrites selected links or source ranges within one network.' }],
  },
  'operation.verify': { role: 'operation', definition: 'Verify parse and structural diagnostics.', constraints: OPERATION },
  'translation.parse': {
    role: 'operation',
    represents: 'operation.parse',
    definition: 'Parse a corpus program into its links network before translation.',
    constraints: STAGE,
  },
  'translation.check': {
    role: 'operation',
    definition: 'Check the parsed links network of a corpus program before emitting it.',
    constraints: STAGE,
  },
  'translation.emit': {
    role: 'operation',
    definition: 'Emit a corpus program in each target language from its links network.',
    constraints: [...STAGE, 'Each target language has its own digest.'],
  },
};

const descriptions = extractSourceDescriptions('.');
const formers = extractFormerNames('.');
const concepts = Object.entries(HAND).map(([id, hand]) => {
  const described = descriptions.filter((entry) => entry.record === id);
  const definition = described.find((entry) => entry.definition !== undefined)?.definition ?? hand.definition;
  if (definition === undefined) throw new Error(`${id} has no definition`);
  const sourceAliases = [];
  for (const alias of [...described.flatMap((entry) => entry.sourceAliases), ...(hand.aliases ?? [])]) {
    if (!sourceAliases.some((known) => known.source === alias.source && known.name === alias.name)) sourceAliases.push(alias);
  }
  const record = {
    id,
    phrase: phraseOf(id),
    role: hand.role ?? 'concept',
    definition,
    constraints: hand.constraints,
    sourceAliases,
    formerNames: [...new Set(formers.filter((entry) => entry.record === id).map((entry) => entry.former))],
  };
  if (hand.represents) record.represents = hand.represents;
  if (hand.distinctFrom) record.distinctFrom = hand.distinctFrom;
  return record;
});

const register = {
  description:
    'The concept record of every canonical name meta-language defines: its stable identity, the readable English phrase the identity spells, its role (a concept is a noun phrase, an operation or relation a verb phrase), its definition, its constraints, the names other sources give it, and the former names that still decode to it. js/scripts/check-naming.mjs checks the records and that every name the sources define has one.',
  exemptions: [{ pattern: '^Q[0-9]+$', reason: 'Wikidata item identifiers are external identifiers, not English names.' }],
  concepts,
};
const text = `${JSON.stringify(register, null, 2)}\n`;
if (process.argv.includes('--write')) writeFileSync('parity/naming/canonical-concepts.json', text);
else process.stdout.write(text);
