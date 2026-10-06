// Tries candidate sources on the native GraphQL grammar and its
// tree-sitter-graphql oracle: SAME, DIFF, REJECT, or ORACLE-ERROR with the
// native outcome, to pick the hand-written matches and rejections of the
// fixture.
//   node experiments/native-graphql-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/graphql.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comma', 'comment'], oracleKinds: nativeOracleKinds(text) };
const candidates = [
  '{ a }', 'query { a }', 'query Q { a b c }', 'mutation M($id: ID!) { like(id: $id) { count } }', 'subscription S { events { id } }',
  'query Q($a: Int = 1, $b: [String!]! = ["x"]) @d { a }', '{ alias: field(arg: 1) }', '{ f(o: {a: 1, b: 2}, x: [1, 2]) }',
  '{ f(a: 1.5e3, b: -2, c: true, d: false, e: null, f: ENUM, g: "s", h: """block""") }', '{ ...F ... on T { a } ... @include(if: $x) { b } }',
  'fragment F on User { id name }', 'fragment F on User @d { id }', '# comment\n{ a }', '{ a # trailing\n b }', '{ a, b, c }', ',,{ a },,',
  'schema { query: Query mutation: Mutation }', 'schema @d { query: Q }', 'extend schema @d', 'extend schema { subscription: S }',
  'scalar Date', 'scalar Date @specifiedBy(url: "https://x")', 'type T { a: Int }', 'type T implements A & B { f(a: Int = 1, b: [String!]!): T @d(x: 1) }',
  'type T implements & A { a: Int }', '"desc" type T { "field" a: Int }', '"""block\ndesc""" type T', 'interface I { a: Int }', 'interface I implements J { a: Int }',
  'union U = A | B', 'union U = | A | B', 'union U @d', 'enum E { A B }', 'enum E { A, B }', 'enum E @d { A @deprecated }', 'input I { a: Int = 1, b: String }',
  'directive @d(a: Int) on FIELD | QUERY', 'directive @d repeatable on | FIELD_DEFINITION', 'extend type T { b: Int }', 'extend type T implements I',
  'extend interface I @d', 'extend union U = C', 'extend enum E { C }', 'extend input I { c: Int }', 'extend scalar S @d',
  'type Query {\n  user(id: ID!): User\n  users(first: Int = 10, after: String): [User!]!\n}\n\nquery Q {\n  user(id: "é") { name }\n}\n',
  '{ a(s: "\\u00e9\\n\\"") }', '{ a(f: 0.5, g: 1E10, h: -0) }', '{ a(l: [], o: {}) }', 'query ($v: Int) { a(v: $v) }',
  '', '{', '}', '{ a', 'query', 'query Q', 'type', 'type T {', 'type T { a }', 'type T { a: }', '{ a(: 1) }', '{ a(b: ) }', 'union U =', 'enum E { A',
  'fragment F { a }', 'fragment on T { a }', '{ ...on }', 'directive @d', 'directive d on FIELD', 'schema { query }', '{ a(b: [1, 2) }', '{ a(b: {c: 1) }',
  'query Q($a) { a }', 'query Q($a: ) { a }', 'scalar', 'extend', '{ a } }', '"desc"', '{ a(s: "unterminated) }', '$a', '@d',
];
for (const source of candidates) {
  const outcome = compiled.parseTree(source);
  let kind;
  if (oracleRecovers(source, 'GraphQL')) kind = `ORACLE-ERROR native ${outcome.ok ? 'ACCEPTS' : 'rejects'}`;
  else if (!outcome.ok) kind = 'REJECT';
  else kind = (JSON.stringify(oracleRows(source, 'GraphQL')) === JSON.stringify(nativeRows(outcome.tree, source, options)) ? 'SAME' : 'DIFF') + (outcome.ambiguities?.length ? '-AMBIGUOUS' : '');
  console.log(`${kind} ${JSON.stringify(source)}`);
}
