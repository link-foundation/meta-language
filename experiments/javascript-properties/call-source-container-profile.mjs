import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { readSourceLibraryClosure } from '../../js/scripts/read-source-library-closure.mjs';

const entry = 'js/src/translation/javascript-call-source.js';
const closure = readSourceLibraryClosure(entry, { aliases: true });
const surface = parseJavaScript(closure.source, { referenceSemantics: false });
const limit = 500_000;
const seen = new WeakSet();

function count(value, result, depth = 0) {
  if (!value || typeof value !== 'object' || seen.has(value)) return;
  if (depth > 256 || ++result.containers > limit) throw new RangeError('source container profile limit exceeded');
  seen.add(value);
  const kind = value.k ?? value.kind;
  if (typeof kind === 'string') result.kinds.set(kind, (result.kinds.get(kind) ?? 0) + 1);
  if (value instanceof Map) {
    for (const [key, child] of value) {
      count(key, result, depth + 1);
      count(child, result, depth + 1);
    }
  } else {
    if (Number.isSafeInteger(value.start) && Number.isSafeInteger(value.end) && Object.keys(value).length === 2) {
      result.spans++;
      result.coordinates.add(value.start + ':' + value.end);
    }
    for (const child of Object.values(value)) count(child, result, depth + 1);
  }
}

const result = { containers: 0, spans: 0, coordinates: new Set(), kinds: new Map() };
const declarations = [];
for (const item of surface.items) {
  const before = result.containers;
  count(item, result);
  declarations.push({ name: item.name, containers: result.containers - before });
}
count(surface, result);
console.log(JSON.stringify({
  sourceBytes: Buffer.byteLength(closure.source),
  declarations: declarations.length,
  containers: result.containers,
  spans: result.spans,
  uniqueSpans: result.coordinates.size,
  largest: declarations.sort((left, right) => right.containers - left.containers).slice(0, 12),
  kinds: [...result.kinds].sort((left, right) => right[1] - left[1]).slice(0, 12),
}, null, 2));

if (process.argv.includes('--native-definitions')) {
  const { checkProgram } = await import('../../js/src/translation/check.js');
  const { emitRust } = await import('../../js/src/translation/emit-rust.js');
  const { shareSourceClosedConstructorMatches } = await import('../../js/scripts/source-boolean-match-sharing.mjs');
  const { projectSourceLibraryEntryPoints } = await import('../../js/scripts/source-library-entry-points.mjs');
  const checked = checkProgram(surface);
  const shared = shareSourceClosedConstructorMatches(checked).program;
  const program = projectSourceLibraryEntryPoints(shared, closure, entry).program;
  const { definitions } = emitRust(program, { arrayConsumption: true });
  console.log(JSON.stringify(definitions.map(definition => ({
    name: definition.split('\n').find(line => line.startsWith('pub '))?.split('(')[0],
    lines: definition.split('\n').length,
  })).sort((left, right) => right.lines - left.lines).slice(0, 25), null, 2));
}
