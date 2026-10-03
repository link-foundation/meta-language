// Drafts the grammar feature union fixture (parity/fixtures/grammar-feature-union.json):
// every feature's listing, positive and negative inputs, and a grammar
// mutation, with the expectations the native executor produces. The
// `interchange` section of the written fixture is kept as it is. The output
// is reviewed by hand before it is written; run from the repository root:
//   node experiments/issue-195-grammar-feature-union-fixture.mjs [--write]
import { readFileSync, writeFileSync } from 'node:fs';
import { compileGrammar, parseNativeGrammar, renderSyntaxTree } from '../js/src/index.js';

const lines = (...items) => `${items.join('\n')}\n`;

const languages = {
  base: lines(
    'start value',
    'extra class(char(" "))',
    'rule value = normal choice(ref(number), ref(word))',
    'rule number = token repeat1(range("0", "9"))',
    'rule word = token repeat1(range("a", "z"))',
  ),
  arithmetic: lines(
    'start sum',
    'rule sum = normal seq(ref(digit), repeat0(seq(literal("+"), ref(digit))))',
    'rule digit = token range("0", "9")',
  ),
  cycleFirst: lines('import cycleSecond', 'rule first = normal literal("a")'),
  cycleSecond: lines('import cycleFirst', 'rule second = normal literal("b")'),
};

const features = [
  {
    id: 'alternatives',
    title: 'ordered and unordered alternatives',
    listing: lines(
      'start s',
      'rule s = normal choice(seq(literal("o:"), ref(ordered)), seq(literal("u:"), ref(unordered)))',
      'rule ordered = normal seq(orderedChoice(literal("a"), literal("ab")), literal("c"))',
      'rule unordered = normal seq(choice(literal("a"), literal("ab")), literal("c"))',
    ),
    positive: ['u:abc', 'u:ac', 'o:ac'],
    negative: [{ input: 'o:abc' }, { input: 'u:abd' }],
    mutation: {
      replace: ['orderedChoice(literal("a"), literal("ab"))', 'orderedChoice(literal("ab"), literal("a"))'],
      input: 'o:abc',
    },
  },
  {
    id: 'recursion',
    title: 'recursion, including left recursion',
    listing: lines(
      'start s',
      'rule s = normal choice(seq(literal("e:"), ref(expression)), seq(literal("c:"), ref(chain)))',
      'rule expression = normal choice(seq(ref(expression), literal("-"), ref(primary)), ref(primary))',
      'rule primary = normal choice(ref(digit), seq(literal("("), ref(expression), literal(")")))',
      'rule digit = token range("0", "9")',
      'rule chain = normal choice(seq(ref(tail), literal("x")), literal("y"))',
      'rule tail = normal ref(chain)',
    ),
    positive: ['e:1-2-3', 'e:(1-(2))', 'c:yxx'],
    negative: [{ input: 'e:1-' }, { input: 'e:(1' }, { input: 'c:xy' }, { input: 'e:((((((((1))))))))', options: { maxDepth: 12 } }],
    mutation: {
      replace: ['seq(ref(expression), literal("-"), ref(primary))', 'seq(ref(primary), literal("-"), ref(expression))'],
      input: 'e:1-2-3',
    },
  },
  {
    id: 'precedence',
    title: 'precedence and associativity',
    listing: lines(
      'start e',
      'rule e = normal choice(precedence(1, left, seq(ref(e), literal("+"), ref(e))), precedence(2, left, seq(ref(e), literal("*"), ref(e))), precedence(3, right, seq(ref(e), literal("^"), ref(e))), precedence(0, none, seq(ref(e), literal("="), ref(e))), precedence(4, left, seq(ref(e), literal("("), literal(")"))), precedence(5, left, seq(ref(e), literal("."), ref(n))), precedence(0, none, seq(literal("?"), ref(n), optional(ref(otherwise)))), ref(n))',
      'rule otherwise = normal precedence(0, none, seq(literal(":"), ref(n)))',
      'rule n = token repeat1(range("0", "9"))',
    ),
    options: { ambiguity: 'reject' },
    // A lower-precedence edge child conflicts only when its own child facing
    // the operator could be the operand: `1().2` stands, `(1+2).3` does not.
    // The child at the edge of an optional is one of its item: `:2` is an
    // `otherwise`, which `:` cannot be, so `?1:2` stands.
    positive: ['1+2*3', '1*2+3', '1+2+3', '2^3^4', '1+2=3', '1().2', '1+2.3', '?1:2'],
    negative: [{ input: '1=2=3' }, { input: '1+*2' }],
    mutation: {
      replace: ['precedence(1, left, seq(ref(e), literal("+"), ref(e)))', 'precedence(1, right, seq(ref(e), literal("+"), ref(e)))'],
      input: '1+2+3',
    },
  },
  {
    id: 'ambiguity',
    title: 'ambiguity and conflicts',
    listing: lines(
      'start s',
      'matching longest',
      'extra class(char("\\n"))',
      'conflict declared',
      'rule s = normal choice(seq(literal("d:"), ref(declared)), seq(literal("u:"), ref(undeclared)), seq(literal("r:"), ref(resolved)), seq(literal("l:"), ref(signed)), seq(literal("k:"), ref(key)), seq(literal("n:"), ref(lines)))',
      'rule declared = normal choice(seq(ref(declared), literal("-"), ref(declared)), ref(n))',
      'rule undeclared = normal choice(seq(ref(undeclared), literal("-"), ref(undeclared)), ref(n))',
      'rule resolved = normal choice(dynamicPrecedence(1, seq(ref(resolved), literal("-"), ref(n))), seq(ref(resolved), literal("-"), ref(resolved)), ref(n))',
      'rule n = token range("0", "9")',
      'rule signed = normal choice(seq(ref(minus), ref(n)), ref(number))',
      'rule minus = token literal("-")',
      'rule number = token seq(literal("-"), range("0", "9"))',
      'rule key = normal choice(literal("if"), ref(name))',
      'rule name = token repeat1(range("a", "z"))',
      'rule lines = normal repeat1(ref(line))',
      'rule line = normal seq(ref(n), optional(literal("\\n")))',
    ),
    options: { ambiguity: 'reject' },
    // Longest matching decides as a lexer does: `if` is the literal, more
    // specific than a name of one length, and a line break that ends a line
    // is the token, not the separator; a token that also takes the line
    // breaks after it is still the `\n` token.
    positive: ['d:1-2-3', 'u:1-2', 'r:1-2-3', 'l:-1', 'k:if', 'k:iffy', 'n:1\n2', 'n:1\n\n2'],
    negative: [{ input: 'u:1-2-3' }, { input: 'd:1--2' }],
    mutation: { replace: ['conflict declared', 'conflict undeclared'], input: 'u:1-2-3' },
    // Without longest matching, "-" "1" and "-1" are two parses alike in cost
    // and dynamic precedence, as are the literal and the name `if`, and the
    // line break as a token and as a separator: ambiguities.
    mutations: [
      { replace: ['matching longest\n', ''], input: 'l:-1' },
      { replace: ['matching longest\n', ''], input: 'k:if' },
      { replace: ['matching longest\n', ''], input: 'n:1\n2' },
    ],
  },
  {
    id: 'lexical',
    title: 'lexical priority and longest-match rules',
    listing: lines(
      'start s',
      'extra class(char(" "))',
      'rule s = normal repeat1(ref(word))',
      'rule word = silent longest(ref(keyword), ref(identifier), ref(number))',
      'rule keyword = token lexicalPrecedence(1, choice(literal("if"), literal("else")))',
      'rule identifier = token seq(range("a", "z"), repeat0(class(range("a", "z"), range("0", "9"))))',
      'rule number = token repeat1(range("0", "9"))',
    ),
    positive: ['if iffy else', '42 x9'],
    negative: [{ input: 'IF' }, { input: 'if +' }],
    mutation: { replace: ['lexicalPrecedence(1,', 'lexicalPrecedence(-1,'], input: 'if iffy else' },
  },
  {
    id: 'unicode',
    title: 'Unicode and byte classes',
    listing: lines(
      'start s',
      'rule s = normal seq(ref(name), literal(":"), ref(payload))',
      'rule name = token repeat1(class(category("Lu"), category("Ll"), script("Han")))',
      'rule payload = token repeat1(byteClass(byteRange(128, 255)))',
    ),
    positive: [{ input: 'Ωmé漢:', hex: '80ff' }, { input: 'a:', hex: 'c3' }],
    negative: [{ input: 'a1:', hex: '80' }, { input: 'a:a' }, { input: '', hex: 'ff3a80' }],
    mutation: { replace: ['byteRange(128, 255)', 'byteRange(0, 127)'], input: 'a:a' },
  },
  {
    id: 'trivia',
    title: 'token boundaries and trivia',
    listing: lines(
      'start s',
      'extra ref(space)',
      'extra ref(comment)',
      'rule s = normal repeat1(ref(call))',
      'rule call = normal seq(ref(name), immediateToken(literal("(")), token(seq(literal("<"), ref(name), literal(">"))), literal(")"))',
      'rule name = token repeat1(range("a", "z"))',
      'rule space = token repeat1(class(char(" "), char("\\n")))',
      'rule comment = token seq(literal("#"), repeat0(notClass(char("\\n"))))',
    ),
    positive: ['f( <x> ) # done\ng(<y>)', 'ab(<c>)'],
    negative: [{ input: 'f (<x>)' }, { input: 'f(< x>)' }, { input: 'f o(<x>)' }],
    mutation: { replace: ['immediateToken(literal("("))', 'token(literal("("))'], input: 'f (<x>)' },
  },
  {
    id: 'modes',
    title: 'lexer modes, channels and state',
    listing: lines(
      'start s',
      'mode string',
      'rule s = normal repeat0(choice(ref(word), ref(quoted)))',
      'rule quoted = normal seq(ref(open), repeat0(ref(chars)), ref(close))',
      'rule open = token literal("\\"") action(pushMode(string))',
      'rule close = token literal("\\"") modes(string) action(popMode)',
      'rule chars = token repeat1(notClass(char("\\""))) modes(string)',
      'rule word = token repeat1(range("a", "z")) modes(default)',
      'rule space = token repeat1(literal(" ")) channel(hidden) modes(default)',
    ),
    positive: ['ab "c d" e', '""'],
    negative: [{ input: 'a "b' }, { input: 'a " b' }, { input: '"a"b"' }],
    mutation: { replace: ['rule space = token repeat1(literal(" ")) channel(hidden) modes(default)', 'rule space = token repeat1(literal(" ")) channel(hidden)'], input: '" b"' },
  },
  {
    id: 'layout',
    title: 'indentation and layout',
    listing: lines(
      'start block',
      'scanner layout tokens(newline, indent, dedent) operations('
        + 'if(valid(newline), then(consume(literal("\\n")), while(next(literal(" ")), do(advance)), set(pending, column), emit(newline))), '
        + 'if(valid(indent), then(if(greater(variable(pending), top(indents)), then(push(indents, variable(pending)), emit(indent))))), '
        + 'if(valid(dedent), then(if(less(variable(pending), top(indents)), then(pop(indents), emit(dedent))))), '
        + 'fail)',
      'rule block = normal repeat1(ref(statement))',
      'rule statement = normal predicate(choice(seq(ref(name), literal(":"), ref(newline), ref(indent), ref(block), ref(dedent)), seq(ref(name), ref(newline))), equal(column, top(indents)))',
      'rule name = token repeat1(range("a", "z"))',
    ),
    positive: ['a\nb:\n  c\n  d\ne\n', 'a:\n  b:\n    c\nd\n', 'a:\n  b:\n    c\n'],
    negative: [{ input: 'a:\n  b\n c\n' }, { input: 'a\n  b\n' }, { input: 'a:\nb\n' }],
    mutation: { replace: ['equal(column, top(indents))', 'not(less(column, integer(0)))'], input: 'a:\n  b\n c\n' },
  },
  {
    id: 'predicates',
    title: 'context-sensitive predicates',
    listing: lines(
      'start heredoc',
      'rule heredoc = normal seq(ref(open), literal("\\n"), repeat0(ref(line)), ref(close))',
      'rule open = normal seq(literal("<<"), capture("label", ref(label))) action(set(delimiter, fieldText(label)))',
      'rule label = token repeat1(range("A", "Z"))',
      'rule close = normal predicate(ref(label), equal(matched, variable(delimiter)))',
      'rule line = token seq(not(seq(ref(close), literal("\\n"))), not(seq(ref(close), not(any))), repeat0(notClass(char("\\n"))), literal("\\n"))',
    ),
    positive: ['<<END\nhi\nEND', '<<EOF\nEND\nEOF', '<<A\nA'],
    negative: [{ input: '<<END\nhi\nEOF' }, { input: '<<END\nhi' }],
    mutation: { replace: ['equal(matched, variable(delimiter))', 'not(equal(matched, variable(delimiter)))'], input: '<<END\nhi\nEOF' },
  },
  {
    id: 'actions',
    title: 'semantic actions and attributes',
    listing: lines(
      'start sum',
      'extra class(char(" "))',
      'rule sum = normal seq(capture("terms", ref(number)), repeat0(seq(literal("+"), capture("terms", ref(number))))) action(setAttribute(value, sumOf(terms, value)), if(greater(sumOf(terms, value), integer(100)), then(fail)))',
      'rule number = token repeat1(range("0", "9")) action(setAttribute(value, number(matched)), if(equal(number(matched), integer(0)), then(buildNode(zero))))',
    ),
    positive: ['1 + 2 + 3', '0 + 40', '007'],
    negative: [{ input: '60 + 50' }, { input: '1 +' }],
    mutation: { replace: ['integer(100)', 'integer(1000)'], input: '60 + 50' },
  },
  {
    id: 'fields',
    title: 'captures, fields and aliases',
    listing: lines(
      'start s',
      'extra class(char(" "))',
      'rule s = normal seq(capture("target", ref(name)), literal("="), capture("value", choice(alias(variable, ref(name)), alias(pair, seq(ref(name), literal(","), ref(name))))))',
      'rule name = token repeat1(range("a", "z"))',
    ),
    positive: ['x = y', 'x=a,b'],
    negative: [{ input: 'x =' }, { input: '= y' }],
    mutation: { replace: ['alias(variable, ref(name))', 'alias(reference, ref(name))'], input: 'x = y' },
  },
  {
    id: 'parameterization',
    title: 'parameterization',
    listing: lines(
      'start s',
      'rule s = normal seq(ref(list, ref(digit), literal(",")), literal(";"), ref(list, ref(letter), literal("|")))',
      'rule list(item, separator) = normal seq(parameter(item), repeat0(seq(parameter(separator), parameter(item))))',
      'rule digit = token range("0", "9")',
      'rule letter = token range("a", "z")',
    ),
    positive: ['1,2;a|b|c', '7;z'],
    negative: [
      { input: '1|2;a' },
      { input: 'a;1' },
      { listing: lines('start s', 'rule s = normal ref(list, ref(digit))', 'rule list(item, separator) = normal seq(parameter(item), parameter(separator))', 'rule digit = token range("0", "9")') },
      { listing: lines('start s', 'rule s = normal parameter(item)') },
    ],
    mutation: { replace: ['literal("|")))', 'literal(",")))'], input: '1,2;a,b' },
  },
  {
    id: 'imports',
    title: 'imports and inheritance',
    listing: lines(
      'start list',
      'import base',
      'rule list = normal seq(literal("["), ref(value), repeat0(seq(literal(","), ref(value))), literal("]"))',
      'rule word = token repeat1(range("A", "Z"))',
    ),
    positive: ['[1, ABC]', '[ 42 ]'],
    negative: [
      { input: '[abc]' },
      { input: '[1,]' },
      { listing: lines('start s', 'import missingGrammar', 'rule s = normal literal("a")') },
      { listing: lines('start s', 'import cycleFirst', 'rule s = normal literal("a")') },
    ],
    mutation: { replace: ['rule word = token repeat1(range("A", "Z"))\n', ''], input: '[abc]' },
  },
  {
    id: 'macros',
    title: 'macros and notation',
    listing: lines(
      'start s',
      'macro commaList(item) = seq(parameter(item), repeat0(seq(literal(","), parameter(item))))',
      'macro parenthesized(inner) = seq(literal("("), parameter(inner), literal(")"))',
      'rule s = normal expand(parenthesized, expand(commaList, ref(digit)))',
      'rule digit = token range("0", "9")',
    ),
    positive: ['(1,2,3)', '(4)'],
    negative: [
      { input: '(1,)' },
      { input: '1,2' },
      { listing: lines('start s', 'macro loop(x) = expand(loop, parameter(x))', 'rule s = normal expand(loop, literal("a"))') },
      { listing: lines('start s', 'rule s = normal expand(undefinedMacro)') },
    ],
    mutation: { replace: ['repeat0(seq(literal(","), parameter(item)))', 'repeat0(seq(literal(";"), parameter(item)))'], input: '(1;2)' },
  },
  {
    id: 'embedded',
    title: 'embedded languages',
    listing: lines(
      'start template',
      'rule template = normal repeat0(choice(ref(text), ref(interpolation)))',
      'rule text = token repeat1(notClass(char("{")))',
      'rule interpolation = normal seq(literal("{"), embed(arithmetic, repeat0(notClass(char("}")))), literal("}"))',
    ),
    positive: ['a{1+2}b', '{3}'],
    negative: [
      { input: 'a{1+}b' },
      { input: 'a{1' },
      { listing: lines('start s', 'rule s = normal embed(unknownLanguage, any)') },
    ],
    mutation: { replace: ['embed(arithmetic, repeat0(notClass(char("}"))))', 'repeat0(notClass(char("}")))'], input: 'a{1+}b' },
  },
  {
    id: 'recovery',
    title: 'error and missing nodes, and recovery',
    listing: lines(
      'start program',
      'extra class(char(" "), char("\\n"))',
      'rule program = normal repeat0(ref(statement))',
      'rule statement = normal seq(recover(seq(ref(name), literal("="), ref(number)), literal(";")), missing(literal(";")))',
      'rule name = token repeat1(range("a", "z"))',
      'rule number = token repeat1(range("0", "9"))',
    ),
    options: { recovery: 'accept' },
    positive: ['x = 1; y = 2;', 'x = ; y = 2;', 'x = 1 y = 2;'],
    negative: [{ input: 'x = ; y = 2;', options: { recovery: 'reject' } }, { input: 'x = 1 y = 2;', options: { recovery: 'reject' } }, { input: 'x = 1; y' }],
    mutation: { replace: ['missing(literal(";"))', 'literal(";")'], input: 'x = 1 y = 2;' },
  },
];

const resolveGrammar = (name) => (languages[name] ? parseNativeGrammar(languages[name]) : undefined);

function inputOf(item) {
  const text = typeof item === 'string' ? item : item.input;
  const hex = typeof item === 'string' ? undefined : item.hex;
  if (!hex) return text;
  const head = new TextEncoder().encode(text);
  const tail = Uint8Array.from(hex.match(/../gu).map((pair) => Number.parseInt(pair, 16)));
  const bytes = new Uint8Array(head.length + tail.length);
  bytes.set(head);
  bytes.set(tail, head.length);
  return bytes;
}

function outcome(listing, item, options) {
  let parser;
  try {
    parser = compileGrammar(parseNativeGrammar(listing), { resolveGrammar, ...options });
  } catch (error) {
    return { load: { reason: error.reason, message: error.message } };
  }
  const result = parser.parseTree(inputOf(item), item.options ?? {});
  const summary = {};
  if (result.tree) summary.tree = renderSyntaxTree(result.tree);
  if (result.ambiguities.length > 0) summary.ambiguities = result.ambiguities;
  if (result.rejection) summary.rejection = result.rejection;
  return summary;
}

function caseInput(item) {
  const input = inputOf(item);
  if (typeof input === 'string') return { input };
  return { inputHex: [...input].map((byte) => byte.toString(16).padStart(2, '0')).join('') };
}

function expectation(listing, item, options) {
  const result = outcome(listing, item, options);
  if (result.load) return { loadError: { reason: result.load.reason } };
  return result;
}

const fixture = {
  description: 'The grammar feature union of docs/vision.md#grammar-feature-union as executable cases: per feature a native grammar listing, positive inputs with the expected concrete syntax tree in the notation of docs/grammar/feature-union.md#tree-notation, negative inputs with the expected rejection, and a grammar mutation that changes the outcome (more in mutations). Grammars named by imports and embedded languages are in languages. Written by experiments/issue-195-grammar-feature-union-fixture.mjs and reviewed by hand.',
  requirement: 'I195-GRAMMAR-FEATURE-UNION',
  languages,
  features: [],
};
for (const feature of features) {
  const options = feature.options ?? {};
  const entry = { id: feature.id, title: feature.title, listing: feature.listing };
  if (feature.options) entry.options = feature.options;
  entry.positive = feature.positive.map((item) => ({
    ...caseInput(item),
    ...(item.options ? { options: item.options } : {}),
    ...expectation(feature.listing, item, options),
  }));
  entry.negative = feature.negative.map((item) => {
    if (item.listing) return { listing: item.listing, ...expectation(item.listing, '', options) };
    return { ...caseInput(item), ...(item.options ? { options: item.options } : {}), ...expectation(feature.listing, item, options) };
  });
  const mutation = ({ replace: [from, to], input }) => ({
    from,
    to,
    ...caseInput(input),
    before: expectation(feature.listing, input, options),
    after: expectation(feature.listing.replace(from, to), input, options),
  });
  entry.mutation = mutation(feature.mutation);
  if (feature.mutations) entry.mutations = feature.mutations.map(mutation);
  fixture.features.push(entry);
}
const fixtureUrl = new URL('../parity/fixtures/grammar-feature-union.json', import.meta.url);
fixture.interchange = JSON.parse(readFileSync(fixtureUrl, 'utf8')).interchange;
const text = `${JSON.stringify(fixture, null, 2)}\n`;
if (process.argv.includes('--write')) writeFileSync(fixtureUrl, text);
else process.stdout.write(text);
