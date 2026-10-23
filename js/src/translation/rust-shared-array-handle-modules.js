// Whole-library publication transports actual checked source owners. It never
// substitutes a selected declaration for the complete Rust source context.
import {unsupported} from './diagnostics.js';
import {parseRust} from './rust.js';
import {checkProgram} from './check.js';
import {emitJavaScript} from './emit-javascript.js';
import {parseProgrammingLanguage} from '../programming-language-parser.js';

export function readRustSharedArrayHandleModule(source, model) {
  const program = checkProgram(parseRust(source, {sharedArrayHandleSourceModel: model}));
  const parsed = parseProgrammingLanguage(source, 'Rust');
  const fail = reason => { throw unsupported('Rust shared array handle module', reason); };
  if (program.main !== null) fail('whole executable entry points require an independent runtime composition proof');
  if (parsed.tree.flags.hasError) fail('the complete concrete syntax tree must have no errors');
  const bytes = Buffer.from(source, 'utf8');
  const text = node => bytes.subarray(node.span.byteRange.start, node.span.byteRange.end).toString('utf8');
  const functions = parsed.tree.children.filter(node => node.term === 'function_item');
  const owners = new Map();
  for (const node of functions) {
    const names = node.children.filter(child => child.field === 'name');
    if (names.length !== 1) fail('each concrete function must have one actual name');
    const name = text(names[0]);
    if (owners.has(name)) fail('each function name must have one complete source owner');
    const visibility = node.children.filter(child => child.term === 'visibility_modifier');
    if (visibility.length > 1) fail('each concrete function has at most one visibility modifier');
    owners.set(name, visibility.length === 1 && text(visibility[0]) === 'pub');
  }
  if (owners.size !== program.declarations.size) fail('every checked declaration must have its actual whole-module concrete owner');
  for (const [name, entry] of program.declarations) {
    if (entry.k !== 'fn' || entry.modulePath.length || entry.fullName !== name || !owners.has(name)) fail('only actual flat function owners are published');
  }
  const emitted = emitJavaScript(program);
  const targets = new Map();
  for (const [name, exported] of owners) {
    const mappings = emitted.mappings.filter(row => row.kind === 'function' && row.source === name);
    if (mappings.length !== 1 || targets.has(mappings[0].target)) fail('each actual checked function requires one distinct emitted target');
    targets.set(mappings[0].target, exported);
  }
  const published = new Set();
  const definitions = emitted.definitions.map(definition => {
    const match = /^function ([A-Za-z_$][A-Za-z0-9_$]*)\(/u.exec(definition);
    if (!match || !targets.has(match[1]) || published.has(match[1])) fail('each emitted definition must have its actual ordered source mapping');
    published.add(match[1]);
    return targets.get(match[1]) ? `export ${definition}` : definition;
  });
  if (published.size !== targets.size) fail('all checked function definitions must be published together');
  const signatures = [...program.declarations.values()].filter(entry => owners.get(entry.name)).map(entry => ({
    k: 'fn', name: entry.name, params: entry.params.map(({name, type}) => ({name, type})), ret: entry.ret,
  }));
  return {program, emitted: {...emitted, definitions}, signatures};
}
