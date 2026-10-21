// Exact-source decorators for an isolated prototype; production files are read only.
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { DecoratorSet } from '../../js/src/decorators.js';
import { writeProjectData, checkProjectData } from '../../js/scripts/project-data.mjs';
import { rawBindingCertificatePolicy } from './generate-rules.mjs';
import { readReviewedOverlayProviderSources, restoreReviewedOverlayProviderSources, writeReviewedOverlayProviderPolicy } from '../rust-text-certificate-adapter/reviewed-providers.mjs';

const hash = source => createHash('sha256').update(source).digest('hex');
const modules = ['typed-program-links.js', 'typed-program-links-validation.js', 'typed-program-checked-projection.js', 'translation/emit-javascript.js', 'translation/emit-rust.js'];
export function rawBindingCertificateOverlayRecords({sourceCommit} = {}) {
  const options = { experiment: 'rust-raw-binding-certificate-adapter', modules };
  const { sources, revision } = sourceCommit === undefined ? readReviewedOverlayProviderSources(options) : restoreReviewedOverlayProviderSources({...options, sourceCommit});
  const changes = new Map([...sources].map(([module, source]) => [module, [{
    from: source.split('\n')[0],
    to: "import { readRustRawBindingCertificate, captureRustRawBindingCertificate, decorateRustRawBindingCertificateEmission, emitRustRawBindingCertificate } from '"
      + (module.startsWith('translation/') ? '../' : './') + "rust-raw-binding-certificate-adapter.js';\n" + source.split('\n')[0],
  }]]));
  const add = (module, from, suffix) => changes.get(module).push({ from, to: from + '\n' + suffix });
  add('typed-program-links.js', "  if (!canonical) throw new TypeError('typed linked source import currently reads JavaScript and Rust');",
    '  if (options.' + rawBindingCertificatePolicy.sourceOption + ' !== undefined) {\n'
    + '    const selected = captureRustRawBindingCertificate(source, canonical, options);\n'
    + '    return encodeTypedProgramLinks(selected.program, {provenance: selected.provenance});\n  }');
  add('typed-program-links-validation.js', 'export function validateLinkedTypedProgram(program, references = new WeakMap(), { graphPatterns = false, patternScopes = new WeakMap() } = {}) {',
    '  const rawBindingCertificate = readRustRawBindingCertificate(program);\n'
    + '  if (rawBindingCertificate) return validateLinkedTypedProgram(rawBindingCertificate.core, references, {graphPatterns, patternScopes});');
  add('typed-program-checked-projection.js', 'export function recheckLinkedTypedProgram(program, {wholeProgram=false}={}) {',
    '  const rawBindingCertificate = readRustRawBindingCertificate(program);\n'
    + '  if (rawBindingCertificate) { recheckLinkedTypedProgram(rawBindingCertificate.core, {wholeProgram:true}); return program; }');
  add('translation/emit-javascript.js', 'export function emitJavaScript(program, options = {}) {',
    '  const rawBindingCertificate = readRustRawBindingCertificate(program);\n'
    + '  if (rawBindingCertificate) return decorateRustRawBindingCertificateEmission(rawBindingCertificate, emitJavaScript(rawBindingCertificate.core, options), options);');
  add('translation/emit-rust.js', 'export function emitRust(program, options = {}) {',
    '  const rawBindingCertificate = readRustRawBindingCertificate(program);\n'
    + '  if (rawBindingCertificate) return emitRustRawBindingCertificate(rawBindingCertificate, options);');
  return [...sources].map(([module, source]) => {
    let current = source;
    for (const { from, to } of changes.get(module)) {
      if (current.split(from).length !== 2) throw new TypeError('an exact raw-binding overlay hook is missing or ambiguous: ' + module);
      current = current.replace(from, to);
    }
    const decorators = new DecoratorSet([{ id: 'raw-binding-certificate-' + module.replace(/[/.]/gu, '-'), level: 'translation-rule', order: 0,
      when: [['module', module]], actions: changes.get(module).map(({ from, to }) => ({ op: 'replace', field: 'source', from, to })),
    }]).toLino();
    const actual = DecoratorSet.fromLino(decorators).decorate('translation-rule', { module, source }).source;
    if (actual !== current) throw new TypeError('the linked raw-binding decorator differs from its exact recipe');
    return { module, sourceSha256: hash(source), overlaySha256: hash(actual), decorators, changes: changes.get(module).length, providerRevision: revision.sourceCommit };
  });
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  writeReviewedOverlayProviderPolicy(process.argv.includes('--check'));
  const file = new URL('./data/overlays-current.lino', import.meta.url), records = rawBindingCertificateOverlayRecords();
  if (process.argv.includes('--check')) checkProjectData(file, records); else writeProjectData(file, records);
  console.log('Raw binding certificate: five exact-source module decorator receipts');
}
