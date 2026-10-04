import { readFileSync } from 'node:fs';
import { constructProgramFromFragments, ProgramRepresentation } from '../js/src/index.js';
const corpus = JSON.parse(readFileSync(new URL('../parity/fixtures/four-language-conformance.json', import.meta.url)));
for (const fixture of corpus.transformationPrograms) {
  const phantom = corpus.phantomImportCases.find((c) => c.language === fixture.language);
  const program = ProgramRepresentation.fromSnapshot(
    constructProgramFromFragments([fixture.first, fixture.second], fixture.language, { root: '/w', files: ['main'], dependencies: [] }).serializeSnapshot());
  console.log(fixture.language, 'diags before', program.diagnostics);
  const withImport = program.insert(0, phantom.source);
  console.log(' after insert', withImport.diagnostics, withImport.network.verifyFullMatch().isClean());
  console.log(' ids', withImport.querySyntax('identifier').map((r) => withImport.emit().slice(r.start, r.end)));
  const removed = withImport.delete({ start: 0, end: phantom.source.length });
  console.log(' after delete', removed.diagnostics, removed.emit() === fixture.source);
  console.log(' mappings', program.sourceMappings.length, program.sourceMappings.slice(0, 3));
}
