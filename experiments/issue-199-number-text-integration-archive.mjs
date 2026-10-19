// Close the draft's real relative import using parsed declarations before
// checking and encoding it. No source envelope, formatter callback or cached
// text output is stored in the resulting typed-program links.
import { readFileSync } from 'node:fs';
import { encodeTypedProgramLinks } from '../js/src/typed-program-links.js';
import { parseJavaScript } from '../js/src/translation/javascript.js';
import { checkProgram } from '../js/src/translation/check.js';

export function createNumberTextIntegrationArchive() {
  const sharedSource = readFileSync(new URL('./issue-199-decimal-number-rules.mjs', import.meta.url), 'utf8');
  const entrySource = readFileSync(new URL('./issue-199-number-text-integration-rules.mjs', import.meta.url), 'utf8');
  const sharedSurface = parseJavaScript(sharedSource, { referenceSemantics: false });
  const sharedProgram = checkProgram(sharedSurface);
  const entrySurface = parseJavaScript(entrySource, {
    referenceSemantics: false, moduleDirectory: ['experiments'],
    externals: [...sharedProgram.declarations.values()].filter(entry => entry.k === 'fn'),
  });
  const imports = entrySurface.imports ?? [];
  const requiredImports = ['acceptBinaryDoublePrecisionFormatModel', 'renderBinaryDoublePrecisionSpecial',
    'renderFiniteBinaryDoublePrecision'];
  if (imports.length !== 1 || imports[0].names.length !== 3
    || imports[0].module.join('/') !== 'experiments/issue-199-decimal-number-rules'
    || imports[0].names.some(({ imported, local }) => imported !== local || !requiredImports.includes(imported)
      || !sharedProgram.declarations.has(imported))
    || new Set(imports[0].names.map(({ imported }) => imported)).size !== requiredImports.length) {
    throw new TypeError('the number text integration draft requires its three owned decimal rule imports');
  }
  // The importer has resolved these declarations into the same closed module.
  // Rechecking without externals ensures every call owns its complete body.
  const program = checkProgram({
    language: 'JavaScript', referenceSemantics: false,
    items: [...sharedSurface.items, ...entrySurface.items], main: entrySurface.main,
  });
  return encodeTypedProgramLinks(program);
}
