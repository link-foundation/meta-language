// CI authoring entry point. Materialize every tracked JavaScript, TypeScript
// and Rust source; translate products without compiling or executing them.
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync, lstatSync, realpathSync } from 'node:fs';
import { dirname, extname, resolve, posix } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import {analyzeSourceModuleRequests} from '../src/source-module-requests.js';
import {createProjectTranslationArchive} from '../src/project-translation-archive.js';
import {metaLanguageArchiveToText} from '../src/meta-language-archive.js';
import { draftProjectTranslation } from '../src/project-translation.js';

const languages = new Map([['.js', 'JavaScript'], ['.mjs', 'JavaScript'], ['.cjs', 'JavaScript'], ['.jsx','JavaScript'],
  ['.ts', 'TypeScript'], ['.tsx', 'TypeScript'], ['.mts','TypeScript'], ['.cts','TypeScript'], ['.rs', 'Rust']]);
const decoder = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true });
const hash = bytes => createHash('sha256').update(bytes).digest('hex');

/** Full tracked-file inventory. No dependency clones, installs, generated
 * scratch directories or language compiler processes are created.
 */
export function materializeProjectTranslationInputs(project, { byteLimit = 128 * 1024 * 1024 } = {}) {
  const root = realpathSync(project);
  const git = args => execFileSync('git', ['-C', root, ...args], { encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
  const revision = git(['rev-parse', 'HEAD']).trim();
  const tracked = git(['ls-files', '--stage', '-z']).split('\0').filter(Boolean);
  const names = new Set(), inputs = [], files = [], unresolved = [];
  let totalBytes = 0;
  for (const row of tracked) {
    const boundary = row.indexOf('\t'), header = row.slice(0, boundary).split(' '), identifier = row.slice(boundary + 1);
    if (boundary < 0 || header.length !== 3 || header[2] !== '0') throw new TypeError('the project has an unresolved tracked index entry');
    const language = languages.get(extname(identifier));
    if (!language) continue;
    if (!['100644', '100755'].includes(header[0])) throw new TypeError(`source is not a tracked regular file: ${identifier}`);
    const path = resolve(root, identifier);
    if (!path.startsWith(root + '/') || !realpathSync(path).startsWith(root + '/')) throw new TypeError('source escapes its project');
    if (!lstatSync(path).isFile()) throw new TypeError(`source is not an ordinary file: ${identifier}`);
    const bytes = readFileSync(path);
    totalBytes += bytes.length;
    if (totalBytes > byteLimit) throw new TypeError('the complete project source inventory exceeds its byte budget');
    const source = decoder.decode(bytes);
    inputs.push({ identifier, language, source, imports: {}, options: {
      moduleDirectory: dirname(identifier) === '.' ? [] : dirname(identifier).split('/'),
    } });
    files.push({ identifier, indexObject: header[1], mode: header[0], bytes: bytes.length, sourceSha256: hash(bytes) });
    names.add(identifier);
  }
  const filesByName=new Map(files.map(file=>[file.identifier,file]));
  for (const input of inputs) {
    if (input.language === 'Rust') continue;
    let requests;
    try { requests=analyzeSourceModuleRequests(input.source,input.language,{tsx:extname(input.identifier)==='.tsx'}).requests; filesByName.get(input.identifier).moduleRequests=requests.map(({linkId,syntaxLinkId,...request})=>({...request,coordinateUnit:'UTF-8 bytes'})); }
    catch (error) { unresolved.push({ identifier: input.identifier, reason: error.message, specifier: null }); continue; }
    for (const request of requests) {
      const {specifier,kind,start,end}=request;
      if(specifier===null){unresolved.push({identifier:input.identifier,specifier:null,kind,start,end,reason:request.specifierUnits===null?'computed module specifier requires a source interpretation':'module request needs an exact scalar filesystem name'});continue;}
      if (!specifier.startsWith('.')) {
        unresolved.push({ identifier: input.identifier, specifier, kind, start, end, reason: 'external dependency requires an explicit source or runtime interpretation' });
        continue;
      }
      const base = posix.normalize(posix.join(posix.dirname(input.identifier), specifier));
      if (base === '..' || base.startsWith('../') || base.startsWith('/')) {
        unresolved.push({ identifier: input.identifier, specifier, kind, start, end, reason: 'import escapes the supplied project' });
        continue;
      }
      const candidates = extname(base) ? [base] : [base, ...['.js', '.mjs', '.cjs', '.jsx', '.ts', '.tsx', '.mts', '.cts'].map(extension => base + extension),
        ...['index.js', 'index.mjs', 'index.jsx', 'index.ts', 'index.tsx', 'index.mts', 'index.cts'].map(name => posix.join(base, name))];
      const matches = candidates.filter(candidate => names.has(candidate));
      if (matches.length !== 1) {
        unresolved.push({ identifier: input.identifier, specifier, kind, start, end, reason: matches.length ? 'ambiguous source resolution requires an explicit module policy' : 'import has no tracked source in the supplied project' });
      } else input.imports[specifier] = matches[0];
    }
  }
  const inputsByName=new Map(inputs.map(input=>[input.identifier,input]));
  for(const issue of unresolved){const input=inputsByName.get(issue.identifier);(input.sourceIssues??=[]).push(issue.reason);}
  return { root, revision, files, inputs, unresolved, totalBytes };
}

export function draftTrackedProjectTranslation(project, options) {
  const inventory = materializeProjectTranslationInputs(project, options);
  const draft = draftProjectTranslation(inventory.inputs, options);
  return { ...draft, project: { revision: inventory.revision, files: inventory.files,
    totalBytes: inventory.totalBytes, unresolved: inventory.unresolved } };
}

/** Complete original CSTs and the actual tracked inventory accompany drafts. */
export function draftTrackedProjectTranslationArchive(project,options={}){
 const inventory=materializeProjectTranslationInputs(project,options);
 return createProjectTranslationArchive(inventory.inputs,{...options,provenance:{user:options.provenance??null,trackedProject:{revision:inventory.revision,files:inventory.files,totalBytes:inventory.totalBytes,unresolved:inventory.unresolved}}});
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const options = {}, args = process.argv.slice(2);
  for (let index = 0; index < args.length; index += 1) {
    if (!['--project', '--to', '--out'].includes(args[index]) || index + 1 >= args.length) throw new TypeError('usage: draft-project-translation.mjs --project <directory> --to <language> --out <links-file>');
    const name = args[index].slice(2);
    if (Object.hasOwn(options, name)) throw new TypeError(`duplicate --${name}`);
    options[name] = args[++index];
  }
  if (!options.project || !options.to || !options.out) throw new TypeError('project, target language and Links output are required');
  const packet=draftTrackedProjectTranslationArchive(options.project,{targetLanguage:options.to});
  writeFileSync(resolve(options.out),metaLanguageArchiveToText(packet,{nodeLimit:1000000,referenceLimit:4000000,byteLimit:128*1024*1024}));
}
