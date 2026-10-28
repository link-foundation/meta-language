// Shared authoring pipeline. It writes complete translated declarations and
// source records without fixtures, behavioral replays or native compilation.
import { readFileSync, writeFileSync, mkdirSync, existsSync, lstatSync, realpathSync, unlinkSync } from 'node:fs';
import {fileURLToPath} from 'node:url';
import { createHash } from 'node:crypto';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableLibraryRelocation } from './record-rule-library.mjs';
import { readPortableSourceParameterStyles, partitionPortableRustLibrary } from './portable-source-library-style.mjs';
import { readPortableSourceLintStyles } from './portable-source-lint-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { readSourceLibraryClosure, readSourceLibraryDeclarationOwnership } from './read-source-library-closure.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {projectSourceLibraryEntryPoints} from './source-library-entry-points.mjs';
import {shareSourceBooleanConstructorMatches} from './source-boolean-match-sharing.mjs';

const root = new URL('../../', import.meta.url);
export function draftSourceLibrary({ sourcePath, moduleName, directory = 'translation', model, programAsset, sourceDependencies = false, sourceAliases = false, referenceSemantics = false, sourceFormatting = 'rustfmt', sourceLints = 'apply', valueSharing='', programEntryPoints='', nativeEntryPoints='', booleanMatchSharing='', recordShapes='' }) {
  if (!/^[a-z][a-z_0-9]*$/u.test(moduleName) || !['translation', ''].includes(directory)) throw new TypeError('invalid generated library location');
  if (typeof sourceDependencies !== 'boolean') throw new TypeError('source dependency capture needs an explicit Boolean');
  if(typeof sourceAliases!=='boolean'||sourceAliases&&!sourceDependencies)throw new TypeError('source alias composition requires explicit dependency capture');
  const sourceClosure = sourceDependencies ? readSourceLibraryClosure(sourcePath,{aliases:sourceAliases}) : null;
  const source = sourceClosure?.source ?? readFileSync(new URL(sourcePath, root), 'utf8');
  const sourceSyntax=parseAuthoredJavaScriptModule(source,sourcePath);
  if (typeof referenceSemantics !== 'boolean') throw new TypeError('source reference semantics must be an explicit Boolean');
  const originalProgram = checkProgram(parseJavaScript(source, { referenceSemantics }));
  const sourceDeclarationOwners = sourceClosure ? readSourceLibraryDeclarationOwnership(sourceClosure,originalProgram) : null;
  if(booleanMatchSharing!==''&&booleanMatchSharing!=='literal Boolean constructor match arms 1')throw new TypeError('unknown source Boolean match sharing selection');
  const sharedMatches=booleanMatchSharing?shareSourceBooleanConstructorMatches(originalProgram):undefined;
  const program=sharedMatches?.program??originalProgram;
  if(nativeEntryPoints!==''&&nativeEntryPoints!=='source module exports 1')throw new TypeError('unknown native source entry-point selection');
  const nativeSelection=nativeEntryPoints?projectSourceLibraryEntryPoints(program,sourceClosure,sourcePath):{program};
  const emitted = emitRust(nativeSelection.program, { arrayConsumption: true });
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
  const stem = 'parity/self-translation/' + moduleName.replaceAll('_', '-');
  const recipeFile=new URL(stem+'-native.lino',root),removedFiles=[];
  // A smaller selected dependency closure can reduce the partition count.
  // Remove only obsolete bytes exactly owned by the preceding generation;
  // preserve any missing, edited, symlinked or unregistered source for review.
  if(nativeEntryPoints&&existsSync(recipeFile)){
    const previous=readProjectData(recipeFile),retained=new Set(files.map(file=>file.path??moduleName+'.rs'));
    if(previous.schemaVersion!==1||previous.model!==model||!Array.isArray(previous.files)||previous.files.length>128)throw new TypeError('native projection needs its complete predecessor source receipt');
    for(const file of previous.files){
      if(typeof file.path!=='string'||retained.has(file.path))continue;
      if(!new RegExp('^'+moduleName+'/part_[0-9]+\\.rs$','u').test(file.path)||typeof file.source!=='string')throw new TypeError('obsolete generated source must retain its actual registered partition owner');
      const path=new URL(file.path,base);if(!existsSync(path))continue;
      const stat=lstatSync(path);if(!stat.isFile()||stat.isSymbolicLink()||realpathSync(path)!==fileURLToPath(path)||readFileSync(path,'utf8')!==file.source)throw new TypeError('preserve edited obsolete generated source for review: '+file.path);
      removedFiles.push({path:file.path,bytes:Buffer.byteLength(file.source),sha256:createHash('sha256').update(file.source).digest('hex')});
    }
    // Complete the ownership preflight before deleting any obsolete partition.
    for(const file of removedFiles)unlinkSync(new URL(file.path,base));
  }
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
    const linked = encodeTypedProgramLinks(selected.program,{valueSharing,recordShapes,...(sourceAliases?{provenance:{sourceModel:model,sourceClosure,sourceDeclarationOwners}}:{})});
    const text = linked.network.toLino();
    writeFileSync(new URL('js/src/data/' + programAsset + '.lino', root), text);
    writeFileSync(new URL('rust/src/data/' + programAsset + '.lino', root), text);
  }
  const decorators = new DecoratorSet([{ id: moduleName.replaceAll('_', '-') + '-source-styles',
    level: 'emitter', order: 0, when: [['format', 'Rust']],
    actions: [...relocation.actions, ...loans.actions, ...styles.actions] }]);
  writeFileSync(new URL(stem + '-decorators.lino', root), decorators.toLino());
  const sourceSha256 = createHash('sha256').update(source).digest('hex');
  writeProjectData(new URL(stem + '-native.lino', root), {
    schemaVersion: 1, model, referenceSemantics, sourceFormatting, sourceSyntax, ...(sharedMatches?{booleanMatchSharing:sharedMatches.proof}:{}), ...(programProjection?{programProjection}:{}), ...(nativeSelection.projection?{nativeProjection:nativeSelection.projection}:{}), ...(valueSharing?{valueSharing}:{}), ...(recordShapes?{recordShapes}:{}), ...(sourceLints === 'preserve' ? { sourceLints } : {}), sourceClosure: sourceClosure ?? { source, bytes: Buffer.byteLength(source), sourceSha256,
      inputs: [{ path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256, removedImports: [] }] },
    declarations: [...program.declarations].map(([name, declaration]) => ({ name, kind: declaration.k })),
    ...(sourceDeclarationOwners ? {sourceDeclarationOwners} : {}),
    emitted: { text: emitted.text, definitions: emitted.definitions, preludes: emitted.preludes, mappings: emitted.mappings },
    files: files.map(file => ({ path: file.path ?? moduleName + '.rs', source: file.source })),
    ...(removedFiles.length?{removedFiles}:{}),
    boundary: 'Complete automatic source generation only. No fixture, behavior comparison, archive replay, product execution or native compilation.',
  });
  return { files, program:nativeSelection.program, emitted };
}
