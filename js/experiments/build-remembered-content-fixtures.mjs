// Own parameterized scanner examples, with trees shared by both executors.
import assert from 'node:assert/strict';
import { writeFileSync } from 'node:fs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../src/index.js';
const cases = [
  { name: 'whole tagged literals', assertion: 'rememberedLiteralText', scanners: [{ family: 'remembered-literal', name: 'tags', token: 'literal', startToken: 'opening', contentToken: 'body', endToken: 'closing', tagPattern: '\\$[^$\\s]*\\$' }], root: '(repeat1 (ref literal))', accept: ['$$$$', '$tag$é😀$tag$', '$a$x$b$y$a$', '$a$x$a$$b$y$b$'], reject: ['$a$x$b$', '$a$x$a', '$ a$x$ a$'] },
  { name: 'grammar surrounded remembered content', assertion: 'rememberedContentText', scanners: [{ family: 'remembered-content', name: 'tags', delimiterToken: 'marker', contentToken: 'body', delimiterPattern: '[a-z]{1,4}', closingPrefix: ')', closingSuffix: '"', allowEmpty: true }], root: '(repeat1 (seq (literal R%22) (optional (ref marker)) (literal %28) (ref body) (literal %29) (optional (ref marker)) (literal %22)))', accept: ['R"()"', 'R"tag(é😀)tag"', 'R"a(x)b"y)a"', 'R"a(x)a"R"b(y)b"'], reject: ['R"a(x)b"', 'R"abcde(x)abcde"', 'R"a(x)a', 'R"a(x'] },
];
for (const fixture of cases) {
 fixture.generated=scannerFamilies(fixture.scanners);
 fixture.listing=`(grammar (start document))\n${fixture.generated}(rule document normal ${fixture.root})\n`;
 delete fixture.root;
 const parser=compileGrammar(parseGrammarLinks(fixture.listing));
 fixture.trees={};
 for(const input of fixture.accept){const result=parser.parseTree(input);assert.equal(result.ok,true,input);fixture.trees[input]=renderSyntaxTree(result.tree);}
 for(const input of fixture.reject)assert.equal(parser.parseTree(input).ok,false,input);
}
writeFileSync(new URL('../../parity/fixtures/scanner-remembered-content.json',import.meta.url),JSON.stringify(cases,null,2)+'\n');
console.log('generated two parameterized remembered content fixtures');
