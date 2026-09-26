import { analyzeProgram } from '../js/src/program-representation.js';

const source = 'const x = 1; const object = { x: 2 }; object.x; x;\n';
const program = analyzeProgram(source, 'JavaScript');
const binding = program.bindings.find(({ name }) => name === 'x');
console.log(JSON.stringify({
  source,
  binding,
  renamed: program.renameBinding(binding.id, 'value').emit(),
  syntax: program.sourceMappings.filter(({ term }) => /identifier|property/u.test(term)),
}, null, 2));

for (const shorthandSource of [
  'const x = 1; const object = { x }; object.x; x;\n',
  'const object = { x: 1 }; const { x } = object; x;\n',
  'const object = { x: 1 }; const { x: local } = object; local;\n',
]) {
  const shorthandProgram = analyzeProgram(shorthandSource, 'JavaScript');
  console.log(JSON.stringify({
    source: shorthandSource,
    bindings: shorthandProgram.bindings,
    syntax: shorthandProgram.sourceMappings.filter(({ term }) => /identifier|shorthand|pair_pattern/u.test(term)),
  }, null, 2));
}
