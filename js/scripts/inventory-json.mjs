// Formats parity/language-grammar-inventory.json the way it is committed:
// top-level objects one key per line, array elements one per line, and each
// element (with everything nested in it) on a single line.
export function formatInventoryJson(value) {
  return `${block(value, '')}\n`;
}

function block(value, indent) {
  const inner = `${indent}  `;
  if (Array.isArray(value)) {
    const scalars = value.every((item) => item === null || typeof item !== 'object');
    if (scalars && inline(value).length <= 100) return inline(value);
    return `[\n${value.map((item) => `${inner}${inline(item)}`).join(',\n')}\n${indent}]`;
  }
  if (value && typeof value === 'object') {
    const lines = Object.entries(value).map(([key, item]) => `${inner}${JSON.stringify(key)}: ${block(item, inner)}`);
    return lines.length ? `{\n${lines.join(',\n')}\n${indent}}` : '{}';
  }
  return JSON.stringify(value);
}

function inline(value) {
  if (Array.isArray(value)) return `[${value.map(inline).join(', ')}]`;
  if (value && typeof value === 'object') {
    const entries = Object.entries(value).map(([key, item]) => `${JSON.stringify(key)}: ${inline(item)}`);
    return entries.length ? `{ ${entries.join(', ')} }` : '{}';
  }
  return JSON.stringify(value);
}
