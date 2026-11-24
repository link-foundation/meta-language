// Move exact checked type and span sharing ahead of the archive validators.
// Expression occurrences, bindings, and declaration owners stay distinct.
const typeFields = new Set(['type', 'domain', 'from', 'to', 'ret', 'semanticRet']);
const typeKinds = new Set(['nat', 'int', 'bool', 'string', 'unit', 'float', 'output', 'data',
  'array', 'option', 'set', 'cell', 'tuple', 'union', 'record', 'map', 'function']);

export function shareSourceCheckedValues(program) {
  const values = new Map(), identities = new WeakMap(), originals = new WeakMap();
  const visited = new WeakMap(), active = new WeakSet();
  let visits = 0;

  function bound(value, depth) {
    if (depth > 256 || ++visits > 1_000_000 || active.has(value)) {
      throw new TypeError('source value sharing exceeds its bounded acyclic input');
    }
  }

  function share(value, depth) {
    if (originals.has(value)) return originals.get(value);
    bound(value, depth);
    active.add(value);
    const entries = Object.entries(value).map(([name, field]) => [
      name, field && typeof field === 'object' ? share(field, depth + 1) : field,
    ]);
    const key = JSON.stringify([Array.isArray(value), entries.map(([name, field]) => [
      name, field && typeof field === 'object' ? ['record', identities.get(field)]
        : [typeof field, Object.is(field, -0) ? '-0' : String(field)],
    ])]);
    if (!values.has(key)) {
      const canonical = Array.isArray(value) ? entries.map(([, field]) => field) : Object.fromEntries(entries);
      identities.set(canonical, values.size);
      values.set(key, canonical);
    }
    const canonical = values.get(key);
    originals.set(value, canonical);
    active.delete(value);
    return canonical;
  }

  function walk(value, depth, field) {
    if (!value || typeof value !== 'object') return value;
    if (typeFields.has(field) && typeKinds.has(value.kind)) return share(value, depth);
    if (field === 'span' && Number.isSafeInteger(value.start) && Number.isSafeInteger(value.end) &&
        Object.keys(value).every(name => name === 'start' || name === 'end')) return share(value, depth);
    if (visited.has(value)) return visited.get(value);
    bound(value, depth);
    active.add(value);
    const entries = value instanceof Map ? [...value] : Object.entries(value);
    let changed = false;
    const mapped = entries.map(([name, child]) => {
      const replacement = walk(child, depth + 1, value instanceof Map ? '' : name);
      changed ||= replacement !== child;
      return [name, replacement];
    });
    const result = !changed ? value : value instanceof Map ? new Map(mapped)
      : Array.isArray(value) ? mapped.map(([, child]) => child) : Object.fromEntries(mapped);
    active.delete(value);
    visited.set(value, result);
    return result;
  }

  const shared = walk(program, 0, '');
  return {program: shared, uniqueValues: values.size};
}
