// CI authoring entry point. Materialize every tracked JavaScript, TypeScript
// and Rust source; translate products without compiling or executing them.
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync, lstatSync, realpathSync } from 'node:fs';
import { dirname, extname, resolve, posix } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import { tokenize } from '../src/translation/lexer.js';
import { draftProjectTranslation, projectTranslationDraftToLinks } from '../src/project-translation.js';

const languages = new Map([['.js', 'JavaScript'], ['.mjs', 'JavaScript'], ['.cjs', 'JavaScript'],
  ['.ts', 'TypeScript'], ['.tsx', 'TypeScript'], ['.rs', 'Rust']]);
const decoder = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true });
const hash = bytes => createHash('sha256').update(bytes).digest('hex');

function importSpecifiers(source) {
  const { tokens } = tokenize(source, 'JavaScript'), result = [];
  let braces = 0;
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token.kind === 'punct' && token.value === '{') braces += 1;
    else if (token.kind === 'punct' && token.value === '}') braces -= 1;
    if (braces !== 0 || token.kind !== 'identifier' || token.value !== 'import' || tokens[index + 1]?.value === '(' || tokens[index + 1]?.value === '.') continue;
    for (let at = index + 1; at < tokens.length; at += 1) {
      if (tokens[at].value === ';') break;
      if (tokens[at].kind === 'string') { result.push(tokens[at].value); break; }
    }
  }
  return [...new Set(result)];
}

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
  for (const input of inputs) {
    if (input.language === 'Rust') continue;
    let specifiers;
    try { specifiers = importSpecifiers(input.source); }
    catch (error) { unresolved.push({ identifier: input.identifier, reason: error.message, specifier: null }); continue; }
    for (const specifier of specifiers) {
      if (!specifier.startsWith('.')) {
        unresolved.push({ identifier: input.identifier, specifier, reason: 'external dependency requires an explicit source or runtime interpretation' });
        continue;
      }
      const base = posix.normalize(posix.join(posix.dirname(input.identifier), specifier));
      if (base === '..' || base.startsWith('../') || base.startsWith('/')) {
        unresolved.push({ identifier: input.identifier, specifier, reason: 'import escapes the supplied project' });
        continue;
      }
      const candidates = extname(base) ? [base] : [base, ...['.js', '.mjs', '.cjs', '.ts', '.tsx'].map(extension => base + extension),
        ...['index.js', 'index.mjs', 'index.ts'].map(name => posix.join(base, name))];
      const matches = candidates.filter(candidate => names.has(candidate));
      if (matches.length !== 1) {
        unresolved.push({ identifier: input.identifier, specifier, reason: matches.length ? 'ambiguous source resolution requires an explicit module policy' : 'import has no tracked source in the supplied project' });
      } else input.imports[specifier] = matches[0];
    }
  }
  return { root, revision, files, inputs, unresolved, totalBytes };
}

export function draftTrackedProjectTranslation(project, options) {
  const inventory = materializeProjectTranslationInputs(project, options);
  const draft = draftProjectTranslation(inventory.inputs, options);
  return { ...draft, project: { revision: inventory.revision, files: inventory.files,
    totalBytes: inventory.totalBytes, unresolved: inventory.unresolved } };
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
  const draft = draftTrackedProjectTranslation(options.project, { targetLanguage: options.to });
  const packet = projectTranslationDraftToLinks(draft);
  writeFileSync(resolve(options.out), packet.network.toLino());
}
