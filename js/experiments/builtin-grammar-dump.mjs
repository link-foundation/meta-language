// Usage: node experiments/builtin-grammar-dump.mjs <language> <text>
// Prints the Syntax links (term, span, flags) of a parse for quick comparison
// with `cargo run --example issue_195_runtime_probe`.
import { LinkNetwork, LinkType } from '../src/index.js';

const [language, text] = process.argv.slice(2);
const network = LinkNetwork.parse(text.replace(/\\n/g, '\n'), language);
for (const link of network.links()) {
  const metadata = link.metadata();
  if (metadata.linkType !== LinkType.Syntax) continue;
  const { isError, hasError, isMissing, isExtra } = metadata.flags;
  const flags = [isError && 'E', hasError && 'H', isMissing && 'M', isExtra && 'X'].filter(Boolean).join('');
  const range = metadata.span?.byteRange;
  console.log(`${metadata.term} [${range?.start},${range?.end}] ${flags}`);
}
