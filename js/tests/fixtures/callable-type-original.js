// Archived development oracle: js/src/translation/javascript-bindings.js at 8a12e8d588e857e47d150ac43dc0bd8d33ac8a56
export function splitType(text, separator) {
  let depth = 0;
  let start = 0;
  const parts = [];
  for (let at = 0; at < text.length; at += 1) {
    if ('{[<('.includes(text[at])) depth += 1;
    if ('}]>)'.includes(text[at]) && !(text[at] === '>' && text[at - 1] === '=')) depth -= 1;
    if (!depth && text[at] === separator) { parts.push(text.slice(start, at).trim()); start = at + 1; }
  }
  parts.push(text.slice(start).trim());
  return parts;
}

export function readCompositeType(parser, name, range) {
  if (name.startsWith('(')) {
    let depth = 0;
    let close = -1;
    for (let at = 0; at < name.length; at += 1) {
      if (name[at] === '(') depth += 1;
      if (name[at] === ')' && --depth === 0) { close = at; break; }
    }
    if (close >= 0 && name.slice(close + 1).trimStart().startsWith('=>')) {
      const result = name.slice(close + 1).trimStart().slice(2).trim();
      if (!result) throw unsupported(`JSDoc type {${name}}`, 'a function type needs a return type', range);
      const params = splitType(name.slice(1, close), ',').filter(Boolean).map((part) => {
        const pieces = splitType(part, ':');
        return parser.type(pieces.length > 1 ? pieces.slice(1).join(':') : part, range);
      });
      return { kind: 'function', params, ret: parser.type(result, range) };
    }
  }
  const callable = /^function\s*\(([\s\S]*)\)\s*:\s*([\s\S]+)$/u.exec(name);
  if (callable) return { kind: 'function', params: splitType(callable[1], ',').filter(Boolean).map((part) => parser.type(part, range)), ret: parser.type(callable[2], range) };
  if (name.startsWith('(') && name.endsWith(')')) return parser.type(name.slice(1, -1), range);
  const alternatives = splitType(name, '|');
  if (alternatives.length > 1) {
    const missing = alternatives.filter((part) => part === 'null' || part === 'undefined');
    const values = alternatives.filter((part) => part !== 'null' && part !== 'undefined');
    if (missing.length && values.length === 1) return { kind: 'option', element: parser.type(values[0], range), absence: missing.length > 1 ? 'both' : missing[0] };
    return { kind: 'union', alternatives: alternatives.map((part) => parser.type(part, range)) };
  }
  if (name === 'null' || name === 'undefined') return { kind: 'absence', absence: name };
  if (name.startsWith('?')) return { kind: 'option', element: parser.type(name.slice(1), range), absence: 'null' };
  if (name.startsWith('[') && name.endsWith(']')) return { kind: 'tuple', elements: splitType(name.slice(1, -1), ',').filter(Boolean).map((part) => parser.type(part, range)) };
  if (name.startsWith('{') && name.endsWith('}')) {
    const fields = splitType(name.slice(1, -1), ',').filter(Boolean).map((part) => {
      const index = part.indexOf(':');
      if (index < 0) throw unsupported(`record type {${name}}`, 'every field has a name and a type', range);
      const field = part.slice(0, index).trim();
      const optional = field.endsWith('?');
      const type = parser.type(part.slice(index + 1), range);
      return { name: optional ? field.slice(0, -1) : field, type: optional ? { kind: 'option', element: type, absence: 'undefined' } : type };
    });
    return { kind: 'record', fields };
  }
  const generic = /^(Set|Map)<([\s\S]+)>$/u.exec(name);
  if (generic) {
    const arguments_ = splitType(generic[2], ',');
    if (generic[1] === 'Set' && arguments_.length === 1) return { kind: 'set', element: parser.type(arguments_[0], range) };
    if (generic[1] === 'Map' && arguments_.length === 2) return { kind: 'map', key: parser.type(arguments_[0], range), value: parser.type(arguments_[1], range) };
    throw unsupported(`JSDoc type {${name}}`, 'Set has one type argument and Map has two', range);
  }
  return null;
}
