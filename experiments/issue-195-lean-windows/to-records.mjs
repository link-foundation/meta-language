// Converts an ISSUE_195_GENERATIVE_TRACE file into replay.c records: every
// incremental edit of a case, then fresh parses of the final text and of its
// prepend-blank-lines variant, as the Rust runtime pass does.
import { readFileSync, writeFileSync } from 'node:fs';

const [input, output] = process.argv.slice(2);
const chunks = [];
const u32 = (value) => { const b = Buffer.alloc(4); b.writeUInt32LE(value); return b; };
const text = (value) => { const b = Buffer.from(value, 'utf8'); return [u32(b.length), b]; };
const apply = (source, { start, end, replacement }) => {
  const bytes = Buffer.from(source, 'utf8');
  return Buffer.concat([bytes.subarray(0, start), Buffer.from(replacement, 'utf8'), bytes.subarray(end)]).toString('utf8');
};
for (const line of readFileSync(input, 'utf8').split('\n').filter(Boolean)) {
  const { base, edits } = JSON.parse(line);
  let source = base;
  chunks.push(u32(0), ...text(source));
  for (const edit of edits) {
    chunks.push(u32(1), ...text(source), u32(edit.start), u32(edit.end), ...text(edit.replacement));
    source = apply(source, edit);
  }
  chunks.push(u32(0), ...text(source), u32(0), ...text(`\n\n${source}`));
}
writeFileSync(output, Buffer.concat(chunks));
