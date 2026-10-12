// Focused operational Yacc/Bison+Lex import checks. Official source programs
// with unlowered C/OCaml actions remain explicit failures in the bulk report.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { gunzipSync } from 'node:zlib';
import { test } from 'node:test';
import { compileGrammar, GrammarBuilder } from '../src/grammar.js';
import { parseGrammarLinks, renderGrammarLinks } from '../src/grammar-links.js';
import { importYacc, importLex, lowerYaccAction } from '../src/grammar-importers/yacc.js';
import { tokenizeYacc } from '../src/grammar-importers/yacc-lexer.js';

const scanner = () => importLex(`%option noyywrap nodefault
DIGIT [0-9]
%%
{DIGIT}+ { yylval = atoi(yytext); return NUMBER; }
[ \\t\\n]+ ;
%%
`);
const lexicalOptions = () => {
  const grammar = scanner();
  return { tokenRules: [...grammar.rules.values()].filter(({ kind }) => kind === 'token'), extras: grammar.declarations.extras };
};
const arithmetic = `%token NUMBER
%left "+" "-"
%left "*"
%right UMINUS
%start expression
%%
expression : expression "+" expression { $$ = $1 + $3; }
 | expression "-" expression { $$ = $1 - $3; }
 | expression "*" expression { $$ = $1 * $3; }
 | "-" expression %prec UMINUS { $$ = -$2; }
 | "(" expression ")" { $$ = $2; }
 | NUMBER { $$ = $1; };
%%
`;

test('Yacc precedence, associativity and actions execute through serialized native links', () => {
  const grammar = importYacc(arithmetic, lexicalOptions());
  const links = renderGrammarLinks(grammar);
  assert.match(links, /\(precedence 3 right/u);
  assert.match(links, /\(setAttribute value \(add /u);
  assert.doesNotMatch(links, /\$\$|atoi|yylval/u);
  const restored = parseGrammarLinks(links);
  assert.deepEqual(restored.normalized(), grammar.normalized());
  for (const imported of [grammar, restored]) {
    const parser = compileGrammar(imported);
    for (const [source, result] of [['2 + 3 * 4', 14], ['8-3-1', 4], ['(2+3)*4', 20], ['-2*3+8', 2]]) {
      const outcome = parser.parseTree(source);
      assert.equal(outcome.ok, true, source);
      assert.equal(outcome.tree.attributes.value, result, source);
      assert.deepEqual(outcome.ambiguities, [], source);
    }
    for (const source of ['2 +', '2 ** 3', 'x+1']) assert.equal(parser.parseTree(source).ok, false, source);
  }
});

test('right associative Yacc reduction differs from the corresponding left declaration', () => {
  const source = '%token NUMBER\n%right "-"\n%%\nexpression : expression "-" expression { $$ = $1 - $3; } | NUMBER { $$ = $1; };\n';
  assert.equal(compileGrammar(importYacc(source, lexicalOptions())).parseTree('8-3-1').tree.attributes.value, 6);
  assert.equal(compileGrammar(importYacc(source.replace('%right', '%left'), lexicalOptions())).parseTree('8-3-1').tree.attributes.value, 4);
});

test('Yacc derives default precedence from the last terminal even when it has no declared precedence', () => {
  const grammar = importYacc('%token NUMBER\n%left "+"\n%%\nexpression : expression "+" NUMBER;\n', lexicalOptions());
  assert.equal(grammar.rule('expression').expression.kind, 'seq');
});

test('Yacc empty productions, numeric aliases and quoted Unicode literals keep their syntax', () => {
  const grammar = importYacc('%token HEART 300 "♥"\n%%\nmessage : %empty | "♥" "\\n";\n', {
    tokenRules: [{ name: 'HEART', kind: 'token', expression: GrammarBuilder.literal('♥') }],
  });
  const parser = compileGrammar(grammar);
  assert.equal(parser.parseTree('').ok, true);
  assert.equal(parser.parseTree('♥\n').ok, true);
  assert.equal(parser.parseTree('♥').ok, false);
  assert.equal(parseGrammarLinks(renderGrammarLinks(grammar)).rule('HEART').expression.value, '♥');
});

test('Yacc action tokenization preserves quoted comment markers and balanced blocks', () => {
  assert.deepEqual(lowerYaccAction('$$ = "/* semantic text */";', 0), [{ operation: 'setAttribute', attribute: 'value', value: { operation: 'text', value: '/* semantic text */' } }]);
  const tokens = tokenizeYacc('rule : "}" { if (x) { f("{"); } /* } */ } ;');
  assert.equal(tokens.filter(({ kind }) => kind === 'action').length, 1);
  assert.match(tokens.find(({ kind }) => kind === 'action').value, /f\("\{"\)/u);
  assert.throws(() => tokenizeYacc('rule : "a" { nested { };'), /unterminated action block/u);
});

test('foreign actions, unsupported directives and unknown scanner behavior fail rather than disappear', () => {
  for (const source of [
    '%%\nrule: "a" { $$ = build_ast($1); };',
    '%%\nrule: "a" { $$ = 1; } "b";',
    '%nonassoc "<"\n%%\nrule: "a";',
    '%union { int value; }\n%%\nrule: "a";',
    '%{ #include "host.h" %}\n%%\nrule: "a";',
    '%%\nrule: "a";\n%%\nint scanner(void) { return 0; }',
  ]) assert.throws(() => importYacc(source));
  assert.throws(() => compileGrammar(importYacc('%token WORD\n%%\nrule: WORD;')), /undefined/u);
  assert.throws(() => lowerYaccAction('$$ = $2;', 1), /semantic action/u);
  const rejecting = compileGrammar(importYacc('%%\nrule: "a" { YYERROR; };'));
  assert.equal(rejecting.parseTree('a').ok, false);
});

test('Lex longest matching, trivia and token values preserve complete input bytes', () => {
  const grammar = importLex('%option noyywrap nodefault\n%%\n"if" return IF;\n[a-z]+ { yylval = yytext; return IDENTIFIER; }\n[ \\t]+ ;\n%%\n');
  const outcome = compileGrammar(parseGrammarLinks(renderGrammarLinks(grammar))).parseTree('if ifx');
  assert.equal(outcome.ok, true);
  assert.deepEqual(outcome.tree.children.filter(({ trivia }) => !trivia).map(({ kind }) => kind), ['IF', 'IDENTIFIER']);
  assert.equal(outcome.tree.children.find(({ kind }) => kind === 'IDENTIFIER').attributes.value, 'ifx');
  assert.equal(outcome.tree.children.map(({ text }) => text).join(''), 'if ifx');
  assert.equal(compileGrammar(grammar).parseTree('7').ok, false);
});

test('Lex refuses unsupported mode behavior, trailing context and recursive definitions', () => {
  for (const [source, failure] of [
    ['%%\na/b return WORD;\n', /trailing context/u],
    ['%option caseless\n%%\na return WORD;\n', /lexer option caseless/u],
    ['FIRST {SECOND}\nSECOND {FIRST}\n%%\n{FIRST} return WORD;\n', /recursive lexical definition/u],
    ['%%\na { return WORD; yylval = yytext; }\n', /statements after a token return/u],
    ['%%\na { custom_scanner(); return WORD; }\n', /scanner action/u],
    ['%%\na return 300;\n', /numeric token 300/u],
    ['%x STRING\n%%\na { BEGIN(STRING); }\n', /mode-changing skipped rules/u],
    ['%%\na return WORD;\nb return OTHER;\nc return WORD;\n', /global order/u],
  ]) assert.throws(() => importLex(`%option nodefault\n${source}`), failure);
  assert.throws(() => importLex('%%\na return WORD;\n'), /default ECHO action/u);
  assert.equal(importLex('%option nodefault\n%%\na return 300;\n', { tokenNamesByCode: { 300: 'WORD' } }).rule('WORD').kind, 'token');
});

test('a Yacc display alias names the supplied token and never invents a lexer', () => {
  const source = '%token LABEL "word"\n%%\nroot : "word";\n';
  assert.throws(() => compileGrammar(importYacc(source)), /undefined/u);
  const lexer = importLex('%option nodefault\n%%\n[a-z]+ return LABEL;\n');
  const grammar = importYacc(source, { tokenRules: [...lexer.rules.values()].filter(({ kind }) => kind === 'token') });
  assert.equal(compileGrammar(grammar).parseTree('hello').ok, true);
  assert.equal(compileGrammar(grammar).parseTree('7').ok, false);
  const restored = parseGrammarLinks(renderGrammarLinks(grammar));
  assert.deepEqual(restored.rule('LABEL').sourceNames.find(({ source }) => source === 'Yacc token alias'), { source: 'Yacc token alias', name: 'word' });
});

test('Lex exclusive modes change through returned tokens and survive native Links serialization', () => {
  const source = '%option nodefault\n%x STRING\n%%\n"\\\"" { BEGIN(STRING); return OPEN; }\n<STRING>[a-z]+ { yylval = yytext; return TEXT; }\n<STRING>"\\\"" { BEGIN(INITIAL); return CLOSE; }\n%%\n';
  const parser = compileGrammar(parseGrammarLinks(renderGrammarLinks(importLex(source))));
  const result = parser.parseTree('"abc"');
  assert.equal(result.ok, true);
  assert.deepEqual(result.tree.children.map(({ kind }) => kind), ['OPEN', 'TEXT', 'CLOSE']);
  assert.equal(result.tree.children[1].attributes.value, 'abc');
  assert.equal(parser.parseTree('abc').ok, false);
  assert.throws(() => importLex('%option nodefault\n%%\na { BEGIN(UNKNOWN); return WORD; }\n'), /unknown lexer mode/u);
});

test('the pinned official Yacc sources retain actionable unlowered host-code failures', () => {
  for (const file of ['ruby-4.0.7-parse.y.gz', 'perl-5.44.0-perly.y.gz', 'bash-5.3-parse.y.gz']) {
    const source = gunzipSync(readFileSync(new URL(`../../parity/grammars/sources/${file}`, import.meta.url))).toString('utf8');
    assert.throws(() => importYacc(source), /Yacc import/u, file);
  }
});

test('official source pins retain verified grammar, license and independent corpus provenance', () => {
  const manifest = JSON.parse(readFileSync(new URL('../../parity/grammar-additional-sources.json', import.meta.url)));
  for (const language of ['Ruby', 'Perl', 'Bash', 'OCaml', 'OCaml Interface']) {
    const [source] = manifest.languages[language];
    assert.match(source.revision, /^[a-f0-9]{40}$/u, language);
    assert.ok(source.files.some(({ role }) => role === 'grammar'), language);
    assert.ok(source.files.some(({ role }) => role === 'license'), language);
    assert.ok(source.corpora.some((corpus) => corpus.revision === source.revision && corpus.role === 'independent upstream tests'), language);
    for (const entry of source.files) {
      const bytes = gunzipSync(readFileSync(new URL(`../../${entry.file}`, import.meta.url)));
      assert.equal(bytes.length, entry.bytes, entry.file);
      assert.equal(createHash('sha256').update(bytes).digest('hex'), entry.sha256, entry.file);
      assert.ok(entry.url.includes(source.revision), entry.file);
    }
  }
  const menhir = manifest.languages.OCaml[0];
  assert.throws(() => importYacc('%%\nstart: "x";', menhir.options), /Menhir parameterized productions and OCaml actions/u);
});
