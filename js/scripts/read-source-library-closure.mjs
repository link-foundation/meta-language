// Mechanical source capture. It neither executes declarations nor replays data.
import { readFileSync, realpathSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { relative, sep } from 'node:path';
import { tokenize } from '../src/translation/lexer.js';

const root = new URL('../../', import.meta.url);
const rootPath = realpathSync(fileURLToPath(root));
const digest = source => createHash('sha256').update(source).digest('hex');
const actual = (source, token, value) => token?.kind === 'identifier' && token.value === value
  && source.slice(token.start, token.end) === value;

/** Capture complete modules with exact unaliased local named-function imports.
 * Duplicate declarations, unknown exports and unsupported imports refuse. This
 * closed authoring profile never substitutes names or dependency semantics. */
export function readSourceLibraryClosure(sourcePath) {
  const states = new Map(), captures = [], parts = [];
  let bytes = 0;
  function visit(url) {
    const absolute = realpathSync(fileURLToPath(url));
    url = pathToFileURL(absolute);
    const path = relative(rootPath, absolute).split(sep).join('/');
    if (path === '..' || path.startsWith('../')) throw new TypeError('source library dependencies must remain in the actual project');
    const previous = states.get(absolute);
    if (previous) return previous;
    if (states.size >= 128) throw new RangeError('source library closure exceeds 128 actual modules');
    const source = readFileSync(absolute, 'utf8');
    bytes += Buffer.byteLength(source);
    if (bytes > 16 * 1024 * 1024) throw new RangeError('source library closure exceeds 16 MiB');
    const tokens = tokenize(source, 'JavaScript').tokens;
    const imports = [], exports = new Set(), declarations = new Set();
    let depth = 0;
    for (let index = 0; index < tokens.length; index += 1) {
      const token = tokens[index];
      if (token.kind === 'punct') {
        if (token.value === '{') depth += 1;
        else if (token.value === '}') depth -= 1;
        continue;
      }
      if (depth !== 0) continue;
      if (actual(source, token, 'function') && tokens[index + 1]?.kind === 'identifier') declarations.add(tokens[index + 1].value);
      if (actual(source, token, 'export')) {
        if (!actual(source, tokens[index + 1], 'function') || tokens[index + 2]?.kind !== 'identifier')
          throw new TypeError('source closure exports need actual named function declarations at '+path+':'+token.start);
        exports.add(tokens[index + 2].value);
      }
      if (!actual(source, token, 'import')) continue;
      let at = index + 1;
      if (tokens[at]?.kind !== 'punct' || tokens[at].value !== '{')
        throw new TypeError('source closure imports need unaliased named functions at '+path+':'+token.start);
      at += 1;
      const names = [];
      while (tokens[at]?.kind === 'identifier') {
        names.push(tokens[at].value);at += 1;
        if (tokens[at]?.kind === 'punct' && tokens[at].value === ',') { at += 1;continue; }
        break;
      }
      if (names.length === 0 || new Set(names).size !== names.length || tokens[at]?.kind !== 'punct' || tokens[at].value !== '}'
        || !actual(source,tokens[at + 1],'from') || tokens[at + 2]?.kind !== 'string')
        throw new TypeError('source closure import is outside its exact named-function profile at '+path+':'+token.start);
      const specifier = tokens[at + 2].value;
      if (!specifier.startsWith('./') && !specifier.startsWith('../')) throw new TypeError('source closure imports require actual relative files');
      if (!specifier.endsWith('.js') || specifier.includes('?') || specifier.includes('#')) throw new TypeError('source closure imports require explicit JavaScript files');
      at += 3;
      const end = tokens[at]?.kind === 'punct' && tokens[at].value === ';' ? tokens[at++].end : tokens[at - 1].end;
      imports.push({statement:source.slice(token.start,end),start:token.start,end,coordinateUnit:'UTF-16 code units',specifier,names});
      index = at - 1;
    }
    const importedNames = new Set();
    for (const imported of imports) for (const name of imported.names) {
      if (importedNames.has(name) || declarations.has(name)) throw new TypeError('source closure import has a duplicate actual binding '+name+' in '+path);
      importedNames.add(name);
    }
    const capture = {path,source,bytes:Buffer.byteLength(source),sourceSha256:digest(source),removedImports:imports,exports:[...exports]};
    states.set(absolute,capture);
    for (const imported of imports) {
      const dependency = visit(new URL(imported.specifier,url));
      for (const name of imported.names) if (!dependency.exports.includes(name))
        throw new TypeError('source closure import '+name+' has no actual exported owner in '+dependency.path);
      imported.dependency = dependency.path;
    }
    let body = source;
    for (const imported of imports.toReversed()) body = body.slice(0,imported.start)+body.slice(imported.end);
    const start = parts.reduce((length,part)=>length+part.length+2,0);
    capture.closureRange = {start,end:start+body.length,coordinateUnit:'UTF-16 code units'};
    capture.declarations = [...declarations];
    captures.push(capture);parts.push(body);
    return capture;
  }
  visit(new URL(sourcePath,root));
  const source = parts.join('\n\n');
  return {source,bytes:Buffer.byteLength(source),sourceSha256:digest(source),inputs:captures};
}

/** Retain actual complete definitions for ownership/effect analysis, while
 * excluding accidental cross-module capture introduced by concatenation. */
export function readSourceLibraryDeclarationOwnership(closure,program) {
  if (program.main?.effects.length) throw new TypeError('source dependency closure needs declaration-only modules');
  const owners = new Map(), rows = [];
  for (const [name,entry] of program.declarations) {
    const input = closure.inputs.find(input=>Number.isSafeInteger(entry.span?.start)
      && entry.span.start >= input.closureRange.start && entry.span.start < input.closureRange.end);
    if (!input || !['fn','data'].includes(entry.k) || entry.k === 'data' && !entry.generated)
      throw new TypeError('source dependency needs a represented declaration owner: '+name);
    if (!entry.generated && !input.declarations.includes(name))
      throw new TypeError('source dependency declaration has no actual original module owner: '+name);
    owners.set(name,input);
    rows.push({name,path:input.path,generated:entry.generated===true});
  }
  for (const [name,entry] of program.declarations) {
    const input = owners.get(name), imports = new Set(input.removedImports.flatMap(imported=>imported.names));
    const pending = [entry], seen = new Set();
    while (pending.length) {
      const value = pending.pop();
      if (!value || typeof value !== 'object' || seen.has(value)) continue;
      seen.add(value);
      if (value.k === 'call' && typeof value.fn === 'string' && owners.get(value.fn) !== input && !imports.has(value.fn))
        throw new TypeError('source dependency cannot capture '+value.fn+' from another module in '+input.path);
      for (const child of Array.isArray(value) ? value : Object.values(value)) pending.push(child);
    }
  }
  return rows;
}
