// Mechanical source capture. It neither executes declarations nor replays data.
import { readFileSync, realpathSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { relative, sep } from 'node:path';
import { composeSourceLibraryAliases } from './compose-source-library-aliases.mjs';
import { tokenize } from '../src/translation/lexer.js';
import {readJavaScriptDocumentationTags,testJavaScriptSourceFunctionStart} from '../src/translation/javascript.js';
import {readSourceLibraryDeclarationBindingKind,readSourceLibraryFunctionDeclarationOffset} from '../src/translation/source-library-binding-rules.js';
import {acceptSourceLibraryFunctionExport} from '../src/translation/source-library-export-rules.js';

const root = new URL('../../', import.meta.url);
const rootPath = realpathSync(fileURLToPath(root));
const digest = source => createHash('sha256').update(source).digest('hex');
const actual = (source, token, value) => token?.kind === 'identifier' && token.value === value
  && source.slice(token.start, token.end) === value;

export function readSourceLibraryModuleFacts(source,path,aliases=false) {
  const tokens = tokenize(source, 'JavaScript').tokens;
  const imports = [], exports = new Set(), declarations = new Set(), declarationBindings=new Map(), expressionHeads=new Set();
  function form(index){
    const word=at=>tokens[at]?.kind==='identifier'&&source.slice(tokens[at].start,tokens[at].end)===tokens[at].value?tokens[at].value:'';
    const offset=readSourceLibraryFunctionDeclarationOffset(word(index),word(index+1),/[\r\n\u2028\u2029]/u.test(source.slice(tokens[index]?.end??0,tokens[index+1]?.start??0)));
    const head=tokens[index+(offset<0?0:offset)],name=tokens[index+(offset<0?0:offset)+1];
    let callable=false,initializer='',expressionName='';
    if(actual(source,head,'const')&&tokens[index+2]?.kind==='punct'&&tokens[index+2].value==='='&&testJavaScriptSourceFunctionStart(tokens,index+3)){
      callable=true;let at=index+3;
      if(actual(source,tokens[at],'async')&&tokens[at+1]?.value!=='=>')at+=1;
      initializer=actual(source,tokens[at],'function')?'function expression':'arrow';
      if(initializer==='function expression'){expressionHeads.add(at);if(tokens[at+1]?.kind==='identifier')expressionName=tokens[at+1].value;}
    }
    const keyword=actual(source,head,'function')?'function':actual(source,head,'const')?'const':'';
    return {name:name?.value??'',nameKind:name?.kind??'',kind:readSourceLibraryDeclarationBindingKind(keyword,name?.kind??'',name?.value??'',callable),keyword,callable,...(initializer?{initializer}:{}),...(expressionName?{expressionName}:{})};
  }
  let depth = 0;
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index];
    if (token.kind === 'punct') {
      if (['{','(','['].includes(token.value)) depth += 1;
      else if (['}',')',']'].includes(token.value)) depth -= 1;
      continue;
    }
    if (depth !== 0) continue;
    if(!expressionHeads.has(index)&&(actual(source,token,'function')||actual(source,token,'const'))){
      const binding=form(index);
      if(binding.kind){if(declarations.has(binding.name))throw new TypeError('source closure needs unique actual module declarations at '+path+':'+token.start);declarations.add(binding.name);declarationBindings.set(binding.name,{name:binding.name,kind:binding.kind,...(binding.initializer?{initializer:binding.initializer}:{}),...(binding.expressionName?{expressionName:binding.expressionName}:{})});}
    }
    if (actual(source, token, 'export')) {
      const binding=form(index+1);
      if (!acceptSourceLibraryFunctionExport(binding.keyword,binding.nameKind,binding.name,binding.callable))
        throw new TypeError('source closure exports need actual named function declarations or constant function bindings at '+path+':'+token.start);
      exports.add(binding.name);
    }
    if (!actual(source, token, 'import')) continue;
    let at = index + 1;
    if (tokens[at]?.kind !== 'punct' || tokens[at].value !== '{')
      throw new TypeError('source closure imports need unaliased named functions at '+path+':'+token.start);
    at += 1;
    const names = [], bindings = [];
    while (tokens[at]?.kind === 'identifier') {
      const imported=tokens[at++].value;
      let local=imported;
      if(actual(source,tokens[at],'as')){
        if(!aliases)throw new TypeError('source closure aliases need their explicit CST composition profile at '+path+':'+token.start);
        at+=1;if(tokens[at]?.kind!=='identifier')throw new TypeError('a named source import alias needs its actual local identifier');
        local=tokens[at++].value;
      }
      names.push(imported);bindings.push({imported,local});
      if (tokens[at]?.kind === 'punct' && tokens[at].value === ',') { at += 1;continue; }
      break;
    }
    if (names.length === 0 || new Set(bindings.map(binding=>binding.local)).size !== bindings.length || tokens[at]?.kind !== 'punct' || tokens[at].value !== '}'
      || !actual(source,tokens[at + 1],'from') || tokens[at + 2]?.kind !== 'string')
      throw new TypeError('source closure import is outside its exact named-function profile at '+path+':'+token.start);
    const specifier = tokens[at + 2].value;
    if (!specifier.startsWith('./') && !specifier.startsWith('../')) throw new TypeError('source closure imports require actual relative files');
    if (!specifier.endsWith('.js') || specifier.includes('?') || specifier.includes('#')) throw new TypeError('source closure imports require explicit JavaScript files');
    at += 3;
    const end = tokens[at]?.kind === 'punct' && tokens[at].value === ';' ? tokens[at++].end : tokens[at - 1].end;
    imports.push({statement:source.slice(token.start,end),start:token.start,end,coordinateUnit:'UTF-16 code units',specifier,names,...(aliases?{bindings}:{})});
    index = at - 1;
  }
  const importedNames = new Set();
  for (const imported of imports) for (const name of imported.bindings?.map(binding=>binding.local)??imported.names) {
    if (importedNames.has(name) || declarations.has(name)) throw new TypeError('source closure import has a duplicate actual binding '+name+' in '+path);
    importedNames.add(name);
  }
  return {imports,exports:[...exports],declarations:[...declarations],...([...declarationBindings.values()].some(binding=>binding.kind==='const')?{declarationBindings:[...declarationBindings.values()]}:{})};
}

/** Capture complete named-function modules. The optional CST alias profile
 * keeps original sources and compiles actual lexical owners automatically. */
export function readSourceLibraryClosure(sourcePath,{aliases=false}={}) {
  if(typeof aliases!=='boolean')throw new TypeError('source alias composition needs an explicit Boolean');
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
    const {imports,exports,declarations,declarationBindings}=readSourceLibraryModuleFacts(source,path,aliases);
    const capture = {path,source,bytes:Buffer.byteLength(source),sourceSha256:digest(source),removedImports:imports,exports:[...exports],...(declarationBindings?{declarationBindings}:{})};
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
  const entry=visit(new URL(sourcePath,root));
  if(aliases){
    const compiled=composeSourceLibraryAliases(captures,entry.path,source=>readSourceLibraryModuleFacts(source,entry.path,true).imports);
    parts.splice(0,parts.length,...compiled);
    let offset=0;
    for(let index=0;index<captures.length;index+=1){captures[index].closureRange={start:offset,end:offset+parts[index].length,coordinateUnit:'UTF-16 code units'};offset+=parts[index].length+2;}
  }
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
    if (!input || !['fn','data'].includes(entry.k))
      throw new TypeError('source dependency needs a represented declaration owner: '+name);
    const declaredType=entry.k==='data'&&!entry.generated;
    const declarationSource=declaredType?closure.source.slice(entry.span.start,entry.span.end):undefined;
    if(declaredType&&(!declarationSource.startsWith('/**')||!readJavaScriptDocumentationTags({text:declarationSource,start:entry.span.start,end:entry.span.end}).some(tag=>tag.tag==='typedef'&&tag.name===name)))
      throw new TypeError('a source datatype needs its actual complete documentation declaration: '+name);
    if (!entry.generated && !declaredType && !(input.compiledDeclarations??input.declarations).includes(name))
      throw new TypeError('source dependency declaration has no actual original module owner: '+name);
    owners.set(name,input);
    const sourceName=input.compiledDeclarations&&!entry.generated?input.declarations[input.compiledDeclarations.indexOf(name)]:undefined;
    rows.push({name,path:input.path,generated:entry.generated===true,...(declaredType?{declarationSource,span:entry.span}:{}),...(sourceName!==undefined?{sourceName}:{})});
  }
  for (const [name,entry] of program.declarations) {
    const input = owners.get(name), imports = new Set(input.compiledImportNames??input.removedImports.flatMap(imported=>imported.names));
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
