// Sorts candidate sources for the native Make fixture: a match is a source
// the tree-sitter-make oracle reads and the native grammar parses to the
// same rows without ambiguity, a rejection one the oracle recovers from and
// the native grammar rejects; any other source is printed as unusable.
//   node experiments/native-make-candidates.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/make.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token', 'raw_line'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
const matches = [
  '', '\n', 'a = b\n', 'a := b\n', 'a ::= b\n', 'a ?= b\n', 'a += b\n', 'a != echo hi\n', 'a =\n', 'a = $(b) $(c)\n', 'a = ${b}\n', 'a = $(b:.c=.o)\n',
  'a = $(subst a,b,c)\n', 'a = $(shell ls)\n', 'a = $(wildcard *.c)\n', 'a = $(patsubst %.c,%.o,$(SRC))\n', 'a = $(foreach x,$(L),$(x).o)\n',
  'a = $(if $(b),c,d)\n', 'a = $(call f,1,2)\n', 'a = $(eval $(b))\n', 'a = $(origin b)\n', 'a = $(info é)\n', 'a = $(error x)\n', 'a = $$HOME\n',
  'all:\n', 'all: a b\n', 'all: a | b\n', 'a b &: c\n', '%.o: %.c\n\t$(CC) -c $< -o $@\n', 'all: ; echo hi\n', 'all:\n\techo a\n\techo b\n',
  'all:\n\t@echo a\n', 'all:\n\t-rm x\n', 'all:\n\t+make -C d\n', 'all:\n\techo a \\\n\tb\n', '.PHONY: all clean\n', 'objs: a.o b.o\n',
  '$(OBJ): %.o: %.c\n\tcc $<\n', 'a: b\n\n\n\tc\n', 'include a.mk b.mk\n', '-include a.mk\n', 'sinclude a.mk\n', 'vpath %.c src\n', 'vpath\n',
  'export a = b\n', 'export\n', 'unexport a\n', 'override a = b\n', 'private a = b\n', 'undefine a\n', 'define a\nb\nendef\n', 'define a =\n  b\n  c\nendef\n',
  'ifeq ($(a),b)\nc = d\nendif\n', 'ifneq "a" "b"\nc = d\nelse\nc = e\nendif\n', 'ifdef a\nb = c\nelse ifndef d\ne = f\nendif\n', 'ifndef a\nendif\n',
  '# comment\n', 'a = b # c\n', 'all: # c\n\techo\n', 'a = b \\\n  c\n', 'a: b\n\t$(MAKE) $@\n', 'a: b\n\techo $(@D) $(<F) $^ $+ $? $* $%\n',
  'VPATH = a:b\n', '.RECIPEPREFIX = >\n', 'a.o: CFLAGS += -O2\n', 'a = é\n', 'ifeq a b\nendif\n', 'a: \nb: c\n',
];
const rejections = [
  'ifeq (a,b\n', 'ifeq (a,b)\n', 'define a\nb\n', 'a = $(b\n', 'a: $(\n', 'endif\n', 'else\n', 'endef\n', 'a = ${b\n', 'ifdef\n', '\techo\n', 'export a = $(\n',
  'a: b\n\techo $(\n', 'vpath %.c $(\n', 'include $(\n', ':\n', '$(a\n', 'override\n',
  'a: private b = c\n', 'a: export b = c\n', '# café\nall: é\n', 'a:\nb\n', 'all:\necho\n', 'a: ;\n>b\n',
];
for (const [label, sources] of [['match', matches], ['rejection', rejections]]) {
  for (const source of sources) {
    const recovers = oracleRecovers(source, 'Make');
    const outcome = compiled.parseTree(source);
    let verdict;
    if (label === 'rejection') verdict = recovers && !outcome.ok ? 'ok' : `UNUSABLE (oracle recovers ${recovers}, native ${outcome.ok ? 'parses' : 'rejects'})`;
    else if (recovers) verdict = 'UNUSABLE (oracle recovers)';
    else if (!outcome.ok) verdict = 'UNUSABLE (native rejects)';
    else if (outcome.ambiguities?.length) verdict = 'UNUSABLE (ambiguous)';
    else verdict = JSON.stringify(oracleRows(source, 'Make')) === JSON.stringify(nativeRows(outcome.tree, source, options)) ? 'ok' : 'UNUSABLE (rows differ)';
    console.log(`${label} ${verdict} ${JSON.stringify(source)}`);
  }
}
