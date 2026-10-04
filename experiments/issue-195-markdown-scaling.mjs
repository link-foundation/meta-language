// Times the JavaScript Markdown parse of the parse_scaling guard's document at
// growing sizes, to tell super-linear parsing from noise:
//   node experiments/issue-195-markdown-scaling.mjs [units...]
import { LinkNetwork } from '../js/src/index.js';

const unit = (index) => `## Section ${index}\n\nParagraph ${index} of the document.\n\n\`\`\`rust\npub fn item_${index}() -> usize { ${index} }\n\`\`\`\n\n`;
const sizes = process.argv.slice(2).map(Number);
for (const units of sizes.length ? sizes : [16, 128, 512]) {
  const source = Array.from({ length: units }, (_, index) => unit(index)).join('');
  let best = Infinity;
  for (let sample = 0; sample < 3; sample += 1) {
    const started = process.hrtime.bigint();
    const network = LinkNetwork.parse(source, 'Markdown');
    best = Math.min(best, Number(process.hrtime.bigint() - started));
    if (network.reconstructText() !== source) throw new Error('lossy parse');
  }
  const bytes = Buffer.byteLength(source);
  console.log(`${String(units).padStart(4)} units ${String(bytes).padStart(6)} bytes ${(best / bytes).toFixed(1).padStart(9)} ns/byte`);
}
