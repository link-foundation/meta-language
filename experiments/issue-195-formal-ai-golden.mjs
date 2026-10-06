// Writes the committed Rust golden for the formal-ai regression fixtures from a
// Rust probe output file (the `rust-outputs.json` of the formal-ai workload CI
// artifact). Small sections are kept whole; documents and queries are stored as
// one SHA-256 digest of the canonical JSON per entry.
// Usage: node experiments/issue-195-formal-ai-golden.mjs <rust-outputs.json>
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { canonical } from '../js/scripts/issue-195-formal-ai-workload-probes.mjs';

const rust = JSON.parse(readFileSync(process.argv[2], 'utf8'));
const digest = (value) => createHash('sha256').update(canonical(value)).digest('hex');
const digests = (section) => Object.fromEntries(Object.keys(section).sort().map((key) => [key, digest(section[key])]));
const golden = {
  source: 'formal-ai d209aac6461b355f1a527831202af3423135f7e6, Rust probe of run 37014655368',
  whole: {
    ruleSet: rust.ruleSet,
    seedNetwork: rust.seedNetwork,
    linkEdits: rust.linkEdits,
    projections: rust.projections,
  },
  digests: { documents: digests(rust.documents), queries: digests(rust.queries) },
};
const target = new URL('../parity/fixtures/formal-ai-regressions/rust-golden.json', import.meta.url);
writeFileSync(target, `${JSON.stringify(golden, null, 2)}\n`);
console.log(`wrote ${target.pathname}`);
