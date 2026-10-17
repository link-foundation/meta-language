// Capture the complete portable adapter policy and generate its native library.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { acceptEmbeddedRegionLimit, readEmbeddedRegionDefaultLimit } from '../src/translation/embedded-region-rules.js';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { writeRecordRuleLibrary } from './record-rule-library.mjs';
import { readEmbeddedRegionNativeStyles } from './embedded-region-native-styles.mjs';
import { recursiveEmbeddedRegionCases } from '../tests/support/recursive-embedded-region-cases.js';

const sourceFile = new URL('../src/translation/embedded-region-rules.js', import.meta.url);
const policyFiles = [new URL('../src/translation/data/embedded-region-rules.lino', import.meta.url),
  new URL('../../rust/src/data/embedded-region-rules.lino', import.meta.url)];
const defaultsFile = new URL('../../rust/src/data/embedded-region-defaults.rs', import.meta.url);
const artifactFile = new URL('../../parity/self-translation/embedded-region-rules-native.lino', import.meta.url);
const fixtureFile = new URL('../../parity/fixtures/recursive-default-embedded-regions.lino', import.meta.url);
const digest = source => createHash('sha256').update(source).digest('hex');

export function embeddedRegionRulesPolicy() {
  const source = readFileSync(sourceFile, 'utf8');
  return { schemaVersion: 1, model: 'bounded recursive default CST attachment 1', source, sourceSha256: digest(source),
    defaults: { depthLimit: readEmbeddedRegionDefaultLimit('depth'),
      countLimit: readEmbeddedRegionDefaultLimit('count'), byteLimit: readEmbeddedRegionDefaultLimit('bytes') },
    primitiveBasis: {
      coordinates: 'Every planner bound is an absolute UTF-8 byte offset in the original host source. Child bounds stay inside their immediate host.',
      ancestry: 'A dense parent column names strictly earlier frames; canonical language identifiers and both byte bounds must agree for an ancestry match.',
      reuse: 'The current whole host is not parsed again, except the original first-level alias view when the requested root spelling differs. Nested already-attached views add no duplicate self-boundary. A more distant identical invocation reports a non-consuming cycle.',
      order: 'Hosts are processed in FIFO order; each original direct region is inserted before descendants and every descendant names its containing Region.',
      charges: 'Every admitted child parse attempt charges one region and its complete physical byte length. The root parse is excluded; overlapping child bytes are charged again for each parse.',
      diagnostics: 'A refused region retains its boundary and an error Relation with the reason, current counters and applicable observed value and limit.',
      budgets: 'Adapter depth, attempted region count and total child parse bytes are separate from the registered parsers own evaluation, recovery, memory and nesting budgets.',
      rendering: 'Overlapping source-token views of the same physical bytes render once; unspanned constructed fragments retain their original order.',
    },
    boundary: 'Default registered parser attachment only. This policy does not establish generic any/all grammar selection or proved global concept identity.',
  };
}

/** Constant DATA keeps the later native const configuration constructor intact. */
export function renderEmbeddedRegionNativeDefaults(defaults) {
  const fields = [['DEPTH', 'depthLimit'], ['COUNT', 'countLimit'], ['BYTE', 'byteLimit']];
  for (const [, field] of fields) {
    if (!acceptEmbeddedRegionLimit(defaults[field])) throw new TypeError('native adapter defaults require nonnegative safe integers');
  }
  return '// Generated DATA from the shared embedded-region-rules.js defaults. Do not edit.\n'
    + fields.map(([name, field]) => `pub const DEFAULT_EMBEDDED_REGION_${name}_LIMIT: usize = ${defaults[field]};`).join('\n') + '\n';
}

export async function generateEmbeddedRegionRules({ check = false, native = false } = {}) {
  const policy = embeddedRegionRulesPolicy();
  for (const file of policyFiles) {
    if (check) {
      if (!isDeepStrictEqual(readProjectData(file), policy)) throw new Error('embedded region source policy is stale');
    } else writeProjectData(file, policy);
  }
  const defaults = renderEmbeddedRegionNativeDefaults(policy.defaults);
  if (check) {
    if (readFileSync(defaultsFile, 'utf8') !== defaults) throw new Error('native embedded region default DATA is stale');
  } else writeFileSync(defaultsFile, defaults);
  const fixture = recursiveEmbeddedRegionCases();
  if (check) {
    if (!isDeepStrictEqual(readProjectData(fixtureFile), fixture)) throw new Error('authored recursive default region fixtures are stale');
  } else writeProjectData(fixtureFile, fixture);
  if (native) {
    const { parseJavaScript } = await import('../src/translation/javascript.js');
    const { checkProgram } = await import('../src/translation/check.js');
    const { emitRust } = await import('../src/translation/emit-rust.js');
    const emitted = emitRust(checkProgram(parseJavaScript(policy.source, { referenceSemantics: false })), { arrayConsumption: true });
    const nativeFiles = writeRecordRuleLibrary(emitted, 'embedded_region_rules', { check, sourceStyles: readEmbeddedRegionNativeStyles });
    const artifact = { schemaVersion: 1, model: policy.model, sourceSha256: policy.sourceSha256,
      assumptions: emitted.assumptions, encodings: emitted.encodings,
      emittedSource: emitted.text, emittedSourceSha256: digest(emitted.text),
      nativeDefaults: defaults, nativeDefaultsSha256: digest(defaults),
      nativeFiles: nativeFiles.map(file => ({ path: file.path ?? 'embedded_region_rules.rs',
        source: file.source, sha256: digest(file.source) })) };
    if (check) {
      if (!isDeepStrictEqual(readProjectData(artifactFile), artifact)) throw new Error('complete embedded region native translation is stale');
    } else writeProjectData(artifactFile, artifact);
  }
  return policy;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  await generateEmbeddedRegionRules({ check: process.argv.includes('--check'), native: process.argv.includes('--native') });
  console.log('embedded region source policy, default DATA and requested native translation match');
}
