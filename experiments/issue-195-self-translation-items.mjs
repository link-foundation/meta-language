// Probes the top-level items of a module from the lossless native parse.
import { parseProgrammingLanguage } from '../js/src/programming-language-parser.js';
const cases = [
  ['JavaScript', "import x from 'y';\n\n/** doc */\nexport function a(b) {\n  return b;\n}\n\nconst c = 1;\n"],
  ['Rust', "use std::fmt;\n\n/// doc\npub fn a(b: u32) -> u32 {\n    b\n}\n"],
  ['TypeScript', "export function a(b: number): number {\n  return b;\n}\n"],
];
for (const [language, source] of cases) {
  const parsed = parseProgrammingLanguage(source, language);
  const t = parsed.tree;
  console.log(language, Object.keys(t), t.kind ?? t.term);
  for (const child of t.children ?? []) console.log('  ', child.kind ?? child.term, JSON.stringify(child.span ?? [child.start, child.end]));
}
