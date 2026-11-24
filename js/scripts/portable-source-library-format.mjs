import { formatProductRuleSource } from './product-rule-style-corrections.mjs';
import { removeTrailingRustLayoutWhitespace } from './rust-layout-whitespace.mjs';

/** Format complete library items in bounded groups without splitting a body. */
export function formatProductRuleItems(items) {
  const maximumGroupBytes = 512 * 1024;
  const boundary = '// Separate complete generated Rust items';
  const separator = `\n\n${boundary}\n\n`;
  const formatted = [];
  let group = [], groupBytes = 0;

  function flush() {
    if (group.length === 0) return;
    const output = formatProductRuleSource(group.join(separator));
    const members = output.split(boundary);
    if (members.length !== group.length) throw new TypeError('Rust formatting changed a complete item boundary');
    formatted.push(...members.map(item => item.trim()));
    group = [];
    groupBytes = 0;
  }

  for (const source of items) {
    if (source.includes(boundary)) throw new TypeError('Rust source contains the formatter item boundary');
    const item = removeTrailingRustLayoutWhitespace(source);
    const bytes = Buffer.byteLength(item) + Buffer.byteLength(separator);
    if (groupBytes + bytes > maximumGroupBytes) flush();
    group.push(item);
    groupBytes += bytes;
  }
  flush();
  return formatted;
}
