// Experimental source admission and links-to-links record lowering. Formatting
// remains the shared checked record rule program; this adapter supplies ordered
// values and retains each source field evaluation, including overwritten fields.
// Production checked IR currently erases source property order/console context,
// so this bounded draft lowers the parsed surface before that information is lost.
import { readFileSync } from 'node:fs';
import { tokenize } from '../js/src/translation/lexer.js';
import { parseJavaScript } from '../js/src/translation/javascript.js';
import { checkProgram } from '../js/src/translation/check.js';
import { encodeTypedProgramLinks } from '../js/src/typed-program-links.js';
import { readStructuredDataLino, structuredDataToLino } from '../js/src/structured-data-links.js';
import { prepareRecordTextDraft, recordTextModels } from './issue-199-record-text-draft.mjs';

const fail = message => { throw new TypeError(`record source draft: ${message}`); };
const string = value => ({ k: 'str', value });
const number = value => ({ k: 'num', value: String(value), type: { kind: 'float' } });
const variable = name => ({ k: 'name', path: [name] });
const array = items => ({ k: 'array', items: items.map(value => ({ value, spread: false })) });

// The current parser gives %s and ordinary console objects the same style,
// although Node uses different recursion depths for those contexts. Preserve
// an honest boundary: only console calls with zero or one argument are admitted.
function admitConsoleCalls(source) {
  const tokens = tokenize(source, 'JavaScript').tokens;
  for (let index = 0; index + 3 < tokens.length; index += 1) {
    if (tokens[index].value !== 'console' || tokens[index + 1].value !== '.'
      || tokens[index + 2].value !== 'log' || tokens[index + 3].value !== '(') continue;
    const stack = ['('];
    for (let position = index + 4; position < tokens.length; position += 1) {
      const token = tokens[position];
      if (token.kind !== 'punct') continue;
      if ('([{'.includes(token.value)) stack.push(token.value);
      else if (')]}'.includes(token.value)) {
        stack.pop(); if (!stack.length) break;
      } else if (token.value === ',' && stack.length === 1) fail('multiple console arguments require their distinct source formatting contexts');
    }
  }
}

/**
 * This admission proves only closed scalar literals or zero-argument functions
 * with bounded scalar returns and literal print effects. It never executes a
 * source function, erases its effects, or substitutes its precomputed output.
 */
function readScalarProof(node, declarations, active = new Set(), depth = 0) {
  if (!node || depth > 16) fail('scalar proof exceeds the bounded source model');
  if (node.k === 'str' || node.k === 'bool') return node.value;
  if (node.k === 'num') return node.type?.kind === 'int' || node.type?.kind === 'nat' ? BigInt(node.value) : Number(node.value);
  if (node.k === 'unary' && node.op === 'neg') {
    const value = readScalarProof(node.arg, declarations, active, depth + 1);
    if (typeof value !== 'number' && typeof value !== 'bigint') fail('negative scalar proof is not numeric');
    return -value;
  }
  if (node.k === 'print') {
    if (!['str', 'bool', 'num', 'unary'].includes(node.expr.k)) fail('a scalar-return proof has nonliteral print effects');
    prepareRecordTextDraft({ effect: readScalarProof(node.expr, declarations, active, depth + 1) });
    return readScalarProof(node.body, declarations, active, depth + 1);
  }
  const reference = node.k === 'app' && node.args.length === 0 ? node.fn : node.k === 'name' ? node : null;
  if (reference?.k === 'name' && reference.path.length === 2 && reference.path[0] === 'crate') {
    const declaration = declarations.get(reference.path[1]);
    if (!declaration || declaration.params.length || active.has(declaration)) fail('unknown, parameterized or recursive scalar source call');
    return readScalarProof(declaration.body, declarations, new Set(active).add(declaration), depth + 1);
  }
  fail('the scalar return requires an unsupported proof');
}

/** The result is a complete checked links program, including the rule algorithm. */
export function createRecordSourceTranslationDraft(source) {
  if (typeof source !== 'string' || Buffer.byteLength(source) > 16 * 1024) fail('source must be bounded JavaScript text');
  admitConsoleCalls(source);
  // Lower the reloaded inspectable syntax graph, rather than retaining the
  // original parser object or hiding source text in a link payload.
  const surface = readStructuredDataLino(structuredDataToLino(parseJavaScript(source, { referenceSemantics: false })),
    { byteLimit: 512 * 1024, nodeLimit: 10_000 });
  if (surface.items.length > 16 || surface.main?.effects.length > 32) fail('source declaration/effect bound');
  if (surface.items.some(item => item.k !== 'fn' || item.params.length)) fail('only zero-argument source functions are admitted');
  if (surface.imports?.length || surface.externals?.length) fail('external source dependencies require explicit proof');
  if (surface.main?.effects.some(effect => effect.k !== 'print')) fail('the draft requires direct console effects');
  const declarations = new Map(surface.items.map(item => [item.name, item]));
  const rules = parseJavaScript(readFileSync(new URL('./issue-199-record-text-rules.mjs', import.meta.url), 'utf8'), { referenceSemantics: false });
  const helperNames = new Set(rules.items.map(item => item.name));
  if (surface.items.some(item => helperNames.has(item.name) || ['console', 'String'].includes(item.name))) {
    fail('source declaration collides with a shared rule or selected intrinsic');
  }
  let fresh = 0, conversions = 0, visits = 0;

  function lowerRecord(record, model) {
    conversions += 1;
    if (conversions > 32) fail('source conversion bound');
    const bindings = [];
    function prepare(node, depth = 0) {
      if (++visits > 256 || node.k === 'record' && depth > 2) fail('record source shape bound');
      if (node.k !== 'record') {
        const proof = readScalarProof(node, declarations);
        const name = `ml_record_draft_value_${++fresh}`;
        bindings.push({ name, value: node });
        return { proof, binding: name };
      }
      if (node.prototypeDefinition) fail('source prototype-setter fields require a separate operational model');
      const selected = new Map();
      for (const entry of node.entries) {
        if (entry.spread || typeof entry.name !== 'string') fail('record spread or computed field needs a source order proof');
        if (entry.name === '__proto__') fail('source prototype-setter fields require their concrete computed-key distinction');
        selected.set(entry.name, prepare(entry.value, depth + 1));
      }
      const proof = {};
      for (const [key, value] of selected) Object.defineProperty(proof, key, { value: value.proof, enumerable: true, configurable: true });
      return { proof, children: selected };
    }
    const shape = prepare(record);
    const tape = prepareRecordTextDraft(shape.proof);
    const values = [];
    function visit(node) {
      if (node.children) {
        values.push(string(''));
        // Object.keys applies OrdinaryOwnPropertyKeys to the captured source
        // insertion order; all source evaluations have already been bound.
        for (const key of Object.keys(node.proof)) visit(node.children.get(key));
      } else {
        const arg = variable(node.binding);
        values.push(typeof node.proof === 'string' ? arg
          : typeof node.proof === 'number' ? { k: 'show', arg, style: 'js-console' }
            : { k: 'toString', arg });
      }
    }
    visit(shape);
    let result = { k: 'app', fn: { k: 'name', path: ['crate', 'renderRecordText'] },
      args: [array(tape.kinds.map(string)), array(tape.keys.map(string)), array(values), array(tape.ends.map(number)), string(model)] };
    // Bind every original field left to right before assembling the selected
    // property-order tape. Overwritten entries still execute exactly once.
    for (let index = bindings.length - 1; index >= 0; index -= 1) result = { k: 'let', ...bindings[index], body: result };
    return result;
  }

  function transform(node) {
    if (!node || typeof node !== 'object') return node;
    if (Array.isArray(node)) return node.map(transform);
    if (node.k === 'print' && node.expr?.k === 'record') {
      if (node.style !== 'js-console') fail('undeclared console record source model');
      return { ...node, expr: lowerRecord(node.expr, recordTextModels[1]), ...(node.body ? { body: transform(node.body) } : {}) };
    }
    if ((node.k === 'toString' || node.k === 'show') && node.arg?.k === 'record') {
      const model = node.k === 'toString' || node.style === 'js-template' ? recordTextModels[0]
        : node.style === 'js-console' || node.style === 'js-format-first' ? recordTextModels[1] : null;
      if (!model) fail('undeclared record conversion context');
      return lowerRecord(node.arg, model);
    }
    if (node.k === 'record') fail('record storage, aliasing and dynamic projections remain outside this draft');
    return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, transform(value)]));
  }

  const lowered = transform(surface);
  if (!conversions) fail('source contains no admitted record conversion');
  const combined = { ...lowered, items: [...rules.items, ...lowered.items] };
  return encodeTypedProgramLinks(checkProgram(combined));
}
