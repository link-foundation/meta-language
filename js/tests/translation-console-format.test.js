import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';

const FORMAT = `const x = -0;
console.log('%s=%d items', 'n', 3n, x, true);
`;

const refusal = (source) => translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message;

test('console.log with several arguments is a semantic translation in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(FORMAT, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
});

test('the arguments print as util.format joins them: directives take arguments, the rest follow after spaces', () => {
  const lean = translateProgram(FORMAT, 'JavaScript', 'Lean').code;
  assert.ok(lean.includes('IO.println ((((((("n" ++ "=") ++ ((toString (3 : Int)) ++ "n")) ++ " items") ++ " ") ++ (ml_js_console x)) ++ " ") ++ (toString true))'), lean);
  const plain = translateProgram("console.log('100%%', 1n, '%s');\n", 'JavaScript', 'Lean').code;
  assert.ok(plain.includes('IO.println (((("100%" ++ " ") ++ ((toString (1 : Int)) ++ "n")) ++ " ") ++ "%s")'), plain);
});

test('the util.format forms that are not kept are refused with a reason', () => {
  assert.equal(refusal("const s = 'a';\nconsole.log(s, 1n);\n"), 'console.log with a computed first string: util.format reads the % directives of a first string argument, which only the run knows; pass a literal format string, or print one template literal at 27..28');
  assert.equal(refusal("console.log('%d', 'x');\n"), '%d of a string: %d prints a BigInt or a Number; the conversion Number(value) of other values is not kept at 18..21');
  assert.equal(refusal("console.log('%i', 1.5);\n"), '%i of a float: %i prints a BigInt; the conversion parseInt(value) of other values is not kept at 18..21');
  assert.equal(refusal("console.log('%j', 1n);\n"), 'console.log %j directive: the portable directives are %s, %d, %i, %c and %% at 12..16');
  assert.equal(refusal("const s = 'x';\nconsole.log('%c', s);\n"), '%c with a computed style: console.log discards the CSS a %c directive takes; pass it as a string literal at 33..34');
});
