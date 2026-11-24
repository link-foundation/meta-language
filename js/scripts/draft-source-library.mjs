import {removeTrailingRustLayoutWhitespace} from './rust-layout-whitespace.mjs';
// Shared authoring pipeline. It writes complete translated declarations and
// source records without fixtures, behavioral replays or native compilation.
import { readFileSync, writeFileSync, mkdirSync, existsSync, lstatSync, realpathSync, unlinkSync } from 'node:fs';
import {fileURLToPath} from 'node:url';
import { createHash } from 'node:crypto';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram, checkSourceLibraryProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { readPortableLibraryRelocation } from './record-rule-library.mjs';
import { readPortableSourceParameterStyles, partitionPortableRustLibrary } from './portable-source-library-style.mjs';
import { readPortableSourceLintStyles } from './portable-source-lint-style.mjs';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { formatProductRuleItems } from './portable-source-library-format.mjs';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { encodeTypedProgramLinks } from '../src/typed-program-links.js';
import { readSourceLibraryClosure, readSourceLibraryDeclarationOwnership } from './read-source-library-closure.mjs';
import {parseAuthoredJavaScriptModule} from './source-module-syntax.mjs';
import {projectSourceLibraryEntryPoints} from './source-library-entry-points.mjs';
import {shareSourceConditionalConstructors} from './source-conditional-constructors.mjs';
import {directCallMatchSharingModel,nominalReferenceMatchSharingModel,readNominalReferenceMatchSharingInputs,shareSourceBooleanConstructorMatches,shareSourceClosedConstructorMatches} from './source-boolean-match-sharing.mjs';
import {shareSourceCheckedValues} from './source-checked-value-sharing.mjs';
import {readSourceLibraryClassProfileError,readSourceLibraryReExportProfileError} from '../src/translation/source-library-class-rules.js';
import {draftJavaScriptErrorSourceClosure} from './draft-javascript-error-source-closure.mjs';
import { readNativeSourceReceipt, writePartitionedNativeSourceReceipt } from './source-library-native-receipt.mjs';

const root = new URL('../../', import.meta.url);
export function draftSourceLibrary({ sourcePath, moduleName, directory = 'translation', model, programAsset, publishSourceReceipt=false, sourceDependencies = false, sourceAliases = false, sourceClasses='', sourceErrorClasses='', sourceReExports='', referenceSemantics = false, sourceFormatting = 'rustfmt', sourceLints = 'apply', valueSharing='', programEntryPoints='', nativeEntryPoints='', booleanMatchSharing='', constructorMatchSharing='', conditionalConstructorSharing='', recordShapes='', sourceOwnerChecking=false, partitionSourceReceipt=false }) {
  const traceEnabled = process.env.META_LANGUAGE_TRACE_SOURCE_AUTHOR === '1';
  const started = traceEnabled ? performance.now() : 0;
  const trace = stage => {
    if (!traceEnabled) return;
    const { heapUsed, rss } = process.memoryUsage();
    console.error(`source author ${sourcePath} ${stage} elapsed=${Math.round(performance.now() - started)}ms heap=${heapUsed}B rss=${rss}B`);
  };
  if(typeof publishSourceReceipt!=='boolean'||publishSourceReceipt&&(!sourceDependencies||programAsset===undefined||sourceErrorClasses!==''||programEntryPoints!==''||booleanMatchSharing!==''||constructorMatchSharing!==''||conditionalConstructorSharing!==''))throw new TypeError('published source receipts require a complete untransformed dependency program asset');
  if(sourceErrorClasses!==''){
    if(moduleName!==undefined||nativeEntryPoints!==''||programEntryPoints!=='')throw new TypeError('semantic Error source assets use their interpreted module entry ownership; an ordinary generated native library or export projection is not implied');
    return draftJavaScriptErrorSourceClosure({sourcePath,programAsset,sourceDependencies,sourceAliases,sourceClasses,sourceErrorClasses,sourceReExports});
  }
  if (!/^[a-z][a-z_0-9]*$/u.test(moduleName) || !['translation', ''].includes(directory)) throw new TypeError('invalid generated library location');
  if (typeof sourceDependencies !== 'boolean') throw new TypeError('source dependency capture needs an explicit Boolean');
  if(typeof sourceAliases!=='boolean'||sourceAliases&&!sourceDependencies)throw new TypeError('source alias composition requires explicit dependency capture');
  if (typeof sourceOwnerChecking !== 'boolean' || sourceOwnerChecking && (!sourceDependencies || sourceClasses || sourceErrorClasses)) {
    throw new TypeError('source owner checking requires original ordinary source modules');
  }
  if (typeof partitionSourceReceipt !== 'boolean' || partitionSourceReceipt && !nativeEntryPoints) {
    throw new TypeError('a partitioned native receipt requires selected actual source entry points');
  }
  const classError=readSourceLibraryClassProfileError(sourceClasses,sourceDependencies,sourceAliases,referenceSemantics);if(classError)throw new TypeError(classError);
  const reExportError=readSourceLibraryReExportProfileError(sourceReExports,sourceDependencies,sourceAliases);if(reExportError)throw new TypeError(reExportError);
  const sourceClosure = sourceDependencies ? readSourceLibraryClosure(sourcePath,{aliases:sourceAliases,classes:sourceClasses,reExports:sourceReExports}) : null;
  const source = sourceClosure?.source ?? readFileSync(new URL(sourcePath, root), 'utf8');
  const sourceSyntax=parseAuthoredJavaScriptModule(source,sourcePath);
  trace('read declarations');
  if (typeof referenceSemantics !== 'boolean') throw new TypeError('source reference semantics must be an explicit Boolean');
  let originalProgram, sourceDeclarationOwners;
  try {
    if (sourceOwnerChecking) {
      ({ program: originalProgram, owners: sourceDeclarationOwners } =
        checkSourceLibraryProgram(sourceClosure, { referenceSemantics, reportDeclarationErrors: traceEnabled }));
    } else {
      originalProgram = checkProgram(parseJavaScript(source, { referenceSemantics }));
      sourceDeclarationOwners = sourceClosure ? readSourceLibraryDeclarationOwnership(sourceClosure, originalProgram) : null;
    }
  } catch (error) {
    if (traceEnabled) {
      const diagnostics = error instanceof AggregateError ? error.errors : [error];
      for (const diagnostic of diagnostics) {
        const span = diagnostic?.span;
        if (!Number.isSafeInteger(span?.start) || !Number.isSafeInteger(span?.end)
          || span.start < 0 || span.end < span.start || span.end > source.length) continue;
        console.error(diagnostic.message, JSON.stringify(source.slice(Math.max(0, span.start - 80), Math.min(span.end + 80, span.start + 320))));
      }
    }
    throw error;
  }
  trace('checked declarations');
  if(booleanMatchSharing!==''&&booleanMatchSharing!=='literal Boolean constructor match arms 1')throw new TypeError('unknown source Boolean match sharing selection');
  if(constructorMatchSharing!==''&&!['closed constructor match sharing 1',directCallMatchSharingModel,nominalReferenceMatchSharingModel].includes(constructorMatchSharing)||constructorMatchSharing&&booleanMatchSharing)throw new TypeError('unknown or conflicting constructor match sharing selection');
  const shareDirectCalls=constructorMatchSharing===directCallMatchSharingModel;
  const sharedMatches=constructorMatchSharing?shareSourceClosedConstructorMatches(originalProgram,{shareDirectCalls,shareImmutableNominalReferences:constructorMatchSharing===nominalReferenceMatchSharingModel}):booleanMatchSharing?shareSourceBooleanConstructorMatches(originalProgram):undefined;
  if (constructorMatchSharing === nominalReferenceMatchSharingModel) {
    sharedMatches.proof.producer = { inputs: readNominalReferenceMatchSharingInputs() };
  }
  if(conditionalConstructorSharing!==''&&conditionalConstructorSharing!=='closed value constructor selection 1')throw new TypeError('unknown conditional constructor selection profile');
  const sharedConstructors=conditionalConstructorSharing?shareSourceConditionalConstructors(sharedMatches?.program??originalProgram):undefined;
  let program=sharedConstructors?.program??sharedMatches?.program??originalProgram;
  if (sourceOwnerChecking && valueSharing === 'exact scalars, types and source spans 1') {
    const shared = shareSourceCheckedValues(program);
    program = shared.program;
    globalThis.gc?.();
    trace('shared checked types and spans (' + shared.uniqueValues + ' unique records)');
  }
  if(nativeEntryPoints!==''&&nativeEntryPoints!=='source module exports 1')throw new TypeError('unknown native source entry-point selection');
  const nativeSelection=nativeEntryPoints?projectSourceLibraryEntryPoints(program,sourceClosure,sourcePath):{program};
  // Construct the complete linked model before costly native emission or any
  // output mutation. A source-model refusal leaves prior native files intact.
  let programProjection,programText;
  if (programAsset !== undefined) {
    if (!/^[a-z][a-z0-9-]*$/u.test(programAsset)) throw new TypeError('invalid typed source program asset');
    if(programEntryPoints!==''&&programEntryPoints!=='source module exports 1')throw new TypeError('unknown source program entry-point selection');
    const selected=programEntryPoints===nativeEntryPoints?nativeSelection:programEntryPoints?projectSourceLibraryEntryPoints(program,sourceClosure,sourcePath):{program};programProjection=selected.projection;
    const linked=encodeTypedProgramLinks(selected.program,{valueSharing,recordShapes,...(sourceOwnerChecking ? {sourceClosure,sourceEntryPath:sourcePath,trace:traceEnabled?trace:undefined} : {}),...(sourceAliases?{provenance:{sourceModel:model,sourceClosure,sourceDeclarationOwners}}:{})});
    if (traceEnabled) trace('encoded ' + linked.network.len() + ' typed links');
    programText=linked.network.toLino();
    if (traceEnabled) trace('rendered ' + Buffer.byteLength(programText) + ' typed link bytes');
  }
  globalThis.gc?.();
  let emitted;
  try {
    emitted = emitRust(nativeSelection.program, { arrayConsumption: true, ...(sourceOwnerChecking ? { sourceClosure } : {}) });
  } catch (error) {
    if (traceEnabled && error?.span) {
      const {start,end}=error.span;
      console.error('Native emission source:', JSON.stringify(source.slice(Math.max(0,start-160),Math.min(end+160,start+480))));
    }
    throw error;
  }
  trace('emitted Rust');
  if (sourceOwnerChecking) globalThis.gc?.();
  const header = emitted.text.match(/^#!\[allow\([^\r\n]*/mu)?.[0];
  if (!header) throw new TypeError('the source translator omitted its library attributes');
  const completeItems = [header, ...emitted.preludes, ...emitted.definitions];
  const relocation = readPortableLibraryRelocation(completeItems.join('\n\n'), sourceOwnerChecking ? completeItems : undefined);
  trace('relocated Rust library paths');
  const names = [...relocation.source.matchAll(/^pub fn (\w+)\(/gmu)].map(match => match[1]);
  const loans = readPortableSourceParameterStyles(relocation.source, { publicNames: names, completeItems: relocation.completeItems });
  trace('applied Rust parameter styles');
  if (!['apply', 'preserve'].includes(sourceLints)) throw new TypeError('unknown source lint authoring selection');
  // Retain complete unstylized output when the existing style producer cannot
  // distinguish its generated contexts. Its guard and every CI gate stay exact;
  // authoring this source does not report the deferred style as repaired.
  const styles = sourceLints === 'preserve' ? { source: loans.source, actions: [] } : readPortableSourceLintStyles(loans.source, {trace: traceEnabled ? trace : undefined});
  trace('selected Rust lint styles');
  if (!['rustfmt', 'preserve'].includes(sourceFormatting)) throw new TypeError('unknown source formatting selection');
  let output, outputItems;
  try {
    if (sourceLints === 'preserve' && loans.completeItems) {
      outputItems = sourceFormatting === 'preserve' ? loans.completeItems : formatProductRuleItems(loans.completeItems);
      output = outputItems.join('\n\n');
    } else {
      output = sourceFormatting === 'preserve' ? styles.source : formatProductRuleSource(removeTrailingRustLayoutWhitespace(styles.source));
    }
  }
  catch (error) {
    const line = Number(String(error.message).match(/rules\.rs:(\d+)/u)?.[1]);
    if (Number.isSafeInteger(line) && line > 0) error.message += '\nSource formatting context:\n'
      + styles.source.split('\n').slice(Math.max(0, line - 3), line + 2)
        .map((text, index) => String(Math.max(1, line - 2) + index) + ': ' + text.slice(0, 400)).join('\n');
    throw error;
  }
  const files = partitionPortableRustLibrary(output, { directory: moduleName,
    completeItems: outputItems,
    notice: '// Generated from complete ' + sourcePath + '. Do not edit by hand.\n' });
  trace('formatted and partitioned Rust');
  const base = new URL('rust/src/' + (directory ? directory + '/' : ''), root);
  const stem = 'parity/self-translation/' + moduleName.replaceAll('_', '-');
  const recipeFile=new URL(stem+'-native.lino',root),removedFiles=[];
  // A smaller selected dependency closure can reduce the partition count.
  // Remove only obsolete bytes exactly owned by the preceding generation;
  // preserve any missing, edited, symlinked or unregistered source for review.
  if(nativeEntryPoints&&existsSync(recipeFile)){
    const previous=readNativeSourceReceipt(recipeFile, ['files']),retained=new Set(files.map(file=>file.path??moduleName+'.rs'));
    if(previous.schemaVersion!==1||previous.model!==model||!Array.isArray(previous.files)||previous.files.length>(partitionSourceReceipt ? 333 : 128))throw new TypeError('native projection needs its complete predecessor source receipt');
    for(const file of previous.files){
      if(typeof file.path!=='string'||retained.has(file.path))continue;
      if(!new RegExp('^'+moduleName+'/(?:[a-z][a-z_0-9]*/)*[a-z][a-z_0-9]*\\.rs$','u').test(file.path)||typeof file.source!=='string')throw new TypeError('obsolete generated source must retain its actual registered partition owner');
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
  if (programAsset !== undefined) {
    writeFileSync(new URL('js/src/data/' + programAsset + '.lino', root), programText);
    writeFileSync(new URL('rust/src/data/' + programAsset + '.lino', root), programText);
  }
  const decoratorGroups = partitionSourceReceipt
    ? [relocation.actions, loans.actions, styles.actions].filter(actions => actions.length)
    : [[...relocation.actions, ...loans.actions, ...styles.actions]];
  const decoratorAssets = decoratorGroups.map((actions, index) => {
    const suffix = index === 0 ? '' : '-' + (index + 1);
    const decorators = new DecoratorSet([{ id: moduleName.replaceAll('_', '-') + '-source-styles' + suffix,
      level: 'emitter', order: index, when: [['format', 'Rust']], actions }]);
    const text = decorators.toLino();
    const bytes = Buffer.byteLength(text);
    if (partitionSourceReceipt && bytes > 72 * 1024 * 1024) throw new RangeError('an ordered source decorator asset exceeds its file bound');
    const path = stem + '-decorators' + suffix + '.lino';
    writeFileSync(new URL(path, root), text);
    return { path, bytes, sha256: createHash('sha256').update(text).digest('hex') };
  });
  const sourceSha256 = createHash('sha256').update(source).digest('hex');
  if(publishSourceReceipt){const sourceReceipt={
    model:'Complete source library asset receipt 1',schemaVersion:1,entryPath:sourcePath,referenceSemantics,
    sourceClosure,sourceDeclarationOwners,
    programAsset:{path:'js/src/data/'+programAsset+'.lino',bytes:Buffer.byteLength(programText),sha256:createHash('sha256').update(programText).digest('hex')},
  };for(const packagePath of ['js/src/data/','rust/src/data/'])writeProjectData(new URL(packagePath+programAsset+'-source.lino',root),sourceReceipt);}
  const writeNativeReceipt = partitionSourceReceipt ? writePartitionedNativeSourceReceipt : writeProjectData;
  writeNativeReceipt(recipeFile, {
    schemaVersion: 1, model, referenceSemantics, sourceFormatting, sourceSyntax, ...(sourceClasses?{sourceClasses}:{}), ...(sourceReExports?{sourceReExports}:{}), ...(sharedConstructors?{conditionalConstructorSharing:sharedConstructors.proof}:{}), ...(sharedMatches?{[constructorMatchSharing?'constructorMatchSharing':'booleanMatchSharing']:sharedMatches.proof}:{}), ...(programProjection?{programProjection}:{}), ...(nativeSelection.projection?{nativeProjection:nativeSelection.projection}:{}), ...(valueSharing?{valueSharing}:{}), ...(recordShapes?{recordShapes}:{}), ...(sourceLints === 'preserve' ? { sourceLints } : {}), sourceClosure: sourceClosure ?? { source, bytes: Buffer.byteLength(source), sourceSha256,
      inputs: [{ path: sourcePath, source, bytes: Buffer.byteLength(source), sourceSha256, removedImports: [] }] },
    declarations: [...program.declarations].map(([name, declaration]) => ({ name, kind: declaration.k })),
    ...(programAsset===undefined?{}:{programAsset:{path:'js/src/data/'+programAsset+'.lino',bytes:Buffer.byteLength(programText),sha256:createHash('sha256').update(programText).digest('hex')}}),
    ...(sourceDeclarationOwners ? {sourceDeclarationOwners} : {}),
    ...(sourceOwnerChecking ? {sourceOwnerChecking: 'original ordinary declaration owners 1'} : {}),
    ...(partitionSourceReceipt ? {decoratorAssets} : {}),
    emitted: { text: emitted.text, definitions: emitted.definitions, preludes: emitted.preludes, mappings: emitted.mappings, encodings: emitted.encodings??[] },
    files: files.map(file => ({ path: file.path ?? moduleName + '.rs', source: file.source })),
    ...(removedFiles.length?{removedFiles}:{}),
    boundary: 'Complete automatic source generation only. No fixture, behavior comparison, archive replay, product execution or native compilation.',
  });
  trace('published source files');
  return { files, program:nativeSelection.program, emitted };
}
