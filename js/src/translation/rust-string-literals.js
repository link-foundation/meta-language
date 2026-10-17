// Internal standard Rust cooked-string rendering, shared by actual emitters.
export function rustEscape(text) {
  return [...text].map((char) => {
    const code = char.codePointAt(0);
    if (char === '"' || char === '\\') return `\\${char}`;
    if (char === '\n') return '\\n';
    if (char === '\r') return '\\r';
    if (char === '\t') return '\\t';
    if (code < 0x20 || code === 0x7f) return `\\u{${code.toString(16)}}`;
    return char;
  }).join('');
}

export function rustString(text) {
  return `"${rustEscape(text)}"`;
}
