// Shared authoring pipeline. It writes complete translated declarations and
// source records without fixtures, behavioral replays or native compilation.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableLibraryRelocation } from './record-rule-library.mjs';
import { readPortableSourceParameterStyles, partitionPortableRustLibrary } from './portable-source-library-style.mjs';
import { readPortableSourceLintStyles } from './portable-source-lint-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { writeProjectData } from './project-data.mjs';
import { encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { readSourceLibraryClosure, readSourceLibraryDeclarationOwnership } from './read-source-library-closure.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {projectSourceLibraryEntryPoints} from './source-library-entry-points.mjs';

const root = new URL('../../', import.meta.url);
export function draftSourceLibrary({ sourcePath, moduleName, directory = 'translation', model, programAsset, sourceDependencies = false, sourceAliases = false, referenceSemantics = false, sourceFormatting = 'rustfmt', sourceLints = 'apply', valueSharing='', programEntryPoints='' }) {
  if (!/^[a-z][a-z_0-9]*$/u.test(moduleName) || !['translation', ''].includes(directory)) throw new TypeError('invalid generated library location');
  if (typeof sourceDependencies !== 'boolean') throw new TypeError('source dependency capture needs an explicit Boolean');
  if(typeof sourceAliases!=='boolean'||sourceAliases&&!sourceDependencies)throw new TypeError('source alias composition requires explicit dependency capture');
  const sourceClosure = sourceDependencies ? readSourceLibraryClosure(sourcePath,{aliases:sourceAliases}) : null;
  const source = sourceClosure?.source ?? readFileSync(new URL(sourcePath, root), 'utf8');
  const sourceSyntax=parseAuthoredJavaScriptModule(source,sourcePath);
  if (typeof referenceSemantics !== 'boolean') throw new TypeError('source reference semantics must be an explicit Boolean');
  const program = checkProgram(parseJavaScript(source, { referenceSemantics }));
  const sourceDeclarationOwners = sourceClosure ? readSourceLibraryDeclarationOwnership(sourceClosure,program) : null;
  const emitted = emitRust(program, { arrayConsumption: true });
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  if (!header) throw new TypeError('the source translator omitted its library attributes');
  const relocation = readPortableLibraryRelocation([header, ...emitted.preludes, ...emitted.definitions].join('\n\n'));
  const names = [...relocation.source.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const loans = readPortableSourceParameterStyles(relocation.source, { publicNames: names });
  if (!['apply', 'preserve'].includes(sourceLints)) throw new TypeError('unknown source lint authoring selection');
  // Retain complete unstylized output when the existing style producer cannot
  // distinguish its generated contexts. Its guard and every CI gate stay exact;
  // authoring this source does not report the deferred style as repaired.
  const styles = sourceLints === 'preserve' ? { source: loans.source, actions: [] } : readPortableSourceLintStyles(loans.source);
  if (!['rustfmt', 'preserve'].includes(sourceFormatting)) throw new TypeError('unknown source formatting selection');
  let output;
  try { output = sourceFormatting === 'preserve' ? styles.source : formatProductRuleSource(styles.source); }
  catch (error) {
    const line = Number(String(error.message).match(/rules\.rs:(\d+)/u)?.[1]);
    if (Number.isSafeInteger(line) && line > 0) error.message += '\nSource formatting context:\n'
      + styles.source.split('\n').slice(Math.max(0, line - 3), line + 2)
        .map((text, index) => String(Math.max(1, line - 2) + index) + ': ' + text.slice(0, 400)).join('\n');
    throw error;
  }
  const files = partitionPortableRustLibrary(output, { directory: moduleName,
    notice: '// Generated from complete ' + sourcePath + '. Do not edit by hand.\n' });
  const base = new URL('rust/src/' + (directory ? directory + '/' : ''), root);
  for (const file of files) {
    const path = new URL(file.path ?? moduleName + '.rs', base);
    mkdirSync(new URL('.', path), { recursive: true });
    writeFileSync(path, file.source);
  }
  let programProjection;
  if (programAsset !== undefined) {
    if (!/^[a-z][a-z0-9-]*$/u.test(programAsset)) throw new TypeError('invalid typed source program asset');
    // Construct inspectable source IR. The encoder rechecks static typing;
    // nothing decodes/replays an archive or executes the represented program.
    if(programEntryPoints!==''&&programEntryPoints!=='source module exports 1')throw new TypeError('unknown source program entry-point selection');
    const selected=programEntryPoints?projectSourceLibraryEntryPoints(program,sourceClosure,sourcePath):{program};programProjection=selected.projection;
    const linked = encodeTypedProgramLinks(selected.program,{valueSharing,...(sourceAliases?{provenance:{sourceModel:model,sourceClosure,sourceDeclarationOwners}}:{})});
    const text = linked.network.toLino();
    writeFileSync(new URL('js/src/data/' + programAsset + '.lino', root), text);
    writeFileSync(new URL('rust/src/data/' + programAsset + '.lino', root), text);
  }
  const decorators = new DecoratorSet([{ id: moduleName.replaceAll('_', '-') + '-source-styles',
    level: 'emitter', order: 0, when: [['format', 'Rust']],
    actions: [...relocation.actions, ...loans.actions, ...styles.actions] }]);
  const stem = 'parity/self-translation/' + moduleName.replaceAll('_', '-');
  writeFileSync(new URL(stem + '-decorators.lino', root), decorators.toLino());
  const sourceSha256 = createHash('sha256').update(source).digest('hex');
  writeProjectData(new URL(stem + '-native.lino', root), {
    schemaVersion: 1, model, referenceSemantics, sourceFormatting, sourceSyntax, ...(programProjection?{programProjection}:{}), ...(valueSharing?{valueSharing}:{}), ...(sourceLints === 'preserve' ? { sourceLints } : {}), sourceClosure: sourceClosure ?? { source, bytes: Buffer.byteLength(source), sourceSha256,
      inputs: [{ path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256, removedImports: [] }] },
    declarations: [...program.declarations].map(([name, declaration]) => ({ name, kind: declaration.k })),
    ...(sourceDeclarationOwners ? {sourceDeclarationOwners} : {}),
    emitted: { text: emitted.text, definitions: emitted.definitions, preludes: emitted.preludes, mappings: emitted.mappings },
    files: files.map(file => ({ path: file.path ?? moduleName + '.rs', source: file.source })),
    boundary: 'Complete automatic source generation only. No fixture, behavior comparison, archive replay, product execution or native compilation.',
  });
  return { files, program, emitted };
}
